// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::RecentData;
use crate::RecentInfo;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    pub struct RecentManager(Object<ffi::GtkRecentManager, ffi::GtkRecentManagerClass>);

    match fn {
        type_ => || ffi::gtk_recent_manager_get_type(),
    }
}

impl RecentManager {
    #[doc(alias = "gtk_recent_manager_new")]
    pub fn new() -> RecentManager {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_recent_manager_new()) }
    }

    #[doc(alias = "gtk_recent_manager_get_default")]
    #[doc(alias = "get_default")]
    pub fn default() -> RecentManager {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_recent_manager_get_default()) }
    }
}

impl Default for RecentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`RecentManager`].
pub struct RecentManagerBuilder {
    filename: Option<String>,
}

impl RecentManagerBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`RecentManagerBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`RecentManager`].
    pub fn build(self) -> RecentManager {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref filename) = self.filename {
            properties.push(("filename", filename));
        }
        glib::Object::new::<RecentManager>(&properties)
            .expect("Failed to create an instance of RecentManager")
    }

    pub fn filename(mut self, filename: &str) -> Self {
        self.filename = Some(filename.to_string());
        self
    }
}

pub const NONE_RECENT_MANAGER: Option<&RecentManager> = None;

pub trait RecentManagerExt: 'static {
    #[doc(alias = "gtk_recent_manager_add_full")]
    fn add_full(&self, uri: &str, recent_data: &RecentData) -> bool;

    #[doc(alias = "gtk_recent_manager_add_item")]
    fn add_item(&self, uri: &str) -> bool;

    #[doc(alias = "gtk_recent_manager_get_items")]
    #[doc(alias = "get_items")]
    fn items(&self) -> Vec<RecentInfo>;

    #[doc(alias = "gtk_recent_manager_has_item")]
    fn has_item(&self, uri: &str) -> bool;

    #[doc(alias = "gtk_recent_manager_lookup_item")]
    fn lookup_item(&self, uri: &str) -> Result<Option<RecentInfo>, glib::Error>;

    #[doc(alias = "gtk_recent_manager_move_item")]
    fn move_item(&self, uri: &str, new_uri: Option<&str>) -> Result<(), glib::Error>;

    #[doc(alias = "gtk_recent_manager_purge_items")]
    fn purge_items(&self) -> Result<i32, glib::Error>;

    #[doc(alias = "gtk_recent_manager_remove_item")]
    fn remove_item(&self, uri: &str) -> Result<(), glib::Error>;

    fn filename(&self) -> Option<glib::GString>;

    fn size(&self) -> i32;

    #[doc(alias = "changed")]
    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "size")]
    fn connect_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RecentManager>> RecentManagerExt for O {
    fn add_full(&self, uri: &str, recent_data: &RecentData) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_manager_add_full(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                recent_data.to_glib_none().0,
            ))
        }
    }

    fn add_item(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_manager_add_item(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            ))
        }
    }

    fn items(&self) -> Vec<RecentInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_recent_manager_get_items(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_item(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_manager_has_item(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            ))
        }
    }

    fn lookup_item(&self, uri: &str) -> Result<Option<RecentInfo>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_recent_manager_lookup_item(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn move_item(&self, uri: &str, new_uri: Option<&str>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_recent_manager_move_item(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                new_uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn purge_items(&self) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                ffi::gtk_recent_manager_purge_items(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn remove_item(&self, uri: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_recent_manager_remove_item(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn filename(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"filename\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `filename` getter")
        }
    }

    fn size(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"size\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `size` getter")
        }
    }

    #[doc(alias = "changed")]
    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkRecentManager,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<RecentManager>,
        {
            let f: &F = &*(f as *const F);
            f(&RecentManager::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "size")]
    fn connect_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkRecentManager,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<RecentManager>,
        {
            let f: &F = &*(f as *const F);
            f(&RecentManager::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for RecentManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RecentManager")
    }
}
