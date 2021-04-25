// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::{TreeIter, TreePath, TreeRowReference};
use glib::translate::*;
use glib::IsA;

impl TreeRowReference {
    #[doc(alias = "gtk_tree_row_reference_reordered")]
    pub fn reordered<P: IsA<glib::Object>>(
        &self,
        proxy: &P,
        path: &TreePath,
        iter: &TreeIter,
        new_order: &[i32],
    ) {
        assert_eq!(
            new_order.len() as i32,
            self.model().iter_n_children(Some(iter)),
            "TreeRowReference got passed a `new_order` bigger than the total childrens in the model for the passed iter"
        );
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_tree_row_reference_reordered(
                proxy.as_ref().to_glib_none().0,
                mut_override(path.to_glib_none().0),
                mut_override(iter.to_glib_none().0),
                mut_override(new_order.as_ptr()),
            )
        }
    }
}
