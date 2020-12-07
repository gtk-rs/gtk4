// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
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

glib::glib_wrapper! {
    pub struct DropControllerMotion(Object<ffi::GtkDropControllerMotion, ffi::GtkDropControllerMotionClass>) @extends EventController;

    match fn {
        get_type => || ffi::gtk_drop_controller_motion_get_type(),
    }
}

impl DropControllerMotion {
    pub fn new() -> DropControllerMotion {
        assert_initialized_main_thread!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_drop_controller_motion_new()).unsafe_cast()
        }
    }

    pub fn contains_pointer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_drop_controller_motion_contains_pointer(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_drop(&self) -> Option<gdk::Drop> {
        unsafe {
            from_glib_none(ffi::gtk_drop_controller_motion_get_drop(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn is_pointer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_drop_controller_motion_is_pointer(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_property_contains_pointer(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"contains-pointer\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `contains-pointer` getter")
                .unwrap()
        }
    }

    pub fn get_property_is_pointer(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"is-pointer\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `is-pointer` getter")
                .unwrap()
        }
    }

    pub fn connect_enter<F: Fn(&DropControllerMotion, f64, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn enter_trampoline<F: Fn(&DropControllerMotion, f64, f64) + 'static>(
            this: *mut ffi::GtkDropControllerMotion,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
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

    pub fn connect_leave<F: Fn(&DropControllerMotion) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn leave_trampoline<F: Fn(&DropControllerMotion) + 'static>(
            this: *mut ffi::GtkDropControllerMotion,
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

    pub fn connect_motion<F: Fn(&DropControllerMotion, f64, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn motion_trampoline<F: Fn(&DropControllerMotion, f64, f64) + 'static>(
            this: *mut ffi::GtkDropControllerMotion,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
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

    pub fn connect_property_contains_pointer_notify<F: Fn(&DropControllerMotion) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_contains_pointer_trampoline<
            F: Fn(&DropControllerMotion) + 'static,
        >(
            this: *mut ffi::GtkDropControllerMotion,
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
                b"notify::contains-pointer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_contains_pointer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_drop_notify<F: Fn(&DropControllerMotion) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_drop_trampoline<F: Fn(&DropControllerMotion) + 'static>(
            this: *mut ffi::GtkDropControllerMotion,
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
                b"notify::drop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_drop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_is_pointer_notify<F: Fn(&DropControllerMotion) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_pointer_trampoline<
            F: Fn(&DropControllerMotion) + 'static,
        >(
            this: *mut ffi::GtkDropControllerMotion,
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
                b"notify::is-pointer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_pointer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for DropControllerMotion {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DropControllerMotion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DropControllerMotion")
    }
}
