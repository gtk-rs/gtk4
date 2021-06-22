// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::LayoutChild;
use crate::LayoutManager;
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
    #[doc(alias = "GtkOverlayLayoutChild")]
    pub struct OverlayLayoutChild(Object<ffi::GtkOverlayLayoutChild, ffi::GtkOverlayLayoutChildClass>) @extends LayoutChild;

    match fn {
        type_ => || ffi::gtk_overlay_layout_child_get_type(),
    }
}

impl OverlayLayoutChild {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`OverlayLayoutChild`] objects.
    ///
    /// This method returns an instance of [`OverlayLayoutChildBuilder`] which can be used to create [`OverlayLayoutChild`] objects.
    pub fn builder() -> OverlayLayoutChildBuilder {
        OverlayLayoutChildBuilder::default()
    }

    #[doc(alias = "gtk_overlay_layout_child_get_clip_overlay")]
    #[doc(alias = "get_clip_overlay")]
    pub fn is_clip_overlay(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_overlay_layout_child_get_clip_overlay(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_overlay_layout_child_get_measure")]
    #[doc(alias = "get_measure")]
    pub fn is_measure(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_overlay_layout_child_get_measure(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_overlay_layout_child_set_clip_overlay")]
    pub fn set_clip_overlay(&self, clip_overlay: bool) {
        unsafe {
            ffi::gtk_overlay_layout_child_set_clip_overlay(
                self.to_glib_none().0,
                clip_overlay.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_overlay_layout_child_set_measure")]
    pub fn set_measure(&self, measure: bool) {
        unsafe {
            ffi::gtk_overlay_layout_child_set_measure(self.to_glib_none().0, measure.into_glib());
        }
    }

    #[doc(alias = "clip-overlay")]
    pub fn connect_clip_overlay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_clip_overlay_trampoline<
            F: Fn(&OverlayLayoutChild) + 'static,
        >(
            this: *mut ffi::GtkOverlayLayoutChild,
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
                b"notify::clip-overlay\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_clip_overlay_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "measure")]
    pub fn connect_measure_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_measure_trampoline<F: Fn(&OverlayLayoutChild) + 'static>(
            this: *mut ffi::GtkOverlayLayoutChild,
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
                b"notify::measure\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_measure_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`OverlayLayoutChild`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct OverlayLayoutChildBuilder {
    clip_overlay: Option<bool>,
    measure: Option<bool>,
    child_widget: Option<Widget>,
    layout_manager: Option<LayoutManager>,
}

impl OverlayLayoutChildBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`OverlayLayoutChildBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`OverlayLayoutChild`].
    pub fn build(self) -> OverlayLayoutChild {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref clip_overlay) = self.clip_overlay {
            properties.push(("clip-overlay", clip_overlay));
        }
        if let Some(ref measure) = self.measure {
            properties.push(("measure", measure));
        }
        if let Some(ref child_widget) = self.child_widget {
            properties.push(("child-widget", child_widget));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        glib::Object::new::<OverlayLayoutChild>(&properties)
            .expect("Failed to create an instance of OverlayLayoutChild")
    }

    pub fn clip_overlay(mut self, clip_overlay: bool) -> Self {
        self.clip_overlay = Some(clip_overlay);
        self
    }

    pub fn measure(mut self, measure: bool) -> Self {
        self.measure = Some(measure);
        self
    }

    pub fn child_widget<P: IsA<Widget>>(mut self, child_widget: &P) -> Self {
        self.child_widget = Some(child_widget.clone().upcast());
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }
}

impl fmt::Display for OverlayLayoutChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("OverlayLayoutChild")
    }
}
