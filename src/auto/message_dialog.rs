// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Bin;
use Buildable;
use Container;
use Dialog;
use MessageType;
use Root;
use Widget;
use Window;

glib_wrapper! {
    pub struct MessageDialog(Object<gtk_sys::GtkMessageDialog, gtk_sys::GtkMessageDialogClass, MessageDialogClass>) @extends Dialog, Window, Bin, Container, Widget, @implements Buildable, Root;

    match fn {
        get_type => || gtk_sys::gtk_message_dialog_get_type(),
    }
}

impl MessageDialog {
    //pub fn new<P: IsA<Window>>(parent: Option<&P>, flags: DialogFlags, type_: MessageType, buttons: ButtonsType, message_format: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> MessageDialog {
    //    unsafe { TODO: call gtk_sys:gtk_message_dialog_new() }
    //}

    //pub fn with_markup<P: IsA<Window>>(parent: Option<&P>, flags: DialogFlags, type_: MessageType, buttons: ButtonsType, message_format: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> MessageDialog {
    //    unsafe { TODO: call gtk_sys:gtk_message_dialog_new_with_markup() }
    //}

    //pub fn format_secondary_markup(&self, message_format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gtk_sys:gtk_message_dialog_format_secondary_markup() }
    //}

    //pub fn format_secondary_text(&self, message_format: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gtk_sys:gtk_message_dialog_format_secondary_text() }
    //}

    pub fn get_message_area(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_message_dialog_get_message_area(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn set_markup(&self, str: &str) {
        unsafe {
            gtk_sys::gtk_message_dialog_set_markup(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    pub fn get_property_message_type(&self) -> MessageType {
        unsafe {
            let mut value = Value::from_type(<MessageType as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"message-type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `message-type` getter")
                .unwrap()
        }
    }

    pub fn set_property_message_type(&self, message_type: MessageType) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"message-type\0".as_ptr() as *const _,
                Value::from(&message_type).to_glib_none().0,
            );
        }
    }

    pub fn get_property_secondary_text(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"secondary-text\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `secondary-text` getter")
        }
    }

    pub fn set_property_secondary_text(&self, secondary_text: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"secondary-text\0".as_ptr() as *const _,
                Value::from(secondary_text).to_glib_none().0,
            );
        }
    }

    pub fn get_property_secondary_use_markup(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"secondary-use-markup\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `secondary-use-markup` getter")
                .unwrap()
        }
    }

    pub fn set_property_secondary_use_markup(&self, secondary_use_markup: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"secondary-use-markup\0".as_ptr() as *const _,
                Value::from(&secondary_use_markup).to_glib_none().0,
            );
        }
    }

    pub fn get_property_text(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"text\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `text` getter")
        }
    }

    pub fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"text\0".as_ptr() as *const _,
                Value::from(text).to_glib_none().0,
            );
        }
    }

    pub fn get_property_use_markup(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"use-markup\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `use-markup` getter")
                .unwrap()
        }
    }

    pub fn set_property_use_markup(&self, use_markup: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"use-markup\0".as_ptr() as *const _,
                Value::from(&use_markup).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_message_area_notify<F: Fn(&MessageDialog) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_message_area_trampoline<F: Fn(&MessageDialog) + 'static>(
            this: *mut gtk_sys::GtkMessageDialog,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::message-area\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_message_area_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_message_type_notify<F: Fn(&MessageDialog) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_message_type_trampoline<F: Fn(&MessageDialog) + 'static>(
            this: *mut gtk_sys::GtkMessageDialog,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::message-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_message_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_secondary_text_notify<F: Fn(&MessageDialog) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_secondary_text_trampoline<F: Fn(&MessageDialog) + 'static>(
            this: *mut gtk_sys::GtkMessageDialog,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::secondary-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_secondary_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_secondary_use_markup_notify<F: Fn(&MessageDialog) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_secondary_use_markup_trampoline<
            F: Fn(&MessageDialog) + 'static,
        >(
            this: *mut gtk_sys::GtkMessageDialog,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::secondary-use-markup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_secondary_use_markup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_text_notify<F: Fn(&MessageDialog) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<F: Fn(&MessageDialog) + 'static>(
            this: *mut gtk_sys::GtkMessageDialog,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_use_markup_notify<F: Fn(&MessageDialog) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_markup_trampoline<F: Fn(&MessageDialog) + 'static>(
            this: *mut gtk_sys::GtkMessageDialog,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-markup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_markup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for MessageDialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MessageDialog")
    }
}
