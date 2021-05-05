// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Expression, StringSorter};
use glib::translate::*;

impl StringSorter {
    #[doc(alias = "gtk_string_sorter_new")]
    pub fn new<E: AsRef<Expression>>(expression: Option<&E>) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_string_sorter_new(
                expression.map(|e| e.as_ref()).to_glib_full(),
            ))
        }
    }

    #[doc(alias = "gtk_string_sorter_set_expression")]
    pub fn set_expression<E: AsRef<Expression>>(&self, expression: Option<&E>) {
        unsafe {
            ffi::gtk_string_sorter_set_expression(
                self.to_glib_none().0,
                expression.map(|e| e.as_ref()).to_glib_none().0,
            );
        }
    }
}
