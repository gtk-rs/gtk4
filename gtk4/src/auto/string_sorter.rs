// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Expression;
use crate::Sorter;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct StringSorter(Object<ffi::GtkStringSorter, ffi::GtkStringSorterClass>) @extends Sorter;

    match fn {
        type_ => || ffi::gtk_string_sorter_get_type(),
    }
}

impl StringSorter {
    #[doc(alias = "gtk_string_sorter_get_expression")]
    #[doc(alias = "get_expression")]
    pub fn expression(&self) -> Option<Expression> {
        unsafe { from_glib_none(ffi::gtk_string_sorter_get_expression(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_string_sorter_get_ignore_case")]
    #[doc(alias = "get_ignore_case")]
    pub fn ignores_case(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_string_sorter_get_ignore_case(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_string_sorter_set_ignore_case")]
    pub fn set_ignore_case(&self, ignore_case: bool) {
        unsafe {
            ffi::gtk_string_sorter_set_ignore_case(self.to_glib_none().0, ignore_case.into_glib());
        }
    }

    #[doc(alias = "expression")]
    pub fn connect_expression_notify<F: Fn(&StringSorter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_expression_trampoline<F: Fn(&StringSorter) + 'static>(
            this: *mut ffi::GtkStringSorter,
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

    #[doc(alias = "ignore-case")]
    pub fn connect_ignore_case_notify<F: Fn(&StringSorter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ignore_case_trampoline<F: Fn(&StringSorter) + 'static>(
            this: *mut ffi::GtkStringSorter,
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
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`StringSorter`].
pub struct StringSorterBuilder {
    expression: Option<Expression>,
    ignore_case: Option<bool>,
}

impl StringSorterBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`StringSorterBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`StringSorter`].
    pub fn build(self) -> StringSorter {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref expression) = self.expression {
            properties.push(("expression", expression));
        }
        if let Some(ref ignore_case) = self.ignore_case {
            properties.push(("ignore-case", ignore_case));
        }
        glib::Object::new::<StringSorter>(&properties)
            .expect("Failed to create an instance of StringSorter")
    }

    pub fn expression(mut self, expression: &Expression) -> Self {
        self.expression = Some(expression.clone());
        self
    }

    pub fn ignore_case(mut self, ignore_case: bool) -> Self {
        self.ignore_case = Some(ignore_case);
        self
    }
}

impl fmt::Display for StringSorter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StringSorter")
    }
}
