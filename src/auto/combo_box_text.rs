// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::translate::*;
use glib::GString;
use gtk_sys;
use std::fmt;
use Bin;
use Buildable;
use CellEditable;
use CellLayout;
use ComboBox;
use Container;
use Widget;

glib_wrapper! {
    pub struct ComboBoxText(Object<gtk_sys::GtkComboBoxText, ComboBoxTextClass>) @extends ComboBox, Bin, Container, Widget, @implements Buildable, CellEditable, CellLayout;

    match fn {
        get_type => || gtk_sys::gtk_combo_box_text_get_type(),
    }
}

impl ComboBoxText {
    pub fn new() -> ComboBoxText {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_combo_box_text_new()).unsafe_cast() }
    }

    pub fn with_entry() -> ComboBoxText {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_combo_box_text_new_with_entry()).unsafe_cast()
        }
    }

    pub fn append(&self, id: Option<&str>, text: &str) {
        unsafe {
            gtk_sys::gtk_combo_box_text_append(
                self.to_glib_none().0,
                id.to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    pub fn append_text(&self, text: &str) {
        unsafe {
            gtk_sys::gtk_combo_box_text_append_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    pub fn get_active_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_combo_box_text_get_active_text(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn insert(&self, position: i32, id: Option<&str>, text: &str) {
        unsafe {
            gtk_sys::gtk_combo_box_text_insert(
                self.to_glib_none().0,
                position,
                id.to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    pub fn insert_text(&self, position: i32, text: &str) {
        unsafe {
            gtk_sys::gtk_combo_box_text_insert_text(
                self.to_glib_none().0,
                position,
                text.to_glib_none().0,
            );
        }
    }

    pub fn prepend(&self, id: Option<&str>, text: &str) {
        unsafe {
            gtk_sys::gtk_combo_box_text_prepend(
                self.to_glib_none().0,
                id.to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    pub fn prepend_text(&self, text: &str) {
        unsafe {
            gtk_sys::gtk_combo_box_text_prepend_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    pub fn remove(&self, position: i32) {
        unsafe {
            gtk_sys::gtk_combo_box_text_remove(self.to_glib_none().0, position);
        }
    }

    pub fn remove_all(&self) {
        unsafe {
            gtk_sys::gtk_combo_box_text_remove_all(self.to_glib_none().0);
        }
    }
}

impl Default for ComboBoxText {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ComboBoxText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ComboBoxText")
    }
}
