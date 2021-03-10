// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gtk4-gir-files
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct Buildable(Interface<ffi::GtkBuildable, ffi::GtkBuildableIface>);

    match fn {
        get_type => || ffi::gtk_buildable_get_type(),
    }
}

pub const NONE_BUILDABLE: Option<&Buildable> = None;

pub trait BuildableExt: 'static {
    #[doc(alias = "gtk_buildable_get_buildable_id")]
    fn get_buildable_id(&self) -> Option<glib::GString>;
}

impl<O: IsA<Buildable>> BuildableExt for O {
    fn get_buildable_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_buildable_get_buildable_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for Buildable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Buildable")
    }
}
