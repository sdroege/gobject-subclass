// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc::c_char;
use std::os::raw::c_void;
use std::ffi::CString;

use std::panic::{self, AssertUnwindSafe};

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use std::u32;
use std::u64;

use error::*;

use glib_ffi;
use gst_ffi;

use glib;
use glib::translate::*;
use gst;

pub type StreamIndex = u32;

#[derive(Debug)]
pub enum SeekResult {
    TooEarly,
    Ok(u64),
    Eos,
}

#[derive(Debug)]
pub enum HandleBufferResult {
    NeedMoreData,
    Again,
    // NeedDataFromOffset(u64),
    StreamAdded(Stream),
    HaveAllStreams,
    StreamChanged(Stream),
    // StreamsAdded(Vec<Stream>), // Implies HaveAllStreams
    StreamsChanged(Vec<Stream>),
    // TODO need something to replace/add new streams
    // TODO should probably directly implement the GstStreams new world order
    BufferForStream(StreamIndex, gst::Buffer),
    Eos(Option<StreamIndex>),
}

pub trait Demuxer {
    fn start(
        &mut self,
        upstream_size: Option<u64>,
        random_access: bool,
    ) -> Result<(), ErrorMessage>;
    fn stop(&mut self) -> Result<(), ErrorMessage>;

    fn seek(&mut self, start: u64, stop: Option<u64>) -> Result<SeekResult, ErrorMessage>;
    fn handle_buffer(
        &mut self,
        buffer: Option<gst::Buffer>,
    ) -> Result<HandleBufferResult, FlowError>;
    fn end_of_stream(&mut self) -> Result<(), ErrorMessage>;

    fn is_seekable(&self) -> bool;
    fn get_position(&self) -> Option<u64>;
    fn get_duration(&self) -> Option<u64>;
}

#[derive(Debug)]
pub struct Stream {
    pub index: StreamIndex,
    pub caps: gst::Caps,
    pub stream_id: String,
}

impl Stream {
    pub fn new(index: StreamIndex, caps: gst::Caps, stream_id: String) -> Stream {
        Stream {
            index: index,
            caps: caps,
            stream_id: stream_id,
        }
    }
}

pub struct DemuxerWrapper {
    raw: *mut gst_ffi::GstElement,
    cat: gst::DebugCategory,
    demuxer: Mutex<Box<Demuxer>>,
    panicked: AtomicBool,
}

impl DemuxerWrapper {
    fn new(raw: *mut gst_ffi::GstElement, demuxer: Box<Demuxer>) -> DemuxerWrapper {
        DemuxerWrapper {
            raw: raw,
            cat: gst::DebugCategory::new(
                "rsdemux",
                gst::DebugColorFlags::empty(),
                "Rust demuxer base class",
            ),
            demuxer: Mutex::new(demuxer),
            panicked: AtomicBool::new(false),
        }
    }

    fn start(&self, upstream_size: u64, random_access: bool) -> bool {
        let demuxer = &mut self.demuxer.lock().unwrap();

        gst_debug!(
            self.cat,
            // TODO obj: demuxer
            "Starting with upstream size {} and random access {}",
            upstream_size,
            random_access
        );

        let upstream_size = if upstream_size == u64::MAX {
            None
        } else {
            Some(upstream_size)
        };

        match demuxer.start(upstream_size, random_access) {
            Ok(..) => {
                gst_trace!(
                    self.cat,
                    /* TODO obj: demuxer,*/ "Successfully started"
                );
                true
            }
            Err(ref msg) => {
                gst_error!(
                    self.cat,
                    /* TODO obj: demuxer,*/ "Failed to start: {:?}",
                    msg
                );
                self.post_message(msg);
                false
            }
        }
    }
    fn stop(&self) -> bool {
        let demuxer = &mut self.demuxer.lock().unwrap();

        gst_debug!(self.cat, /* TODO obj: demuxer,*/ "Stopping");

        match demuxer.stop() {
            Ok(..) => {
                gst_trace!(self.cat, /* TODO obj: demuxer,*/ "Successfully stop");
                true
            }
            Err(ref msg) => {
                gst_error!(
                    self.cat,
                    /* TODO obj: demuxer,*/ "Failed to stop: {:?}",
                    msg
                );
                self.post_message(msg);
                false
            }
        }
    }

    fn is_seekable(&self) -> bool {
        let demuxer = &self.demuxer.lock().unwrap();

        let seekable = demuxer.is_seekable();
        gst_debug!(
            self.cat,
            /* TODO obj: demuxer,*/ "Seekable {}",
            seekable
        );

        seekable
    }


