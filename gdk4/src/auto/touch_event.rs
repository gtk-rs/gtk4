// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Event;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct TouchEvent(Object<ffi::GdkTouchEvent>) @extends Event;

    match fn {
        get_type => || ffi::gdk_touch_event_get_type(),
    }
}

impl TouchEvent {
    pub fn get_emulating_pointer(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_touch_event_get_emulating_pointer(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for TouchEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TouchEvent")
    }
}
