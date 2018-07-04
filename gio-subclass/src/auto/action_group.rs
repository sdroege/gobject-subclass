// This file was generated by gir (https://github.com/gtk-rs/gir @ 666fc3d+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 47c69e6)
// DO NOT EDIT

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib_ffi::{gboolean, gconstpointer, gpointer, GType};

use gio;
use gio_ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

use gobject_subclass::anyimpl::*;
use gobject_subclass::object::*;


pub trait ActionGroupImpl: AnyImpl + 'static {

    fn action_added(&self, action_group: &gio::ActionGroup, action_name: &str);

    fn action_enabled_changed(&self, action_group: &gio::ActionGroup, action_name: &str, enabled: bool);

    fn action_removed(&self, action_group: &gio::ActionGroup, action_name: &str);

    fn action_state_changed(&self, action_group: &gio::ActionGroup, action_name: &str, state: &glib::Variant);

    fn activate_action(&self, action_group: &gio::ActionGroup, action_name: &str, parameter: Option<&glib::Variant>);

    fn change_action_state(&self, action_group: &gio::ActionGroup, action_name: &str, value: &glib::Variant);

    fn get_action_enabled(&self, action_group: &gio::ActionGroup, action_name: &str) -> bool;

    fn get_action_parameter_type(&self, action_group: &gio::ActionGroup, action_name: &str) -> Option<glib::VariantType>;

    fn get_action_state(&self, action_group: &gio::ActionGroup, action_name: &str) -> Option<glib::Variant>;

    fn get_action_state_hint(&self, action_group: &gio::ActionGroup, action_name: &str) -> Option<glib::Variant>;

    fn get_action_state_type(&self, action_group: &gio::ActionGroup, action_name: &str) -> Option<glib::VariantType>;

    fn has_action(&self, action_group: &gio::ActionGroup, action_name: &str) -> bool;

    fn list_actions(&self, action_group: &gio::ActionGroup) -> Vec<String>;

    fn query_action(&self, action_group: &gio::ActionGroup, action_name: &str) -> Option<(bool, glib::VariantType, glib::VariantType, glib::Variant, glib::Variant)>;

}

any_impl!(ActionGroupImpl);

pub trait ActionGroupImplStatic<T: ObjectType>: 'static {
    fn get_impl<'a>(&self, imp: &'a T::ImplType) -> &'a ActionGroupImpl;
}

struct ActionGroupStatic<T: ObjectType>{
    imp_static: *const ActionGroupImplStatic<T>
}


unsafe extern "C" fn action_group_action_added<T: ObjectType>
(gptr: *mut gio_ffi::GActionGroup, action_name: *const c_char)
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_group_get_type())
                                     as *const ActionGroupStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    imp.action_added(&wrap, &String::from_glib_none(action_name));
}

unsafe extern "C" fn action_group_action_enabled_changed<T: ObjectType>
(gptr: *mut gio_ffi::GActionGroup, action_name: *const c_char, enabled: gboolean)
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_group_get_type())
                                     as *const ActionGroupStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    imp.action_enabled_changed(&wrap, &String::from_glib_none(action_name), from_glib(enabled));
}

unsafe extern "C" fn action_group_action_removed<T: ObjectType>
(gptr: *mut gio_ffi::GActionGroup, action_name: *const c_char)
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_group_get_type())
                                     as *const ActionGroupStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    imp.action_removed(&wrap, &String::from_glib_none(action_name));
}

unsafe extern "C" fn action_group_action_state_changed<T: ObjectType>
(gptr: *mut gio_ffi::GActionGroup, action_name: *const c_char, state: *mut glib_ffi::GVariant)
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_group_get_type())
                                     as *const ActionGroupStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    imp.action_state_changed(&wrap, &String::from_glib_none(action_name), &from_glib_none(state));
}

unsafe extern "C" fn action_group_activate_action<T: ObjectType>
(gptr: *mut gio_ffi::GActionGroup, action_name: *const c_char, parameter: *mut glib_ffi::GVariant)
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_group_get_type())
                                     as *const ActionGroupStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    imp.activate_action(&wrap, &String::from_glib_none(action_name), (if parameter.is_null() { None } else { Some(from_glib_borrow(parameter)) }).as_ref());
}

unsafe extern "C" fn action_group_change_action_state<T: ObjectType>
(gptr: *mut gio_ffi::GActionGroup, action_name: *const c_char, value: *mut glib_ffi::GVariant)
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_group_get_type())
                                     as *const ActionGroupStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    imp.change_action_state(&wrap, &String::from_glib_none(action_name), &from_glib_none(value));
}