    fn get_position(&self, position: &mut u64) -> glib_ffi::gboolean {
        let demuxer = &self.demuxer.lock().unwrap();

        match demuxer.get_position() {
            None => {
                gst_trace!(self.cat, /* TODO obj: demuxer,*/ "Got no position");
                *position = u64::MAX;
                glib_ffi::GFALSE
            }
            Some(pos) => {
                gst_trace!(
                    self.cat,
                    /* TODO obj: demuxer,*/ "Returning position {}",
                    pos
                );
                *position = pos;
                glib_ffi::GTRUE
            }
        }
    }

    fn get_duration(&self, duration: &mut u64) -> glib_ffi::gboolean {
        let demuxer = &self.demuxer.lock().unwrap();

        match demuxer.get_duration() {
            None => {
                gst_trace!(self.cat, /* TODO obj: demuxer,*/ "Got no duration");
                *duration = u64::MAX;
                glib_ffi::GFALSE
            }
            Some(dur) => {
                gst_trace!(
                    self.cat,
                    /* TODO obj: demuxer,*/ "Returning duration {}",
                    dur
                );
                *duration = dur;
                glib_ffi::GTRUE
            }
        }
    }

    fn seek(&self, start: u64, stop: u64, offset: &mut u64) -> bool {
        extern "C" {
            fn gst_rs_demuxer_stream_eos(raw: *mut gst_ffi::GstElement, index: u32);
        };

        let stop = if stop == u64::MAX { None } else { Some(stop) };

        gst_debug!(
            self.cat,
            /* TODO obj: demuxer,*/ "Seeking to {:?}-{:?}",
            start,
            stop
        );

        let res = {
            let mut demuxer = &mut self.demuxer.lock().unwrap();

            match demuxer.seek(start, stop) {
                Ok(res) => res,
                Err(ref msg) => {
                    gst_error!(
                        self.cat,
                        /* TODO obj: demuxer,*/ "Failed to seek: {:?}",
                        msg
                    );
                    self.post_message(msg);
                    return false;
                }
            }
        };

        match res {
            SeekResult::TooEarly => {
                gst_debug!(self.cat, /* TODO obj: demuxer,*/ "Seeked too early");
                false
            }
            SeekResult::Ok(off) => {
                gst_trace!(self.cat, /* TODO obj: demuxer,*/ "Seeked successfully");
                *offset = off;
                true
            }
            SeekResult::Eos => {
                gst_debug!(self.cat, /* TODO obj: demuxer,*/ "Seeked after EOS");
                *offset = u64::MAX;

                unsafe {
                    gst_rs_demuxer_stream_eos(self.raw, u32::MAX);
                }

                true
            }
        }
    }

