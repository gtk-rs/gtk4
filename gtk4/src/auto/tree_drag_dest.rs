// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::TreePath;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkTreeDragDest")]
    pub struct TreeDragDest(Interface<ffi::GtkTreeDragDest, ffi::GtkTreeDragDestIface>);

    match fn {
        type_ => || ffi::gtk_tree_drag_dest_get_type(),
    }
}

pub const NONE_TREE_DRAG_DEST: Option<&TreeDragDest> = None;

pub trait TreeDragDestExt: 'static {
    #[doc(alias = "gtk_tree_drag_dest_drag_data_received")]
    fn drag_data_received(&self, dest: &TreePath, value: &glib::Value) -> bool;

    #[doc(alias = "gtk_tree_drag_dest_row_drop_possible")]
    fn row_drop_possible(&self, dest_path: &TreePath, value: &glib::Value) -> bool;
}

impl<O: IsA<TreeDragDest>> TreeDragDestExt for O {
    fn drag_data_received(&self, dest: &TreePath, value: &glib::Value) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_drag_dest_drag_data_received(
                self.as_ref().to_glib_none().0,
                mut_override(dest.to_glib_none().0),
                value.to_glib_none().0,
            ))
        }
    }

    fn row_drop_possible(&self, dest_path: &TreePath, value: &glib::Value) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_drag_dest_row_drop_possible(
                self.as_ref().to_glib_none().0,
                mut_override(dest_path.to_glib_none().0),
                value.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for TreeDragDest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TreeDragDest")
    }
}
