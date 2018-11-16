// Copyright (C) 2017,2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use glib_ffi;
use gobject_ffi;

use std::mem;
use std::ops;
use std::ptr;

use glib;
use glib::translate::*;

use properties::*;

/// A newly registered `glib::Type` that is currently still being initialized
///
/// This allows running additional type-setup functions, e.g. for implementing
/// interfaces on the type
#[derive(Debug, PartialEq, Eq)]
pub struct InitializingType(glib::Type);

impl ops::Deref for InitializingType {
    type Target = glib::Type;

    fn deref(&self) -> &glib::Type {
        &self.0
    }
}

/// Trait implemented by structs that implement a `GObject` C instance struct
pub trait InstanceStruct: Sized + 'static {
    type Type: ObjectSubclass;

    fn get_impl(&self) -> &Self::Type {
        unsafe {
            let data = Self::Type::type_data();
            let private_offset = data.as_ref().private_offset;
            let ptr: *const u8 = self as *const _ as *const u8;
            let priv_ptr = ptr.offset(private_offset);
            let imp = priv_ptr as *const Option<Self::Type>;

            (*imp).as_ref().expect("No private struct")
        }
    }

    fn get_class(&self) -> &<Self::Type as ObjectSubclass>::Class {
        unsafe { &**(self as *const _ as *const *const <Self::Type as ObjectSubclass>::Class) }
    }
}

/// Trait implemented by structs that implement a `GObject` C class struct
pub trait ClassStruct: Sized + 'static {
    type Type: ObjectSubclass;
}

/// Type-specific data that is filled in during type creation
pub struct TypeData {
    pub type_: glib::Type,
    pub parent_class: glib_ffi::gpointer,
    pub interfaces: *const Vec<(glib_ffi::GType, glib_ffi::gpointer)>,
    pub private_offset: isize,
}

impl TypeData {
    /// Returns the type ID
    pub fn get_type(&self) -> glib::Type {
        self.type_
    }

    /// Returns a pointer to the native parent class
    ///
    /// This is used for chaining up to the parent class' implementation
    /// of virtual methods
    pub fn get_parent_class(&self) -> glib_ffi::gpointer {
        self.parent_class
    }

    // FIXME: do we need this, do we want to do this different now?
    pub fn get_interface(&self, type_: glib_ffi::GType) -> glib_ffi::gpointer {
        unsafe {
            if self.interfaces.is_null() {
                return ptr::null_mut();
            }

            for &(t, p) in &(*self.interfaces) {
                if t == type_ {
                    return p;
                }
            }

            ptr::null_mut()
        }
    }

    /// Returns the offset of the private struct in bytes relative to the
    /// beginning of the instance struct
    pub fn get_private_offset(&self) -> isize {
        self.private_offset
    }
}

/// The central trait for subclassing a `GObject` type
///
/// Links together the type name, parent type and the instance and
/// class structs for type registration and allows subclasses to
/// hook into various steps of the type registration and initialization.
pub unsafe trait ObjectSubclass: ObjectImpl + Sized + 'static {
    /// `GObject` type name.
    ///
    /// This must be unique in the whole process.
    const NAME: &'static str;

    /// Parent Rust type to inherit from
    type ParentType: glib::IsA<glib::Object>;

    /// The C instance struct
    ///
    /// This must be `#[repr(C)]` and contain the `ParentType`'s instance struct
    /// as its first member.
    type Instance: InstanceStruct<Type = Self>;

    /// The C class struct
    ///
    /// This must be `#[repr(C)]` and contain the `ParentType`'s class struct
    /// as its first member.
    type Class: ClassStruct<Type = Self>;

    /// The Rust wrapper type for our new subclass
    type RustType: glib::IsA<glib::Object> + FromGlibPtrBorrow<*mut Self::Instance>;

    // TODO: Define a macro for this
    /// Storage for the type-specific data used during registration
    ///
    /// This is usually generated by the TODO macro.
    fn type_data() -> ptr::NonNull<TypeData>;

    /// Returns the `glib::Type` ID of the subclass
    ///
    /// This will panic if called before the type was registered at
    /// runtime with the `GObject` type system.
    fn static_type() -> glib::Type {
        unsafe {
            let data = Self::type_data();
            let type_ = data.as_ref().get_type();
            assert_ne!(type_, glib::Type::Invalid);

            type_
        }
    }

    /// Additional type initialization
    ///
    /// This is called right after the type was registered and allows
    /// subclasses to do additional type-specific initialization, e.g.
    /// for implementing `GObject` interfaces.
    ///
    /// Optional
    fn type_init(_type_: &InitializingType) {}

    /// Class initialization
    ///
    /// This is called after `type_init` and before the first instance
    /// of the subclass is created. Subclasses can use this to do class-
    /// specific initialization, e.g. for installing properties or signals
    /// on the class or calling class methods.
    ///
    /// Optional
    fn class_init(_klass: &mut Self::Class) {}

    /// Constructor
    ///
    /// This is called during object instantiation before further subclasses
    /// are initialized, and should return a new instance of the subclass
    /// private struct.
    fn new(obj: &Self::RustType) -> Self;
}

