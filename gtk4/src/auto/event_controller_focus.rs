// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gtk4-gir-files
// DO NOT EDIT

use crate::EventController;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct EventControllerFocus(Object<ffi::GtkEventControllerFocus, ffi::GtkEventControllerFocusClass>) @extends EventController;

    match fn {
        get_type => || ffi::gtk_event_controller_focus_get_type(),
    }
}

impl EventControllerFocus {
    #[doc(alias = "gtk_event_controller_focus_new")]
    pub fn new() -> EventControllerFocus {
        assert_initialized_main_thread!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_event_controller_focus_new()).unsafe_cast()
        }
    }

    #[doc(alias = "gtk_event_controller_focus_contains_focus")]
    pub fn contains_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_controller_focus_contains_focus(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_event_controller_focus_is_focus")]
    pub fn is_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_controller_focus_is_focus(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_property_contains_focus(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"contains-focus\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `contains-focus` getter")
                .unwrap()
        }
    }

    pub fn get_property_is_focus(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"is-focus\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `is-focus` getter")
                .unwrap()
        }
    }

    pub fn connect_enter<F: Fn(&EventControllerFocus) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn enter_trampoline<F: Fn(&EventControllerFocus) + 'static>(
            this: *mut ffi::GtkEventControllerFocus,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"enter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    enter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_leave<F: Fn(&EventControllerFocus) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn leave_trampoline<F: Fn(&EventControllerFocus) + 'static>(
            this: *mut ffi::GtkEventControllerFocus,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"leave\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    leave_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_contains_focus_notify<F: Fn(&EventControllerFocus) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_contains_focus_trampoline<
            F: Fn(&EventControllerFocus) + 'static,
        >(
            this: *mut ffi::GtkEventControllerFocus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::contains-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_contains_focus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_is_focus_notify<F: Fn(&EventControllerFocus) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_focus_trampoline<F: Fn(&EventControllerFocus) + 'static>(
            this: *mut ffi::GtkEventControllerFocus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_focus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for EventControllerFocus {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EventControllerFocus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EventControllerFocus")
    }
}
