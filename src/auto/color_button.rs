// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
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
use Buildable;
use ColorChooser;
use Widget;

glib_wrapper! {
    pub struct ColorButton(Object<gtk_sys::GtkColorButton, ColorButtonClass>) @extends Widget, @implements Buildable, ColorChooser;

    match fn {
        get_type => || gtk_sys::gtk_color_button_get_type(),
    }
}

impl ColorButton {
    pub fn new() -> ColorButton {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_color_button_new()).unsafe_cast() }
    }

    //pub fn with_rgba(rgba: /*Ignored*/&gdk::RGBA) -> ColorButton {
    //    unsafe { TODO: call gtk_sys:gtk_color_button_new_with_rgba() }
    //}

    pub fn get_title(&self) -> Option<GString> {
        unsafe { from_glib_none(gtk_sys::gtk_color_button_get_title(self.to_glib_none().0)) }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            gtk_sys::gtk_color_button_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    pub fn get_property_show_editor(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"show-editor\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `show-editor` getter")
                .unwrap()
        }
    }

    pub fn set_property_show_editor(&self, show_editor: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"show-editor\0".as_ptr() as *const _,
                Value::from(&show_editor).to_glib_none().0,
            );
        }
    }

    pub fn connect_color_set<F: Fn(&ColorButton) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn color_set_trampoline<F: Fn(&ColorButton) + 'static>(
            this: *mut gtk_sys::GtkColorButton,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"color-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    color_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_rgba_notify<F: Fn(&ColorButton) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_rgba_trampoline<F: Fn(&ColorButton) + 'static>(
            this: *mut gtk_sys::GtkColorButton,
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
                b"notify::rgba\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_rgba_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_show_editor_notify<F: Fn(&ColorButton) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_editor_trampoline<F: Fn(&ColorButton) + 'static>(
            this: *mut gtk_sys::GtkColorButton,
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
                b"notify::show-editor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_editor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_title_notify<F: Fn(&ColorButton) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&ColorButton) + 'static>(
            this: *mut gtk_sys::GtkColorButton,
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
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_use_alpha_notify<F: Fn(&ColorButton) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_alpha_trampoline<F: Fn(&ColorButton) + 'static>(
            this: *mut gtk_sys::GtkColorButton,
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
                b"notify::use-alpha\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_alpha_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for ColorButton {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ColorButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ColorButton")
    }
}
