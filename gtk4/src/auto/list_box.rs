// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Adjustment;
use crate::Align;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::LayoutManager;
use crate::ListBoxRow;
use crate::MovementStep;
use crate::Overflow;
use crate::SelectionMode;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
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
    pub struct ListBox(Object<ffi::GtkListBox>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_list_box_get_type(),
    }
}

impl ListBox {
    #[doc(alias = "gtk_list_box_new")]
    pub fn new() -> ListBox {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_list_box_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`ListBox`]
    /// This method returns an instance of [`ListBoxBuilder`] which can be used to create a [`ListBox`].
    pub fn builder() -> ListBoxBuilder {
        ListBoxBuilder::default()
    }

    #[doc(alias = "gtk_list_box_append")]
    pub fn append<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_list_box_append(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_list_box_bind_model")]
    pub fn bind_model<P: IsA<gio::ListModel>, Q: Fn(&glib::Object) -> Widget + 'static>(
        &self,
        model: Option<&P>,
        create_widget_func: Q,
    ) {
        let create_widget_func_data: Box_<Q> = Box_::new(create_widget_func);
        unsafe extern "C" fn create_widget_func_func<
            P: IsA<gio::ListModel>,
            Q: Fn(&glib::Object) -> Widget + 'static,
        >(
            item: *mut glib::gobject_ffi::GObject,
            user_data: glib::ffi::gpointer,
        ) -> *mut ffi::GtkWidget {
            let item = from_glib_borrow(item);
            let callback: &Q = &*(user_data as *mut _);
            let res = (*callback)(&item);
            res.to_glib_full()
        }
        let create_widget_func = Some(create_widget_func_func::<P, Q> as _);
        unsafe extern "C" fn user_data_free_func_func<
            P: IsA<gio::ListModel>,
            Q: Fn(&glib::Object) -> Widget + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<Q> = Box_::from_raw(data as *mut _);
        }
        let destroy_call4 = Some(user_data_free_func_func::<P, Q> as _);
        let super_callback0: Box_<Q> = create_widget_func_data;
        unsafe {
            ffi::gtk_list_box_bind_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
                create_widget_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call4,
            );
        }
    }

    #[doc(alias = "gtk_list_box_drag_highlight_row")]
    pub fn drag_highlight_row<P: IsA<ListBoxRow>>(&self, row: &P) {
        unsafe {
            ffi::gtk_list_box_drag_highlight_row(
                self.to_glib_none().0,
                row.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_list_box_drag_unhighlight_row")]
    pub fn drag_unhighlight_row(&self) {
        unsafe {
            ffi::gtk_list_box_drag_unhighlight_row(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_list_box_get_activate_on_single_click")]
    #[doc(alias = "get_activate_on_single_click")]
    pub fn activates_on_single_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_get_activate_on_single_click(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_list_box_get_adjustment")]
    #[doc(alias = "get_adjustment")]
    pub fn adjustment(&self) -> Option<Adjustment> {
        unsafe { from_glib_none(ffi::gtk_list_box_get_adjustment(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_list_box_get_row_at_index")]
    #[doc(alias = "get_row_at_index")]
    pub fn row_at_index(&self, index_: i32) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_row_at_index(
                self.to_glib_none().0,
                index_,
            ))
        }
    }

    #[doc(alias = "gtk_list_box_get_row_at_y")]
    #[doc(alias = "get_row_at_y")]
    pub fn row_at_y(&self, y: i32) -> Option<ListBoxRow> {
        unsafe { from_glib_none(ffi::gtk_list_box_get_row_at_y(self.to_glib_none().0, y)) }
    }

    #[doc(alias = "gtk_list_box_get_selected_row")]
    #[doc(alias = "get_selected_row")]
    pub fn selected_row(&self) -> Option<ListBoxRow> {
        unsafe { from_glib_none(ffi::gtk_list_box_get_selected_row(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_list_box_get_selected_rows")]
    #[doc(alias = "get_selected_rows")]
    pub fn selected_rows(&self) -> Vec<ListBoxRow> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_list_box_get_selected_rows(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_list_box_get_selection_mode")]
    #[doc(alias = "get_selection_mode")]
    pub fn selection_mode(&self) -> SelectionMode {
        unsafe { from_glib(ffi::gtk_list_box_get_selection_mode(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_list_box_get_show_separators")]
    #[doc(alias = "get_show_separators")]
    pub fn shows_separators(&self) -> bool {
        unsafe { from_glib(ffi::gtk_list_box_get_show_separators(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_list_box_insert")]
    pub fn insert<P: IsA<Widget>>(&self, child: &P, position: i32) {
        unsafe {
            ffi::gtk_list_box_insert(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    #[doc(alias = "gtk_list_box_invalidate_filter")]
    pub fn invalidate_filter(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_filter(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_list_box_invalidate_headers")]
    pub fn invalidate_headers(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_headers(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_list_box_invalidate_sort")]
    pub fn invalidate_sort(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_sort(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_list_box_prepend")]
    pub fn prepend<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_list_box_prepend(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_list_box_remove")]
    pub fn remove<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_list_box_remove(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_list_box_select_all")]
    pub fn select_all(&self) {
        unsafe {
            ffi::gtk_list_box_select_all(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_list_box_select_row")]
    pub fn select_row<P: IsA<ListBoxRow>>(&self, row: Option<&P>) {
        unsafe {
            ffi::gtk_list_box_select_row(
                self.to_glib_none().0,
                row.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_list_box_selected_foreach")]
    pub fn selected_foreach<P: FnMut(&ListBox, &ListBoxRow)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&ListBox, &ListBoxRow)>(
            box_: *mut ffi::GtkListBox,
            row: *mut ffi::GtkListBoxRow,
            user_data: glib::ffi::gpointer,
        ) {
            let box_ = from_glib_borrow(box_);
            let row = from_glib_borrow(row);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&box_, &row);
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            ffi::gtk_list_box_selected_foreach(
                self.to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    #[doc(alias = "gtk_list_box_set_activate_on_single_click")]
    pub fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_list_box_set_activate_on_single_click(
                self.to_glib_none().0,
                single.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_list_box_set_adjustment")]
    pub fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: Option<&P>) {
        unsafe {
            ffi::gtk_list_box_set_adjustment(
                self.to_glib_none().0,
                adjustment.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_list_box_set_filter_func")]
    pub fn set_filter_func<P: Fn(&ListBoxRow) -> bool + 'static>(&self, filter_func: P) {
        let filter_func_data: Box_<P> = Box_::new(filter_func);
        unsafe extern "C" fn filter_func_func<P: Fn(&ListBoxRow) -> bool + 'static>(
            row: *mut ffi::GtkListBoxRow,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let row = from_glib_borrow(row);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&row);
            res.into_glib()
        }
        let filter_func = Some(filter_func_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn(&ListBoxRow) -> bool + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = filter_func_data;
        unsafe {
            ffi::gtk_list_box_set_filter_func(
                self.to_glib_none().0,
                filter_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "gtk_list_box_set_header_func")]
    pub fn set_header_func<P: Fn(&ListBoxRow, Option<&ListBoxRow>) + 'static>(
        &self,
        update_header: P,
    ) {
        let update_header_data: Box_<P> = Box_::new(update_header);
        unsafe extern "C" fn update_header_func<
            P: Fn(&ListBoxRow, Option<&ListBoxRow>) + 'static,
        >(
            row: *mut ffi::GtkListBoxRow,
            before: *mut ffi::GtkListBoxRow,
            user_data: glib::ffi::gpointer,
        ) {
            let row = from_glib_borrow(row);
            let before: Borrowed<Option<ListBoxRow>> = from_glib_borrow(before);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(&row, before.as_ref().as_ref());
        }
        let update_header = Some(update_header_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn(&ListBoxRow, Option<&ListBoxRow>) + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = update_header_data;
        unsafe {
            ffi::gtk_list_box_set_header_func(
                self.to_glib_none().0,
                update_header,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "gtk_list_box_set_placeholder")]
    pub fn set_placeholder<P: IsA<Widget>>(&self, placeholder: Option<&P>) {
        unsafe {
            ffi::gtk_list_box_set_placeholder(
                self.to_glib_none().0,
                placeholder.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_list_box_set_selection_mode")]
    pub fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe {
            ffi::gtk_list_box_set_selection_mode(self.to_glib_none().0, mode.into_glib());
        }
    }

    #[doc(alias = "gtk_list_box_set_show_separators")]
    pub fn set_show_separators(&self, show_separators: bool) {
        unsafe {
            ffi::gtk_list_box_set_show_separators(
                self.to_glib_none().0,
                show_separators.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_list_box_set_sort_func")]
    pub fn set_sort_func<P: Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>(&self, sort_func: P) {
        let sort_func_data: Box_<P> = Box_::new(sort_func);
        unsafe extern "C" fn sort_func_func<P: Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>(
            row1: *mut ffi::GtkListBoxRow,
            row2: *mut ffi::GtkListBoxRow,
            user_data: glib::ffi::gpointer,
        ) -> libc::c_int {
            let row1 = from_glib_borrow(row1);
            let row2 = from_glib_borrow(row2);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&row1, &row2);
            res
        }
        let sort_func = Some(sort_func_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = sort_func_data;
        unsafe {
            ffi::gtk_list_box_set_sort_func(
                self.to_glib_none().0,
                sort_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "gtk_list_box_unselect_all")]
    pub fn unselect_all(&self) {
        unsafe {
            ffi::gtk_list_box_unselect_all(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_list_box_unselect_row")]
    pub fn unselect_row<P: IsA<ListBoxRow>>(&self, row: &P) {
        unsafe {
            ffi::gtk_list_box_unselect_row(self.to_glib_none().0, row.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "accept-unpaired-release")]
    pub fn accepts_unpaired_release(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"accept-unpaired-release\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `accept-unpaired-release` getter")
        }
    }

    #[doc(alias = "accept-unpaired-release")]
    pub fn set_accept_unpaired_release(&self, accept_unpaired_release: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"accept-unpaired-release\0".as_ptr() as *const _,
                accept_unpaired_release.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "activate-cursor-row")]
    pub fn connect_activate_cursor_row<F: Fn(&ListBox) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_cursor_row_trampoline<F: Fn(&ListBox) + 'static>(
            this: *mut ffi::GtkListBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate-cursor-row\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_cursor_row_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_activate_cursor_row(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("activate-cursor-row", &[])
                .unwrap()
        };
    }

    #[doc(alias = "move-cursor")]
    pub fn connect_move_cursor<F: Fn(&ListBox, MovementStep, i32, bool, bool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_cursor_trampoline<
            F: Fn(&ListBox, MovementStep, i32, bool, bool) + 'static,
        >(
            this: *mut ffi::GtkListBox,
            object: ffi::GtkMovementStep,
            p0: libc::c_int,
            p1: glib::ffi::gboolean,
            p2: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                from_glib(object),
                p0,
                from_glib(p1),
                from_glib(p2),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-cursor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_cursor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_move_cursor(&self, object: MovementStep, p0: i32, p1: bool, p2: bool) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("move-cursor", &[&object, &p0, &p1, &p2])
                .unwrap()
        };
    }

    #[doc(alias = "row-activated")]
    pub fn connect_row_activated<F: Fn(&ListBox, &ListBoxRow) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn row_activated_trampoline<F: Fn(&ListBox, &ListBoxRow) + 'static>(
            this: *mut ffi::GtkListBox,
            row: *mut ffi::GtkListBoxRow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(row))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"row-activated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    row_activated_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "row-selected")]
    pub fn connect_row_selected<F: Fn(&ListBox, Option<&ListBoxRow>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn row_selected_trampoline<
            F: Fn(&ListBox, Option<&ListBoxRow>) + 'static,
        >(
            this: *mut ffi::GtkListBox,
            row: *mut ffi::GtkListBoxRow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<ListBoxRow>::from_glib_borrow(row)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"row-selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    row_selected_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "select-all")]
    pub fn connect_select_all<F: Fn(&ListBox) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn select_all_trampoline<F: Fn(&ListBox) + 'static>(
            this: *mut ffi::GtkListBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"select-all\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    select_all_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_select_all(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("select-all", &[])
                .unwrap()
        };
    }

    #[doc(alias = "selected-rows-changed")]
    pub fn connect_selected_rows_changed<F: Fn(&ListBox) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn selected_rows_changed_trampoline<F: Fn(&ListBox) + 'static>(
            this: *mut ffi::GtkListBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"selected-rows-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    selected_rows_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "toggle-cursor-row")]
    pub fn connect_toggle_cursor_row<F: Fn(&ListBox) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn toggle_cursor_row_trampoline<F: Fn(&ListBox) + 'static>(
            this: *mut ffi::GtkListBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toggle-cursor-row\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    toggle_cursor_row_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_toggle_cursor_row(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("toggle-cursor-row", &[])
                .unwrap()
        };
    }

    #[doc(alias = "unselect-all")]
    pub fn connect_unselect_all<F: Fn(&ListBox) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn unselect_all_trampoline<F: Fn(&ListBox) + 'static>(
            this: *mut ffi::GtkListBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"unselect-all\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    unselect_all_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_unselect_all(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("unselect-all", &[])
                .unwrap()
        };
    }

    #[doc(alias = "accept-unpaired-release")]
    pub fn connect_accept_unpaired_release_notify<F: Fn(&ListBox) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accept_unpaired_release_trampoline<
            F: Fn(&ListBox) + 'static,
        >(
            this: *mut ffi::GtkListBox,
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
                b"notify::accept-unpaired-release\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accept_unpaired_release_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "activate-on-single-click")]
    pub fn connect_activate_on_single_click_notify<F: Fn(&ListBox) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_activate_on_single_click_trampoline<
            F: Fn(&ListBox) + 'static,
        >(
            this: *mut ffi::GtkListBox,
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
                b"notify::activate-on-single-click\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_activate_on_single_click_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selection-mode")]
    pub fn connect_selection_mode_notify<F: Fn(&ListBox) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_selection_mode_trampoline<F: Fn(&ListBox) + 'static>(
            this: *mut ffi::GtkListBox,
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
                b"notify::selection-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selection_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-separators")]
    pub fn connect_show_separators_notify<F: Fn(&ListBox) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_separators_trampoline<F: Fn(&ListBox) + 'static>(
            this: *mut ffi::GtkListBox,
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
                b"notify::show-separators\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_separators_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for ListBox {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`ListBox`].
pub struct ListBoxBuilder {
    accept_unpaired_release: Option<bool>,
    activate_on_single_click: Option<bool>,
    selection_mode: Option<SelectionMode>,
    show_separators: Option<bool>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
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
    accessible_role: Option<AccessibleRole>,
}

impl ListBoxBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ListBoxBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ListBox`].
    pub fn build(self) -> ListBox {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref accept_unpaired_release) = self.accept_unpaired_release {
            properties.push(("accept-unpaired-release", accept_unpaired_release));
        }
        if let Some(ref activate_on_single_click) = self.activate_on_single_click {
            properties.push(("activate-on-single-click", activate_on_single_click));
        }
        if let Some(ref selection_mode) = self.selection_mode {
            properties.push(("selection-mode", selection_mode));
        }
        if let Some(ref show_separators) = self.show_separators {
            properties.push(("show-separators", show_separators));
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
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
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
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        glib::Object::new::<ListBox>(&properties).expect("Failed to create an instance of ListBox")
    }

    pub fn accept_unpaired_release(mut self, accept_unpaired_release: bool) -> Self {
        self.accept_unpaired_release = Some(accept_unpaired_release);
        self
    }

    pub fn activate_on_single_click(mut self, activate_on_single_click: bool) -> Self {
        self.activate_on_single_click = Some(activate_on_single_click);
        self
    }

    pub fn selection_mode(mut self, selection_mode: SelectionMode) -> Self {
        self.selection_mode = Some(selection_mode);
        self
    }

    pub fn show_separators(mut self, show_separators: bool) -> Self {
        self.show_separators = Some(show_separators);
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

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
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

    pub fn accessible_role(mut self, accessible_role: AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }
}

impl fmt::Display for ListBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ListBox")
    }
}
