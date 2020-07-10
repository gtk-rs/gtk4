// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use EventController;
use Gesture;

glib_wrapper! {
    pub struct GestureSingle(Object<gtk_sys::GtkGestureSingle, gtk_sys::GtkGestureSingleClass, GestureSingleClass>) @extends Gesture, EventController;

    match fn {
        get_type => || gtk_sys::gtk_gesture_single_get_type(),
    }
}

pub const NONE_GESTURE_SINGLE: Option<&GestureSingle> = None;

pub trait GestureSingleExt: 'static {
    fn get_button(&self) -> u32;

    fn get_current_button(&self) -> u32;

    //fn get_current_sequence(&self) -> /*Ignored*/Option<gdk::EventSequence>;

    fn get_exclusive(&self) -> bool;

    fn get_touch_only(&self) -> bool;

    fn set_button(&self, button: u32);

    fn set_exclusive(&self, exclusive: bool);

    fn set_touch_only(&self, touch_only: bool);

    fn connect_property_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_exclusive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_touch_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GestureSingle>> GestureSingleExt for O {
    fn get_button(&self) -> u32 {
        unsafe { gtk_sys::gtk_gesture_single_get_button(self.as_ref().to_glib_none().0) }
    }

    fn get_current_button(&self) -> u32 {
        unsafe { gtk_sys::gtk_gesture_single_get_current_button(self.as_ref().to_glib_none().0) }
    }

    //fn get_current_sequence(&self) -> /*Ignored*/Option<gdk::EventSequence> {
    //    unsafe { TODO: call gtk_sys:gtk_gesture_single_get_current_sequence() }
    //}

    fn get_exclusive(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_gesture_single_get_exclusive(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_touch_only(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_gesture_single_get_touch_only(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_button(&self, button: u32) {
        unsafe {
            gtk_sys::gtk_gesture_single_set_button(self.as_ref().to_glib_none().0, button);
        }
    }

    fn set_exclusive(&self, exclusive: bool) {
        unsafe {
            gtk_sys::gtk_gesture_single_set_exclusive(
                self.as_ref().to_glib_none().0,
                exclusive.to_glib(),
            );
        }
    }

    fn set_touch_only(&self, touch_only: bool) {
        unsafe {
            gtk_sys::gtk_gesture_single_set_touch_only(
                self.as_ref().to_glib_none().0,
                touch_only.to_glib(),
            );
        }
    }

    fn connect_property_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_button_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkGestureSingle,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GestureSingle>,
        {
            let f: &F = &*(f as *const F);
            f(&GestureSingle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::button\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_button_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_exclusive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_exclusive_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkGestureSingle,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GestureSingle>,
        {
            let f: &F = &*(f as *const F);
            f(&GestureSingle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::exclusive\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_exclusive_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_touch_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_touch_only_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkGestureSingle,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GestureSingle>,
        {
            let f: &F = &*(f as *const F);
            f(&GestureSingle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::touch-only\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_touch_only_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for GestureSingle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GestureSingle")
    }
}
