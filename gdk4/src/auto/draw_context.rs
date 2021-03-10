// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gtk4-gir-files
// DO NOT EDIT

use crate::Display;
use crate::Surface;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct DrawContext(Object<ffi::GdkDrawContext>);

    match fn {
        get_type => || ffi::gdk_draw_context_get_type(),
    }
}

pub const NONE_DRAW_CONTEXT: Option<&DrawContext> = None;

pub trait DrawContextExt: 'static {
    #[doc(alias = "gdk_draw_context_begin_frame")]
    fn begin_frame(&self, region: &cairo::Region);

    #[doc(alias = "gdk_draw_context_end_frame")]
    fn end_frame(&self);

    #[doc(alias = "gdk_draw_context_get_display")]
    fn get_display(&self) -> Option<Display>;

    #[doc(alias = "gdk_draw_context_get_surface")]
    fn get_surface(&self) -> Option<Surface>;

    #[doc(alias = "gdk_draw_context_is_in_frame")]
    fn is_in_frame(&self) -> bool;

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DrawContext>> DrawContextExt for O {
    fn begin_frame(&self, region: &cairo::Region) {
        unsafe {
            ffi::gdk_draw_context_begin_frame(
                self.as_ref().to_glib_none().0,
                region.to_glib_none().0,
            );
        }
    }

    fn end_frame(&self) {
        unsafe {
            ffi::gdk_draw_context_end_frame(self.as_ref().to_glib_none().0);
        }
    }

    fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_draw_context_get_display(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_surface(&self) -> Option<Surface> {
        unsafe {
            from_glib_none(ffi::gdk_draw_context_get_surface(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_in_frame(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_draw_context_is_in_frame(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_display_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkDrawContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DrawContext>,
        {
            let f: &F = &*(f as *const F);
            f(&DrawContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::display\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_display_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DrawContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DrawContext")
    }
}
