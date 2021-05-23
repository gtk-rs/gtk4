// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::SelectionModel;
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

glib::wrapper! {
    pub struct MultiSelection(Object<ffi::GtkMultiSelection, ffi::GtkMultiSelectionClass>) @implements gio::ListModel, SelectionModel;

    match fn {
        type_ => || ffi::gtk_multi_selection_get_type(),
    }
}

impl MultiSelection {
    #[doc(alias = "gtk_multi_selection_new")]
    pub fn new(model: Option<&impl IsA<gio::ListModel>>) -> MultiSelection {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_multi_selection_new(
                model.map(|p| p.as_ref()).to_glib_full(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`MultiSelection`]
    /// This method returns an instance of [`MultiSelectionBuilder`] which can be used to create a [`MultiSelection`].
    pub fn builder() -> MultiSelectionBuilder {
        MultiSelectionBuilder::default()
    }

    #[doc(alias = "gtk_multi_selection_get_model")]
    #[doc(alias = "get_model")]
    pub fn model(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(ffi::gtk_multi_selection_get_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_multi_selection_set_model")]
    pub fn set_model(&self, model: Option<&impl IsA<gio::ListModel>>) {
        unsafe {
            ffi::gtk_multi_selection_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "model")]
    pub fn connect_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&MultiSelection) + 'static>(
            this: *mut ffi::GtkMultiSelection,
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
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`MultiSelection`].
pub struct MultiSelectionBuilder {
    model: Option<gio::ListModel>,
}

impl MultiSelectionBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`MultiSelectionBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`MultiSelection`].
    pub fn build(self) -> MultiSelection {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref model) = self.model {
            properties.push(("model", model));
        }
        glib::Object::new::<MultiSelection>(&properties)
            .expect("Failed to create an instance of MultiSelection")
    }

    pub fn model(mut self, model: &impl IsA<gio::ListModel>) -> Self {
        self.model = Some(model.clone().upcast());
        self
    }
}

impl fmt::Display for MultiSelection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MultiSelection")
    }
}
