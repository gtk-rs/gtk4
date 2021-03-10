// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gtk4-gir-files
// DO NOT EDIT

use crate::ShortcutAction;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct NothingAction(Object<ffi::GtkNothingAction, ffi::GtkNothingActionClass>) @extends ShortcutAction;

    match fn {
        get_type => || ffi::gtk_nothing_action_get_type(),
    }
}

impl NothingAction {
    #[doc(alias = "gtk_nothing_action_get")]
    pub fn get() -> Option<NothingAction> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_nothing_action_get()) }
    }
}

impl fmt::Display for NothingAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NothingAction")
    }
}
