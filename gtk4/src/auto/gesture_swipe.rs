// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
use crate::Gesture;
use crate::GestureSingle;
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
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    pub struct GestureSwipe(Object<ffi::GtkGestureSwipe, ffi::GtkGestureSwipeClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_swipe_get_type(),
    }
}

impl GestureSwipe {
    #[doc(alias = "gtk_gesture_swipe_new")]
    pub fn new() -> GestureSwipe {
        assert_initialized_main_thread!();
        unsafe { Gesture::from_glib_full(ffi::gtk_gesture_swipe_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_gesture_swipe_get_velocity")]
    pub fn get_velocity(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut velocity_x = mem::MaybeUninit::uninit();
            let mut velocity_y = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_gesture_swipe_get_velocity(
                self.to_glib_none().0,
                velocity_x.as_mut_ptr(),
                velocity_y.as_mut_ptr(),
            ));
            let velocity_x = velocity_x.assume_init();
            let velocity_y = velocity_y.assume_init();
            if ret {
                Some((velocity_x, velocity_y))
            } else {
                None
            }
        }
    }

    pub fn connect_swipe<F: Fn(&GestureSwipe, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn swipe_trampoline<F: Fn(&GestureSwipe, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureSwipe,
            velocity_x: libc::c_double,
            velocity_y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), velocity_x, velocity_y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"swipe\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    swipe_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GestureSwipe {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct GestureSwipeBuilder {
    button: Option<u32>,
    exclusive: Option<bool>,
    touch_only: Option<bool>,
    n_points: Option<u32>,
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl GestureSwipeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> GestureSwipe {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref button) = self.button {
            properties.push(("button", button));
        }
        if let Some(ref exclusive) = self.exclusive {
            properties.push(("exclusive", exclusive));
        }
        if let Some(ref touch_only) = self.touch_only {
            properties.push(("touch-only", touch_only));
        }
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
        let ret = glib::Object::new::<GestureSwipe>(&properties).expect("object new");
        ret
    }

    pub fn button(mut self, button: u32) -> Self {
        self.button = Some(button);
        self
    }

    pub fn exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = Some(exclusive);
        self
    }

    pub fn touch_only(mut self, touch_only: bool) -> Self {
        self.touch_only = Some(touch_only);
        self
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

impl fmt::Display for GestureSwipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GestureSwipe")
    }
}
