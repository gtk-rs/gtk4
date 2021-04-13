// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::BuilderScope;
use crate::ListItemFactory;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    pub struct BuilderListItemFactory(Object<ffi::GtkBuilderListItemFactory, ffi::GtkBuilderListItemFactoryClass>) @extends ListItemFactory;

    match fn {
        get_type => || ffi::gtk_builder_list_item_factory_get_type(),
    }
}

impl BuilderListItemFactory {
    #[doc(alias = "gtk_builder_list_item_factory_new_from_bytes")]
    pub fn from_bytes<P: IsA<BuilderScope>>(
        scope: Option<&P>,
        bytes: &glib::Bytes,
    ) -> BuilderListItemFactory {
        assert_initialized_main_thread!();
        unsafe {
            ListItemFactory::from_glib_full(ffi::gtk_builder_list_item_factory_new_from_bytes(
                scope.map(|p| p.as_ref()).to_glib_none().0,
                bytes.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_builder_list_item_factory_new_from_resource")]
    pub fn from_resource<P: IsA<BuilderScope>>(
        scope: Option<&P>,
        resource_path: &str,
    ) -> BuilderListItemFactory {
        assert_initialized_main_thread!();
        unsafe {
            ListItemFactory::from_glib_full(ffi::gtk_builder_list_item_factory_new_from_resource(
                scope.map(|p| p.as_ref()).to_glib_none().0,
                resource_path.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_builder_list_item_factory_get_bytes")]
    pub fn bytes(&self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_none(ffi::gtk_builder_list_item_factory_get_bytes(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_builder_list_item_factory_get_resource")]
    pub fn resource(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_builder_list_item_factory_get_resource(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_builder_list_item_factory_get_scope")]
    pub fn scope(&self) -> Option<BuilderScope> {
        unsafe {
            from_glib_none(ffi::gtk_builder_list_item_factory_get_scope(
                self.to_glib_none().0,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct BuilderListItemFactoryBuilder {
    bytes: Option<glib::Bytes>,
    resource: Option<String>,
    scope: Option<BuilderScope>,
}

impl BuilderListItemFactoryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> BuilderListItemFactory {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref bytes) = self.bytes {
            properties.push(("bytes", bytes));
        }
        if let Some(ref resource) = self.resource {
            properties.push(("resource", resource));
        }
        if let Some(ref scope) = self.scope {
            properties.push(("scope", scope));
        }
        let ret = glib::Object::new::<BuilderListItemFactory>(&properties).expect("object new");
        ret
    }

    pub fn bytes(mut self, bytes: &glib::Bytes) -> Self {
        self.bytes = Some(bytes.clone());
        self
    }

    pub fn resource(mut self, resource: &str) -> Self {
        self.resource = Some(resource.to_string());
        self
    }

    pub fn scope<P: IsA<BuilderScope>>(mut self, scope: &P) -> Self {
        self.scope = Some(scope.clone().upcast());
        self
    }
}

impl fmt::Display for BuilderListItemFactory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BuilderListItemFactory")
    }
}
