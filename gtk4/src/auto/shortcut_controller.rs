// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Buildable;
use crate::EventController;
use crate::PropagationLimit;
use crate::PropagationPhase;
use crate::Shortcut;
use crate::ShortcutScope;
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
    pub struct ShortcutController(Object<ffi::GtkShortcutController, ffi::GtkShortcutControllerClass>) @extends EventController, @implements gio::ListModel, Buildable;

    match fn {
        type_ => || ffi::gtk_shortcut_controller_get_type(),
    }
}

impl ShortcutController {
    #[doc(alias = "gtk_shortcut_controller_new")]
    pub fn new() -> ShortcutController {
        assert_initialized_main_thread!();
        unsafe { EventController::from_glib_full(ffi::gtk_shortcut_controller_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_shortcut_controller_new_for_model")]
    pub fn for_model<P: IsA<gio::ListModel>>(model: &P) -> ShortcutController {
        assert_initialized_main_thread!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_shortcut_controller_new_for_model(
                model.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_shortcut_controller_add_shortcut")]
    pub fn add_shortcut(&self, shortcut: &Shortcut) {
        unsafe {
            ffi::gtk_shortcut_controller_add_shortcut(
                self.to_glib_none().0,
                shortcut.to_glib_full(),
            );
        }
    }

    #[doc(alias = "gtk_shortcut_controller_get_mnemonics_modifiers")]
    pub fn mnemonics_modifiers(&self) -> gdk::ModifierType {
        unsafe {
            from_glib(ffi::gtk_shortcut_controller_get_mnemonics_modifiers(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_shortcut_controller_get_scope")]
    pub fn scope(&self) -> ShortcutScope {
        unsafe {
            from_glib(ffi::gtk_shortcut_controller_get_scope(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_shortcut_controller_remove_shortcut")]
    pub fn remove_shortcut(&self, shortcut: &Shortcut) {
        unsafe {
            ffi::gtk_shortcut_controller_remove_shortcut(
                self.to_glib_none().0,
                shortcut.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_shortcut_controller_set_mnemonics_modifiers")]
    pub fn set_mnemonics_modifiers(&self, modifiers: gdk::ModifierType) {
        unsafe {
            ffi::gtk_shortcut_controller_set_mnemonics_modifiers(
                self.to_glib_none().0,
                modifiers.to_glib(),
            );
        }
    }

    #[doc(alias = "gtk_shortcut_controller_set_scope")]
    pub fn set_scope(&self, scope: ShortcutScope) {
        unsafe {
            ffi::gtk_shortcut_controller_set_scope(self.to_glib_none().0, scope.to_glib());
        }
    }

    #[doc(alias = "get_property_mnemonic_modifiers")]
    pub fn mnemonic_modifiers(&self) -> gdk::ModifierType {
        unsafe {
            let mut value =
                glib::Value::from_type(<gdk::ModifierType as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"mnemonic-modifiers\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `mnemonic-modifiers` getter")
                .unwrap()
        }
    }

    #[doc(alias = "set_property_mnemonic_modifiers")]
    pub fn set_mnemonic_modifiers(&self, mnemonic_modifiers: gdk::ModifierType) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"mnemonic-modifiers\0".as_ptr() as *const _,
                glib::Value::from(&mnemonic_modifiers).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_mnemonic_modifiers_notify<F: Fn(&ShortcutController) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mnemonic_modifiers_trampoline<
            F: Fn(&ShortcutController) + 'static,
        >(
            this: *mut ffi::GtkShortcutController,
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
                b"notify::mnemonic-modifiers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mnemonic_modifiers_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_scope_notify<F: Fn(&ShortcutController) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_scope_trampoline<F: Fn(&ShortcutController) + 'static>(
            this: *mut ffi::GtkShortcutController,
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
}

impl Default for ShortcutController {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct ShortcutControllerBuilder {
    mnemonic_modifiers: Option<gdk::ModifierType>,
    model: Option<gio::ListModel>,
    scope: Option<ShortcutScope>,
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl ShortcutControllerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> ShortcutController {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref mnemonic_modifiers) = self.mnemonic_modifiers {
            properties.push(("mnemonic-modifiers", mnemonic_modifiers));
        }
        if let Some(ref model) = self.model {
            properties.push(("model", model));
        }
        if let Some(ref scope) = self.scope {
            properties.push(("scope", scope));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref propagation_limit) = self.propagation_limit {
            properties.push(("propagation-limit", propagation_limit));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        let ret = glib::Object::new::<ShortcutController>(&properties).expect("object new");
        ret
    }

    pub fn mnemonic_modifiers(mut self, mnemonic_modifiers: gdk::ModifierType) -> Self {
        self.mnemonic_modifiers = Some(mnemonic_modifiers);
        self
    }

    pub fn model<P: IsA<gio::ListModel>>(mut self, model: &P) -> Self {
        self.model = Some(model.clone().upcast());
        self
    }

    pub fn scope(mut self, scope: ShortcutScope) -> Self {
        self.scope = Some(scope);
        self
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

impl fmt::Display for ShortcutController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ShortcutController")
    }
}