unsafe extern "C" fn action_group_get_action_enabled<T: ObjectType>
(gptr: *mut gio_ffi::GActionGroup, action_name: *const c_char) -> gboolean
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_group_get_type())
                                     as *const ActionGroupStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    let rs_ret = imp.get_action_enabled(&wrap, &String::from_glib_none(action_name));
    rs_ret.to_glib()
}

unsafe extern "C" fn action_group_get_action_parameter_type<T: ObjectType>
(gptr: *mut gio_ffi::GActionGroup, action_name: *const c_char) -> *const glib_ffi::GVariantType
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_group_get_type())
                                     as *const ActionGroupStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    let rs_ret = imp.get_action_parameter_type(&wrap, &String::from_glib_none(action_name));
    
    match rs_ret {
        Some(t_ret) => {
        let ret = t_ret.to_glib_full();
        unsafe extern "C" fn destroy_ret(p: glib_ffi::gpointer){
            glib::VariantType::from_glib_full(p as *const glib_ffi::GVariantType);
        };
        gobject_ffi::g_object_set_qdata_full(gptr as *mut gobject_ffi::GObject,
            glib_ffi::g_quark_from_string("rs_action_group_get_action_parameter_type_ret".to_glib_none().0),
            ret as *mut c_void,
            Some(destroy_ret)
        );
        ret
    },
        None => ptr::null()
    }
}

unsafe extern "C" fn action_group_get_action_state<T: ObjectType>
(gptr: *mut gio_ffi::GActionGroup, action_name: *const c_char) -> *mut glib_ffi::GVariant
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_group_get_type())
                                     as *const ActionGroupStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    let rs_ret = imp.get_action_state(&wrap, &String::from_glib_none(action_name));
    match rs_ret { Some(t_ret) => t_ret.to_glib_full(), None => ptr::null_mut()}
}

unsafe extern "C" fn action_group_get_action_state_hint<T: ObjectType>
(gptr: *mut gio_ffi::GActionGroup, action_name: *const c_char) -> *mut glib_ffi::GVariant
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_group_get_type())
                                     as *const ActionGroupStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    let rs_ret = imp.get_action_state_hint(&wrap, &String::from_glib_none(action_name));
    match rs_ret { Some(t_ret) => t_ret.to_glib_full(), None => ptr::null_mut()}
}

unsafe extern "C" fn action_group_get_action_state_type<T: ObjectType>
(gptr: *mut gio_ffi::GActionGroup, action_name: *const c_char) -> *const glib_ffi::GVariantType
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_group_get_type())
                                     as *const ActionGroupStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    let rs_ret = imp.get_action_state_type(&wrap, &String::from_glib_none(action_name));
    
    match rs_ret {
        Some(t_ret) => {
        let ret = t_ret.to_glib_full();
        unsafe extern "C" fn destroy_ret(p: glib_ffi::gpointer){
            glib::VariantType::from_glib_full(p as *const glib_ffi::GVariantType);
        };
        gobject_ffi::g_object_set_qdata_full(gptr as *mut gobject_ffi::GObject,
            glib_ffi::g_quark_from_string("rs_action_group_get_action_state_type_ret".to_glib_none().0),
            ret as *mut c_void,
            Some(destroy_ret)
        );
        ret
    },
        None => ptr::null()
    }
}

unsafe extern "C" fn action_group_has_action<T: ObjectType>
(gptr: *mut gio_ffi::GActionGroup, action_name: *const c_char) -> gboolean
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_group_get_type())
                                     as *const ActionGroupStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    let rs_ret = imp.has_action(&wrap, &String::from_glib_none(action_name));
    rs_ret.to_glib()
}

unsafe extern "C" fn action_group_list_actions<T: ObjectType>
(gptr: *mut gio_ffi::GActionGroup) -> *mut *mut c_char
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_group_get_type())
                                     as *const ActionGroupStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    let rs_ret = imp.list_actions(&wrap);
    match rs_ret { Some(t_ret) => t_ret.to_glib_full(), None => ptr::null_mut()}
}

