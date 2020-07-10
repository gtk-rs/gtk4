// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
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
use Container;
use Orientable;
use Orientation;
use ScrollType;
use Widget;

glib_wrapper! {
    pub struct Paned(Object<gtk_sys::GtkPaned, PanedClass>) @extends Container, Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_paned_get_type(),
    }
}

impl Paned {
    pub fn new(orientation: Orientation) -> Paned {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_paned_new(orientation.to_glib())).unsafe_cast()
        }
    }

    pub fn add1<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            gtk_sys::gtk_paned_add1(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    pub fn add2<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            gtk_sys::gtk_paned_add2(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    pub fn get_child1(&self) -> Option<Widget> {
        unsafe { from_glib_none(gtk_sys::gtk_paned_get_child1(self.to_glib_none().0)) }
    }

    pub fn get_child2(&self) -> Option<Widget> {
        unsafe { from_glib_none(gtk_sys::gtk_paned_get_child2(self.to_glib_none().0)) }
    }

    pub fn get_position(&self) -> i32 {
        unsafe { gtk_sys::gtk_paned_get_position(self.to_glib_none().0) }
    }

    pub fn get_wide_handle(&self) -> bool {
        unsafe { from_glib(gtk_sys::gtk_paned_get_wide_handle(self.to_glib_none().0)) }
    }

    pub fn pack1<P: IsA<Widget>>(&self, child: &P, resize: bool, shrink: bool) {
        unsafe {
            gtk_sys::gtk_paned_pack1(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                resize.to_glib(),
                shrink.to_glib(),
            );
        }
    }

    pub fn pack2<P: IsA<Widget>>(&self, child: &P, resize: bool, shrink: bool) {
        unsafe {
            gtk_sys::gtk_paned_pack2(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                resize.to_glib(),
                shrink.to_glib(),
            );
        }
    }

    pub fn set_position(&self, position: i32) {
        unsafe {
            gtk_sys::gtk_paned_set_position(self.to_glib_none().0, position);
        }
    }

    pub fn set_wide_handle(&self, wide: bool) {
        unsafe {
            gtk_sys::gtk_paned_set_wide_handle(self.to_glib_none().0, wide.to_glib());
        }
    }

    pub fn get_property_max_position(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"max-position\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-position` getter")
                .unwrap()
        }
    }

    pub fn get_property_min_position(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"min-position\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `min-position` getter")
                .unwrap()
        }
    }

    pub fn get_property_position_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"position-set\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `position-set` getter")
                .unwrap()
        }
    }

    pub fn set_property_position_set(&self, position_set: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"position-set\0".as_ptr() as *const _,
                Value::from(&position_set).to_glib_none().0,
            );
        }
    }

    pub fn get_property_resize_child1(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"resize-child1\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `resize-child1` getter")
                .unwrap()
        }
    }

    pub fn set_property_resize_child1(&self, resize_child1: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"resize-child1\0".as_ptr() as *const _,
                Value::from(&resize_child1).to_glib_none().0,
            );
        }
    }

    pub fn get_property_resize_child2(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"resize-child2\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `resize-child2` getter")
                .unwrap()
        }
    }

    pub fn set_property_resize_child2(&self, resize_child2: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"resize-child2\0".as_ptr() as *const _,
                Value::from(&resize_child2).to_glib_none().0,
            );
        }
    }

    pub fn get_property_shrink_child1(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"shrink-child1\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `shrink-child1` getter")
                .unwrap()
        }
    }

    pub fn set_property_shrink_child1(&self, shrink_child1: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"shrink-child1\0".as_ptr() as *const _,
                Value::from(&shrink_child1).to_glib_none().0,
            );
        }
    }

    pub fn get_property_shrink_child2(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"shrink-child2\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `shrink-child2` getter")
                .unwrap()
        }
    }

    pub fn set_property_shrink_child2(&self, shrink_child2: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"shrink-child2\0".as_ptr() as *const _,
                Value::from(&shrink_child2).to_glib_none().0,
            );
        }
    }

    pub fn connect_accept_position<F: Fn(&Paned) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn accept_position_trampoline<F: Fn(&Paned) -> bool + 'static>(
            this: *mut gtk_sys::GtkPaned,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accept-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    accept_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_accept_position(&self) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("accept-position", &[])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_accept_position`")
            .unwrap()
    }

    pub fn connect_cancel_position<F: Fn(&Paned) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cancel_position_trampoline<F: Fn(&Paned) -> bool + 'static>(
            this: *mut gtk_sys::GtkPaned,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancel-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancel_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_cancel_position(&self) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("cancel-position", &[])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_cancel_position`")
            .unwrap()
    }

    pub fn connect_cycle_child_focus<F: Fn(&Paned, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cycle_child_focus_trampoline<F: Fn(&Paned, bool) -> bool + 'static>(
            this: *mut gtk_sys::GtkPaned,
            reversed: glib_sys::gboolean,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(reversed)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cycle-child-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cycle_child_focus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_cycle_child_focus(&self, reversed: bool) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("cycle-child-focus", &[&reversed])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_cycle_child_focus`")
            .unwrap()
    }

    pub fn connect_cycle_handle_focus<F: Fn(&Paned, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cycle_handle_focus_trampoline<
            F: Fn(&Paned, bool) -> bool + 'static,
        >(
            this: *mut gtk_sys::GtkPaned,
            reversed: glib_sys::gboolean,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(reversed)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cycle-handle-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cycle_handle_focus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_cycle_handle_focus(&self, reversed: bool) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("cycle-handle-focus", &[&reversed])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_cycle_handle_focus`")
            .unwrap()
    }

    pub fn connect_move_handle<F: Fn(&Paned, ScrollType) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_handle_trampoline<F: Fn(&Paned, ScrollType) -> bool + 'static>(
            this: *mut gtk_sys::GtkPaned,
            scroll_type: gtk_sys::GtkScrollType,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(scroll_type)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-handle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_handle_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_move_handle(&self, scroll_type: ScrollType) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("move-handle", &[&scroll_type])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_move_handle`")
            .unwrap()
    }

    pub fn connect_toggle_handle_focus<F: Fn(&Paned) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn toggle_handle_focus_trampoline<F: Fn(&Paned) -> bool + 'static>(
            this: *mut gtk_sys::GtkPaned,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toggle-handle-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    toggle_handle_focus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_toggle_handle_focus(&self) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("toggle-handle-focus", &[])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_toggle_handle_focus`")
            .unwrap()
    }

    pub fn connect_property_max_position_notify<F: Fn(&Paned) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_position_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut gtk_sys::GtkPaned,
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
                b"notify::max-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_min_position_notify<F: Fn(&Paned) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_position_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut gtk_sys::GtkPaned,
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
                b"notify::min-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_position_notify<F: Fn(&Paned) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut gtk_sys::GtkPaned,
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
                b"notify::position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_position_set_notify<F: Fn(&Paned) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_set_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut gtk_sys::GtkPaned,
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
                b"notify::position-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_position_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_resize_child1_notify<F: Fn(&Paned) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_resize_child1_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut gtk_sys::GtkPaned,
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
                b"notify::resize-child1\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resize_child1_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_resize_child2_notify<F: Fn(&Paned) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_resize_child2_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut gtk_sys::GtkPaned,
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
                b"notify::resize-child2\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resize_child2_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_shrink_child1_notify<F: Fn(&Paned) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_shrink_child1_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut gtk_sys::GtkPaned,
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
                b"notify::shrink-child1\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_shrink_child1_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_shrink_child2_notify<F: Fn(&Paned) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_shrink_child2_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut gtk_sys::GtkPaned,
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
                b"notify::shrink-child2\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_shrink_child2_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_wide_handle_notify<F: Fn(&Paned) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_wide_handle_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut gtk_sys::GtkPaned,
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
                b"notify::wide-handle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wide_handle_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Paned {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Paned")
    }
}
