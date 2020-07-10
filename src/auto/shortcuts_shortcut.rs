// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::value::SetValueOptional;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Buildable;
use ShortcutType;
use SizeGroup;
use Widget;

glib_wrapper! {
    pub struct ShortcutsShortcut(Object<gtk_sys::GtkShortcutsShortcut, gtk_sys::GtkShortcutsShortcutClass, ShortcutsShortcutClass>) @extends Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_shortcuts_shortcut_get_type(),
    }
}

impl ShortcutsShortcut {
    pub fn set_property_accel_size_group(&self, accel_size_group: Option<&SizeGroup>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"accel-size-group\0".as_ptr() as *const _,
                Value::from(accel_size_group).to_glib_none().0,
            );
        }
    }

    pub fn get_property_accelerator(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"accelerator\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `accelerator` getter")
        }
    }

    pub fn set_property_accelerator(&self, accelerator: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"accelerator\0".as_ptr() as *const _,
                Value::from(accelerator).to_glib_none().0,
            );
        }
    }

    pub fn get_property_action_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"action-name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `action-name` getter")
        }
    }

    pub fn set_property_action_name(&self, action_name: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"action-name\0".as_ptr() as *const _,
                Value::from(action_name).to_glib_none().0,
            );
        }
    }

    pub fn get_property_icon(&self) -> Option<gio::Icon> {
        unsafe {
            let mut value = Value::from_type(<gio::Icon as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"icon\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `icon` getter")
        }
    }

    pub fn set_property_icon<P: IsA<gio::Icon> + SetValueOptional>(&self, icon: Option<&P>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"icon\0".as_ptr() as *const _,
                Value::from(icon).to_glib_none().0,
            );
        }
    }

    pub fn get_property_icon_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"icon-set\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `icon-set` getter")
                .unwrap()
        }
    }

    pub fn set_property_icon_set(&self, icon_set: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"icon-set\0".as_ptr() as *const _,
                Value::from(&icon_set).to_glib_none().0,
            );
        }
    }

    pub fn get_property_shortcut_type(&self) -> ShortcutType {
        unsafe {
            let mut value = Value::from_type(<ShortcutType as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"shortcut-type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `shortcut-type` getter")
                .unwrap()
        }
    }

    pub fn set_property_shortcut_type(&self, shortcut_type: ShortcutType) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"shortcut-type\0".as_ptr() as *const _,
                Value::from(&shortcut_type).to_glib_none().0,
            );
        }
    }

    pub fn get_property_subtitle(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"subtitle\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `subtitle` getter")
        }
    }

    pub fn set_property_subtitle(&self, subtitle: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"subtitle\0".as_ptr() as *const _,
                Value::from(subtitle).to_glib_none().0,
            );
        }
    }

    pub fn get_property_subtitle_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"subtitle-set\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `subtitle-set` getter")
                .unwrap()
        }
    }

    pub fn set_property_subtitle_set(&self, subtitle_set: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"subtitle-set\0".as_ptr() as *const _,
                Value::from(&subtitle_set).to_glib_none().0,
            );
        }
    }

    pub fn get_property_title(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"title\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `title` getter")
        }
    }

    pub fn set_property_title(&self, title: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"title\0".as_ptr() as *const _,
                Value::from(title).to_glib_none().0,
            );
        }
    }

    pub fn set_property_title_size_group(&self, title_size_group: Option<&SizeGroup>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"title-size-group\0".as_ptr() as *const _,
                Value::from(title_size_group).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_accel_size_group_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accel_size_group_trampoline<
            F: Fn(&ShortcutsShortcut) + 'static,
        >(
            this: *mut gtk_sys::GtkShortcutsShortcut,
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
                b"notify::accel-size-group\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accel_size_group_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_accelerator_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accelerator_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut gtk_sys::GtkShortcutsShortcut,
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
                b"notify::accelerator\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accelerator_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_action_name_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_action_name_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut gtk_sys::GtkShortcutsShortcut,
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
                b"notify::action-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_action_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_direction_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut gtk_sys::GtkShortcutsShortcut,
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
                b"notify::direction\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_direction_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_icon_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut gtk_sys::GtkShortcutsShortcut,
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
                b"notify::icon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_icon_set_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_set_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut gtk_sys::GtkShortcutsShortcut,
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
                b"notify::icon-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_shortcut_type_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_shortcut_type_trampoline<
            F: Fn(&ShortcutsShortcut) + 'static,
        >(
            this: *mut gtk_sys::GtkShortcutsShortcut,
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
                b"notify::shortcut-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_shortcut_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_subtitle_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut gtk_sys::GtkShortcutsShortcut,
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
                b"notify::subtitle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_subtitle_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_subtitle_set_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_set_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut gtk_sys::GtkShortcutsShortcut,
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
                b"notify::subtitle-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_subtitle_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_title_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut gtk_sys::GtkShortcutsShortcut,
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
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_title_size_group_notify<F: Fn(&ShortcutsShortcut) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_size_group_trampoline<
            F: Fn(&ShortcutsShortcut) + 'static,
        >(
            this: *mut gtk_sys::GtkShortcutsShortcut,
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
                b"notify::title-size-group\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_size_group_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ShortcutsShortcut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ShortcutsShortcut")
    }
}
