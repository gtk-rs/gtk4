// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Buildable;
use Widget;

glib_wrapper! {
    pub struct Statusbar(Object<gtk_sys::GtkStatusbar, StatusbarClass>) @extends Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_statusbar_get_type(),
    }
}

impl Statusbar {
    pub fn new() -> Statusbar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_statusbar_new()).unsafe_cast() }
    }

    pub fn get_context_id(&self, context_description: &str) -> u32 {
        unsafe {
            gtk_sys::gtk_statusbar_get_context_id(
                self.to_glib_none().0,
                context_description.to_glib_none().0,
            )
        }
    }

    pub fn pop(&self, context_id: u32) {
        unsafe {
            gtk_sys::gtk_statusbar_pop(self.to_glib_none().0, context_id);
        }
    }

    pub fn push(&self, context_id: u32, text: &str) -> u32 {
        unsafe {
            gtk_sys::gtk_statusbar_push(self.to_glib_none().0, context_id, text.to_glib_none().0)
        }
    }

    pub fn remove(&self, context_id: u32, message_id: u32) {
        unsafe {
            gtk_sys::gtk_statusbar_remove(self.to_glib_none().0, context_id, message_id);
        }
    }

    pub fn remove_all(&self, context_id: u32) {
        unsafe {
            gtk_sys::gtk_statusbar_remove_all(self.to_glib_none().0, context_id);
        }
    }

    pub fn connect_text_popped<F: Fn(&Statusbar, u32, &str) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn text_popped_trampoline<F: Fn(&Statusbar, u32, &str) + 'static>(
            this: *mut gtk_sys::GtkStatusbar,
            context_id: libc::c_uint,
            text: *mut libc::c_char,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                context_id,
                &GString::from_glib_borrow(text),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"text-popped\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    text_popped_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_text_pushed<F: Fn(&Statusbar, u32, &str) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn text_pushed_trampoline<F: Fn(&Statusbar, u32, &str) + 'static>(
            this: *mut gtk_sys::GtkStatusbar,
            context_id: libc::c_uint,
            text: *mut libc::c_char,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                context_id,
                &GString::from_glib_borrow(text),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"text-pushed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    text_pushed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Statusbar {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Statusbar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Statusbar")
    }
}
