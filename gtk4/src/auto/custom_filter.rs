// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Filter;
use std::fmt;

glib::wrapper! {
    pub struct CustomFilter(Object<ffi::GtkCustomFilter, ffi::GtkCustomFilterClass>) @extends Filter;

    match fn {
        type_ => || ffi::gtk_custom_filter_get_type(),
    }
}

impl CustomFilter {}

impl fmt::Display for CustomFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CustomFilter")
    }
}
