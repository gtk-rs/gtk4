// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib;
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
use Accessible;
use Buildable;
use ConstraintTarget;
use Expression;
use ListItemFactory;
use Widget;

glib_wrapper! {
    pub struct DropDown(Object<gtk_sys::GtkDropDown, gtk_sys::GtkDropDownClass, DropDownClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        get_type => || gtk_sys::gtk_drop_down_get_type(),
    }
}

impl DropDown {
    pub fn new<P: IsA<gio::ListModel>, Q: IsA<Expression>>(
        model: Option<&P>,
        expression: Option<&Q>,
    ) -> DropDown {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_drop_down_new(
                model.map(|p| p.as_ref()).to_glib_full(),
                expression.map(|p| p.as_ref()).to_glib_full(),
            ))
            .unsafe_cast()
        }
    }

    pub fn from_strings(strings: &[&str]) -> DropDown {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_drop_down_new_from_strings(
                strings.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

pub const NONE_DROP_DOWN: Option<&DropDown> = None;

pub trait DropDownExt: 'static {
    fn get_enable_search(&self) -> bool;

    fn get_expression(&self) -> Option<Expression>;

    fn get_factory(&self) -> Option<ListItemFactory>;

    fn get_list_factory(&self) -> Option<ListItemFactory>;

    fn get_model(&self) -> Option<gio::ListModel>;

    fn get_selected(&self) -> u32;

    fn get_selected_item(&self) -> Option<glib::Object>;

    fn set_enable_search(&self, enable_search: bool);

    fn set_expression<P: IsA<Expression>>(&self, expression: Option<&P>);

    fn set_factory<P: IsA<ListItemFactory>>(&self, factory: Option<&P>);

    fn set_list_factory<P: IsA<ListItemFactory>>(&self, factory: Option<&P>);

    fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>);

    fn set_selected(&self, position: u32);

    fn connect_property_enable_search_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_expression_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_list_factory_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selected_item_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<DropDown>> DropDownExt for O {
    fn get_enable_search(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_drop_down_get_enable_search(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_expression(&self) -> Option<Expression> {
        unsafe {
            from_glib_none(gtk_sys::gtk_drop_down_get_expression(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_factory(&self) -> Option<ListItemFactory> {
        unsafe {
            from_glib_none(gtk_sys::gtk_drop_down_get_factory(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_list_factory(&self) -> Option<ListItemFactory> {
        unsafe {
            from_glib_none(gtk_sys::gtk_drop_down_get_list_factory(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_model(&self) -> Option<gio::ListModel> {
        unsafe {
            from_glib_none(gtk_sys::gtk_drop_down_get_model(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_selected(&self) -> u32 {
        unsafe { gtk_sys::gtk_drop_down_get_selected(self.as_ref().to_glib_none().0) }
    }

    fn get_selected_item(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_none(gtk_sys::gtk_drop_down_get_selected_item(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_enable_search(&self, enable_search: bool) {
        unsafe {
            gtk_sys::gtk_drop_down_set_enable_search(
                self.as_ref().to_glib_none().0,
                enable_search.to_glib(),
            );
        }
    }

    fn set_expression<P: IsA<Expression>>(&self, expression: Option<&P>) {
        unsafe {
            gtk_sys::gtk_drop_down_set_expression(
                self.as_ref().to_glib_none().0,
                expression.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_factory<P: IsA<ListItemFactory>>(&self, factory: Option<&P>) {
        unsafe {
            gtk_sys::gtk_drop_down_set_factory(
                self.as_ref().to_glib_none().0,
                factory.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_list_factory<P: IsA<ListItemFactory>>(&self, factory: Option<&P>) {
        unsafe {
            gtk_sys::gtk_drop_down_set_list_factory(
                self.as_ref().to_glib_none().0,
                factory.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>) {
        unsafe {
            gtk_sys::gtk_drop_down_set_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_selected(&self, position: u32) {
        unsafe {
            gtk_sys::gtk_drop_down_set_selected(self.as_ref().to_glib_none().0, position);
        }
    }

    fn connect_property_enable_search_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_search_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkDropDown,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DropDown>,
        {
            let f: &F = &*(f as *const F);
            f(&DropDown::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enable-search\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enable_search_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_expression_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expression_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkDropDown,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DropDown>,
        {
            let f: &F = &*(f as *const F);
            f(&DropDown::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::expression\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expression_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_factory_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkDropDown,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DropDown>,
        {
            let f: &F = &*(f as *const F);
            f(&DropDown::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::factory\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_factory_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_list_factory_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_list_factory_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkDropDown,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DropDown>,
        {
            let f: &F = &*(f as *const F);
            f(&DropDown::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::list-factory\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_list_factory_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkDropDown,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DropDown>,
        {
            let f: &F = &*(f as *const F);
            f(&DropDown::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkDropDown,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DropDown>,
        {
            let f: &F = &*(f as *const F);
            f(&DropDown::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_selected_item_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_item_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkDropDown,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DropDown>,
        {
            let f: &F = &*(f as *const F);
            f(&DropDown::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selected-item\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_item_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DropDown {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DropDown")
    }
}