    fn handle_buffer(&self, buffer: gst::Buffer) -> gst::FlowReturn {
        extern "C" {
            fn gst_rs_demuxer_stream_eos(raw: *mut gst_ffi::GstElement, index: u32);
            fn gst_rs_demuxer_add_stream(
                raw: *mut gst_ffi::GstElement,
                index: u32,
                caps: *const gst_ffi::GstCaps,
                stream_id: *const c_char,
            );
            fn gst_rs_demuxer_added_all_streams(raw: *mut gst_ffi::GstElement);
            // fn gst_rs_demuxer_remove_all_streams(raw: *mut gst_ffi::GstElement);
            fn gst_rs_demuxer_stream_format_changed(
                raw: *mut gst_ffi::GstElement,
                index: u32,
                caps: *const gst_ffi::GstCaps,
            );
            fn gst_rs_demuxer_stream_push_buffer(
                raw: *mut gst_ffi::GstElement,
                index: u32,
                buffer: *mut gst_ffi::GstBuffer,
            ) -> gst_ffi::GstFlowReturn;
        };

        let mut res = {
            let mut demuxer = &mut self.demuxer.lock().unwrap();

            gst_trace!(
                self.cat,
                /* TODO obj: demuxer,*/ "Handling buffer {:?}",
                buffer
            );

            match demuxer.handle_buffer(Some(buffer)) {
                Ok(res) => res,
                Err(flow_error) => {
                    gst_error!(
                        self.cat,
                        /* TODO obj: demuxer,*/ "Failed handling buffer: {:?}",
                        flow_error
                    );
                    match flow_error {
                        FlowError::NotNegotiated(ref msg) | FlowError::Error(ref msg) => {
                            self.post_message(msg)
                        }
                        _ => (),
                    }
                    return flow_error.to_native();
                }
            }
        };

        // Loop until AllEos, NeedMoreData or error when pushing downstream
        loop {
            gst_trace!(self.cat, /* TODO obj: demuxer,*/ "Handled {:?}", res);

            match res {
                HandleBufferResult::NeedMoreData => {
                    return gst::FlowReturn::Ok;
                }
                HandleBufferResult::StreamAdded(stream) => unsafe {
                    gst_rs_demuxer_add_stream(
                        self.raw,
                        stream.index,
                        stream.caps.to_glib_none().0,
                        stream.stream_id.to_glib_none().0,
                    );
                },
                HandleBufferResult::HaveAllStreams => unsafe {
                    gst_rs_demuxer_added_all_streams(self.raw);
                },
                HandleBufferResult::StreamChanged(stream) => unsafe {
                    gst_rs_demuxer_stream_format_changed(
                        self.raw,
                        stream.index,
                        stream.caps.to_glib_none().0,
                    );
                },
                HandleBufferResult::StreamsChanged(streams) => for stream in streams {
                    unsafe {
                        gst_rs_demuxer_stream_format_changed(
                            self.raw,
                            stream.index,
                            stream.caps.to_glib_none().0,
                        );
                    }
                },
                HandleBufferResult::BufferForStream(index, buffer) => {
                    let flow_ret = unsafe {
                        gst_rs_demuxer_stream_push_buffer(self.raw, index, buffer.into_ptr())
                    };
                    if flow_ret != gst_ffi::GST_FLOW_OK {
                        return from_glib(flow_ret);
                    }
                }
                HandleBufferResult::Eos(index) => {
                    let index = index.unwrap_or(u32::MAX);

                    unsafe {
                        gst_rs_demuxer_stream_eos(self.raw, index);
                    }

                    return gst::FlowReturn::Eos;
                }
                HandleBufferResult::Again => {
                    // nothing, just call again
                }
            };

            gst_trace!(self.cat, /* TODO obj: demuxer,*/ "Calling again");

            res = {
                let mut demuxer = &mut self.demuxer.lock().unwrap();
                match demuxer.handle_buffer(None) {
                    Ok(res) => res,
                    Err(flow_error) => {
                        gst_error!(
                            self.cat,
                            /* TODO obj: demuxer,*/ "Failed calling again: {:?}",
                            flow_error
                        );
                        match flow_error {
                            FlowError::NotNegotiated(ref msg) | FlowError::Error(ref msg) => {
                                self.post_message(msg)
                            }
                            _ => (),
                        }
                        return flow_error.to_native();
                    }
                }
            }
        }
    }

    fn end_of_stream(&self) {
        let mut demuxer = &mut self.demuxer.lock().unwrap();

        gst_debug!(self.cat, /* TODO obj: demuxer,*/ "End of stream");
        match demuxer.end_of_stream() {
            Ok(_) => (),
            Err(ref msg) => {
                gst_error!(
                    self.cat,
                    /* TODO obj: demuxer,*/ "Failed end of stream: {:?}",
                    msg
                );
                self.post_message(msg);
            }
        }
    }

