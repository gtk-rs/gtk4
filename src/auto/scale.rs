// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use Adjustment;
use Align;
use Buildable;
use LayoutManager;
use Orientable;
use Orientation;
use Overflow;
use PositionType;
use Range;
use Widget;

glib_wrapper! {
    pub struct Scale(Object<gtk_sys::GtkScale, gtk_sys::GtkScaleClass, ScaleClass>) @extends Range, Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_scale_get_type(),
    }
}

impl Scale {
    pub fn new<P: IsA<Adjustment>>(orientation: Orientation, adjustment: Option<&P>) -> Scale {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_scale_new(
                orientation.to_glib(),
                adjustment.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    pub fn with_range(orientation: Orientation, min: f64, max: f64, step: f64) -> Scale {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_scale_new_with_range(
                orientation.to_glib(),
                min,
                max,
                step,
            ))
            .unsafe_cast()
        }
    }
}

#[derive(Clone, Default)]
pub struct ScaleBuilder {
    digits: Option<i32>,
    draw_value: Option<bool>,
    has_origin: Option<bool>,
    value_pos: Option<PositionType>,
    adjustment: Option<Adjustment>,
    fill_level: Option<f64>,
    inverted: Option<bool>,
    restrict_to_fill_level: Option<bool>,
    round_digits: Option<i32>,
    show_fill_level: Option<bool>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    //cursor: /*Unknown type*/,
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    orientation: Option<Orientation>,
}

