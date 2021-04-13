// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;
use std::mem;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ContentFormats(Shared<ffi::GdkContentFormats>);

    match fn {
        ref => |ptr| ffi::gdk_content_formats_ref(ptr),
        unref => |ptr| ffi::gdk_content_formats_unref(ptr),
        get_type => || ffi::gdk_content_formats_get_type(),
    }
}

impl ContentFormats {
    #[doc(alias = "gdk_content_formats_new")]
    pub fn new(mime_types: &[&str]) -> ContentFormats {
        assert_initialized_main_thread!();
        let n_mime_types = mime_types.len() as u32;
        unsafe {
            from_glib_full(ffi::gdk_content_formats_new(
                mime_types.to_glib_none().0,
                n_mime_types,
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_new_for_gtype")]
    pub fn for_type(type_: glib::types::Type) -> ContentFormats {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gdk_content_formats_new_for_gtype(type_.to_glib())) }
    }

    #[doc(alias = "gdk_content_formats_contain_gtype")]
    pub fn contains_type(&self, type_: glib::types::Type) -> bool {
        unsafe {
            from_glib(ffi::gdk_content_formats_contain_gtype(
                self.to_glib_none().0,
                type_.to_glib(),
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_contain_mime_type")]
    pub fn contain_mime_type(&self, mime_type: &str) -> bool {
        unsafe {
            from_glib(ffi::gdk_content_formats_contain_mime_type(
                self.to_glib_none().0,
                mime_type.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_get_mime_types")]
    pub fn mime_types(&self) -> (Vec<glib::GString>, usize) {
        unsafe {
            let mut n_mime_types = mem::MaybeUninit::uninit();
            let ret =
                FromGlibPtrContainer::from_glib_none(ffi::gdk_content_formats_get_mime_types(
                    self.to_glib_none().0,
                    n_mime_types.as_mut_ptr(),
                ));
            let n_mime_types = n_mime_types.assume_init();
            (ret, n_mime_types)
        }
    }

    #[doc(alias = "gdk_content_formats_match")]
    pub fn match_(&self, second: &ContentFormats) -> bool {
        unsafe {
            from_glib(ffi::gdk_content_formats_match(
                self.to_glib_none().0,
                second.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_match_gtype")]
    pub fn match_type(&self, second: &ContentFormats) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gdk_content_formats_match_gtype(
                self.to_glib_none().0,
                second.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_match_mime_type")]
    pub fn match_mime_type(&self, second: &ContentFormats) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gdk_content_formats_match_mime_type(
                self.to_glib_none().0,
                second.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_print")]
    pub fn print(&self, string: &mut glib::String) {
        unsafe {
            ffi::gdk_content_formats_print(self.to_glib_none().0, string.to_glib_none_mut().0);
        }
    }

    #[doc(alias = "gdk_content_formats_to_string")]
    pub fn to_str(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::gdk_content_formats_to_string(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_content_formats_union")]
    pub fn union(&self, second: &ContentFormats) -> Option<ContentFormats> {
        unsafe {
            from_glib_full(ffi::gdk_content_formats_union(
                self.to_glib_full(),
                second.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_union_deserialize_gtypes")]
    pub fn union_deserialize_types(&self) -> Option<ContentFormats> {
        unsafe {
            from_glib_full(ffi::gdk_content_formats_union_deserialize_gtypes(
                self.to_glib_full(),
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_union_deserialize_mime_types")]
    pub fn union_deserialize_mime_types(&self) -> Option<ContentFormats> {
        unsafe {
            from_glib_full(ffi::gdk_content_formats_union_deserialize_mime_types(
                self.to_glib_full(),
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_union_serialize_gtypes")]
    pub fn union_serialize_types(&self) -> Option<ContentFormats> {
        unsafe {
            from_glib_full(ffi::gdk_content_formats_union_serialize_gtypes(
                self.to_glib_full(),
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_union_serialize_mime_types")]
    pub fn union_serialize_mime_types(&self) -> Option<ContentFormats> {
        unsafe {
            from_glib_full(ffi::gdk_content_formats_union_serialize_mime_types(
                self.to_glib_full(),
            ))
        }
    }
}

impl fmt::Display for ContentFormats {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}
