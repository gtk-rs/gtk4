// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
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
    pub struct ColorChooserWidget(Object<gtk_sys::GtkColorChooserWidget, ColorChooserWidgetClass>) @extends Widget, @implements Buildable, ColorChooser;

    match fn {
        get_type => || gtk_sys::gtk_color_chooser_widget_get_type(),
    }
}

impl ColorChooserWidget {
    pub fn new() -> ColorChooserWidget {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_color_chooser_widget_new()).unsafe_cast() }
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

    pub fn connect_property_show_editor_notify<F: Fn(&ColorChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_editor_trampoline<F: Fn(&ColorChooserWidget) + 'static>(
            this: *mut gtk_sys::GtkColorChooserWidget,
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
}

impl Default for ColorChooserWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ColorChooserWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ColorChooserWidget")
    }
}