impl ScaleBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Scale {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref digits) = self.digits {
            properties.push(("digits", digits));
        }
        if let Some(ref draw_value) = self.draw_value {
            properties.push(("draw-value", draw_value));
        }
        if let Some(ref has_origin) = self.has_origin {
            properties.push(("has-origin", has_origin));
        }
        if let Some(ref value_pos) = self.value_pos {
            properties.push(("value-pos", value_pos));
        }
        if let Some(ref adjustment) = self.adjustment {
            properties.push(("adjustment", adjustment));
        }
        if let Some(ref fill_level) = self.fill_level {
            properties.push(("fill-level", fill_level));
        }
        if let Some(ref inverted) = self.inverted {
            properties.push(("inverted", inverted));
        }
        if let Some(ref restrict_to_fill_level) = self.restrict_to_fill_level {
            properties.push(("restrict-to-fill-level", restrict_to_fill_level));
        }
        if let Some(ref round_digits) = self.round_digits {
            properties.push(("round-digits", round_digits));
        }
        if let Some(ref show_fill_level) = self.show_fill_level {
            properties.push(("show-fill-level", show_fill_level));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        let ret = glib::Object::new(Scale::static_type(), &properties)
            .expect("object new")
            .downcast::<Scale>()
            .expect("downcast");
        ret
    }

    pub fn digits(mut self, digits: i32) -> Self {
        self.digits = Some(digits);
        self
    }

    pub fn draw_value(mut self, draw_value: bool) -> Self {
        self.draw_value = Some(draw_value);
        self
    }

    pub fn has_origin(mut self, has_origin: bool) -> Self {
        self.has_origin = Some(has_origin);
        self
    }

    pub fn value_pos(mut self, value_pos: PositionType) -> Self {
        self.value_pos = Some(value_pos);
        self
    }

    pub fn adjustment<P: IsA<Adjustment>>(mut self, adjustment: &P) -> Self {
        self.adjustment = Some(adjustment.clone().upcast());
        self
    }

    pub fn fill_level(mut self, fill_level: f64) -> Self {
        self.fill_level = Some(fill_level);
        self
    }

    pub fn inverted(mut self, inverted: bool) -> Self {
        self.inverted = Some(inverted);
        self
    }

    pub fn restrict_to_fill_level(mut self, restrict_to_fill_level: bool) -> Self {
        self.restrict_to_fill_level = Some(restrict_to_fill_level);
        self
    }

    pub fn round_digits(mut self, round_digits: i32) -> Self {
        self.round_digits = Some(round_digits);
        self
    }

    pub fn show_fill_level(mut self, show_fill_level: bool) -> Self {
        self.show_fill_level = Some(show_fill_level);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

pub const NONE_SCALE: Option<&Scale> = None;

pub trait ScaleExt: 'static {
    fn add_mark(&self, value: f64, position: PositionType, markup: Option<&str>);

    fn clear_marks(&self);

    fn get_digits(&self) -> i32;

    fn get_draw_value(&self) -> bool;

    fn get_has_origin(&self) -> bool;

    //fn get_layout(&self) -> /*Ignored*/Option<pango::Layout>;

    fn get_layout_offsets(&self) -> (i32, i32);

    fn get_value_pos(&self) -> PositionType;

    fn set_digits(&self, digits: i32);

    fn set_draw_value(&self, draw_value: bool);

    fn set_format_value_func(&self, func: Option<Box_<dyn Fn(&Scale, f64) -> String + 'static>>);

    fn set_has_origin(&self, has_origin: bool);

    fn set_value_pos(&self, pos: PositionType);

    fn connect_property_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_draw_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_origin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_pos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Scale>> ScaleExt for O {
    fn add_mark(&self, value: f64, position: PositionType, markup: Option<&str>) {
        unsafe {
            gtk_sys::gtk_scale_add_mark(
                self.as_ref().to_glib_none().0,
                value,
                position.to_glib(),
                markup.to_glib_none().0,
            );
        }
    }

    fn clear_marks(&self) {
        unsafe {
            gtk_sys::gtk_scale_clear_marks(self.as_ref().to_glib_none().0);
        }
    }

    fn get_digits(&self) -> i32 {
        unsafe { gtk_sys::gtk_scale_get_digits(self.as_ref().to_glib_none().0) }
    }

    fn get_draw_value(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_scale_get_draw_value(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_has_origin(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_scale_get_has_origin(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn get_layout(&self) -> /*Ignored*/Option<pango::Layout> {
    //    unsafe { TODO: call gtk_sys:gtk_scale_get_layout() }
    //}

    fn get_layout_offsets(&self) -> (i32, i32) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            gtk_sys::gtk_scale_get_layout_offsets(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            );
            let x = x.assume_init();
            let y = y.assume_init();
            (x, y)
        }
    }

    fn get_value_pos(&self) -> PositionType {
        unsafe {
            from_glib(gtk_sys::gtk_scale_get_value_pos(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_digits(&self, digits: i32) {
        unsafe {
            gtk_sys::gtk_scale_set_digits(self.as_ref().to_glib_none().0, digits);
        }
    }

    fn set_draw_value(&self, draw_value: bool) {
        unsafe {
            gtk_sys::gtk_scale_set_draw_value(self.as_ref().to_glib_none().0, draw_value.to_glib());
        }
    }

    fn set_format_value_func(&self, func: Option<Box_<dyn Fn(&Scale, f64) -> String + 'static>>) {
        let func_data: Box_<Option<Box_<dyn Fn(&Scale, f64) -> String + 'static>>> =
            Box_::new(func);
        unsafe extern "C" fn func_func(
            scale: *mut gtk_sys::GtkScale,
            value: libc::c_double,
            user_data: glib_sys::gpointer,
        ) -> *mut libc::c_char {
            let scale = from_glib_borrow(scale);
            let callback: &Option<Box_<dyn Fn(&Scale, f64) -> String + 'static>> =
                &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&scale, value)
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib_full()
        }
        let func = if func_data.is_some() {
            Some(func_func as _)
        } else {
            None
        };
        unsafe extern "C" fn destroy_notify_func(data: glib_sys::gpointer) {
            let _callback: Box_<Option<Box_<dyn Fn(&Scale, f64) -> String + 'static>>> =
                Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_notify_func as _);
        let super_callback0: Box_<Option<Box_<dyn Fn(&Scale, f64) -> String + 'static>>> =
            func_data;
        unsafe {
            gtk_sys::gtk_scale_set_format_value_func(
                self.as_ref().to_glib_none().0,
                func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn set_has_origin(&self, has_origin: bool) {
        unsafe {
            gtk_sys::gtk_scale_set_has_origin(self.as_ref().to_glib_none().0, has_origin.to_glib());
        }
    }

    fn set_value_pos(&self, pos: PositionType) {
        unsafe {
            gtk_sys::gtk_scale_set_value_pos(self.as_ref().to_glib_none().0, pos.to_glib());
        }
    }

    fn connect_property_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_digits_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkScale,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Scale>,
        {
            let f: &F = &*(f as *const F);
            f(&Scale::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::digits\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_digits_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_draw_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_draw_value_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkScale,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Scale>,
        {
            let f: &F = &*(f as *const F);
            f(&Scale::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::draw-value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_draw_value_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_has_origin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_origin_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkScale,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Scale>,
        {
            let f: &F = &*(f as *const F);
            f(&Scale::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-origin\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_origin_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_value_pos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_pos_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkScale,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Scale>,
        {
            let f: &F = &*(f as *const F);
            f(&Scale::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value-pos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_pos_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scale")
    }
}
