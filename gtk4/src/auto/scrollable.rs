// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Adjustment;
use crate::Border;
use crate::ScrollablePolicy;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct Scrollable(Interface<ffi::GtkScrollable, ffi::GtkScrollableInterface>);

    match fn {
        type_ => || ffi::gtk_scrollable_get_type(),
    }
}

pub const NONE_SCROLLABLE: Option<&Scrollable> = None;

pub trait ScrollableExt: 'static {
    #[doc(alias = "gtk_scrollable_get_border")]
    #[doc(alias = "get_border")]
    fn border(&self) -> Option<Border>;

    #[doc(alias = "gtk_scrollable_get_hadjustment")]
    #[doc(alias = "get_hadjustment")]
    fn hadjustment(&self) -> Option<Adjustment>;

    #[doc(alias = "gtk_scrollable_get_hscroll_policy")]
    #[doc(alias = "get_hscroll_policy")]
    fn hscroll_policy(&self) -> ScrollablePolicy;

    #[doc(alias = "gtk_scrollable_get_vadjustment")]
    #[doc(alias = "get_vadjustment")]
    fn vadjustment(&self) -> Option<Adjustment>;

    #[doc(alias = "gtk_scrollable_get_vscroll_policy")]
    #[doc(alias = "get_vscroll_policy")]
    fn vscroll_policy(&self) -> ScrollablePolicy;

    #[doc(alias = "gtk_scrollable_set_hadjustment")]
    fn set_hadjustment<P: IsA<Adjustment>>(&self, hadjustment: Option<&P>);

    #[doc(alias = "gtk_scrollable_set_hscroll_policy")]
    fn set_hscroll_policy(&self, policy: ScrollablePolicy);

    #[doc(alias = "gtk_scrollable_set_vadjustment")]
    fn set_vadjustment<P: IsA<Adjustment>>(&self, vadjustment: Option<&P>);

    #[doc(alias = "gtk_scrollable_set_vscroll_policy")]
    fn set_vscroll_policy(&self, policy: ScrollablePolicy);

    #[doc(alias = "hadjustment")]
    fn connect_hadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "hscroll-policy")]
    fn connect_hscroll_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "vadjustment")]
    fn connect_vadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "vscroll-policy")]
    fn connect_vscroll_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Scrollable>> ScrollableExt for O {
    fn border(&self) -> Option<Border> {
        unsafe {
            let mut border = Border::uninitialized();
            let ret = from_glib(ffi::gtk_scrollable_get_border(
                self.as_ref().to_glib_none().0,
                border.to_glib_none_mut().0,
            ));
            if ret {
                Some(border)
            } else {
                None
            }
        }
    }

    fn hadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_scrollable_get_hadjustment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn hscroll_policy(&self) -> ScrollablePolicy {
        unsafe {
            from_glib(ffi::gtk_scrollable_get_hscroll_policy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn vadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_scrollable_get_vadjustment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn vscroll_policy(&self) -> ScrollablePolicy {
        unsafe {
            from_glib(ffi::gtk_scrollable_get_vscroll_policy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_hadjustment<P: IsA<Adjustment>>(&self, hadjustment: Option<&P>) {
        unsafe {
            ffi::gtk_scrollable_set_hadjustment(
                self.as_ref().to_glib_none().0,
                hadjustment.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_hscroll_policy(&self, policy: ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_hscroll_policy(
                self.as_ref().to_glib_none().0,
                policy.into_glib(),
            );
        }
    }

    fn set_vadjustment<P: IsA<Adjustment>>(&self, vadjustment: Option<&P>) {
        unsafe {
            ffi::gtk_scrollable_set_vadjustment(
                self.as_ref().to_glib_none().0,
                vadjustment.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_vscroll_policy(&self, policy: ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_vscroll_policy(
                self.as_ref().to_glib_none().0,
                policy.into_glib(),
            );
        }
    }

    #[doc(alias = "hadjustment")]
    fn connect_hadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hadjustment_trampoline<
            P: IsA<Scrollable>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkScrollable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Scrollable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::hadjustment\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_hadjustment_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "hscroll-policy")]
    fn connect_hscroll_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hscroll_policy_trampoline<
            P: IsA<Scrollable>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkScrollable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Scrollable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::hscroll-policy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_hscroll_policy_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "vadjustment")]
    fn connect_vadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vadjustment_trampoline<
            P: IsA<Scrollable>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkScrollable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Scrollable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::vadjustment\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_vadjustment_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "vscroll-policy")]
    fn connect_vscroll_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vscroll_policy_trampoline<
            P: IsA<Scrollable>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkScrollable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Scrollable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::vscroll-policy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_vscroll_policy_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Scrollable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Scrollable")
    }
}
