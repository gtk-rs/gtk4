// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ShortcutTrigger;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    pub struct AlternativeTrigger(Object<ffi::GtkAlternativeTrigger, ffi::GtkAlternativeTriggerClass>) @extends ShortcutTrigger;

    match fn {
        get_type => || ffi::gtk_alternative_trigger_get_type(),
    }
}

impl AlternativeTrigger {
    #[doc(alias = "gtk_alternative_trigger_new")]
    pub fn new<P: IsA<ShortcutTrigger>, Q: IsA<ShortcutTrigger>>(
        first: &P,
        second: &Q,
    ) -> AlternativeTrigger {
        skip_assert_initialized!();
        unsafe {
            ShortcutTrigger::from_glib_full(ffi::gtk_alternative_trigger_new(
                first.as_ref().to_glib_full(),
                second.as_ref().to_glib_full(),
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_alternative_trigger_get_first")]
    pub fn get_first(&self) -> ShortcutTrigger {
        unsafe {
            from_glib_none(ffi::gtk_alternative_trigger_get_first(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_alternative_trigger_get_second")]
    pub fn get_second(&self) -> ShortcutTrigger {
        unsafe {
            from_glib_none(ffi::gtk_alternative_trigger_get_second(
                self.to_glib_none().0,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct AlternativeTriggerBuilder {
    first: Option<ShortcutTrigger>,
    second: Option<ShortcutTrigger>,
}

impl AlternativeTriggerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> AlternativeTrigger {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref first) = self.first {
            properties.push(("first", first));
        }
        if let Some(ref second) = self.second {
            properties.push(("second", second));
        }
        let ret = glib::Object::new::<AlternativeTrigger>(&properties).expect("object new");
        ret
    }

    pub fn first<P: IsA<ShortcutTrigger>>(mut self, first: &P) -> Self {
        self.first = Some(first.clone().upcast());
        self
    }

    pub fn second<P: IsA<ShortcutTrigger>>(mut self, second: &P) -> Self {
        self.second = Some(second.clone().upcast());
        self
    }
}

impl fmt::Display for AlternativeTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AlternativeTrigger")
    }
}