unsafe extern "C" fn class_init<T: ObjectSubclass>(
    klass: glib_ffi::gpointer,
    _klass_data: glib_ffi::gpointer,
) {
    let mut data = T::type_data();

    // We have to update the private struct offset once the class is actually
    // being initialized
    {
        let mut private_offset = data.as_ref().private_offset as i32;
        gobject_ffi::g_type_class_adjust_private_offset(klass, &mut private_offset);
        (*data.as_mut()).private_offset = private_offset as isize;
    }

    // Set trampolines for the basic GObject virtual methods
    {
        let gobject_klass = &mut *(klass as *mut gobject_ffi::GObjectClass);

        gobject_klass.finalize = Some(finalize::<T>);
    }

    // And finally peek the parent class struct (containing the parent class'
    // implementations of virtual methods for chaining up), and call the subclass'
    // class initialization function
    {
        let klass = &mut *(klass as *mut T::Class);
        let parent_class =
            gobject_ffi::g_type_class_peek_parent(klass as *mut _ as glib_ffi::gpointer)
                as *mut <T::ParentType as glib::wrapper::Wrapper>::GlibClassType;
        assert!(!parent_class.is_null());

        (*data.as_mut()).parent_class = parent_class as glib_ffi::gpointer;

        T::class_init(klass);
    }
}

unsafe extern "C" fn instance_init<T: ObjectSubclass>(
    obj: *mut gobject_ffi::GTypeInstance,
    _klass: glib_ffi::gpointer,
) {
    floating_reference_guard!(obj);
    let rs_instance: T::RustType = from_glib_borrow(obj as *mut T::Instance);

    // Get offset to the storage of our private struct, create it
    // and actually store it in that place
    let mut data = T::type_data();
    let private_offset = (*data.as_mut()).private_offset;
    let ptr: *mut u8 = obj as *mut _ as *mut u8;
    let priv_ptr = ptr.offset(private_offset);
    let imp_storage = priv_ptr as *mut Option<T>;

    let imp = T::new(&rs_instance);

    ptr::write(imp_storage, Some(imp));
}

unsafe extern "C" fn finalize<T: ObjectSubclass>(obj: *mut gobject_ffi::GObject) {
    floating_reference_guard!(obj);

    // Retrieve the private struct, take it out of its storage and
    // drop it for freeing all associated memory
    let mut data = T::type_data();
    let private_offset = (*data.as_mut()).private_offset;
    let ptr: *mut u8 = obj as *mut _ as *mut u8;
    let priv_ptr = ptr.offset(private_offset);
    let imp_storage = priv_ptr as *mut Option<T>;

    let imp = (*imp_storage).take().expect("No private struct");
    drop(imp);

    // Chain up to the parent class' finalize implementation, if any
    let parent_class = &*(data.as_ref().get_parent_class() as *const gobject_ffi::GObjectClass);
    if let Some(ref func) = parent_class.finalize {
        func(obj);
    }
}

/// Register a `glib::Type` ID for `T`
///
/// This must be called only once and will panic on a second call.
pub fn register_type<T: ObjectSubclass>() -> glib::Type {
    unsafe {
        use std::ffi::CString;

        let type_info = gobject_ffi::GTypeInfo {
            class_size: mem::size_of::<T::Class>() as u16,
            base_init: None,
            base_finalize: None,
            class_init: Some(class_init::<T>),
            class_finalize: None,
            class_data: ptr::null_mut(),
            instance_size: mem::size_of::<T::Instance>() as u16,
            n_preallocs: 0,
            instance_init: Some(instance_init::<T>),
            value_table: ptr::null(),
        };

        let type_name = CString::new(T::NAME).unwrap();
        assert_eq!(
            gobject_ffi::g_type_from_name(type_name.as_ptr()),
            gobject_ffi::G_TYPE_INVALID
        );

        let type_ = from_glib(gobject_ffi::g_type_register_static(
            <T::ParentType as glib::StaticType>::static_type().to_glib(),
            type_name.as_ptr(),
            &type_info,
            0,
        ));

        let mut data = T::type_data();
        (*data.as_mut()).type_ = type_;
        let private_offset =
            gobject_ffi::g_type_add_instance_private(type_.to_glib(), mem::size_of::<Option<T>>());
        (*data.as_mut()).private_offset = private_offset as isize;

        T::type_init(&InitializingType(type_));

        type_
    }
}

