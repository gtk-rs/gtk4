// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ConstraintStrength;
use crate::ConstraintTarget;
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
    #[doc(alias = "GtkConstraintGuide")]
    pub struct ConstraintGuide(Object<ffi::GtkConstraintGuide, ffi::GtkConstraintGuideClass>) @implements ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_constraint_guide_get_type(),
    }
}

impl ConstraintGuide {
    #[doc(alias = "gtk_constraint_guide_new")]
    pub fn new() -> ConstraintGuide {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_constraint_guide_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ConstraintGuide`] objects.
    ///
    /// This method returns an instance of [`ConstraintGuideBuilder`] which can be used to create [`ConstraintGuide`] objects.
    pub fn builder() -> ConstraintGuideBuilder {
        ConstraintGuideBuilder::default()
    }

    #[doc(alias = "gtk_constraint_guide_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_constraint_guide_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_guide_get_strength")]
    #[doc(alias = "get_strength")]
    pub fn strength(&self) -> ConstraintStrength {
        unsafe {
            from_glib(ffi::gtk_constraint_guide_get_strength(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_constraint_guide_set_max_size")]
    pub fn set_max_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_constraint_guide_set_max_size(self.to_glib_none().0, width, height);
        }
    }

    #[doc(alias = "gtk_constraint_guide_set_min_size")]
    pub fn set_min_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_constraint_guide_set_min_size(self.to_glib_none().0, width, height);
        }
    }

    #[doc(alias = "gtk_constraint_guide_set_name")]
    pub fn set_name(&self, name: Option<&str>) {
        unsafe {
            ffi::gtk_constraint_guide_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_constraint_guide_set_nat_size")]
    pub fn set_nat_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_constraint_guide_set_nat_size(self.to_glib_none().0, width, height);
        }
    }

    #[doc(alias = "gtk_constraint_guide_set_strength")]
    pub fn set_strength(&self, strength: ConstraintStrength) {
        unsafe {
            ffi::gtk_constraint_guide_set_strength(self.to_glib_none().0, strength.into_glib());
        }
    }

    #[doc(alias = "max-height")]
    pub fn max_height(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"max-height\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-height` getter")
        }
    }

    #[doc(alias = "max-height")]
    pub fn set_max_height(&self, max_height: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"max-height\0".as_ptr() as *const _,
                max_height.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "max-width")]
    pub fn max_width(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"max-width\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-width` getter")
        }
    }

    #[doc(alias = "max-width")]
    pub fn set_max_width(&self, max_width: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"max-width\0".as_ptr() as *const _,
                max_width.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "min-height")]
    pub fn min_height(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"min-height\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `min-height` getter")
        }
    }

    #[doc(alias = "min-height")]
    pub fn set_min_height(&self, min_height: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"min-height\0".as_ptr() as *const _,
                min_height.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "min-width")]
    pub fn min_width(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"min-width\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `min-width` getter")
        }
    }

    #[doc(alias = "min-width")]
    pub fn set_min_width(&self, min_width: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"min-width\0".as_ptr() as *const _,
                min_width.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "nat-height")]
    pub fn nat_height(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"nat-height\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `nat-height` getter")
        }
    }

    #[doc(alias = "nat-height")]
    pub fn set_nat_height(&self, nat_height: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"nat-height\0".as_ptr() as *const _,
                nat_height.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "nat-width")]
    pub fn nat_width(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"nat-width\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `nat-width` getter")
        }
    }

    #[doc(alias = "nat-width")]
    pub fn set_nat_width(&self, nat_width: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"nat-width\0".as_ptr() as *const _,
                nat_width.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "max-height")]
    pub fn connect_max_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_height_trampoline<F: Fn(&ConstraintGuide) + 'static>(
            this: *mut ffi::GtkConstraintGuide,
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
                b"notify::max-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-width")]
    pub fn connect_max_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_width_trampoline<F: Fn(&ConstraintGuide) + 'static>(
            this: *mut ffi::GtkConstraintGuide,
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
                b"notify::max-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-height")]
    pub fn connect_min_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_height_trampoline<F: Fn(&ConstraintGuide) + 'static>(
            this: *mut ffi::GtkConstraintGuide,
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
                b"notify::min-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-width")]
    pub fn connect_min_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_width_trampoline<F: Fn(&ConstraintGuide) + 'static>(
            this: *mut ffi::GtkConstraintGuide,
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
                b"notify::min-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "name")]
    pub fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&ConstraintGuide) + 'static>(
            this: *mut ffi::GtkConstraintGuide,
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
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "nat-height")]
    pub fn connect_nat_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_nat_height_trampoline<F: Fn(&ConstraintGuide) + 'static>(
            this: *mut ffi::GtkConstraintGuide,
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
                b"notify::nat-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_nat_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "nat-width")]
    pub fn connect_nat_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_nat_width_trampoline<F: Fn(&ConstraintGuide) + 'static>(
            this: *mut ffi::GtkConstraintGuide,
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
                b"notify::nat-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_nat_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "strength")]
    pub fn connect_strength_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_strength_trampoline<F: Fn(&ConstraintGuide) + 'static>(
            this: *mut ffi::GtkConstraintGuide,
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
                b"notify::strength\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_strength_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for ConstraintGuide {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ConstraintGuide`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct ConstraintGuideBuilder {
    max_height: Option<i32>,
    max_width: Option<i32>,
    min_height: Option<i32>,
    min_width: Option<i32>,
    name: Option<String>,
    nat_height: Option<i32>,
    nat_width: Option<i32>,
    strength: Option<ConstraintStrength>,
}

impl ConstraintGuideBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ConstraintGuideBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ConstraintGuide`].
    pub fn build(self) -> ConstraintGuide {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref max_height) = self.max_height {
            properties.push(("max-height", max_height));
        }
        if let Some(ref max_width) = self.max_width {
            properties.push(("max-width", max_width));
        }
        if let Some(ref min_height) = self.min_height {
            properties.push(("min-height", min_height));
        }
        if let Some(ref min_width) = self.min_width {
            properties.push(("min-width", min_width));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref nat_height) = self.nat_height {
            properties.push(("nat-height", nat_height));
        }
        if let Some(ref nat_width) = self.nat_width {
            properties.push(("nat-width", nat_width));
        }
        if let Some(ref strength) = self.strength {
            properties.push(("strength", strength));
        }
        glib::Object::new::<ConstraintGuide>(&properties)
            .expect("Failed to create an instance of ConstraintGuide")
    }

    pub fn max_height(mut self, max_height: i32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    pub fn max_width(mut self, max_width: i32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    pub fn min_height(mut self, min_height: i32) -> Self {
        self.min_height = Some(min_height);
        self
    }

    pub fn min_width(mut self, min_width: i32) -> Self {
        self.min_width = Some(min_width);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn nat_height(mut self, nat_height: i32) -> Self {
        self.nat_height = Some(nat_height);
        self
    }

    pub fn nat_width(mut self, nat_width: i32) -> Self {
        self.nat_width = Some(nat_width);
        self
    }

    pub fn strength(mut self, strength: ConstraintStrength) -> Self {
        self.strength = Some(strength);
        self
    }
}

impl fmt::Display for ConstraintGuide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ConstraintGuide")
    }
}
