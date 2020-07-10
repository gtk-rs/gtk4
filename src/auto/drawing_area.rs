// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Buildable;
use Widget;

glib_wrapper! {
    pub struct DrawingArea(Object<gtk_sys::GtkDrawingArea, gtk_sys::GtkDrawingAreaClass, DrawingAreaClass>) @extends Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_drawing_area_get_type(),
    }
}

impl DrawingArea {
    pub fn new() -> DrawingArea {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_drawing_area_new()).unsafe_cast() }
    }
}

impl Default for DrawingArea {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_DRAWING_AREA: Option<&DrawingArea> = None;

pub trait DrawingAreaExt: 'static {
    fn get_content_height(&self) -> i32;

    fn get_content_width(&self) -> i32;

    fn set_content_height(&self, height: i32);

    fn set_content_width(&self, width: i32);

    //fn set_draw_func(&self, draw_func: /*Unimplemented*/Fn(&DrawingArea, /*Ignored*/cairo::Context, i32, i32), user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    fn connect_property_content_height_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_content_width_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<DrawingArea>> DrawingAreaExt for O {
    fn get_content_height(&self) -> i32 {
        unsafe { gtk_sys::gtk_drawing_area_get_content_height(self.as_ref().to_glib_none().0) }
    }

    fn get_content_width(&self) -> i32 {
        unsafe { gtk_sys::gtk_drawing_area_get_content_width(self.as_ref().to_glib_none().0) }
    }

    fn set_content_height(&self, height: i32) {
        unsafe {
            gtk_sys::gtk_drawing_area_set_content_height(self.as_ref().to_glib_none().0, height);
        }
    }

    fn set_content_width(&self, width: i32) {
        unsafe {
            gtk_sys::gtk_drawing_area_set_content_width(self.as_ref().to_glib_none().0, width);
        }
    }

    //fn set_draw_func(&self, draw_func: /*Unimplemented*/Fn(&DrawingArea, /*Ignored*/cairo::Context, i32, i32), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call gtk_sys:gtk_drawing_area_set_draw_func() }
    //}

    fn connect_property_content_height_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_height_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkDrawingArea,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DrawingArea>,
        {
            let f: &F = &*(f as *const F);
            f(&DrawingArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::content-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_content_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_content_width_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_width_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkDrawingArea,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DrawingArea>,
        {
            let f: &F = &*(f as *const F);
            f(&DrawingArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::content-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_content_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DrawingArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DrawingArea")
    }
}
