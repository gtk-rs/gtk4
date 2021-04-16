// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::CellRenderer;
use crate::CellRendererMode;
use crate::Orientable;
use crate::Orientation;
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
    pub struct CellRendererProgress(Object<ffi::GtkCellRendererProgress>) @extends CellRenderer, @implements Orientable;

    match fn {
        type_ => || ffi::gtk_cell_renderer_progress_get_type(),
    }
}

impl CellRendererProgress {
    #[doc(alias = "gtk_cell_renderer_progress_new")]
    pub fn new() -> CellRendererProgress {
        assert_initialized_main_thread!();
        unsafe { CellRenderer::from_glib_none(ffi::gtk_cell_renderer_progress_new()).unsafe_cast() }
    }

    #[doc(alias = "get_property_inverted")]
    pub fn is_inverted(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"inverted\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `inverted` getter")
                .unwrap()
        }
    }

    #[doc(alias = "set_property_inverted")]
    pub fn set_inverted(&self, inverted: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"inverted\0".as_ptr() as *const _,
                glib::Value::from(&inverted).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_pulse")]
    pub fn pulse(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"pulse\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `pulse` getter")
                .unwrap()
        }
    }

    #[doc(alias = "set_property_pulse")]
    pub fn set_pulse(&self, pulse: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"pulse\0".as_ptr() as *const _,
                glib::Value::from(&pulse).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_text")]
    pub fn text(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"text\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `text` getter")
        }
    }

    #[doc(alias = "set_property_text")]
    pub fn set_text(&self, text: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"text\0".as_ptr() as *const _,
                glib::Value::from(text).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_text_xalign")]
    pub fn text_xalign(&self) -> f32 {
        unsafe {
            let mut value = glib::Value::from_type(<f32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"text-xalign\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `text-xalign` getter")
                .unwrap()
        }
    }

    #[doc(alias = "set_property_text_xalign")]
    pub fn set_text_xalign(&self, text_xalign: f32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"text-xalign\0".as_ptr() as *const _,
                glib::Value::from(&text_xalign).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_text_yalign")]
    pub fn text_yalign(&self) -> f32 {
        unsafe {
            let mut value = glib::Value::from_type(<f32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"text-yalign\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `text-yalign` getter")
                .unwrap()
        }
    }

    #[doc(alias = "set_property_text_yalign")]
    pub fn set_text_yalign(&self, text_yalign: f32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"text-yalign\0".as_ptr() as *const _,
                glib::Value::from(&text_yalign).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_value")]
    pub fn value(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"value\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `value` getter")
                .unwrap()
        }
    }

    #[doc(alias = "set_property_value")]
    pub fn set_value(&self, value: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"value\0".as_ptr() as *const _,
                glib::Value::from(&value).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_inverted_notify<F: Fn(&CellRendererProgress) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_inverted_trampoline<F: Fn(&CellRendererProgress) + 'static>(
            this: *mut ffi::GtkCellRendererProgress,
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
                b"notify::inverted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inverted_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_pulse_notify<F: Fn(&CellRendererProgress) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pulse_trampoline<F: Fn(&CellRendererProgress) + 'static>(
            this: *mut ffi::GtkCellRendererProgress,
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
                b"notify::pulse\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pulse_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_text_notify<F: Fn(&CellRendererProgress) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<F: Fn(&CellRendererProgress) + 'static>(
            this: *mut ffi::GtkCellRendererProgress,
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
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_text_xalign_notify<F: Fn(&CellRendererProgress) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_xalign_trampoline<
            F: Fn(&CellRendererProgress) + 'static,
        >(
            this: *mut ffi::GtkCellRendererProgress,
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
                b"notify::text-xalign\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_xalign_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_text_yalign_notify<F: Fn(&CellRendererProgress) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_yalign_trampoline<
            F: Fn(&CellRendererProgress) + 'static,
        >(
            this: *mut ffi::GtkCellRendererProgress,
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
                b"notify::text-yalign\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_yalign_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_value_notify<F: Fn(&CellRendererProgress) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<F: Fn(&CellRendererProgress) + 'static>(
            this: *mut ffi::GtkCellRendererProgress,
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
                b"notify::value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for CellRendererProgress {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct CellRendererProgressBuilder {
    inverted: Option<bool>,
    pulse: Option<i32>,
    text: Option<String>,
    text_xalign: Option<f32>,
    text_yalign: Option<f32>,
    value: Option<i32>,
    cell_background: Option<String>,
    cell_background_rgba: Option<gdk::RGBA>,
    cell_background_set: Option<bool>,
    height: Option<i32>,
    is_expanded: Option<bool>,
    is_expander: Option<bool>,
    mode: Option<CellRendererMode>,
    sensitive: Option<bool>,
    visible: Option<bool>,
    width: Option<i32>,
    xalign: Option<f32>,
    xpad: Option<u32>,
    yalign: Option<f32>,
    ypad: Option<u32>,
    orientation: Option<Orientation>,
}

impl CellRendererProgressBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> CellRendererProgress {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref inverted) = self.inverted {
            properties.push(("inverted", inverted));
        }
        if let Some(ref pulse) = self.pulse {
            properties.push(("pulse", pulse));
        }
        if let Some(ref text) = self.text {
            properties.push(("text", text));
        }
        if let Some(ref text_xalign) = self.text_xalign {
            properties.push(("text-xalign", text_xalign));
        }
        if let Some(ref text_yalign) = self.text_yalign {
            properties.push(("text-yalign", text_yalign));
        }
        if let Some(ref value) = self.value {
            properties.push(("value", value));
        }
        if let Some(ref cell_background) = self.cell_background {
            properties.push(("cell-background", cell_background));
        }
        if let Some(ref cell_background_rgba) = self.cell_background_rgba {
            properties.push(("cell-background-rgba", cell_background_rgba));
        }
        if let Some(ref cell_background_set) = self.cell_background_set {
            properties.push(("cell-background-set", cell_background_set));
        }
        if let Some(ref height) = self.height {
            properties.push(("height", height));
        }
        if let Some(ref is_expanded) = self.is_expanded {
            properties.push(("is-expanded", is_expanded));
        }
        if let Some(ref is_expander) = self.is_expander {
            properties.push(("is-expander", is_expander));
        }
        if let Some(ref mode) = self.mode {
            properties.push(("mode", mode));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width) = self.width {
            properties.push(("width", width));
        }
        if let Some(ref xalign) = self.xalign {
            properties.push(("xalign", xalign));
        }
        if let Some(ref xpad) = self.xpad {
            properties.push(("xpad", xpad));
        }
        if let Some(ref yalign) = self.yalign {
            properties.push(("yalign", yalign));
        }
        if let Some(ref ypad) = self.ypad {
            properties.push(("ypad", ypad));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        let ret = glib::Object::new::<CellRendererProgress>(&properties).expect("object new");
        ret
    }

    pub fn inverted(mut self, inverted: bool) -> Self {
        self.inverted = Some(inverted);
        self
    }

    pub fn pulse(mut self, pulse: i32) -> Self {
        self.pulse = Some(pulse);
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn text_xalign(mut self, text_xalign: f32) -> Self {
        self.text_xalign = Some(text_xalign);
        self
    }

    pub fn text_yalign(mut self, text_yalign: f32) -> Self {
        self.text_yalign = Some(text_yalign);
        self
    }

    pub fn value(mut self, value: i32) -> Self {
        self.value = Some(value);
        self
    }

    pub fn cell_background(mut self, cell_background: &str) -> Self {
        self.cell_background = Some(cell_background.to_string());
        self
    }

    pub fn cell_background_rgba(mut self, cell_background_rgba: &gdk::RGBA) -> Self {
        self.cell_background_rgba = Some(cell_background_rgba.clone());
        self
    }

    pub fn cell_background_set(mut self, cell_background_set: bool) -> Self {
        self.cell_background_set = Some(cell_background_set);
        self
    }

    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn is_expanded(mut self, is_expanded: bool) -> Self {
        self.is_expanded = Some(is_expanded);
        self
    }

    pub fn is_expander(mut self, is_expander: bool) -> Self {
        self.is_expander = Some(is_expander);
        self
    }

    pub fn mode(mut self, mode: CellRendererMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn xalign(mut self, xalign: f32) -> Self {
        self.xalign = Some(xalign);
        self
    }

    pub fn xpad(mut self, xpad: u32) -> Self {
        self.xpad = Some(xpad);
        self
    }

    pub fn yalign(mut self, yalign: f32) -> Self {
        self.yalign = Some(yalign);
        self
    }

    pub fn ypad(mut self, ypad: u32) -> Self {
        self.ypad = Some(ypad);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

impl fmt::Display for CellRendererProgress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellRendererProgress")
    }
}
