// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Widget;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;

glib::wrapper! {
    pub struct TextChildAnchor(Object<ffi::GtkTextChildAnchor, ffi::GtkTextChildAnchorClass>);

    match fn {
        get_type => || ffi::gtk_text_child_anchor_get_type(),
    }
}

impl TextChildAnchor {
    #[doc(alias = "gtk_text_child_anchor_new")]
    pub fn new() -> TextChildAnchor {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_text_child_anchor_new()) }
    }
}

impl Default for TextChildAnchor {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_TEXT_CHILD_ANCHOR: Option<&TextChildAnchor> = None;

pub trait TextChildAnchorExt: 'static {
    #[doc(alias = "gtk_text_child_anchor_get_deleted")]
    fn get_deleted(&self) -> bool;

    #[doc(alias = "gtk_text_child_anchor_get_widgets")]
    fn get_widgets(&self) -> Vec<Widget>;
}

impl<O: IsA<TextChildAnchor>> TextChildAnchorExt for O {
    fn get_deleted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_child_anchor_get_deleted(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_widgets(&self) -> Vec<Widget> {
        unsafe {
            let mut out_len = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_container_num(
                ffi::gtk_text_child_anchor_get_widgets(
                    self.as_ref().to_glib_none().0,
                    out_len.as_mut_ptr(),
                ),
                out_len.assume_init() as usize,
            );
            ret
        }
    }
}

impl fmt::Display for TextChildAnchor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TextChildAnchor")
    }
}
