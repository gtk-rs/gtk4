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
use LayoutChild;

glib_wrapper! {
    pub struct FixedLayoutChild(Object<gtk_sys::GtkFixedLayoutChild, gtk_sys::GtkFixedLayoutChildClass, FixedLayoutChildClass>) @extends LayoutChild;

    match fn {
        get_type => || gtk_sys::gtk_fixed_layout_child_get_type(),
    }
}

pub const NONE_FIXED_LAYOUT_CHILD: Option<&FixedLayoutChild> = None;

pub trait FixedLayoutChildExt: 'static {
    //fn get_transform(&self) -> /*Ignored*/Option<gsk::Transform>;

    //fn set_transform(&self, transform: /*Ignored*/&gsk::Transform);

    fn connect_property_transform_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FixedLayoutChild>> FixedLayoutChildExt for O {
    //fn get_transform(&self) -> /*Ignored*/Option<gsk::Transform> {
    //    unsafe { TODO: call gtk_sys:gtk_fixed_layout_child_get_transform() }
    //}

    //fn set_transform(&self, transform: /*Ignored*/&gsk::Transform) {
    //    unsafe { TODO: call gtk_sys:gtk_fixed_layout_child_set_transform() }
    //}

    fn connect_property_transform_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transform_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFixedLayoutChild,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FixedLayoutChild>,
        {
            let f: &F = &*(f as *const F);
            f(&FixedLayoutChild::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::transform\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transform_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FixedLayoutChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FixedLayoutChild")
    }
}
