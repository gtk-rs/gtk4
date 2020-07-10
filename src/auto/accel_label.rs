// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use Buildable;
use Widget;

glib_wrapper! {
    pub struct AccelLabel(Object<gtk_sys::GtkAccelLabel, AccelLabelClass>) @extends Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_accel_label_get_type(),
    }
}

impl AccelLabel {
    pub fn new(string: &str) -> AccelLabel {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_accel_label_new(string.to_glib_none().0))
                .unsafe_cast()
        }
    }

    pub fn get_accel(&self) -> (u32, gdk::ModifierType) {
        unsafe {
            let mut accelerator_key = mem::MaybeUninit::uninit();
            let mut accelerator_mods = mem::MaybeUninit::uninit();
            gtk_sys::gtk_accel_label_get_accel(
                self.to_glib_none().0,
                accelerator_key.as_mut_ptr(),
                accelerator_mods.as_mut_ptr(),
            );
            let accelerator_key = accelerator_key.assume_init();
            let accelerator_mods = accelerator_mods.assume_init();
            (accelerator_key, from_glib(accelerator_mods))
        }
    }

    pub fn get_accel_width(&self) -> u32 {
        unsafe { gtk_sys::gtk_accel_label_get_accel_width(self.to_glib_none().0) }
    }

    pub fn get_label(&self) -> Option<GString> {
        unsafe { from_glib_none(gtk_sys::gtk_accel_label_get_label(self.to_glib_none().0)) }
    }

    pub fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_accel_label_get_use_underline(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn refetch(&self) -> bool {
        unsafe { from_glib(gtk_sys::gtk_accel_label_refetch(self.to_glib_none().0)) }
    }

    pub fn set_accel(&self, accelerator_key: u32, accelerator_mods: gdk::ModifierType) {
        unsafe {
            gtk_sys::gtk_accel_label_set_accel(
                self.to_glib_none().0,
                accelerator_key,
                accelerator_mods.to_glib(),
            );
        }
    }

    pub fn set_label(&self, text: &str) {
        unsafe {
            gtk_sys::gtk_accel_label_set_label(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    pub fn set_use_underline(&self, setting: bool) {
        unsafe {
            gtk_sys::gtk_accel_label_set_use_underline(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn connect_property_label_notify<F: Fn(&AccelLabel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<F: Fn(&AccelLabel) + 'static>(
            this: *mut gtk_sys::GtkAccelLabel,
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
                b"notify::label\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_label_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_use_underline_notify<F: Fn(&AccelLabel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_underline_trampoline<F: Fn(&AccelLabel) + 'static>(
            this: *mut gtk_sys::GtkAccelLabel,
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
                b"notify::use-underline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_underline_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for AccelLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AccelLabel")
    }
}
