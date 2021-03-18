// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
use crate::IMContext;
use crate::PropagationLimit;
use crate::PropagationPhase;
use crate::Widget;
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
    pub struct EventControllerKey(Object<ffi::GtkEventControllerKey, ffi::GtkEventControllerKeyClass>) @extends EventController;

    match fn {
        get_type => || ffi::gtk_event_controller_key_get_type(),
    }
}

impl EventControllerKey {
    #[doc(alias = "gtk_event_controller_key_new")]
    pub fn new() -> EventControllerKey {
        assert_initialized_main_thread!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_event_controller_key_new()).unsafe_cast()
        }
    }

    #[doc(alias = "gtk_event_controller_key_forward")]
    pub fn forward<P: IsA<Widget>>(&self, widget: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_controller_key_forward(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_event_controller_key_get_group")]
    pub fn get_group(&self) -> u32 {
        unsafe { ffi::gtk_event_controller_key_get_group(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_event_controller_key_get_im_context")]
    pub fn get_im_context(&self) -> Option<IMContext> {
        unsafe {
            from_glib_none(ffi::gtk_event_controller_key_get_im_context(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_event_controller_key_set_im_context")]
    pub fn set_im_context<P: IsA<IMContext>>(&self, im_context: &P) {
        unsafe {
            ffi::gtk_event_controller_key_set_im_context(
                self.to_glib_none().0,
                im_context.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn connect_im_update<F: Fn(&EventControllerKey) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn im_update_trampoline<F: Fn(&EventControllerKey) + 'static>(
            this: *mut ffi::GtkEventControllerKey,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"im-update\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    im_update_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_modifiers<
        F: Fn(&EventControllerKey, gdk::ModifierType) -> glib::signal::Inhibit + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn modifiers_trampoline<
            F: Fn(&EventControllerKey, gdk::ModifierType) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut ffi::GtkEventControllerKey,
            keyval: gdk::ffi::GdkModifierType,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(keyval)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"modifiers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    modifiers_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for EventControllerKey {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct EventControllerKeyBuilder {
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl EventControllerKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> EventControllerKey {
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
        let ret = glib::Object::new::<EventControllerKey>(&properties).expect("object new");
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

impl fmt::Display for EventControllerKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EventControllerKey")
    }
}
