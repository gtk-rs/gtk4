// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use ExpressionWatch;

glib_wrapper! {
    pub struct Expression(Object<gtk_sys::GtkExpression, ExpressionClass>);

    match fn {
        get_type => || gtk_sys::gtk_expression_get_type(),
    }
}

pub const NONE_EXPRESSION: Option<&Expression> = None;

pub trait ExpressionExt: 'static {
    fn bind<P: IsA<glib::Object>, Q: IsA<glib::Object>>(
        &self,
        target: &P,
        property: &str,
        this_: Option<&Q>,
    ) -> Option<ExpressionWatch>;

    fn evaluate<P: IsA<glib::Object>>(&self, this_: Option<&P>, value: &mut glib::Value) -> bool;

    fn get_value_type(&self) -> glib::types::Type;

    fn is_static(&self) -> bool;

    fn watch<P: IsA<glib::Object>, Q: Fn() + 'static>(
        &self,
        this_: Option<&P>,
        notify: Q,
    ) -> Option<ExpressionWatch>;
}

impl<O: IsA<Expression>> ExpressionExt for O {
    fn bind<P: IsA<glib::Object>, Q: IsA<glib::Object>>(
        &self,
        target: &P,
        property: &str,
        this_: Option<&Q>,
    ) -> Option<ExpressionWatch> {
        unsafe {
            from_glib_none(gtk_sys::gtk_expression_bind(
                self.as_ref().to_glib_full(),
                target.as_ref().to_glib_none().0,
                property.to_glib_none().0,
                this_.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    fn evaluate<P: IsA<glib::Object>>(&self, this_: Option<&P>, value: &mut glib::Value) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_expression_evaluate(
                self.as_ref().to_glib_none().0,
                this_.map(|p| p.as_ref()).to_glib_none().0,
                value.to_glib_none_mut().0,
            ))
        }
    }

    fn get_value_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(gtk_sys::gtk_expression_get_value_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_static(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_expression_is_static(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn watch<P: IsA<glib::Object>, Q: Fn() + 'static>(
        &self,
        this_: Option<&P>,
        notify: Q,
    ) -> Option<ExpressionWatch> {
        let notify_data: Box_<Q> = Box_::new(notify);
        unsafe extern "C" fn notify_func<P: IsA<glib::Object>, Q: Fn() + 'static>(
            user_data: glib_sys::gpointer,
        ) {
            let callback: &Q = &*(user_data as *mut _);
            (*callback)();
        }
        let notify = Some(notify_func::<P, Q> as _);
        unsafe extern "C" fn user_destroy_func<P: IsA<glib::Object>, Q: Fn() + 'static>(
            data: glib_sys::gpointer,
        ) {
            let _callback: Box_<Q> = Box_::from_raw(data as *mut _);
        }
        let destroy_call4 = Some(user_destroy_func::<P, Q> as _);
        let super_callback0: Box_<Q> = notify_data;
        unsafe {
            from_glib_none(gtk_sys::gtk_expression_watch(
                self.as_ref().to_glib_none().0,
                this_.map(|p| p.as_ref()).to_glib_none().0,
                notify,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call4,
            ))
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expression")
    }
}