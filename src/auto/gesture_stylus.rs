// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use EventController;
use Gesture;
use GestureSingle;

glib_wrapper! {
    pub struct GestureStylus(Object<gtk_sys::GtkGestureStylus, gtk_sys::GtkGestureStylusClass, GestureStylusClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        get_type => || gtk_sys::gtk_gesture_stylus_get_type(),
    }
}

impl GestureStylus {
    pub fn new() -> GestureStylus {
        assert_initialized_main_thread!();
        unsafe { Gesture::from_glib_full(gtk_sys::gtk_gesture_stylus_new()).unsafe_cast() }
    }

    //pub fn get_axes(&self, axes: /*Unimplemented*/&CArray TypeId { ns_id: 11, id: 4 }, values: Vec<f64>) -> bool {
    //    unsafe { TODO: call gtk_sys:gtk_gesture_stylus_get_axes() }
    //}

    //pub fn get_axis(&self, axis: /*Ignored*/gdk::AxisUse) -> Option<f64> {
    //    unsafe { TODO: call gtk_sys:gtk_gesture_stylus_get_axis() }
    //}

    //pub fn get_backlog(&self, backlog: /*Ignored*/Vec<gdk::TimeCoord>) -> Option<u32> {
    //    unsafe { TODO: call gtk_sys:gtk_gesture_stylus_get_backlog() }
    //}

    //pub fn get_device_tool(&self) -> /*Ignored*/Option<gdk::DeviceTool> {
    //    unsafe { TODO: call gtk_sys:gtk_gesture_stylus_get_device_tool() }
    //}

    pub fn connect_down<F: Fn(&GestureStylus, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn down_trampoline<F: Fn(&GestureStylus, f64, f64) + 'static>(
            this: *mut gtk_sys::GtkGestureStylus,
            object: libc::c_double,
            p0: libc::c_double,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object, p0)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"down\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    down_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_motion<F: Fn(&GestureStylus, f64, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn motion_trampoline<F: Fn(&GestureStylus, f64, f64) + 'static>(
            this: *mut gtk_sys::GtkGestureStylus,
            object: libc::c_double,
            p0: libc::c_double,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object, p0)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"motion\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    motion_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_proximity<F: Fn(&GestureStylus, f64, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn proximity_trampoline<F: Fn(&GestureStylus, f64, f64) + 'static>(
            this: *mut gtk_sys::GtkGestureStylus,
            object: libc::c_double,
            p0: libc::c_double,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object, p0)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"proximity\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    proximity_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_up<F: Fn(&GestureStylus, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn up_trampoline<F: Fn(&GestureStylus, f64, f64) + 'static>(
            this: *mut gtk_sys::GtkGestureStylus,
            object: libc::c_double,
            p0: libc::c_double,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object, p0)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"up\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    up_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GestureStylus {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for GestureStylus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GestureStylus")
    }
}
