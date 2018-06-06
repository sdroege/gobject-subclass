// This file was generated by gir (https://github.com/gtk-rs/gir @ e26b60f+)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8+)
// DO NOT EDIT

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

use gobject_subclass::anyimpl::*;
use gobject_subclass::object::*;

#[cfg(any(feature = "v2_38", feature = "dox"))]
use Error;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;


pub trait ActionImpl: AnyImpl + 'static {

    fn activate(&self, action: &Action, parameter: Option<&glib::Variant>);

    fn change_state(&self, action: &Action, value: &glib::Variant);

    fn get_enabled(&self, action: &Action) -> bool;

    fn get_name(&self, action: &Action) -> Option<String>;

    fn get_parameter_type(&self, action: &Action) -> Option<glib::VariantType>;

    fn get_state(&self, action: &Action) -> Option<glib::Variant>;

    fn get_state_hint(&self, action: &Action) -> Option<glib::Variant>;

    fn get_state_type(&self, action: &Action) -> Option<glib::VariantType>;

}

any_impl!(ActionImpl);

pub trait ActionImplStatic<T: ObjectType>: 'static {
    fn get_impl<'a>(&self, imp: &'a T::ImplType) -> &'a ActionImpl;
}

struct ActionStatic<T: ObjectType>{
    imp_static: *const ActionImplStatic<T>
}

unsafe extern "C" fn action_activate<T: >
(ptr: *mut GAction, parameter: *mut glib::GVariant)
where
    T::ImplType: ActionImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let klass = &**(ptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(ffi::action_get_type())
                                     as *const ActionStatic<T>;
    let instance = &*(ptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    imp.activate()
}

unsafe extern "C" fn action_change_state<T: >
(ptr: *mut GAction, value: *mut glib::GVariant)
where
    T::ImplType: ActionImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let klass = &**(ptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(ffi::action_get_type())
                                     as *const ActionStatic<T>;
    let instance = &*(ptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    imp.change_state()
}

unsafe extern "C" fn action_get_enabled<T: >
(ptr: *mut GAction) -> gboolean
where
    T::ImplType: ActionImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let klass = &**(ptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(ffi::action_get_type())
                                     as *const ActionStatic<T>;
    let instance = &*(ptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    imp.get_enabled()
}

unsafe extern "C" fn action_get_name<T: >
(ptr: *mut GAction) -> *const c_char
where
    T::ImplType: ActionImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let klass = &**(ptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(ffi::action_get_type())
                                     as *const ActionStatic<T>;
    let instance = &*(ptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    imp.get_name()
}

unsafe extern "C" fn action_get_parameter_type<T: >
(ptr: *mut GAction) -> *const glib::GVariantType
where
    T::ImplType: ActionImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let klass = &**(ptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(ffi::action_get_type())
                                     as *const ActionStatic<T>;
    let instance = &*(ptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    imp.get_parameter_type()
}

unsafe extern "C" fn action_get_state<T: >
(ptr: *mut GAction) -> *mut glib::GVariant
where
    T::ImplType: ActionImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let klass = &**(ptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(ffi::action_get_type())
                                     as *const ActionStatic<T>;
    let instance = &*(ptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    imp.get_state()
}

unsafe extern "C" fn action_get_state_hint<T: >
(ptr: *mut GAction) -> *mut glib::GVariant
where
    T::ImplType: ActionImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let klass = &**(ptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(ffi::action_get_type())
                                     as *const ActionStatic<T>;
    let instance = &*(ptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    imp.get_state_hint()
}

unsafe extern "C" fn action_get_state_type<T: >
(ptr: *mut GAction) -> *const glib::GVariantType
where
    T::ImplType: ActionImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let klass = &**(ptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(ffi::action_get_type())
                                     as *const ActionStatic<T>;
    let instance = &*(ptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    imp.get_state_type()
}

unsafe extern "C" fn action_init<T: ObjectType>(
    iface: glib_ffi::gpointer,
    iface_data: glib_ffi::gpointer
) {
    callback_guard!();
    let action_iface = &mut *(iface as *mut ffi::GAction);
    let iface_type = (*(iface as *const gobject_ffi::GTypeInterface)).g_type;
    let type_ = (*(iface as *const gobject_ffi::GTypeInterface)).g_instance_type;
    let klass = &mut *(gobject_ffi::g_type_class_ref(type_) as *mut ClassStruct<T>);
    let interfaces_static = &mut *(klass.interfaces_static as *mut Vec<_>);
    interfaces_static.push((iface_type, iface_data))
    action_iface.activate = Some(action_activate::<T>);
    action_iface.change_state = Some(action_change_state::<T>);
    action_iface.get_enabled = Some(action_get_enabled::<T>);
    action_iface.get_name = Some(action_get_name::<T>);
    action_iface.get_parameter_type = Some(action_get_parameter_type::<T>);
    action_iface.get_state = Some(action_get_state::<T>);
    action_iface.get_state_hint = Some(action_get_state_hint::<T>);
    action_iface.get_state_type = Some(action_get_state_type::<T>);
}