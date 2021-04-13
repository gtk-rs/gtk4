// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Buildable;
use crate::CellArea;
use crate::CellLayout;
use crate::TreeIter;
use crate::TreeModel;
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
    pub struct EntryCompletion(Object<ffi::GtkEntryCompletion>) @implements Buildable, CellLayout;

    match fn {
        get_type => || ffi::gtk_entry_completion_get_type(),
    }
}

impl EntryCompletion {
    #[doc(alias = "gtk_entry_completion_new")]
    pub fn new() -> EntryCompletion {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_entry_completion_new()) }
    }

    #[doc(alias = "gtk_entry_completion_new_with_area")]
    pub fn with_area<P: IsA<CellArea>>(area: &P) -> EntryCompletion {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_entry_completion_new_with_area(
                area.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_entry_completion_complete")]
    pub fn complete(&self) {
        unsafe {
            ffi::gtk_entry_completion_complete(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_entry_completion_compute_prefix")]
    pub fn compute_prefix(&self, key: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_entry_completion_compute_prefix(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_entry_completion_get_completion_prefix")]
    pub fn completion_prefix(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_entry_completion_get_completion_prefix(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_entry_completion_get_inline_completion")]
    pub fn is_inline_completion(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_inline_completion(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_entry_completion_get_inline_selection")]
    pub fn is_inline_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_inline_selection(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_entry_completion_get_minimum_key_length")]
    pub fn minimum_key_length(&self) -> i32 {
        unsafe { ffi::gtk_entry_completion_get_minimum_key_length(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_entry_completion_get_model")]
    pub fn model(&self) -> Option<TreeModel> {
        unsafe { from_glib_none(ffi::gtk_entry_completion_get_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_entry_completion_get_popup_completion")]
    pub fn is_popup_completion(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_popup_completion(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_entry_completion_get_popup_set_width")]
    pub fn is_popup_set_width(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_popup_set_width(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_entry_completion_get_popup_single_match")]
    pub fn is_popup_single_match(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_popup_single_match(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_entry_completion_get_text_column")]
    pub fn text_column(&self) -> i32 {
        unsafe { ffi::gtk_entry_completion_get_text_column(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_entry_completion_insert_prefix")]
    pub fn insert_prefix(&self) {
        unsafe {
            ffi::gtk_entry_completion_insert_prefix(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_entry_completion_set_inline_completion")]
    pub fn set_inline_completion(&self, inline_completion: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_inline_completion(
                self.to_glib_none().0,
                inline_completion.to_glib(),
            );
        }
    }

    #[doc(alias = "gtk_entry_completion_set_inline_selection")]
    pub fn set_inline_selection(&self, inline_selection: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_inline_selection(
                self.to_glib_none().0,
                inline_selection.to_glib(),
            );
        }
    }

    #[doc(alias = "gtk_entry_completion_set_match_func")]
    pub fn set_match_func<P: Fn(&EntryCompletion, &str, &TreeIter) -> bool + 'static>(
        &self,
        func: P,
    ) {
        let func_data: Box_<P> = Box_::new(func);
        unsafe extern "C" fn func_func<
            P: Fn(&EntryCompletion, &str, &TreeIter) -> bool + 'static,
        >(
            completion: *mut ffi::GtkEntryCompletion,
            key: *const libc::c_char,
            iter: *mut ffi::GtkTreeIter,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let completion = from_glib_borrow(completion);
            let key: Borrowed<glib::GString> = from_glib_borrow(key);
            let iter = from_glib_borrow(iter);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&completion, key.as_str(), &iter);
            res.to_glib()
        }
        let func = Some(func_func::<P> as _);
        unsafe extern "C" fn func_notify_func<
            P: Fn(&EntryCompletion, &str, &TreeIter) -> bool + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(func_notify_func::<P> as _);
        let super_callback0: Box_<P> = func_data;
        unsafe {
            ffi::gtk_entry_completion_set_match_func(
                self.to_glib_none().0,
                func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "gtk_entry_completion_set_minimum_key_length")]
    pub fn set_minimum_key_length(&self, length: i32) {
        unsafe {
            ffi::gtk_entry_completion_set_minimum_key_length(self.to_glib_none().0, length);
        }
    }

    #[doc(alias = "gtk_entry_completion_set_model")]
    pub fn set_model<P: IsA<TreeModel>>(&self, model: Option<&P>) {
        unsafe {
            ffi::gtk_entry_completion_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_entry_completion_set_popup_completion")]
    pub fn set_popup_completion(&self, popup_completion: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_popup_completion(
                self.to_glib_none().0,
                popup_completion.to_glib(),
            );
        }
    }

    #[doc(alias = "gtk_entry_completion_set_popup_set_width")]
    pub fn set_popup_set_width(&self, popup_set_width: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_popup_set_width(
                self.to_glib_none().0,
                popup_set_width.to_glib(),
            );
        }
    }

    #[doc(alias = "gtk_entry_completion_set_popup_single_match")]
    pub fn set_popup_single_match(&self, popup_single_match: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_popup_single_match(
                self.to_glib_none().0,
                popup_single_match.to_glib(),
            );
        }
    }

    #[doc(alias = "gtk_entry_completion_set_text_column")]
    pub fn set_text_column(&self, column: i32) {
        unsafe {
            ffi::gtk_entry_completion_set_text_column(self.to_glib_none().0, column);
        }
    }

    #[doc(alias = "get_property_cell_area")]
    pub fn cell_area(&self) -> Option<CellArea> {
        unsafe {
            let mut value = glib::Value::from_type(<CellArea as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"cell-area\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `cell-area` getter")
        }
    }

    pub fn connect_cursor_on_match<
        F: Fn(&EntryCompletion, &TreeModel, &TreeIter) -> glib::signal::Inhibit + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cursor_on_match_trampoline<
            F: Fn(&EntryCompletion, &TreeModel, &TreeIter) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
            model: *mut ffi::GtkTreeModel,
            iter: *mut ffi::GtkTreeIter,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(model),
                &from_glib_borrow(iter),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cursor-on-match\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cursor_on_match_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_insert_prefix<
        F: Fn(&EntryCompletion, &str) -> glib::signal::Inhibit + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn insert_prefix_trampoline<
            F: Fn(&EntryCompletion, &str) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
            prefix: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(prefix),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"insert-prefix\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    insert_prefix_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_match_selected<
        F: Fn(&EntryCompletion, &TreeModel, &TreeIter) -> glib::signal::Inhibit + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn match_selected_trampoline<
            F: Fn(&EntryCompletion, &TreeModel, &TreeIter) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
            model: *mut ffi::GtkTreeModel,
            iter: *mut ffi::GtkTreeIter,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(model),
                &from_glib_borrow(iter),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"match-selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    match_selected_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_no_matches<F: Fn(&EntryCompletion) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn no_matches_trampoline<F: Fn(&EntryCompletion) + 'static>(
            this: *mut ffi::GtkEntryCompletion,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"no-matches\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    no_matches_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_inline_completion_notify<F: Fn(&EntryCompletion) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_inline_completion_trampoline<
            F: Fn(&EntryCompletion) + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
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
                b"notify::inline-completion\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inline_completion_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_inline_selection_notify<F: Fn(&EntryCompletion) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_inline_selection_trampoline<
            F: Fn(&EntryCompletion) + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
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
                b"notify::inline-selection\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inline_selection_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_minimum_key_length_notify<F: Fn(&EntryCompletion) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_minimum_key_length_trampoline<
            F: Fn(&EntryCompletion) + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
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
                b"notify::minimum-key-length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_minimum_key_length_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_model_notify<F: Fn(&EntryCompletion) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&EntryCompletion) + 'static>(
            this: *mut ffi::GtkEntryCompletion,
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
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_popup_completion_notify<F: Fn(&EntryCompletion) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_popup_completion_trampoline<
            F: Fn(&EntryCompletion) + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
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
                b"notify::popup-completion\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_popup_completion_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_popup_set_width_notify<F: Fn(&EntryCompletion) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_popup_set_width_trampoline<
            F: Fn(&EntryCompletion) + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
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
                b"notify::popup-set-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_popup_set_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_popup_single_match_notify<F: Fn(&EntryCompletion) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_popup_single_match_trampoline<
            F: Fn(&EntryCompletion) + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
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
                b"notify::popup-single-match\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_popup_single_match_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_text_column_notify<F: Fn(&EntryCompletion) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_column_trampoline<F: Fn(&EntryCompletion) + 'static>(
            this: *mut ffi::GtkEntryCompletion,
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
                b"notify::text-column\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_column_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for EntryCompletion {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct EntryCompletionBuilder {
    cell_area: Option<CellArea>,
    inline_completion: Option<bool>,
    inline_selection: Option<bool>,
    minimum_key_length: Option<i32>,
    model: Option<TreeModel>,
    popup_completion: Option<bool>,
    popup_set_width: Option<bool>,
    popup_single_match: Option<bool>,
    text_column: Option<i32>,
}

impl EntryCompletionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> EntryCompletion {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref cell_area) = self.cell_area {
            properties.push(("cell-area", cell_area));
        }
        if let Some(ref inline_completion) = self.inline_completion {
            properties.push(("inline-completion", inline_completion));
        }
        if let Some(ref inline_selection) = self.inline_selection {
            properties.push(("inline-selection", inline_selection));
        }
        if let Some(ref minimum_key_length) = self.minimum_key_length {
            properties.push(("minimum-key-length", minimum_key_length));
        }
        if let Some(ref model) = self.model {
            properties.push(("model", model));
        }
        if let Some(ref popup_completion) = self.popup_completion {
            properties.push(("popup-completion", popup_completion));
        }
        if let Some(ref popup_set_width) = self.popup_set_width {
            properties.push(("popup-set-width", popup_set_width));
        }
        if let Some(ref popup_single_match) = self.popup_single_match {
            properties.push(("popup-single-match", popup_single_match));
        }
        if let Some(ref text_column) = self.text_column {
            properties.push(("text-column", text_column));
        }
        let ret = glib::Object::new::<EntryCompletion>(&properties).expect("object new");
        ret
    }

    pub fn cell_area<P: IsA<CellArea>>(mut self, cell_area: &P) -> Self {
        self.cell_area = Some(cell_area.clone().upcast());
        self
    }

    pub fn inline_completion(mut self, inline_completion: bool) -> Self {
        self.inline_completion = Some(inline_completion);
        self
    }

    pub fn inline_selection(mut self, inline_selection: bool) -> Self {
        self.inline_selection = Some(inline_selection);
        self
    }

    pub fn minimum_key_length(mut self, minimum_key_length: i32) -> Self {
        self.minimum_key_length = Some(minimum_key_length);
        self
    }

    pub fn model<P: IsA<TreeModel>>(mut self, model: &P) -> Self {
        self.model = Some(model.clone().upcast());
        self
    }

    pub fn popup_completion(mut self, popup_completion: bool) -> Self {
        self.popup_completion = Some(popup_completion);
        self
    }

    pub fn popup_set_width(mut self, popup_set_width: bool) -> Self {
        self.popup_set_width = Some(popup_set_width);
        self
    }

    pub fn popup_single_match(mut self, popup_single_match: bool) -> Self {
        self.popup_single_match = Some(popup_single_match);
        self
    }

    pub fn text_column(mut self, text_column: i32) -> Self {
        self.text_column = Some(text_column);
        self
    }
}

impl fmt::Display for EntryCompletion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EntryCompletion")
    }
}
