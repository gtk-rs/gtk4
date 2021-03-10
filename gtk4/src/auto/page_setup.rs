// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gtk4-gir-files
// DO NOT EDIT

use crate::PageOrientation;
use crate::PaperSize;
use crate::Unit;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    pub struct PageSetup(Object<ffi::GtkPageSetup>);

    match fn {
        get_type => || ffi::gtk_page_setup_get_type(),
    }
}

impl PageSetup {
    #[doc(alias = "gtk_page_setup_new")]
    pub fn new() -> PageSetup {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_page_setup_new()) }
    }

    #[doc(alias = "gtk_page_setup_new_from_file")]
    pub fn from_file<P: AsRef<std::path::Path>>(file_name: P) -> Result<PageSetup, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                ffi::gtk_page_setup_new_from_file(file_name.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_page_setup_new_from_gvariant")]
    pub fn from_gvariant(variant: &glib::Variant) -> PageSetup {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_page_setup_new_from_gvariant(
                variant.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_page_setup_new_from_key_file")]
    pub fn from_key_file(
        key_file: &glib::KeyFile,
        group_name: Option<&str>,
    ) -> Result<PageSetup, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_page_setup_new_from_key_file(
                key_file.to_glib_none().0,
                group_name.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_page_setup_copy")]
    pub fn copy(&self) -> Option<PageSetup> {
        unsafe { from_glib_full(ffi::gtk_page_setup_copy(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_page_setup_get_bottom_margin")]
    pub fn get_bottom_margin(&self, unit: Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_bottom_margin(self.to_glib_none().0, unit.to_glib()) }
    }

    #[doc(alias = "gtk_page_setup_get_left_margin")]
    pub fn get_left_margin(&self, unit: Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_left_margin(self.to_glib_none().0, unit.to_glib()) }
    }

    #[doc(alias = "gtk_page_setup_get_orientation")]
    pub fn get_orientation(&self) -> PageOrientation {
        unsafe { from_glib(ffi::gtk_page_setup_get_orientation(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_page_setup_get_page_height")]
    pub fn get_page_height(&self, unit: Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_page_height(self.to_glib_none().0, unit.to_glib()) }
    }

    #[doc(alias = "gtk_page_setup_get_page_width")]
    pub fn get_page_width(&self, unit: Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_page_width(self.to_glib_none().0, unit.to_glib()) }
    }

    #[doc(alias = "gtk_page_setup_get_paper_height")]
    pub fn get_paper_height(&self, unit: Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_paper_height(self.to_glib_none().0, unit.to_glib()) }
    }

    #[doc(alias = "gtk_page_setup_get_paper_size")]
    pub fn get_paper_size(&self) -> PaperSize {
        unsafe { from_glib_none(ffi::gtk_page_setup_get_paper_size(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_page_setup_get_paper_width")]
    pub fn get_paper_width(&self, unit: Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_paper_width(self.to_glib_none().0, unit.to_glib()) }
    }

    #[doc(alias = "gtk_page_setup_get_right_margin")]
    pub fn get_right_margin(&self, unit: Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_right_margin(self.to_glib_none().0, unit.to_glib()) }
    }

    #[doc(alias = "gtk_page_setup_get_top_margin")]
    pub fn get_top_margin(&self, unit: Unit) -> f64 {
        unsafe { ffi::gtk_page_setup_get_top_margin(self.to_glib_none().0, unit.to_glib()) }
    }

    #[doc(alias = "gtk_page_setup_load_file")]
    pub fn load_file<P: AsRef<std::path::Path>>(&self, file_name: P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_page_setup_load_file(
                self.to_glib_none().0,
                file_name.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_page_setup_load_key_file")]
    pub fn load_key_file(
        &self,
        key_file: &glib::KeyFile,
        group_name: Option<&str>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_page_setup_load_key_file(
                self.to_glib_none().0,
                key_file.to_glib_none().0,
                group_name.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_page_setup_set_bottom_margin")]
    pub fn set_bottom_margin(&self, margin: f64, unit: Unit) {
        unsafe {
            ffi::gtk_page_setup_set_bottom_margin(self.to_glib_none().0, margin, unit.to_glib());
        }
    }

    #[doc(alias = "gtk_page_setup_set_left_margin")]
    pub fn set_left_margin(&self, margin: f64, unit: Unit) {
        unsafe {
            ffi::gtk_page_setup_set_left_margin(self.to_glib_none().0, margin, unit.to_glib());
        }
    }

    #[doc(alias = "gtk_page_setup_set_orientation")]
    pub fn set_orientation(&self, orientation: PageOrientation) {
        unsafe {
            ffi::gtk_page_setup_set_orientation(self.to_glib_none().0, orientation.to_glib());
        }
    }

    #[doc(alias = "gtk_page_setup_set_paper_size")]
    pub fn set_paper_size(&self, size: &PaperSize) {
        unsafe {
            ffi::gtk_page_setup_set_paper_size(
                self.to_glib_none().0,
                mut_override(size.to_glib_none().0),
            );
        }
    }

    #[doc(alias = "gtk_page_setup_set_paper_size_and_default_margins")]
    pub fn set_paper_size_and_default_margins(&self, size: &PaperSize) {
        unsafe {
            ffi::gtk_page_setup_set_paper_size_and_default_margins(
                self.to_glib_none().0,
                mut_override(size.to_glib_none().0),
            );
        }
    }

    #[doc(alias = "gtk_page_setup_set_right_margin")]
    pub fn set_right_margin(&self, margin: f64, unit: Unit) {
        unsafe {
            ffi::gtk_page_setup_set_right_margin(self.to_glib_none().0, margin, unit.to_glib());
        }
    }

    #[doc(alias = "gtk_page_setup_set_top_margin")]
    pub fn set_top_margin(&self, margin: f64, unit: Unit) {
        unsafe {
            ffi::gtk_page_setup_set_top_margin(self.to_glib_none().0, margin, unit.to_glib());
        }
    }

    #[doc(alias = "gtk_page_setup_to_file")]
    pub fn to_file<P: AsRef<std::path::Path>>(&self, file_name: P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_page_setup_to_file(
                self.to_glib_none().0,
                file_name.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_page_setup_to_gvariant")]
    pub fn to_gvariant(&self) -> Option<glib::Variant> {
        unsafe { from_glib_none(ffi::gtk_page_setup_to_gvariant(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_page_setup_to_key_file")]
    pub fn to_key_file(&self, key_file: &glib::KeyFile, group_name: Option<&str>) {
        unsafe {
            ffi::gtk_page_setup_to_key_file(
                self.to_glib_none().0,
                key_file.to_glib_none().0,
                group_name.to_glib_none().0,
            );
        }
    }
}

impl Default for PageSetup {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PageSetup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PageSetup")
    }
}
