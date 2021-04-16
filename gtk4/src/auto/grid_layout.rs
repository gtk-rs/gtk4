// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::BaselinePosition;
use crate::LayoutManager;
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
    pub struct GridLayout(Object<ffi::GtkGridLayout, ffi::GtkGridLayoutClass>) @extends LayoutManager;

    match fn {
        type_ => || ffi::gtk_grid_layout_get_type(),
    }
}

impl GridLayout {
    #[doc(alias = "gtk_grid_layout_new")]
    pub fn new() -> GridLayout {
        assert_initialized_main_thread!();
        unsafe { LayoutManager::from_glib_full(ffi::gtk_grid_layout_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_grid_layout_get_baseline_row")]
    pub fn baseline_row(&self) -> i32 {
        unsafe { ffi::gtk_grid_layout_get_baseline_row(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_grid_layout_get_column_homogeneous")]
    pub fn is_column_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_grid_layout_get_column_homogeneous(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_grid_layout_get_column_spacing")]
    pub fn column_spacing(&self) -> u32 {
        unsafe { ffi::gtk_grid_layout_get_column_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_grid_layout_get_row_baseline_position")]
    pub fn row_baseline_position(&self, row: i32) -> BaselinePosition {
        unsafe {
            from_glib(ffi::gtk_grid_layout_get_row_baseline_position(
                self.to_glib_none().0,
                row,
            ))
        }
    }

    #[doc(alias = "gtk_grid_layout_get_row_homogeneous")]
    pub fn is_row_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_grid_layout_get_row_homogeneous(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_grid_layout_get_row_spacing")]
    pub fn row_spacing(&self) -> u32 {
        unsafe { ffi::gtk_grid_layout_get_row_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_grid_layout_set_baseline_row")]
    pub fn set_baseline_row(&self, row: i32) {
        unsafe {
            ffi::gtk_grid_layout_set_baseline_row(self.to_glib_none().0, row);
        }
    }

    #[doc(alias = "gtk_grid_layout_set_column_homogeneous")]
    pub fn set_column_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_grid_layout_set_column_homogeneous(
                self.to_glib_none().0,
                homogeneous.to_glib(),
            );
        }
    }

    #[doc(alias = "gtk_grid_layout_set_column_spacing")]
    pub fn set_column_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_grid_layout_set_column_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[doc(alias = "gtk_grid_layout_set_row_baseline_position")]
    pub fn set_row_baseline_position(&self, row: i32, pos: BaselinePosition) {
        unsafe {
            ffi::gtk_grid_layout_set_row_baseline_position(
                self.to_glib_none().0,
                row,
                pos.to_glib(),
            );
        }
    }

    #[doc(alias = "gtk_grid_layout_set_row_homogeneous")]
    pub fn set_row_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_grid_layout_set_row_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    #[doc(alias = "gtk_grid_layout_set_row_spacing")]
    pub fn set_row_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_grid_layout_set_row_spacing(self.to_glib_none().0, spacing);
        }
    }

    pub fn connect_property_baseline_row_notify<F: Fn(&GridLayout) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_baseline_row_trampoline<F: Fn(&GridLayout) + 'static>(
            this: *mut ffi::GtkGridLayout,
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
                b"notify::baseline-row\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_baseline_row_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_column_homogeneous_notify<F: Fn(&GridLayout) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_homogeneous_trampoline<F: Fn(&GridLayout) + 'static>(
            this: *mut ffi::GtkGridLayout,
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
                b"notify::column-homogeneous\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_column_homogeneous_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_column_spacing_notify<F: Fn(&GridLayout) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_spacing_trampoline<F: Fn(&GridLayout) + 'static>(
            this: *mut ffi::GtkGridLayout,
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
                b"notify::column-spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_column_spacing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_row_homogeneous_notify<F: Fn(&GridLayout) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_row_homogeneous_trampoline<F: Fn(&GridLayout) + 'static>(
            this: *mut ffi::GtkGridLayout,
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
                b"notify::row-homogeneous\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_row_homogeneous_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_row_spacing_notify<F: Fn(&GridLayout) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_row_spacing_trampoline<F: Fn(&GridLayout) + 'static>(
            this: *mut ffi::GtkGridLayout,
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
                b"notify::row-spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_row_spacing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GridLayout {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct GridLayoutBuilder {
    baseline_row: Option<i32>,
    column_homogeneous: Option<bool>,
    column_spacing: Option<i32>,
    row_homogeneous: Option<bool>,
    row_spacing: Option<i32>,
}

impl GridLayoutBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> GridLayout {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref baseline_row) = self.baseline_row {
            properties.push(("baseline-row", baseline_row));
        }
        if let Some(ref column_homogeneous) = self.column_homogeneous {
            properties.push(("column-homogeneous", column_homogeneous));
        }
        if let Some(ref column_spacing) = self.column_spacing {
            properties.push(("column-spacing", column_spacing));
        }
        if let Some(ref row_homogeneous) = self.row_homogeneous {
            properties.push(("row-homogeneous", row_homogeneous));
        }
        if let Some(ref row_spacing) = self.row_spacing {
            properties.push(("row-spacing", row_spacing));
        }
        let ret = glib::Object::new::<GridLayout>(&properties).expect("object new");
        ret
    }

    pub fn baseline_row(mut self, baseline_row: i32) -> Self {
        self.baseline_row = Some(baseline_row);
        self
    }

    pub fn column_homogeneous(mut self, column_homogeneous: bool) -> Self {
        self.column_homogeneous = Some(column_homogeneous);
        self
    }

    pub fn column_spacing(mut self, column_spacing: i32) -> Self {
        self.column_spacing = Some(column_spacing);
        self
    }

    pub fn row_homogeneous(mut self, row_homogeneous: bool) -> Self {
        self.row_homogeneous = Some(row_homogeneous);
        self
    }

    pub fn row_spacing(mut self, row_spacing: i32) -> Self {
        self.row_spacing = Some(row_spacing);
        self
    }
}

impl fmt::Display for GridLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GridLayout")
    }
}