unsafe extern "C" fn action_group_query_action<T: ObjectType>
(gptr: *mut gio_ffi::GActionGroup, action_name: *const c_char, enabled: *mut gboolean, parameter_type: *mut *const glib_ffi::GVariantType, state_type: *mut *const glib_ffi::GVariantType, state_hint: *mut *mut glib_ffi::GVariant, state: *mut *mut glib_ffi::GVariant) -> gboolean
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_group_get_type())
                                     as *const ActionGroupStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    let (rs_ret, rs_enabled, rs_parameter_type, rs_state_type, rs_state_hint, rs_state) = imp.query_action(&wrap, &String::from_glib_none(action_name));
    ptr::write(enabled, rs_enabled.to_glib());
    ptr::write(parameter_type, {
        let ret = t_parameter_type.to_glib_full();
        unsafe extern "C" fn destroy_parameter_type(p: glib_ffi::gpointer){
            glib::VariantType::from_glib_full(p as *mut *const glib_ffi::GVariantType);
        };
        gobject_ffi::g_object_set_qdata_full(gptr as *mut gobject_ffi::GObject,
            glib_ffi::g_quark_from_string("rs_action_group_query_action_parameter_type".to_glib_none().0),
            ret as *mut c_void,
            Some(destroy_parameter_type)
        );
        ret
    });
    ptr::write(state_type, {
        let ret = t_state_type.to_glib_full();
        unsafe extern "C" fn destroy_state_type(p: glib_ffi::gpointer){
            glib::VariantType::from_glib_full(p as *mut *const glib_ffi::GVariantType);
        };
        gobject_ffi::g_object_set_qdata_full(gptr as *mut gobject_ffi::GObject,
            glib_ffi::g_quark_from_string("rs_action_group_query_action_state_type".to_glib_none().0),
            ret as *mut c_void,
            Some(destroy_state_type)
        );
        ret
    });
    ptr::write(state_hint, {
        let ret = t_state_hint.to_glib_full();
        unsafe extern "C" fn destroy_state_hint(p: glib_ffi::gpointer){
            glib::Variant::from_glib_full(p as *mut *mut glib_ffi::GVariant);
        };
        gobject_ffi::g_object_set_qdata_full(gptr as *mut gobject_ffi::GObject,
            glib_ffi::g_quark_from_string("rs_action_group_query_action_state_hint".to_glib_none().0),
            ret as *mut c_void,
            Some(destroy_state_hint)
        );
        ret
    });
    ptr::write(state, {
        let ret = t_state.to_glib_full();
        unsafe extern "C" fn destroy_state(p: glib_ffi::gpointer){
            glib::Variant::from_glib_full(p as *mut *mut glib_ffi::GVariant);
        };
        gobject_ffi::g_object_set_qdata_full(gptr as *mut gobject_ffi::GObject,
            glib_ffi::g_quark_from_string("rs_action_group_query_action_state".to_glib_none().0),
            ret as *mut c_void,
            Some(destroy_state)
        );
        ret
    });
    rs_ret.to_glib()
}

unsafe extern "C" fn action_group_init<T: ObjectType>(
    iface: glib_ffi::gpointer,
    iface_data: glib_ffi::gpointer
) {
    let action_group_iface = &mut *(iface as *mut gio_ffi::GActionGroupInterface);
    let iface_type = (*(iface as *const gobject_ffi::GTypeInterface)).g_type;
    let type_ = (*(iface as *const gobject_ffi::GTypeInterface)).g_instance_type;
    let klass = &mut *(gobject_ffi::g_type_class_ref(type_) as *mut ClassStruct<T>);
    let interfaces_static = &mut *(klass.interfaces_static as *mut Vec<_>);
    interfaces_static.push((iface_type, iface_data));
    action_group_iface.action_added = Some(action_group_action_added::<T>);
    action_group_iface.action_enabled_changed = Some(action_group_action_enabled_changed::<T>);
    action_group_iface.action_removed = Some(action_group_action_removed::<T>);
    action_group_iface.action_state_changed = Some(action_group_action_state_changed::<T>);
    action_group_iface.activate_action = Some(action_group_activate_action::<T>);
    action_group_iface.change_action_state = Some(action_group_change_action_state::<T>);
    action_group_iface.get_action_enabled = Some(action_group_get_action_enabled::<T>);
    action_group_iface.get_action_parameter_type = Some(action_group_get_action_parameter_type::<T>);
    action_group_iface.get_action_state = Some(action_group_get_action_state::<T>);
    action_group_iface.get_action_state_hint = Some(action_group_get_action_state_hint::<T>);
    action_group_iface.get_action_state_type = Some(action_group_get_action_state_type::<T>);
    action_group_iface.has_action = Some(action_group_has_action::<T>);
    action_group_iface.list_actions = Some(action_group_list_actions::<T>);
    action_group_iface.query_action = Some(action_group_query_action::<T>);
}

pub fn register_ActionGroup<T: ObjectType, I: ActionGroupImplStatic<T>>(
    _: &TypeInitToken,
    type_: glib::Type,
    imp: &I,
) {

    unsafe {
        let imp = imp as &ActionGroupImplStatic<T> as *const ActionGroupImplStatic<T>;
        let interface_static = Box::new(ActionGroupStatic {
            imp_static: imp,
        });
        let iface_info = gobject_ffi::GInterfaceInfo {
            interface_init: Some(action_group_init::<T>),
            interface_finalize: None,
            interface_data: Box::into_raw(interface_static) as glib_ffi::gpointer,
        };
        gobject_ffi::g_type_add_interface_static(
            type_.to_glib(),
            gio_ffi::g_action_group_get_type(),
            &iface_info,
        );
    }
        
}
