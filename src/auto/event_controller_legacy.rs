// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::translate::*;
use gtk_sys;
use std::fmt;
use EventController;

glib_wrapper! {
    pub struct EventControllerLegacy(Object<gtk_sys::GtkEventControllerLegacy, gtk_sys::GtkEventControllerLegacyClass, EventControllerLegacyClass>) @extends EventController;

    match fn {
        get_type => || gtk_sys::gtk_event_controller_legacy_get_type(),
    }
}

impl EventControllerLegacy {
    pub fn new() -> EventControllerLegacy {
        assert_initialized_main_thread!();
        unsafe {
            EventController::from_glib_full(gtk_sys::gtk_event_controller_legacy_new())
                .unsafe_cast()
        }
    }

    //pub fn connect_event<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored event: Gdk.Event
    //}
}

impl Default for EventControllerLegacy {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EventControllerLegacy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EventControllerLegacy")
    }
}
