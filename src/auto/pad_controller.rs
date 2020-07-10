// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use gtk_sys;
use std::fmt;
use EventController;
use PadActionType;

glib_wrapper! {
    pub struct PadController(Object<gtk_sys::GtkPadController, gtk_sys::GtkPadControllerClass, PadControllerClass>) @extends EventController;

    match fn {
        get_type => || gtk_sys::gtk_pad_controller_get_type(),
    }
}

impl PadController {
    //pub fn new(group: /*Ignored*/&gio::ActionGroup, pad: /*Ignored*/Option<&gdk::Device>) -> PadController {
    //    unsafe { TODO: call gtk_sys:gtk_pad_controller_new() }
    //}

    pub fn set_action(
        &self,
        type_: PadActionType,
        index: i32,
        mode: i32,
        label: &str,
        action_name: &str,
    ) {
        unsafe {
            gtk_sys::gtk_pad_controller_set_action(
                self.to_glib_none().0,
                type_.to_glib(),
                index,
                mode,
                label.to_glib_none().0,
                action_name.to_glib_none().0,
            );
        }
    }

    //pub fn get_property_action_group(&self) -> /*Ignored*/Option<gio::ActionGroup> {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"action-group\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().expect("Return Value for property `action-group` getter")
    //    }
    //}

    //pub fn get_property_pad(&self) -> /*Ignored*/Option<gdk::Device> {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"pad\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().expect("Return Value for property `pad` getter")
    //    }
    //}
}

impl fmt::Display for PadController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PadController")
    }
}
