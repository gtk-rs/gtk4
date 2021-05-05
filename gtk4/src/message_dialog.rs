// Take a look at the license at the top of the repository in the LICENSE file.

use glib::object::{Cast, IsA};
use glib::translate::*;
use libc::c_char;
use std::ptr;

use crate::{ButtonsType, DialogFlags, MessageDialog, MessageType, Widget, Window};

impl MessageDialog {
    #[doc(alias = "gtk_message_dialog_new")]
    pub fn new<T: IsA<Window>>(
        parent: Option<&T>,
        flags: DialogFlags,
        type_: MessageType,
        buttons: ButtonsType,
        message: &str,
    ) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let message: Stash<*const c_char, _> = message.to_glib_none();
            Widget::from_glib_none(ffi::gtk_message_dialog_new(
                parent.map(|p| p.as_ref()).to_glib_none().0,
                flags.into_glib(),
                type_.into_glib(),
                buttons.into_glib(),
                b"%s\0".as_ptr() as *const c_char,
                message.0,
                ptr::null::<c_char>(),
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_message_dialog_new_with_markup")]
    #[doc(alias = "new_with_markup")]
    pub fn with_markup<P: IsA<Window>>(
        parent: Option<&P>,
        flags: DialogFlags,
        type_: MessageType,
        buttons: ButtonsType,
        message: Option<&str>,
    ) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_message_dialog_new_with_markup(
                parent.map(|p| p.as_ref()).to_glib_none().0,
                flags.into_glib(),
                type_.into_glib(),
                buttons.into_glib(),
                message.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_message_dialog_format_secondary_markup")]
    pub fn format_secondary_markup(&self, message: Option<&str>) {
        unsafe {
            ffi::gtk_message_dialog_format_secondary_markup(
                self.to_glib_none().0,
                message.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "gtk_message_dialog_format_secondary_text")]
    pub fn format_secondary_text(&self, message: Option<&str>) {
        unsafe {
            ffi::gtk_message_dialog_format_secondary_text(
                self.to_glib_none().0,
                message.to_glib_none().0,
            )
        }
    }
}
