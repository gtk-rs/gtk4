// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::EventController;
use crate::Gesture;
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
    pub struct GestureRotate(Object<ffi::GtkGestureRotate, ffi::GtkGestureRotateClass>) @extends Gesture, EventController;

    match fn {
        type_ => || ffi::gtk_gesture_rotate_get_type(),
    }
}

impl GestureRotate {
    #[doc(alias = "gtk_gesture_rotate_new")]
    pub fn new() -> GestureRotate {
        assert_initialized_main_thread!();
        unsafe { Gesture::from_glib_full(ffi::gtk_gesture_rotate_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_gesture_rotate_get_angle_delta")]
    pub fn angle_delta(&self) -> f64 {
        unsafe { ffi::gtk_gesture_rotate_get_angle_delta(self.to_glib_none().0) }
    }

    pub fn connect_angle_changed<F: Fn(&GestureRotate, f64, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn angle_changed_trampoline<F: Fn(&GestureRotate, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureRotate,
            angle: libc::c_double,
            angle_delta: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), angle, angle_delta)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"angle-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    angle_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GestureRotate {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct GestureRotateBuilder {
    n_points: Option<u32>,
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl GestureRotateBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> GestureRotate {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref n_points) = self.n_points {
            properties.push(("n-points", n_points));
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
        let ret = glib::Object::new::<GestureRotate>(&properties).expect("object new");
        ret
    }

    pub fn n_points(mut self, n_points: u32) -> Self {
        self.n_points = Some(n_points);
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

impl fmt::Display for GestureRotate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GestureRotate")
    }
}