/// Trait for declaring the subclass relationship between classes
///
/// This is the class version of `glib::IsA`.
pub unsafe trait IsAClass<T> {}

unsafe impl<T> IsAClass<T> for T {}

// TODO: Everything below should be in glib-rs/object.rs

/// Trait for implementors of `glib::Object` subclasses
///
/// This allows overriding the virtual methods of `glib::Object`
pub trait ObjectImpl: 'static {
    // TODO: Define a macro for this
    /// Storage for the type-specific data used during registration
    ///
    /// This is usually generated by the TODO macro.
    fn get_type_data(&self) -> ptr::NonNull<TypeData>;

    /// Property setter
    ///
    /// This is called whenever the property of this specific subclass with the
    /// given index is set. The new value is passed as `glib::Value`.
    fn set_property(&self, _obj: &glib::Object, _id: u32, _value: &glib::Value) {
        unimplemented!()
    }

    /// Property getter
    ///
    /// This is called whenever the property value of the specific subclass with the
    /// given index should be returned.
    fn get_property(&self, _obj: &glib::Object, _id: u32) -> Result<glib::Value, ()> {
        unimplemented!()
    }

    /// Constructed
    ///
    /// This is called once construction of the instance is finished.
    ///
    /// Should chain up to the parent class' implementation.
    fn constructed(&self, obj: &glib::Object) {
        self.parent_constructed(obj);
    }

    /// Chain up to the parent class' implementation of `glib::Object::constructed()`
    ///
    /// Do not override this, it has no effect.
    fn parent_constructed(&self, obj: &glib::Object) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gobject_ffi::GObjectClass;

            if let Some(ref func) = (*parent_class).constructed {
                func(obj.to_glib_none().0);
            }
        }
    }
}

/// Class struct for `glib::Object`
#[repr(C)]
pub struct ObjectClass(gobject_ffi::GObjectClass);

