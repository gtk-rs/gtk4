// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::BuilderClosureFlags;
use crate::BuilderScope;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    pub struct Builder(Object<ffi::GtkBuilder, ffi::GtkBuilderClass>);

    match fn {
        type_ => || ffi::gtk_builder_get_type(),
    }
}

impl Builder {
    #[doc(alias = "gtk_builder_new")]
    pub fn new() -> Builder {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_builder_new()) }
    }

    #[doc(alias = "gtk_builder_new_from_resource")]
    #[doc(alias = "new_from_resource")]
    pub fn from_resource(resource_path: &str) -> Builder {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_builder_new_from_resource(
                resource_path.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_builder_new_from_string")]
    #[doc(alias = "new_from_string")]
    pub fn from_string(string: &str) -> Builder {
        assert_initialized_main_thread!();
        let length = string.len() as isize;
        unsafe {
            from_glib_full(ffi::gtk_builder_new_from_string(
                string.to_glib_none().0,
                length,
            ))
        }
    }

    #[doc(alias = "gtk_builder_add_from_resource")]
    pub fn add_from_resource(&self, resource_path: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_builder_add_from_resource(
                self.to_glib_none().0,
                resource_path.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_builder_add_from_string")]
    pub fn add_from_string(&self, buffer: &str) -> Result<(), glib::Error> {
        let length = buffer.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_builder_add_from_string(
                self.to_glib_none().0,
                buffer.to_glib_none().0,
                length,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_builder_add_objects_from_resource")]
    pub fn add_objects_from_resource(
        &self,
        resource_path: &str,
        object_ids: &[&str],
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_builder_add_objects_from_resource(
                self.to_glib_none().0,
                resource_path.to_glib_none().0,
                object_ids.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_builder_add_objects_from_string")]
    pub fn add_objects_from_string(
        &self,
        buffer: &str,
        object_ids: &[&str],
    ) -> Result<(), glib::Error> {
        let length = buffer.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_builder_add_objects_from_string(
                self.to_glib_none().0,
                buffer.to_glib_none().0,
                length,
                object_ids.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_builder_create_closure")]
    pub fn create_closure<P: IsA<glib::Object>>(
        &self,
        function_name: &str,
        flags: BuilderClosureFlags,
        object: Option<&P>,
    ) -> Result<Option<glib::Closure>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_builder_create_closure(
                self.to_glib_none().0,
                function_name.to_glib_none().0,
                flags.into_glib(),
                object.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_builder_expose_object")]
    pub fn expose_object<P: IsA<glib::Object>>(&self, name: &str, object: &P) {
        unsafe {
            ffi::gtk_builder_expose_object(
                self.to_glib_none().0,
                name.to_glib_none().0,
                object.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_builder_extend_with_template")]
    pub fn extend_with_template<P: IsA<glib::Object>>(
        &self,
        object: &P,
        template_type: glib::types::Type,
        buffer: &str,
    ) -> Result<(), glib::Error> {
        let length = buffer.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_builder_extend_with_template(
                self.to_glib_none().0,
                object.as_ref().to_glib_none().0,
                template_type.into_glib(),
                buffer.to_glib_none().0,
                length,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_builder_get_current_object")]
    #[doc(alias = "get_current_object")]
    pub fn current_object(&self) -> Option<glib::Object> {
        unsafe { from_glib_none(ffi::gtk_builder_get_current_object(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_builder_get_objects")]
    #[doc(alias = "get_objects")]
    pub fn objects(&self) -> Vec<glib::Object> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_builder_get_objects(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_builder_get_scope")]
    #[doc(alias = "get_scope")]
    pub fn scope(&self) -> BuilderScope {
        unsafe { from_glib_none(ffi::gtk_builder_get_scope(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_builder_get_translation_domain")]
    #[doc(alias = "get_translation_domain")]
    pub fn translation_domain(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_builder_get_translation_domain(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_builder_get_type_from_name")]
    #[doc(alias = "get_type_from_name")]
    pub fn type_from_name(&self, type_name: &str) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gtk_builder_get_type_from_name(
                self.to_glib_none().0,
                type_name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_builder_set_current_object")]
    pub fn set_current_object<P: IsA<glib::Object>>(&self, current_object: Option<&P>) {
        unsafe {
            ffi::gtk_builder_set_current_object(
                self.to_glib_none().0,
                current_object.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_builder_set_scope")]
    pub fn set_scope<P: IsA<BuilderScope>>(&self, scope: Option<&P>) {
        unsafe {
            ffi::gtk_builder_set_scope(
                self.to_glib_none().0,
                scope.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_builder_set_translation_domain")]
    pub fn set_translation_domain(&self, domain: Option<&str>) {
        unsafe {
            ffi::gtk_builder_set_translation_domain(self.to_glib_none().0, domain.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_builder_value_from_string_type")]
    pub fn value_from_string_type(
        &self,
        type_: glib::types::Type,
        string: &str,
    ) -> Result<glib::Value, glib::Error> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_builder_value_from_string_type(
                self.to_glib_none().0,
                type_.into_glib(),
                string.to_glib_none().0,
                value.to_glib_none_mut().0,
                &mut error,
            );
            if error.is_null() {
                Ok(value)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "current-object")]
    pub fn connect_current_object_notify<F: Fn(&Builder) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_object_trampoline<F: Fn(&Builder) + 'static>(
            this: *mut ffi::GtkBuilder,
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
                b"notify::current-object\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_current_object_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scope")]
    pub fn connect_scope_notify<F: Fn(&Builder) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scope_trampoline<F: Fn(&Builder) + 'static>(
            this: *mut ffi::GtkBuilder,
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
                b"notify::scope\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scope_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "translation-domain")]
    pub fn connect_translation_domain_notify<F: Fn(&Builder) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_translation_domain_trampoline<F: Fn(&Builder) + 'static>(
            this: *mut ffi::GtkBuilder,
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
                b"notify::translation-domain\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_translation_domain_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Builder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Builder")
    }
}
