// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Bitset(Shared<ffi::GtkBitset>);

    match fn {
        ref => |ptr| ffi::gtk_bitset_ref(ptr),
        unref => |ptr| ffi::gtk_bitset_unref(ptr),
        get_type => || ffi::gtk_bitset_get_type(),
    }
}

impl Bitset {
    #[doc(alias = "gtk_bitset_new_empty")]
    pub fn new_empty() -> Bitset {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_bitset_new_empty()) }
    }

    #[doc(alias = "gtk_bitset_new_range")]
    pub fn new_range(start: u32, n_items: u32) -> Bitset {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_bitset_new_range(start, n_items)) }
    }

    #[doc(alias = "gtk_bitset_add")]
    pub fn add(&self, value: u32) -> bool {
        unsafe { from_glib(ffi::gtk_bitset_add(self.to_glib_none().0, value)) }
    }

    #[doc(alias = "gtk_bitset_add_range")]
    pub fn add_range(&self, start: u32, n_items: u32) {
        unsafe {
            ffi::gtk_bitset_add_range(self.to_glib_none().0, start, n_items);
        }
    }

    #[doc(alias = "gtk_bitset_add_range_closed")]
    pub fn add_range_closed(&self, first: u32, last: u32) {
        unsafe {
            ffi::gtk_bitset_add_range_closed(self.to_glib_none().0, first, last);
        }
    }

    #[doc(alias = "gtk_bitset_add_rectangle")]
    pub fn add_rectangle(&self, start: u32, width: u32, height: u32, stride: u32) {
        unsafe {
            ffi::gtk_bitset_add_rectangle(self.to_glib_none().0, start, width, height, stride);
        }
    }

    #[doc(alias = "gtk_bitset_contains")]
    pub fn contains(&self, value: u32) -> bool {
        unsafe { from_glib(ffi::gtk_bitset_contains(self.to_glib_none().0, value)) }
    }

    #[doc(alias = "gtk_bitset_copy")]
    pub fn copy(&self) -> Option<Bitset> {
        unsafe { from_glib_full(ffi::gtk_bitset_copy(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_bitset_difference")]
    pub fn difference(&self, other: &Bitset) {
        unsafe {
            ffi::gtk_bitset_difference(self.to_glib_none().0, other.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_bitset_equals")]
    pub fn equals(&self, other: &Bitset) -> bool {
        unsafe {
            from_glib(ffi::gtk_bitset_equals(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_bitset_get_maximum")]
    pub fn maximum(&self) -> u32 {
        unsafe { ffi::gtk_bitset_get_maximum(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_bitset_get_minimum")]
    pub fn minimum(&self) -> u32 {
        unsafe { ffi::gtk_bitset_get_minimum(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_bitset_get_nth")]
    pub fn get_nth(&self, nth: u32) -> u32 {
        unsafe { ffi::gtk_bitset_get_nth(self.to_glib_none().0, nth) }
    }

    #[doc(alias = "gtk_bitset_get_size")]
    pub fn size(&self) -> u64 {
        unsafe { ffi::gtk_bitset_get_size(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_bitset_get_size_in_range")]
    pub fn get_size_in_range(&self, first: u32, last: u32) -> u64 {
        unsafe { ffi::gtk_bitset_get_size_in_range(self.to_glib_none().0, first, last) }
    }

    #[doc(alias = "gtk_bitset_intersect")]
    pub fn intersect(&self, other: &Bitset) {
        unsafe {
            ffi::gtk_bitset_intersect(self.to_glib_none().0, other.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_bitset_is_empty")]
    pub fn is_empty(&self) -> bool {
        unsafe { from_glib(ffi::gtk_bitset_is_empty(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_bitset_remove")]
    pub fn remove(&self, value: u32) -> bool {
        unsafe { from_glib(ffi::gtk_bitset_remove(self.to_glib_none().0, value)) }
    }

    #[doc(alias = "gtk_bitset_remove_all")]
    pub fn remove_all(&self) {
        unsafe {
            ffi::gtk_bitset_remove_all(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_bitset_remove_range")]
    pub fn remove_range(&self, start: u32, n_items: u32) {
        unsafe {
            ffi::gtk_bitset_remove_range(self.to_glib_none().0, start, n_items);
        }
    }

    #[doc(alias = "gtk_bitset_remove_range_closed")]
    pub fn remove_range_closed(&self, first: u32, last: u32) {
        unsafe {
            ffi::gtk_bitset_remove_range_closed(self.to_glib_none().0, first, last);
        }
    }

    #[doc(alias = "gtk_bitset_remove_rectangle")]
    pub fn remove_rectangle(&self, start: u32, width: u32, height: u32, stride: u32) {
        unsafe {
            ffi::gtk_bitset_remove_rectangle(self.to_glib_none().0, start, width, height, stride);
        }
    }

    #[doc(alias = "gtk_bitset_shift_left")]
    pub fn shift_left(&self, amount: u32) {
        unsafe {
            ffi::gtk_bitset_shift_left(self.to_glib_none().0, amount);
        }
    }

    #[doc(alias = "gtk_bitset_shift_right")]
    pub fn shift_right(&self, amount: u32) {
        unsafe {
            ffi::gtk_bitset_shift_right(self.to_glib_none().0, amount);
        }
    }

    #[doc(alias = "gtk_bitset_splice")]
    pub fn splice(&self, position: u32, removed: u32, added: u32) {
        unsafe {
            ffi::gtk_bitset_splice(self.to_glib_none().0, position, removed, added);
        }
    }

    #[doc(alias = "gtk_bitset_subtract")]
    pub fn subtract(&self, other: &Bitset) {
        unsafe {
            ffi::gtk_bitset_subtract(self.to_glib_none().0, other.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_bitset_union")]
    pub fn union(&self, other: &Bitset) {
        unsafe {
            ffi::gtk_bitset_union(self.to_glib_none().0, other.to_glib_none().0);
        }
    }
}
