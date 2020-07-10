// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use CellRenderer;
use TreePath;

glib_wrapper! {
    pub struct CellRendererToggle(Object<gtk_sys::GtkCellRendererToggle, CellRendererToggleClass>) @extends CellRenderer;

    match fn {
        get_type => || gtk_sys::gtk_cell_renderer_toggle_get_type(),
    }
}

impl CellRendererToggle {
    pub fn new() -> CellRendererToggle {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(gtk_sys::gtk_cell_renderer_toggle_new()).unsafe_cast()
        }
    }

    pub fn get_activatable(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_cell_renderer_toggle_get_activatable(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_active(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_cell_renderer_toggle_get_active(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_radio(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_cell_renderer_toggle_get_radio(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn set_activatable(&self, setting: bool) {
        unsafe {
            gtk_sys::gtk_cell_renderer_toggle_set_activatable(
                self.to_glib_none().0,
                setting.to_glib(),
            );
        }
    }

    pub fn set_active(&self, setting: bool) {
        unsafe {
            gtk_sys::gtk_cell_renderer_toggle_set_active(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_radio(&self, radio: bool) {
        unsafe {
            gtk_sys::gtk_cell_renderer_toggle_set_radio(self.to_glib_none().0, radio.to_glib());
        }
    }

    pub fn get_property_inconsistent(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"inconsistent\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `inconsistent` getter")
                .unwrap()
        }
    }

    pub fn set_property_inconsistent(&self, inconsistent: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"inconsistent\0".as_ptr() as *const _,
                Value::from(&inconsistent).to_glib_none().0,
            );
        }
    }

    pub fn connect_toggled<F: Fn(&CellRendererToggle, TreePath) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn toggled_trampoline<F: Fn(&CellRendererToggle, TreePath) + 'static>(
            this: *mut gtk_sys::GtkCellRendererToggle,
            path: *mut libc::c_char,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            let path = from_glib_full(gtk_sys::gtk_tree_path_new_from_string(path));
            f(&from_glib_borrow(this), path)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toggled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    toggled_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_activatable_notify<F: Fn(&CellRendererToggle) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_activatable_trampoline<F: Fn(&CellRendererToggle) + 'static>(
            this: *mut gtk_sys::GtkCellRendererToggle,
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
                b"notify::activatable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_activatable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_active_notify<F: Fn(&CellRendererToggle) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<F: Fn(&CellRendererToggle) + 'static>(
            this: *mut gtk_sys::GtkCellRendererToggle,
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
                b"notify::active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_inconsistent_notify<F: Fn(&CellRendererToggle) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_inconsistent_trampoline<
            F: Fn(&CellRendererToggle) + 'static,
        >(
            this: *mut gtk_sys::GtkCellRendererToggle,
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
                b"notify::inconsistent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inconsistent_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_radio_notify<F: Fn(&CellRendererToggle) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_radio_trampoline<F: Fn(&CellRendererToggle) + 'static>(
            this: *mut gtk_sys::GtkCellRendererToggle,
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
                b"notify::radio\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_radio_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for CellRendererToggle {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CellRendererToggle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CellRendererToggle")
    }
}