/// Extension trait for `glib::Object`'s class struct
///
/// This contains various class methods and allows subclasses
/// to override the virtual methods
pub trait ObjectClassExt: Sized + 'static {
    /// Override the virtual methods
    fn override_vfuncs(&mut self)
    where
        Self: ClassStruct,
    {
        unsafe {
            let klass = &mut *(self as *const Self as *mut gobject_ffi::GObjectClass);
            klass.set_property = Some(set_property::<<Self as ClassStruct>::Type>);
            klass.get_property = Some(get_property::<<Self as ClassStruct>::Type>);
            klass.constructed = Some(constructed::<<Self as ClassStruct>::Type>);
        }
    }

    /// Install properties on the subclass
    ///
    /// This must be called after `override_vfuncs()` to work correctly.
    /// The index in the properties array is going to be the index passed to the
    /// property setters and getters.
    // TODO: Use a different Property struct
    //   struct Property {
    //     name: &'static str,
    //     pspec: fn () -> glib::ParamSpec,
    //   }
    fn install_properties(&mut self, properties: &[Property]) {
        if properties.is_empty() {
            return;
        }

        let mut pspecs = Vec::with_capacity(properties.len());

        pspecs.push(ptr::null_mut());

        for property in properties {
            pspecs.push(property.into());
        }

        unsafe {
            gobject_ffi::g_object_class_install_properties(
                self as *mut _ as *mut gobject_ffi::GObjectClass,
                pspecs.len() as u32,
                pspecs.as_mut_ptr(),
            );
        }
    }

    /// Add a new signal to the subclass
    ///
    /// This can be emitted later by `glib::Object::emit` and external code
    /// can connect to the signal to get notified about emissions.
    fn add_signal(&mut self, name: &str, arg_types: &[glib::Type], ret_type: glib::Type) {
        let arg_types = arg_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();
        unsafe {
            gobject_ffi::g_signal_newv(
                name.to_glib_none().0,
                *(self as *mut _ as *mut glib_ffi::GType),
                gobject_ffi::G_SIGNAL_RUN_LAST,
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                None,
                ret_type.to_glib(),
                arg_types.len() as u32,
                arg_types.as_ptr() as *mut _,
            );
        }
    }

    /// Add a new signal with accumulator to the subclass
    ///
    /// This can be emitted later by `glib::Object::emit` and external code
    /// can connect to the signal to get notified about emissions.
    ///
    /// The accumulator function is used for accumulating the return values of
    /// multiple signal handlers. The new value is passed as second argument and
    /// should be combined with the old value in the first argument. If no further
    /// signal handlers should be called, `false` should be returned.
    fn add_signal_with_accumulator<F>(
        &mut self,
        name: &str,
        arg_types: &[glib::Type],
        ret_type: glib::Type,
        accumulator: F,
    ) where
        F: Fn(&mut glib::Value, &glib::Value) -> bool + Send + Sync + 'static,
    {
        let arg_types = arg_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();

        let accumulator: Box<
            Box<Fn(&mut glib::Value, &glib::Value) -> bool + Send + Sync + 'static>,
        > = Box::new(Box::new(accumulator));

        unsafe extern "C" fn accumulator_trampoline(
            _ihint: *mut gobject_ffi::GSignalInvocationHint,
            return_accu: *mut gobject_ffi::GValue,
            handler_return: *const gobject_ffi::GValue,
            data: glib_ffi::gpointer,
        ) -> glib_ffi::gboolean {
            let accumulator: &&(Fn(&mut glib::Value, &glib::Value) -> bool
                                    + Send
                                    + Sync
                                    + 'static) =
                &*(data as *const &(Fn(&mut glib::Value, &glib::Value) -> bool
                      + Send
                      + Sync
                      + 'static));
            accumulator(
                &mut *(return_accu as *mut glib::Value),
                &*(handler_return as *const glib::Value),
            )
            .to_glib()
        }

        unsafe {
            gobject_ffi::g_signal_newv(
                name.to_glib_none().0,
                *(self as *mut _ as *mut glib_ffi::GType),
                gobject_ffi::G_SIGNAL_RUN_LAST,
                ptr::null_mut(),
                Some(accumulator_trampoline),
                Box::into_raw(accumulator) as glib_ffi::gpointer,
                None,
                ret_type.to_glib(),
                arg_types.len() as u32,
                arg_types.as_ptr() as *mut _,
            );
        }
    }

    /// Add a new action signal with accumulator to the subclass
    ///
    /// Different to normal signals, action signals are supposed to be emitted
    /// by external code and will cause the provided handler to be called.
    ///
    /// It can be thought of as a dynamic function call.
    fn add_action_signal<F>(
        &mut self,
        name: &str,
        arg_types: &[glib::Type],
        ret_type: glib::Type,
        handler: F,
    ) where
        F: Fn(&[glib::Value]) -> Option<glib::Value> + Send + Sync + 'static,
    {
        let arg_types = arg_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();
        let handler = glib::Closure::new(handler);
        unsafe {
            gobject_ffi::g_signal_newv(
                name.to_glib_none().0,
                *(self as *mut _ as *mut glib_ffi::GType),
                gobject_ffi::G_SIGNAL_RUN_LAST | gobject_ffi::G_SIGNAL_ACTION,
                handler.to_glib_none().0,
                None,
                ptr::null_mut(),
                None,
                ret_type.to_glib(),
                arg_types.len() as u32,
                arg_types.as_ptr() as *mut _,
            );
        }
    }
}

impl<T: IsAClass<ObjectClass> + 'static> ObjectClassExt for T {}

unsafe extern "C" fn get_property<T: ObjectSubclass>(
    obj: *mut gobject_ffi::GObject,
    id: u32,
    value: *mut gobject_ffi::GValue,
    _pspec: *mut gobject_ffi::GParamSpec,
) {
    floating_reference_guard!(obj);
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.get_impl();

    match imp.get_property(&from_glib_borrow(obj), id - 1) {
        Ok(v) => {
            // Here we overwrite the value directly with ours
            // and forget ours because otherwise we would do
            // an additional copy of the value, which for
            // non-refcounted types involves a deep copy
            gobject_ffi::g_value_unset(value);
            ptr::write(value, ptr::read(v.to_glib_none().0));
            mem::forget(v);
        }
        Err(()) => eprintln!("Failed to get property"),
    }
}

unsafe extern "C" fn set_property<T: ObjectSubclass>(
    obj: *mut gobject_ffi::GObject,
    id: u32,
    value: *mut gobject_ffi::GValue,
    _pspec: *mut gobject_ffi::GParamSpec,
) {
    floating_reference_guard!(obj);
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.get_impl();
    imp.set_property(
        &from_glib_borrow(obj),
        id - 1,
        &*(value as *mut glib::Value),
    );
}

unsafe extern "C" fn constructed<T: ObjectSubclass>(obj: *mut gobject_ffi::GObject) {
    floating_reference_guard!(obj);
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.get_impl();

    imp.constructed(&from_glib_borrow(obj));
}
