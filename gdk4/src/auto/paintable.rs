// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gtk4-gir-files
// DO NOT EDIT

use crate::PaintableFlags;
use crate::Snapshot;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    pub struct Paintable(Interface<ffi::GdkPaintable, ffi::GdkPaintableInterface>);

    match fn {
        get_type => || ffi::gdk_paintable_get_type(),
    }
}

impl Paintable {
    #[doc(alias = "gdk_paintable_new_empty")]
    pub fn new_empty(intrinsic_width: i32, intrinsic_height: i32) -> Option<Paintable> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gdk_paintable_new_empty(
                intrinsic_width,
                intrinsic_height,
            ))
        }
    }
}

pub const NONE_PAINTABLE: Option<&Paintable> = None;

pub trait PaintableExt: 'static {
    #[doc(alias = "gdk_paintable_compute_concrete_size")]
    fn compute_concrete_size(
        &self,
        specified_width: f64,
        specified_height: f64,
        default_width: f64,
        default_height: f64,
    ) -> (f64, f64);

    #[doc(alias = "gdk_paintable_get_current_image")]
    fn get_current_image(&self) -> Option<Paintable>;

    #[doc(alias = "gdk_paintable_get_flags")]
    fn get_flags(&self) -> PaintableFlags;

    #[doc(alias = "gdk_paintable_get_intrinsic_aspect_ratio")]
    fn get_intrinsic_aspect_ratio(&self) -> f64;

    #[doc(alias = "gdk_paintable_get_intrinsic_height")]
    fn get_intrinsic_height(&self) -> i32;

    #[doc(alias = "gdk_paintable_get_intrinsic_width")]
    fn get_intrinsic_width(&self) -> i32;

    #[doc(alias = "gdk_paintable_invalidate_contents")]
    fn invalidate_contents(&self);

    #[doc(alias = "gdk_paintable_invalidate_size")]
    fn invalidate_size(&self);

    #[doc(alias = "gdk_paintable_snapshot")]
    fn snapshot(&self, snapshot: &Snapshot, width: f64, height: f64);

    fn connect_invalidate_contents<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_invalidate_size<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Paintable>> PaintableExt for O {
    fn compute_concrete_size(
        &self,
        specified_width: f64,
        specified_height: f64,
        default_width: f64,
        default_height: f64,
    ) -> (f64, f64) {
        unsafe {
            let mut concrete_width = mem::MaybeUninit::uninit();
            let mut concrete_height = mem::MaybeUninit::uninit();
            ffi::gdk_paintable_compute_concrete_size(
                self.as_ref().to_glib_none().0,
                specified_width,
                specified_height,
                default_width,
                default_height,
                concrete_width.as_mut_ptr(),
                concrete_height.as_mut_ptr(),
            );
            let concrete_width = concrete_width.assume_init();
            let concrete_height = concrete_height.assume_init();
            (concrete_width, concrete_height)
        }
    }

    fn get_current_image(&self) -> Option<Paintable> {
        unsafe {
            from_glib_full(ffi::gdk_paintable_get_current_image(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_flags(&self) -> PaintableFlags {
        unsafe { from_glib(ffi::gdk_paintable_get_flags(self.as_ref().to_glib_none().0)) }
    }

    fn get_intrinsic_aspect_ratio(&self) -> f64 {
        unsafe { ffi::gdk_paintable_get_intrinsic_aspect_ratio(self.as_ref().to_glib_none().0) }
    }

    fn get_intrinsic_height(&self) -> i32 {
        unsafe { ffi::gdk_paintable_get_intrinsic_height(self.as_ref().to_glib_none().0) }
    }

    fn get_intrinsic_width(&self) -> i32 {
        unsafe { ffi::gdk_paintable_get_intrinsic_width(self.as_ref().to_glib_none().0) }
    }

    fn invalidate_contents(&self) {
        unsafe {
            ffi::gdk_paintable_invalidate_contents(self.as_ref().to_glib_none().0);
        }
    }

    fn invalidate_size(&self) {
        unsafe {
            ffi::gdk_paintable_invalidate_size(self.as_ref().to_glib_none().0);
        }
    }

    fn snapshot(&self, snapshot: &Snapshot, width: f64, height: f64) {
        unsafe {
            ffi::gdk_paintable_snapshot(
                self.as_ref().to_glib_none().0,
                snapshot.to_glib_none().0,
                width,
                height,
            );
        }
    }

    fn connect_invalidate_contents<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn invalidate_contents_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkPaintable,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Paintable>,
        {
            let f: &F = &*(f as *const F);
            f(&Paintable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"invalidate-contents\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    invalidate_contents_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_invalidate_size<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn invalidate_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkPaintable,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Paintable>,
        {
            let f: &F = &*(f as *const F);
            f(&Paintable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"invalidate-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    invalidate_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Paintable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Paintable")
    }
}
