// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Buildable;
use Widget;

glib_wrapper! {
    pub struct Calendar(Object<gtk_sys::GtkCalendar, CalendarClass>) @extends Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_calendar_get_type(),
    }
}

impl Calendar {
    pub fn new() -> Calendar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_calendar_new()).unsafe_cast() }
    }

    pub fn clear_marks(&self) {
        unsafe {
            gtk_sys::gtk_calendar_clear_marks(self.to_glib_none().0);
        }
    }

    //pub fn get_date(&self) -> /*Ignored*/Option<glib::DateTime> {
    //    unsafe { TODO: call gtk_sys:gtk_calendar_get_date() }
    //}

    pub fn get_day_is_marked(&self, day: u32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_calendar_get_day_is_marked(
                self.to_glib_none().0,
                day,
            ))
        }
    }

    pub fn get_show_day_names(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_calendar_get_show_day_names(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_show_heading(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_calendar_get_show_heading(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_show_week_numbers(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_calendar_get_show_week_numbers(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn mark_day(&self, day: u32) {
        unsafe {
            gtk_sys::gtk_calendar_mark_day(self.to_glib_none().0, day);
        }
    }

    //pub fn select_day(&self, date: /*Ignored*/&glib::DateTime) {
    //    unsafe { TODO: call gtk_sys:gtk_calendar_select_day() }
    //}

    pub fn set_show_day_names(&self, value: bool) {
        unsafe {
            gtk_sys::gtk_calendar_set_show_day_names(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_show_heading(&self, value: bool) {
        unsafe {
            gtk_sys::gtk_calendar_set_show_heading(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_show_week_numbers(&self, value: bool) {
        unsafe {
            gtk_sys::gtk_calendar_set_show_week_numbers(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn unmark_day(&self, day: u32) {
        unsafe {
            gtk_sys::gtk_calendar_unmark_day(self.to_glib_none().0, day);
        }
    }

    pub fn get_property_day(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"day\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `day` getter")
                .unwrap()
        }
    }

    pub fn set_property_day(&self, day: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"day\0".as_ptr() as *const _,
                Value::from(&day).to_glib_none().0,
            );
        }
    }

    pub fn get_property_month(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"month\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `month` getter")
                .unwrap()
        }
    }

    pub fn set_property_month(&self, month: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"month\0".as_ptr() as *const _,
                Value::from(&month).to_glib_none().0,
            );
        }
    }

    pub fn get_property_year(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"year\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `year` getter")
                .unwrap()
        }
    }

    pub fn set_property_year(&self, year: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"year\0".as_ptr() as *const _,
                Value::from(&year).to_glib_none().0,
            );
        }
    }

    pub fn connect_day_selected<F: Fn(&Calendar) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn day_selected_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut gtk_sys::GtkCalendar,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"day-selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    day_selected_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_next_month<F: Fn(&Calendar) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn next_month_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut gtk_sys::GtkCalendar,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"next-month\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    next_month_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_next_year<F: Fn(&Calendar) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn next_year_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut gtk_sys::GtkCalendar,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"next-year\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    next_year_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_prev_month<F: Fn(&Calendar) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn prev_month_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut gtk_sys::GtkCalendar,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"prev-month\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    prev_month_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_prev_year<F: Fn(&Calendar) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn prev_year_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut gtk_sys::GtkCalendar,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"prev-year\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    prev_year_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_day_notify<F: Fn(&Calendar) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_day_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut gtk_sys::GtkCalendar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::day\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_day_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_month_notify<F: Fn(&Calendar) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_month_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut gtk_sys::GtkCalendar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::month\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_month_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_show_day_names_notify<F: Fn(&Calendar) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_day_names_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut gtk_sys::GtkCalendar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-day-names\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_day_names_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_show_heading_notify<F: Fn(&Calendar) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_heading_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut gtk_sys::GtkCalendar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-heading\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_heading_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_show_week_numbers_notify<F: Fn(&Calendar) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_week_numbers_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut gtk_sys::GtkCalendar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-week-numbers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_week_numbers_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_year_notify<F: Fn(&Calendar) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_year_trampoline<F: Fn(&Calendar) + 'static>(
            this: *mut gtk_sys::GtkCalendar,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::year\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_year_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Calendar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Calendar")
    }
}
