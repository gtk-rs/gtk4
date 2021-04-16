// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::EventController;
use crate::PropagationLimit;
use crate::PropagationPhase;
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
    pub struct EventControllerLegacy(Object<ffi::GtkEventControllerLegacy, ffi::GtkEventControllerLegacyClass>) @extends EventController;

    match fn {
        type_ => || ffi::gtk_event_controller_legacy_get_type(),
    }
}

impl EventControllerLegacy {
    #[doc(alias = "gtk_event_controller_legacy_new")]
    pub fn new() -> EventControllerLegacy {
        assert_initialized_main_thread!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_event_controller_legacy_new()).unsafe_cast()
        }
    }

    pub fn connect_event<
        F: Fn(&EventControllerLegacy, &gdk::Event) -> glib::signal::Inhibit + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn event_trampoline<
            F: Fn(&EventControllerLegacy, &gdk::Event) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut ffi::GtkEventControllerLegacy,
            event: *mut gdk::ffi::GdkEvent,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(event)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    event_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for EventControllerLegacy {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct EventControllerLegacyBuilder {
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl EventControllerLegacyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> EventControllerLegacy {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref propagation_limit) = self.propagation_limit {
            properties.push(("propagation-limit", propagation_limit));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        let ret = glib::Object::new::<EventControllerLegacy>(&properties).expect("object new");
        ret
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn propagation_limit(mut self, propagation_limit: PropagationLimit) -> Self {
        self.propagation_limit = Some(propagation_limit);
        self
    }

    pub fn propagation_phase(mut self, propagation_phase: PropagationPhase) -> Self {
        self.propagation_phase = Some(propagation_phase);
        self
    }
}

impl fmt::Display for EventControllerLegacy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EventControllerLegacy")
    }
}
