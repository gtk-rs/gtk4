// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct Actionable(Interface<ffi::GtkActionable, ffi::GtkActionableInterface>) @requires Widget, Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_actionable_get_type(),
    }
}

pub const NONE_ACTIONABLE: Option<&Actionable> = None;

pub trait ActionableExt: 'static {
    #[doc(alias = "gtk_actionable_get_action_name")]
    #[doc(alias = "get_action_name")]
    fn action_name(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_actionable_get_action_target_value")]
    #[doc(alias = "get_action_target_value")]
    fn action_target_value(&self) -> Option<glib::Variant>;

    #[doc(alias = "gtk_actionable_set_action_name")]
    fn set_action_name(&self, action_name: Option<&str>);

    #[doc(alias = "gtk_actionable_set_action_target_value")]
    fn set_action_target_value(&self, target_value: Option<&glib::Variant>);

    #[doc(alias = "gtk_actionable_set_detailed_action_name")]
    fn set_detailed_action_name(&self, detailed_action_name: &str);

    #[doc(alias = "action-name")]
    fn connect_action_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "action-target")]
    fn connect_action_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Actionable>> ActionableExt for O {
    fn action_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_actionable_get_action_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn action_target_value(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::gtk_actionable_get_action_target_value(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_action_name(&self, action_name: Option<&str>) {
        unsafe {
            ffi::gtk_actionable_set_action_name(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
            );
        }
    }

    fn set_action_target_value(&self, target_value: Option<&glib::Variant>) {
        unsafe {
            ffi::gtk_actionable_set_action_target_value(
                self.as_ref().to_glib_none().0,
                target_value.to_glib_none().0,
            );
        }
    }

    fn set_detailed_action_name(&self, detailed_action_name: &str) {
        unsafe {
            ffi::gtk_actionable_set_detailed_action_name(
                self.as_ref().to_glib_none().0,
                detailed_action_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "action-name")]
    fn connect_action_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_action_name_trampoline<
            P: IsA<Actionable>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkActionable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Actionable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::action-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_action_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "action-target")]
    fn connect_action_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_action_target_trampoline<
            P: IsA<Actionable>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkActionable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Actionable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::action-target\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_action_target_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Actionable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Actionable")
    }
}
