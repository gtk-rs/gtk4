// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gtk4-gir-files
// DO NOT EDIT

use crate::LayoutManager;
use crate::Widget;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct LayoutChild(Object<ffi::GtkLayoutChild, ffi::GtkLayoutChildClass>);

    match fn {
        get_type => || ffi::gtk_layout_child_get_type(),
    }
}

pub const NONE_LAYOUT_CHILD: Option<&LayoutChild> = None;

pub trait LayoutChildExt: 'static {
    #[doc(alias = "gtk_layout_child_get_child_widget")]
    fn get_child_widget(&self) -> Option<Widget>;

    #[doc(alias = "gtk_layout_child_get_layout_manager")]
    fn get_layout_manager(&self) -> Option<LayoutManager>;
}

impl<O: IsA<LayoutChild>> LayoutChildExt for O {
    fn get_child_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_layout_child_get_child_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_layout_manager(&self) -> Option<LayoutManager> {
        unsafe {
            from_glib_none(ffi::gtk_layout_child_get_layout_manager(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for LayoutChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("LayoutChild")
    }
}
