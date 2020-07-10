// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Bin;
use Buildable;
use Container;
use PositionType;
use Widget;

glib_wrapper! {
    pub struct Popover(Object<gtk_sys::GtkPopover, gtk_sys::GtkPopoverClass, PopoverClass>) @extends Bin, Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_popover_get_type(),
    }
}

impl Popover {
    pub fn new() -> Popover {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_popover_new()).unsafe_cast() }
    }
}

impl Default for Popover {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_POPOVER: Option<&Popover> = None;

pub trait PopoverExt: 'static {
    fn get_autohide(&self) -> bool;

    fn get_has_arrow(&self) -> bool;

    fn get_mnemonics_visible(&self) -> bool;

    fn get_pointing_to(&self) -> Option<gdk::Rectangle>;

    fn get_position(&self) -> PositionType;

    fn popdown(&self);

    fn popup(&self);

    fn set_autohide(&self, autohide: bool);

    fn set_default_widget<P: IsA<Widget>>(&self, widget: &P);

    fn set_has_arrow(&self, has_arrow: bool);

    fn set_mnemonics_visible(&self, mnemonics_visible: bool);

    fn set_pointing_to(&self, rect: &gdk::Rectangle);

    fn set_position(&self, position: PositionType);

    fn get_property_default_widget(&self) -> Option<Widget>;

    fn connect_activate_default<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate_default(&self);

    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_autohide_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_default_widget_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_has_arrow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mnemonics_visible_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_pointing_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Popover>> PopoverExt for O {
    fn get_autohide(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_popover_get_autohide(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_has_arrow(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_popover_get_has_arrow(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_mnemonics_visible(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_popover_get_mnemonics_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_pointing_to(&self) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(gtk_sys::gtk_popover_get_pointing_to(
                self.as_ref().to_glib_none().0,
                rect.to_glib_none_mut().0,
            ));
            if ret {
                Some(rect)
            } else {
                None
            }
        }
    }

    fn get_position(&self) -> PositionType {
        unsafe {
            from_glib(gtk_sys::gtk_popover_get_position(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn popdown(&self) {
        unsafe {
            gtk_sys::gtk_popover_popdown(self.as_ref().to_glib_none().0);
        }
    }

    fn popup(&self) {
        unsafe {
            gtk_sys::gtk_popover_popup(self.as_ref().to_glib_none().0);
        }
    }

    fn set_autohide(&self, autohide: bool) {
        unsafe {
            gtk_sys::gtk_popover_set_autohide(self.as_ref().to_glib_none().0, autohide.to_glib());
        }
    }

    fn set_default_widget<P: IsA<Widget>>(&self, widget: &P) {
        unsafe {
            gtk_sys::gtk_popover_set_default_widget(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_has_arrow(&self, has_arrow: bool) {
        unsafe {
            gtk_sys::gtk_popover_set_has_arrow(self.as_ref().to_glib_none().0, has_arrow.to_glib());
        }
    }

    fn set_mnemonics_visible(&self, mnemonics_visible: bool) {
        unsafe {
            gtk_sys::gtk_popover_set_mnemonics_visible(
                self.as_ref().to_glib_none().0,
                mnemonics_visible.to_glib(),
            );
        }
    }

    fn set_pointing_to(&self, rect: &gdk::Rectangle) {
        unsafe {
            gtk_sys::gtk_popover_set_pointing_to(
                self.as_ref().to_glib_none().0,
                rect.to_glib_none().0,
            );
        }
    }

    fn set_position(&self, position: PositionType) {
        unsafe {
            gtk_sys::gtk_popover_set_position(self.as_ref().to_glib_none().0, position.to_glib());
        }
    }

    fn get_property_default_widget(&self) -> Option<Widget> {
        unsafe {
            let mut value = Value::from_type(<Widget as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"default-widget\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `default-widget` getter")
        }
    }

    fn connect_activate_default<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_default_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPopover,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Popover>,
        {
            let f: &F = &*(f as *const F);
            f(&Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate-default\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_default_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_activate_default(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("activate-default", &[])
                .unwrap()
        };
    }

    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn closed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPopover,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Popover>,
        {
            let f: &F = &*(f as *const F);
            f(&Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"closed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    closed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_autohide_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_autohide_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPopover,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Popover>,
        {
            let f: &F = &*(f as *const F);
            f(&Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::autohide\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_autohide_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_default_widget_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_widget_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPopover,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Popover>,
        {
            let f: &F = &*(f as *const F);
            f(&Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::default-widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_default_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_has_arrow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_arrow_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPopover,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Popover>,
        {
            let f: &F = &*(f as *const F);
            f(&Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-arrow\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_arrow_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_mnemonics_visible_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mnemonics_visible_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPopover,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Popover>,
        {
            let f: &F = &*(f as *const F);
            f(&Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mnemonics-visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mnemonics_visible_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_pointing_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pointing_to_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPopover,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Popover>,
        {
            let f: &F = &*(f as *const F);
            f(&Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pointing-to\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pointing_to_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPopover,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Popover>,
        {
            let f: &F = &*(f as *const F);
            f(&Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_position_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Popover {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Popover")
    }
}