    fn post_message(&self, msg: &ErrorMessage) {
        unsafe {
            msg.post(&gst::Element::from_glib_borrow(self.raw));
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn demuxer_new(
    demuxer: *mut gst_ffi::GstElement,
    create_instance: fn(&gst::Element) -> Box<Demuxer>,
) -> *mut DemuxerWrapper {
    let instance = create_instance(&from_glib_borrow(demuxer));
    Box::into_raw(Box::new(DemuxerWrapper::new(demuxer, instance)))
}

#[no_mangle]
pub unsafe extern "C" fn demuxer_drop(ptr: *mut DemuxerWrapper) {
    let _ = Box::from_raw(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn demuxer_start(
    ptr: *const DemuxerWrapper,
    upstream_size: u64,
    random_access: glib_ffi::gboolean,
) -> glib_ffi::gboolean {
    let wrap: &DemuxerWrapper = &*ptr;

    panic_to_error!(
        wrap,
        &gst::Element::from_glib_borrow(wrap.raw as *mut _),
        false,
        { wrap.start(upstream_size, random_access != glib_ffi::GFALSE) }
    ).to_glib()
}

#[no_mangle]
pub unsafe extern "C" fn demuxer_stop(ptr: *const DemuxerWrapper) -> glib_ffi::gboolean {
    let wrap: &DemuxerWrapper = &*ptr;

    panic_to_error!(
        wrap,
        &gst::Element::from_glib_borrow(wrap.raw as *mut _),
        glib_ffi::GTRUE,
        {
            if wrap.stop() {
                glib_ffi::GTRUE
            } else {
                glib_ffi::GFALSE
            }
        }
    )
}

#[no_mangle]
pub unsafe extern "C" fn demuxer_is_seekable(ptr: *const DemuxerWrapper) -> glib_ffi::gboolean {
    let wrap: &DemuxerWrapper = &*ptr;

    panic_to_error!(
        wrap,
        &gst::Element::from_glib_borrow(wrap.raw as *mut _),
        glib_ffi::GFALSE,
        {
            if wrap.is_seekable() {
                glib_ffi::GTRUE
            } else {
                glib_ffi::GFALSE
            }
        }
    )
}

#[no_mangle]
pub unsafe extern "C" fn demuxer_get_position(
    ptr: *const DemuxerWrapper,
    position: *mut u64,
) -> glib_ffi::gboolean {
    let wrap: &DemuxerWrapper = &*ptr;

    panic_to_error!(
        wrap,
        &gst::Element::from_glib_borrow(wrap.raw as *mut _),
        glib_ffi::GFALSE,
        {
            let position = &mut *position;
            wrap.get_position(position)
        }
    )
}

#[no_mangle]
pub unsafe extern "C" fn demuxer_get_duration(
    ptr: *const DemuxerWrapper,
    duration: *mut u64,
) -> glib_ffi::gboolean {
    let wrap: &DemuxerWrapper = &*ptr;

    panic_to_error!(
        wrap,
        &gst::Element::from_glib_borrow(wrap.raw as *mut _),
        glib_ffi::GFALSE,
        {
            let duration = &mut *duration;
            wrap.get_duration(duration)
        }
    )
}

#[no_mangle]
pub unsafe extern "C" fn demuxer_seek(
    ptr: *mut DemuxerWrapper,
    start: u64,
    stop: u64,
    offset: *mut u64,
) -> glib_ffi::gboolean {
    let wrap: &mut DemuxerWrapper = &mut *ptr;

    panic_to_error!(
        wrap,
        &gst::Element::from_glib_borrow(wrap.raw as *mut _),
        glib_ffi::GFALSE,
        {
            let offset = &mut *offset;

            if wrap.seek(start, stop, offset) {
                glib_ffi::GTRUE
            } else {
                glib_ffi::GFALSE
            }
        }
    )
}

#[no_mangle]
pub unsafe extern "C" fn demuxer_handle_buffer(
    ptr: *mut DemuxerWrapper,
    buffer: *mut gst_ffi::GstBuffer,
) -> gst_ffi::GstFlowReturn {
    let wrap: &mut DemuxerWrapper = &mut *ptr;

    panic_to_error!(
        wrap,
        &gst::Element::from_glib_borrow(wrap.raw as *mut _),
        gst::FlowReturn::Error,
        { wrap.handle_buffer(from_glib_full(buffer)) }
    ).to_glib()
}

#[no_mangle]
pub unsafe extern "C" fn demuxer_end_of_stream(ptr: *mut DemuxerWrapper) {
    let wrap: &mut DemuxerWrapper = &mut *ptr;

    panic_to_error!(
        wrap,
        &gst::Element::from_glib_borrow(wrap.raw as *mut _),
        (),
        {
            wrap.end_of_stream();
        }
    )
}

pub struct DemuxerInfo<'a> {
    pub name: &'a str,
    pub long_name: &'a str,
    pub description: &'a str,
    pub classification: &'a str,
    pub author: &'a str,
    pub rank: i32,
    pub create_instance: fn(&gst::Element) -> Box<Demuxer>,
    pub input_caps: &'a gst::Caps,
    pub output_caps: &'a gst::Caps,
}

pub fn demuxer_register(plugin: &gst::Plugin, demuxer_info: &DemuxerInfo) {
    extern "C" {
        fn gst_rs_demuxer_register(
            plugin: *const gst_ffi::GstPlugin,
            name: *const c_char,
            long_name: *const c_char,
            description: *const c_char,
            classification: *const c_char,
            author: *const c_char,
            rank: i32,
            create_instance: *const c_void,
            input_caps: *const gst_ffi::GstCaps,
            output_caps: *const gst_ffi::GstCaps,
        ) -> glib_ffi::gboolean;
    }

    let cname = CString::new(demuxer_info.name).unwrap();
    let clong_name = CString::new(demuxer_info.long_name).unwrap();
    let cdescription = CString::new(demuxer_info.description).unwrap();
    let cclassification = CString::new(demuxer_info.classification).unwrap();
    let cauthor = CString::new(demuxer_info.author).unwrap();

    unsafe {
        gst_rs_demuxer_register(
            plugin.to_glib_none().0,
            cname.as_ptr(),
            clong_name.as_ptr(),
            cdescription.as_ptr(),
            cclassification.as_ptr(),
            cauthor.as_ptr(),
            demuxer_info.rank,
            demuxer_info.create_instance as *const c_void,
            demuxer_info.input_caps.to_glib_none().0,
            demuxer_info.output_caps.to_glib_none().0,
        );
    }
}
