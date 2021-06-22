// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::EventController;
use crate::EventControllerScrollFlags;
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
    #[doc(alias = "GtkEventControllerScroll")]
    pub struct EventControllerScroll(Object<ffi::GtkEventControllerScroll, ffi::GtkEventControllerScrollClass>) @extends EventController;

    match fn {
        type_ => || ffi::gtk_event_controller_scroll_get_type(),
    }
}

impl EventControllerScroll {
    #[doc(alias = "gtk_event_controller_scroll_new")]
    pub fn new(flags: EventControllerScrollFlags) -> EventControllerScroll {
        assert_initialized_main_thread!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_event_controller_scroll_new(flags.into_glib()))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`EventControllerScroll`] objects.
    ///
    /// This method returns an instance of [`EventControllerScrollBuilder`] which can be used to create [`EventControllerScroll`] objects.
    pub fn builder() -> EventControllerScrollBuilder {
        EventControllerScrollBuilder::default()
    }

    #[doc(alias = "gtk_event_controller_scroll_get_flags")]
    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> EventControllerScrollFlags {
        unsafe {
            from_glib(ffi::gtk_event_controller_scroll_get_flags(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_event_controller_scroll_set_flags")]
    pub fn set_flags(&self, flags: EventControllerScrollFlags) {
        unsafe {
            ffi::gtk_event_controller_scroll_set_flags(self.to_glib_none().0, flags.into_glib());
        }
    }

    #[doc(alias = "decelerate")]
    pub fn connect_decelerate<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn decelerate_trampoline<
            F: Fn(&EventControllerScroll, f64, f64) + 'static,
        >(
            this: *mut ffi::GtkEventControllerScroll,
            vel_x: libc::c_double,
            vel_y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), vel_x, vel_y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"decelerate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    decelerate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scroll")]
    pub fn connect_scroll<F: Fn(&Self, f64, f64) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn scroll_trampoline<
            F: Fn(&EventControllerScroll, f64, f64) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut ffi::GtkEventControllerScroll,
            dx: libc::c_double,
            dy: libc::c_double,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), dx, dy).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scroll\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    scroll_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scroll-begin")]
    pub fn connect_scroll_begin<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn scroll_begin_trampoline<F: Fn(&EventControllerScroll) + 'static>(
            this: *mut ffi::GtkEventControllerScroll,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scroll-begin\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    scroll_begin_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scroll-end")]
    pub fn connect_scroll_end<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn scroll_end_trampoline<F: Fn(&EventControllerScroll) + 'static>(
            this: *mut ffi::GtkEventControllerScroll,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scroll-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    scroll_end_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "flags")]
    pub fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<F: Fn(&EventControllerScroll) + 'static>(
            this: *mut ffi::GtkEventControllerScroll,
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
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`EventControllerScroll`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct EventControllerScrollBuilder {
    flags: Option<EventControllerScrollFlags>,
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl EventControllerScrollBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`EventControllerScrollBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`EventControllerScroll`].
    pub fn build(self) -> EventControllerScroll {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref flags) = self.flags {
            properties.push(("flags", flags));
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
        glib::Object::new::<EventControllerScroll>(&properties)
            .expect("Failed to create an instance of EventControllerScroll")
    }

    pub fn flags(mut self, flags: EventControllerScrollFlags) -> Self {
        self.flags = Some(flags);
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

impl fmt::Display for EventControllerScroll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EventControllerScroll")
    }
}
