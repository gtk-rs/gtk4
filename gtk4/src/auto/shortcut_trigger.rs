// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct ShortcutTrigger(Object<ffi::GtkShortcutTrigger, ffi::GtkShortcutTriggerClass>);

    match fn {
        get_type => || ffi::gtk_shortcut_trigger_get_type(),
    }
}

impl ShortcutTrigger {
    #[doc(alias = "gtk_shortcut_trigger_parse_string")]
    pub fn parse_string(string: &str) -> Option<ShortcutTrigger> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_shortcut_trigger_parse_string(
                string.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for ShortcutTrigger {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&ShortcutTriggerExt::to_str(self))
    }
}

pub const NONE_SHORTCUT_TRIGGER: Option<&ShortcutTrigger> = None;

pub trait ShortcutTriggerExt: 'static {
    #[doc(alias = "gtk_shortcut_trigger_print")]
    fn print(&self, string: &mut glib::String);

    #[doc(alias = "gtk_shortcut_trigger_print_label")]
    fn print_label(&self, display: &gdk::Display, string: &mut glib::String) -> bool;

    #[doc(alias = "gtk_shortcut_trigger_to_label")]
    fn to_label(&self, display: &gdk::Display) -> glib::GString;

    #[doc(alias = "gtk_shortcut_trigger_to_string")]
    fn to_str(&self) -> glib::GString;

    #[doc(alias = "gtk_shortcut_trigger_trigger")]
    fn trigger<P: IsA<gdk::Event>>(&self, event: &P, enable_mnemonics: bool) -> gdk::KeyMatch;
}

impl<O: IsA<ShortcutTrigger>> ShortcutTriggerExt for O {
    fn print(&self, string: &mut glib::String) {
        unsafe {
            ffi::gtk_shortcut_trigger_print(
                self.as_ref().to_glib_none().0,
                string.to_glib_none_mut().0,
            );
        }
    }

    fn print_label(&self, display: &gdk::Display, string: &mut glib::String) -> bool {
        unsafe {
            from_glib(ffi::gtk_shortcut_trigger_print_label(
                self.as_ref().to_glib_none().0,
                display.to_glib_none().0,
                string.to_glib_none_mut().0,
            ))
        }
    }

    fn to_label(&self, display: &gdk::Display) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gtk_shortcut_trigger_to_label(
                self.as_ref().to_glib_none().0,
                display.to_glib_none().0,
            ))
        }
    }

    fn to_str(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gtk_shortcut_trigger_to_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn trigger<P: IsA<gdk::Event>>(&self, event: &P, enable_mnemonics: bool) -> gdk::KeyMatch {
        unsafe {
            from_glib(ffi::gtk_shortcut_trigger_trigger(
                self.as_ref().to_glib_none().0,
                event.as_ref().to_glib_none().0,
                enable_mnemonics.to_glib(),
            ))
        }
    }
}
