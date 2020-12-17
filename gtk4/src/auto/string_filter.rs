// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Filter;
use crate::StringFilterMatchMode;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct StringFilter(Object<ffi::GtkStringFilter, ffi::GtkStringFilterClass>) @extends Filter;

    match fn {
        get_type => || ffi::gtk_string_filter_get_type(),
    }
}

impl StringFilter {
    //#[doc(alias = "gtk_string_filter_new")]
    //pub fn new(expression: /*Ignored*/Option<&Expression>) -> StringFilter {
    //    unsafe { TODO: call ffi:gtk_string_filter_new() }
    //}

    //#[doc(alias = "gtk_string_filter_get_expression")]
    //pub fn get_expression(&self) -> /*Ignored*/Expression {
    //    unsafe { TODO: call ffi:gtk_string_filter_get_expression() }
    //}

    #[doc(alias = "gtk_string_filter_get_ignore_case")]
    pub fn get_ignore_case(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_string_filter_get_ignore_case(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_string_filter_get_match_mode")]
    pub fn get_match_mode(&self) -> StringFilterMatchMode {
        unsafe { from_glib(ffi::gtk_string_filter_get_match_mode(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_string_filter_get_search")]
    pub fn get_search(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_string_filter_get_search(self.to_glib_none().0)) }
    }

    //#[doc(alias = "gtk_string_filter_set_expression")]
    //pub fn set_expression(&self, expression: /*Ignored*/&Expression) {
    //    unsafe { TODO: call ffi:gtk_string_filter_set_expression() }
    //}

    #[doc(alias = "gtk_string_filter_set_ignore_case")]
    pub fn set_ignore_case(&self, ignore_case: bool) {
        unsafe {
            ffi::gtk_string_filter_set_ignore_case(self.to_glib_none().0, ignore_case.to_glib());
        }
    }

    #[doc(alias = "gtk_string_filter_set_match_mode")]
    pub fn set_match_mode(&self, mode: StringFilterMatchMode) {
        unsafe {
            ffi::gtk_string_filter_set_match_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    #[doc(alias = "gtk_string_filter_set_search")]
    pub fn set_search(&self, search: Option<&str>) {
        unsafe {
            ffi::gtk_string_filter_set_search(self.to_glib_none().0, search.to_glib_none().0);
        }
    }

    pub fn connect_property_expression_notify<F: Fn(&StringFilter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_expression_trampoline<F: Fn(&StringFilter) + 'static>(
            this: *mut ffi::GtkStringFilter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::expression\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expression_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_ignore_case_notify<F: Fn(&StringFilter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ignore_case_trampoline<F: Fn(&StringFilter) + 'static>(
            this: *mut ffi::GtkStringFilter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ignore-case\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ignore_case_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_match_mode_notify<F: Fn(&StringFilter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_match_mode_trampoline<F: Fn(&StringFilter) + 'static>(
            this: *mut ffi::GtkStringFilter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::match-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_match_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_search_notify<F: Fn(&StringFilter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_trampoline<F: Fn(&StringFilter) + 'static>(
            this: *mut ffi::GtkStringFilter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::search\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_search_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct StringFilterBuilder {
    //expression: /*Unknown type*/,
    ignore_case: Option<bool>,
    match_mode: Option<StringFilterMatchMode>,
    search: Option<String>,
}

impl StringFilterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> StringFilter {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref ignore_case) = self.ignore_case {
            properties.push(("ignore-case", ignore_case));
        }
        if let Some(ref match_mode) = self.match_mode {
            properties.push(("match-mode", match_mode));
        }
        if let Some(ref search) = self.search {
            properties.push(("search", search));
        }
        let ret = glib::Object::new::<StringFilter>(&properties).expect("object new");
        ret
    }

    pub fn ignore_case(mut self, ignore_case: bool) -> Self {
        self.ignore_case = Some(ignore_case);
        self
    }

    pub fn match_mode(mut self, match_mode: StringFilterMatchMode) -> Self {
        self.match_mode = Some(match_mode);
        self
    }

    pub fn search(mut self, search: &str) -> Self {
        self.search = Some(search.to_string());
        self
    }
}

impl fmt::Display for StringFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StringFilter")
    }
}
