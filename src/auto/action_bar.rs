// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Buildable;
use Container;
use Widget;

glib_wrapper! {
    pub struct ActionBar(Object<gtk_sys::GtkActionBar, ActionBarClass>) @extends Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_action_bar_get_type(),
    }
}

impl ActionBar {
    pub fn new() -> ActionBar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_action_bar_new()).unsafe_cast() }
    }

    pub fn get_center_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_action_bar_get_center_widget(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_revealed(&self) -> bool {
        unsafe { from_glib(gtk_sys::gtk_action_bar_get_revealed(self.to_glib_none().0)) }
    }

    pub fn pack_end<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            gtk_sys::gtk_action_bar_pack_end(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn pack_start<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            gtk_sys::gtk_action_bar_pack_start(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn set_center_widget<P: IsA<Widget>>(&self, center_widget: Option<&P>) {
        unsafe {
            gtk_sys::gtk_action_bar_set_center_widget(
                self.to_glib_none().0,
                center_widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn set_revealed(&self, revealed: bool) {
        unsafe {
            gtk_sys::gtk_action_bar_set_revealed(self.to_glib_none().0, revealed.to_glib());
        }
    }

    pub fn connect_property_revealed_notify<F: Fn(&ActionBar) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_revealed_trampoline<F: Fn(&ActionBar) + 'static>(
            this: *mut gtk_sys::GtkActionBar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::revealed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_revealed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for ActionBar {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ActionBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActionBar")
    }
}
