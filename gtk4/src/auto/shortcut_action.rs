// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ShortcutActionFlags;
use crate::Widget;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct ShortcutAction(Object<ffi::GtkShortcutAction, ffi::GtkShortcutActionClass>);

    match fn {
        type_ => || ffi::gtk_shortcut_action_get_type(),
    }
}

impl ShortcutAction {
    #[doc(alias = "gtk_shortcut_action_parse_string")]
    pub fn parse_string(string: &str) -> Option<ShortcutAction> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_shortcut_action_parse_string(
                string.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for ShortcutAction {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&ShortcutActionExt::to_str(self))
    }
}

pub const NONE_SHORTCUT_ACTION: Option<&ShortcutAction> = None;

pub trait ShortcutActionExt: 'static {
    #[doc(alias = "gtk_shortcut_action_activate")]
    fn activate<P: IsA<Widget>>(
        &self,
        flags: ShortcutActionFlags,
        widget: &P,
        args: Option<&glib::Variant>,
    ) -> bool;

    #[doc(alias = "gtk_shortcut_action_print")]
    fn print(&self, string: &mut glib::String);

    #[doc(alias = "gtk_shortcut_action_to_string")]
    fn to_str(&self) -> glib::GString;
}

impl<O: IsA<ShortcutAction>> ShortcutActionExt for O {
    fn activate<P: IsA<Widget>>(
        &self,
        flags: ShortcutActionFlags,
        widget: &P,
        args: Option<&glib::Variant>,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_shortcut_action_activate(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                widget.as_ref().to_glib_none().0,
                args.to_glib_none().0,
            ))
        }
    }

    fn print(&self, string: &mut glib::String) {
        unsafe {
            ffi::gtk_shortcut_action_print(
                self.as_ref().to_glib_none().0,
                string.to_glib_none_mut().0,
            );
        }
    }

    fn to_str(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gtk_shortcut_action_to_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}
