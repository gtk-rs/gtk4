// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Accessible;
use Buildable;
use ConstraintTarget;
use ListBase;
use ListItemFactory;
use Orientable;
use Scrollable;
use Widget;

glib_wrapper! {
    pub struct ListView(Object<gtk_sys::GtkListView, gtk_sys::GtkListViewClass, ListViewClass>) @extends ListBase, Widget, @implements Accessible, Buildable, ConstraintTarget, Orientable, Scrollable;

    match fn {
        get_type => || gtk_sys::gtk_list_view_get_type(),
    }
}

impl ListView {
    pub fn new<P: IsA<gio::ListModel>>(model: Option<&P>) -> ListView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_list_view_new(
                model.map(|p| p.as_ref()).to_glib_full(),
            ))
            .unsafe_cast()
        }
    }

    pub fn with_factory<P: IsA<gio::ListModel>, Q: IsA<ListItemFactory>>(
        model: Option<&P>,
        factory: Option<&Q>,
    ) -> ListView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_list_view_new_with_factory(
                model.map(|p| p.as_ref()).to_glib_full(),
                factory.map(|p| p.as_ref()).to_glib_full(),
            ))
            .unsafe_cast()
        }
    }

    pub fn get_enable_rubberband(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_list_view_get_enable_rubberband(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_factory(&self) -> Option<ListItemFactory> {
        unsafe { from_glib_none(gtk_sys::gtk_list_view_get_factory(self.to_glib_none().0)) }
    }

    pub fn get_model(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(gtk_sys::gtk_list_view_get_model(self.to_glib_none().0)) }
    }

    pub fn get_show_separators(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_list_view_get_show_separators(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_single_click_activate(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_list_view_get_single_click_activate(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn set_enable_rubberband(&self, enable_rubberband: bool) {
        unsafe {
            gtk_sys::gtk_list_view_set_enable_rubberband(
                self.to_glib_none().0,
                enable_rubberband.to_glib(),
            );
        }
    }

    pub fn set_factory<P: IsA<ListItemFactory>>(&self, factory: Option<&P>) {
        unsafe {
            gtk_sys::gtk_list_view_set_factory(
                self.to_glib_none().0,
                factory.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>) {
        unsafe {
            gtk_sys::gtk_list_view_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn set_show_separators(&self, show_separators: bool) {
        unsafe {
            gtk_sys::gtk_list_view_set_show_separators(
                self.to_glib_none().0,
                show_separators.to_glib(),
            );
        }
    }

    pub fn set_single_click_activate(&self, single_click_activate: bool) {
        unsafe {
            gtk_sys::gtk_list_view_set_single_click_activate(
                self.to_glib_none().0,
                single_click_activate.to_glib(),
            );
        }
    }

    pub fn connect_activate<F: Fn(&ListView, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<F: Fn(&ListView, u32) + 'static>(
            this: *mut gtk_sys::GtkListView,
            position: libc::c_uint,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), position)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_enable_rubberband_notify<F: Fn(&ListView) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_rubberband_trampoline<F: Fn(&ListView) + 'static>(
            this: *mut gtk_sys::GtkListView,
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
                b"notify::enable-rubberband\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enable_rubberband_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_factory_notify<F: Fn(&ListView) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_factory_trampoline<F: Fn(&ListView) + 'static>(
            this: *mut gtk_sys::GtkListView,
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
                b"notify::factory\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_factory_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_model_notify<F: Fn(&ListView) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&ListView) + 'static>(
            this: *mut gtk_sys::GtkListView,
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
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_show_separators_notify<F: Fn(&ListView) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_separators_trampoline<F: Fn(&ListView) + 'static>(
            this: *mut gtk_sys::GtkListView,
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
                b"notify::show-separators\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_separators_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_single_click_activate_notify<F: Fn(&ListView) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_single_click_activate_trampoline<F: Fn(&ListView) + 'static>(
            this: *mut gtk_sys::GtkListView,
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
                b"notify::single-click-activate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_single_click_activate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ListView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ListView")
    }
}