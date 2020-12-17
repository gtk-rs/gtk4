// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Accessible;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    pub struct Editable(Interface<ffi::GtkEditable>) @requires Widget, Accessible, Buildable, ConstraintTarget;

    match fn {
        get_type => || ffi::gtk_editable_get_type(),
    }
}

impl Editable {
    //#[doc(alias = "gtk_editable_install_properties")]
    //pub fn install_properties(object_class: /*Ignored*/&mut glib::ObjectClass, first_prop: u32) -> u32 {
    //    unsafe { TODO: call ffi:gtk_editable_install_properties() }
    //}
}

pub const NONE_EDITABLE: Option<&Editable> = None;

pub trait EditableExt: 'static {
    #[doc(alias = "gtk_editable_delete_selection")]
    fn delete_selection(&self);

    #[doc(alias = "gtk_editable_delete_text")]
    fn delete_text(&self, start_pos: i32, end_pos: i32);

    #[doc(alias = "gtk_editable_finish_delegate")]
    fn finish_delegate(&self);

    #[doc(alias = "gtk_editable_get_alignment")]
    fn get_alignment(&self) -> f32;

    #[doc(alias = "gtk_editable_get_chars")]
    fn get_chars(&self, start_pos: i32, end_pos: i32) -> Option<glib::GString>;

    #[doc(alias = "gtk_editable_get_delegate")]
    fn get_delegate(&self) -> Option<Editable>;

    #[doc(alias = "gtk_editable_get_editable")]
    fn get_editable(&self) -> bool;

    #[doc(alias = "gtk_editable_get_enable_undo")]
    fn get_enable_undo(&self) -> bool;

    #[doc(alias = "gtk_editable_get_max_width_chars")]
    fn get_max_width_chars(&self) -> i32;

    #[doc(alias = "gtk_editable_get_position")]
    fn get_position(&self) -> i32;

    #[doc(alias = "gtk_editable_get_selection_bounds")]
    fn get_selection_bounds(&self) -> Option<(i32, i32)>;

    #[doc(alias = "gtk_editable_get_text")]
    fn get_text(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_editable_get_width_chars")]
    fn get_width_chars(&self) -> i32;

    #[doc(alias = "gtk_editable_init_delegate")]
    fn init_delegate(&self);

    #[doc(alias = "gtk_editable_insert_text")]
    fn insert_text(&self, text: &str, position: &mut i32);

    #[doc(alias = "gtk_editable_select_region")]
    fn select_region(&self, start_pos: i32, end_pos: i32);

    #[doc(alias = "gtk_editable_set_alignment")]
    fn set_alignment(&self, xalign: f32);

    #[doc(alias = "gtk_editable_set_editable")]
    fn set_editable(&self, is_editable: bool);

    #[doc(alias = "gtk_editable_set_enable_undo")]
    fn set_enable_undo(&self, enable_undo: bool);

    #[doc(alias = "gtk_editable_set_max_width_chars")]
    fn set_max_width_chars(&self, n_chars: i32);

    #[doc(alias = "gtk_editable_set_position")]
    fn set_position(&self, position: i32);

    #[doc(alias = "gtk_editable_set_text")]
    fn set_text(&self, text: &str);

    #[doc(alias = "gtk_editable_set_width_chars")]
    fn set_width_chars(&self, n_chars: i32);

    fn get_property_cursor_position(&self) -> i32;

    fn get_property_selection_bound(&self) -> i32;

    fn get_property_xalign(&self) -> f32;

    fn set_property_xalign(&self, xalign: f32);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_delete_text<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cursor_position_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_enable_undo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_width_chars_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_selection_bound_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_chars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Editable>> EditableExt for O {
    fn delete_selection(&self) {
        unsafe {
            ffi::gtk_editable_delete_selection(self.as_ref().to_glib_none().0);
        }
    }

    fn delete_text(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::gtk_editable_delete_text(self.as_ref().to_glib_none().0, start_pos, end_pos);
        }
    }

    fn finish_delegate(&self) {
        unsafe {
            ffi::gtk_editable_finish_delegate(self.as_ref().to_glib_none().0);
        }
    }

    fn get_alignment(&self) -> f32 {
        unsafe { ffi::gtk_editable_get_alignment(self.as_ref().to_glib_none().0) }
    }

    fn get_chars(&self, start_pos: i32, end_pos: i32) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_editable_get_chars(
                self.as_ref().to_glib_none().0,
                start_pos,
                end_pos,
            ))
        }
    }

    fn get_delegate(&self) -> Option<Editable> {
        unsafe {
            from_glib_none(ffi::gtk_editable_get_delegate(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_editable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_editable_get_editable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_enable_undo(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_editable_get_enable_undo(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_max_width_chars(&self) -> i32 {
        unsafe { ffi::gtk_editable_get_max_width_chars(self.as_ref().to_glib_none().0) }
    }

    fn get_position(&self) -> i32 {
        unsafe { ffi::gtk_editable_get_position(self.as_ref().to_glib_none().0) }
    }

    fn get_selection_bounds(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut start_pos = mem::MaybeUninit::uninit();
            let mut end_pos = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_editable_get_selection_bounds(
                self.as_ref().to_glib_none().0,
                start_pos.as_mut_ptr(),
                end_pos.as_mut_ptr(),
            ));
            let start_pos = start_pos.assume_init();
            let end_pos = end_pos.assume_init();
            if ret {
                Some((start_pos, end_pos))
            } else {
                None
            }
        }
    }

    fn get_text(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_editable_get_text(self.as_ref().to_glib_none().0)) }
    }

    fn get_width_chars(&self) -> i32 {
        unsafe { ffi::gtk_editable_get_width_chars(self.as_ref().to_glib_none().0) }
    }

    fn init_delegate(&self) {
        unsafe {
            ffi::gtk_editable_init_delegate(self.as_ref().to_glib_none().0);
        }
    }

    fn insert_text(&self, text: &str, position: &mut i32) {
        let length = text.len() as i32;
        unsafe {
            ffi::gtk_editable_insert_text(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
                length,
                position,
            );
        }
    }

    fn select_region(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::gtk_editable_select_region(self.as_ref().to_glib_none().0, start_pos, end_pos);
        }
    }

    fn set_alignment(&self, xalign: f32) {
        unsafe {
            ffi::gtk_editable_set_alignment(self.as_ref().to_glib_none().0, xalign);
        }
    }

    fn set_editable(&self, is_editable: bool) {
        unsafe {
            ffi::gtk_editable_set_editable(self.as_ref().to_glib_none().0, is_editable.to_glib());
        }
    }

    fn set_enable_undo(&self, enable_undo: bool) {
        unsafe {
            ffi::gtk_editable_set_enable_undo(
                self.as_ref().to_glib_none().0,
                enable_undo.to_glib(),
            );
        }
    }

    fn set_max_width_chars(&self, n_chars: i32) {
        unsafe {
            ffi::gtk_editable_set_max_width_chars(self.as_ref().to_glib_none().0, n_chars);
        }
    }

    fn set_position(&self, position: i32) {
        unsafe {
            ffi::gtk_editable_set_position(self.as_ref().to_glib_none().0, position);
        }
    }

    fn set_text(&self, text: &str) {
        unsafe {
            ffi::gtk_editable_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn set_width_chars(&self, n_chars: i32) {
        unsafe {
            ffi::gtk_editable_set_width_chars(self.as_ref().to_glib_none().0, n_chars);
        }
    }

    fn get_property_cursor_position(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"cursor-position\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `cursor-position` getter")
                .unwrap()
        }
    }

    fn get_property_selection_bound(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"selection-bound\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `selection-bound` getter")
                .unwrap()
        }
    }

    fn get_property_xalign(&self) -> f32 {
        unsafe {
            let mut value = glib::Value::from_type(<f32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"xalign\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `xalign` getter")
                .unwrap()
        }
    }

    fn set_property_xalign(&self, xalign: f32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"xalign\0".as_ptr() as *const _,
                glib::Value::from(&xalign).to_glib_none().0,
            );
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEditable,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Editable>,
        {
            let f: &F = &*(f as *const F);
            f(&Editable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_delete_text<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn delete_text_trampoline<P, F: Fn(&P, i32, i32) + 'static>(
            this: *mut ffi::GtkEditable,
            start_pos: libc::c_int,
            end_pos: libc::c_int,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Editable>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Editable::from_glib_borrow(this).unsafe_cast_ref(),
                start_pos,
                end_pos,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"delete-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    delete_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_cursor_position_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_cursor_position_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEditable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Editable>,
        {
            let f: &F = &*(f as *const F);
            f(&Editable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::cursor-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_cursor_position_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_editable_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEditable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Editable>,
        {
            let f: &F = &*(f as *const F);
            f(&Editable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::editable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_editable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_enable_undo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_undo_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEditable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Editable>,
        {
            let f: &F = &*(f as *const F);
            f(&Editable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enable-undo\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enable_undo_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_max_width_chars_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_width_chars_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEditable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Editable>,
        {
            let f: &F = &*(f as *const F);
            f(&Editable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-width-chars\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_width_chars_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_selection_bound_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_selection_bound_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEditable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Editable>,
        {
            let f: &F = &*(f as *const F);
            f(&Editable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selection-bound\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selection_bound_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEditable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Editable>,
        {
            let f: &F = &*(f as *const F);
            f(&Editable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_width_chars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_chars_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEditable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Editable>,
        {
            let f: &F = &*(f as *const F);
            f(&Editable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::width-chars\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_width_chars_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_xalign_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEditable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Editable>,
        {
            let f: &F = &*(f as *const F);
            f(&Editable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::xalign\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_xalign_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Editable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Editable")
    }
}
