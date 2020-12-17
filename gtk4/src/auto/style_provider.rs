// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct StyleProvider(Interface<ffi::GtkStyleProvider>);

    match fn {
        get_type => || ffi::gtk_style_provider_get_type(),
    }
}

pub const NONE_STYLE_PROVIDER: Option<&StyleProvider> = None;

pub trait StyleProviderExt: 'static {
    fn connect_gtk_private_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StyleProvider>> StyleProviderExt for O {
    fn connect_gtk_private_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn gtk_private_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkStyleProvider,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<StyleProvider>,
        {
            let f: &F = &*(f as *const F);
            f(&StyleProvider::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"gtk-private-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    gtk_private_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for StyleProvider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StyleProvider")
    }
}
