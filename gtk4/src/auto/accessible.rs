// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::AccessibleProperty;
use crate::AccessibleRelation;
use crate::AccessibleRole;
use crate::AccessibleState;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct Accessible(Interface<ffi::GtkAccessible, ffi::GtkAccessibleInterface>);

    match fn {
        type_ => || ffi::gtk_accessible_get_type(),
    }
}

pub const NONE_ACCESSIBLE: Option<&Accessible> = None;

pub trait AccessibleExt: 'static {
    #[doc(alias = "gtk_accessible_get_accessible_role")]
    fn accessible_role(&self) -> AccessibleRole;

    #[doc(alias = "gtk_accessible_reset_property")]
    fn reset_property(&self, property: AccessibleProperty);

    #[doc(alias = "gtk_accessible_reset_relation")]
    fn reset_relation(&self, relation: AccessibleRelation);

    #[doc(alias = "gtk_accessible_reset_state")]
    fn reset_state(&self, state: AccessibleState);

    #[doc(alias = "set_property_accessible_role")]
    fn set_accessible_role(&self, accessible_role: AccessibleRole);

    fn connect_property_accessible_role_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Accessible>> AccessibleExt for O {
    fn accessible_role(&self) -> AccessibleRole {
        unsafe {
            from_glib(ffi::gtk_accessible_get_accessible_role(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn reset_property(&self, property: AccessibleProperty) {
        unsafe {
            ffi::gtk_accessible_reset_property(
                self.as_ref().to_glib_none().0,
                property.into_glib(),
            );
        }
    }

    fn reset_relation(&self, relation: AccessibleRelation) {
        unsafe {
            ffi::gtk_accessible_reset_relation(
                self.as_ref().to_glib_none().0,
                relation.into_glib(),
            );
        }
    }

    fn reset_state(&self, state: AccessibleState) {
        unsafe {
            ffi::gtk_accessible_reset_state(self.as_ref().to_glib_none().0, state.into_glib());
        }
    }

    fn set_accessible_role(&self, accessible_role: AccessibleRole) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"accessible-role\0".as_ptr() as *const _,
                accessible_role.to_value().to_glib_none().0,
            );
        }
    }

    fn connect_property_accessible_role_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_role_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkAccessible,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Accessible>,
        {
            let f: &F = &*(f as *const F);
            f(&Accessible::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-role\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_role_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Accessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Accessible")
    }
}
