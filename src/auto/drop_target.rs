// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gdk_sys;
use glib;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use EventController;

glib_wrapper! {
    pub struct DropTarget(Object<gtk_sys::GtkDropTarget, gtk_sys::GtkDropTargetClass, DropTargetClass>) @extends EventController;

    match fn {
        get_type => || gtk_sys::gtk_drop_target_get_type(),
    }
}

impl DropTarget {
    pub fn new(type_: glib::types::Type, actions: gdk::DragAction) -> DropTarget {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_drop_target_new(
                type_.to_glib(),
                actions.to_glib(),
            ))
        }
    }

    pub fn get_actions(&self) -> gdk::DragAction {
        unsafe { from_glib(gtk_sys::gtk_drop_target_get_actions(self.to_glib_none().0)) }
    }

    pub fn get_drop(&self) -> Option<gdk::Drop> {
        unsafe { from_glib_none(gtk_sys::gtk_drop_target_get_drop(self.to_glib_none().0)) }
    }

    pub fn get_formats(&self) -> Option<gdk::ContentFormats> {
        unsafe { from_glib_full(gtk_sys::gtk_drop_target_get_formats(self.to_glib_none().0)) }
    }

    //pub fn get_gtypes(&self) -> /*Unimplemented*/Option<CArray TypeId { ns_id: 0, id: 30 }> {
    //    unsafe { TODO: call gtk_sys:gtk_drop_target_get_gtypes() }
    //}

    pub fn get_preload(&self) -> bool {
        unsafe { from_glib(gtk_sys::gtk_drop_target_get_preload(self.to_glib_none().0)) }
    }

    pub fn get_value(&self) -> Option<glib::Value> {
        unsafe { from_glib_none(gtk_sys::gtk_drop_target_get_value(self.to_glib_none().0)) }
    }

    pub fn reject(&self) {
        unsafe {
            gtk_sys::gtk_drop_target_reject(self.to_glib_none().0);
        }
    }

    pub fn set_actions(&self, actions: gdk::DragAction) {
        unsafe {
            gtk_sys::gtk_drop_target_set_actions(self.to_glib_none().0, actions.to_glib());
        }
    }

    //pub fn set_gtypes(&self, types: /*Unimplemented*/Option<&CArray TypeId { ns_id: 0, id: 30 }>) {
    //    unsafe { TODO: call gtk_sys:gtk_drop_target_set_gtypes() }
    //}

    pub fn set_preload(&self, preload: bool) {
        unsafe {
            gtk_sys::gtk_drop_target_set_preload(self.to_glib_none().0, preload.to_glib());
        }
    }

    pub fn connect_accept<F: Fn(&DropTarget, &gdk::Drop) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn accept_trampoline<F: Fn(&DropTarget, &gdk::Drop) -> bool + 'static>(
            this: *mut gtk_sys::GtkDropTarget,
            drop: *mut gdk_sys::GdkDrop,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(drop)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accept\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    accept_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_drop<F: Fn(&DropTarget, &glib::Value, f64, f64) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drop_trampoline<
            F: Fn(&DropTarget, &glib::Value, f64, f64) -> bool + 'static,
        >(
            this: *mut gtk_sys::GtkDropTarget,
            value: *mut gobject_sys::GValue,
            x: libc::c_double,
            y: libc::c_double,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(value), x, y).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_enter<F: Fn(&DropTarget, f64, f64) -> gdk::DragAction + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn enter_trampoline<
            F: Fn(&DropTarget, f64, f64) -> gdk::DragAction + 'static,
        >(
            this: *mut gtk_sys::GtkDropTarget,
            x: libc::c_double,
            y: libc::c_double,
            f: glib_sys::gpointer,
        ) -> gdk_sys::GdkDragAction {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y).to_glib()
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

    pub fn connect_leave<F: Fn(&DropTarget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn leave_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut gtk_sys::GtkDropTarget,
            f: glib_sys::gpointer,
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

    pub fn connect_motion<F: Fn(&DropTarget, f64, f64) -> gdk::DragAction + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn motion_trampoline<
            F: Fn(&DropTarget, f64, f64) -> gdk::DragAction + 'static,
        >(
            this: *mut gtk_sys::GtkDropTarget,
            x: libc::c_double,
            y: libc::c_double,
            f: glib_sys::gpointer,
        ) -> gdk_sys::GdkDragAction {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y).to_glib()
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

    pub fn connect_property_actions_notify<F: Fn(&DropTarget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_actions_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut gtk_sys::GtkDropTarget,
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
                b"notify::actions\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_actions_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_drop_notify<F: Fn(&DropTarget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_drop_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut gtk_sys::GtkDropTarget,
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
                b"notify::drop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_drop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_formats_notify<F: Fn(&DropTarget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_formats_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut gtk_sys::GtkDropTarget,
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
                b"notify::formats\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_formats_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_preload_notify<F: Fn(&DropTarget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_preload_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut gtk_sys::GtkDropTarget,
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
                b"notify::preload\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_preload_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_value_notify<F: Fn(&DropTarget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<F: Fn(&DropTarget) + 'static>(
            this: *mut gtk_sys::GtkDropTarget,
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
                b"notify::value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DropTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DropTarget")
    }
}