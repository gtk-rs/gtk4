// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gtk4-gir-files
// DO NOT EDIT

use crate::FileChooser;
use crate::FileChooserAction;
use crate::FileFilter;
use crate::NativeDialog;
use crate::Window;
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
    pub struct FileChooserNative(Object<ffi::GtkFileChooserNative, ffi::GtkFileChooserNativeClass>) @extends NativeDialog, @implements FileChooser;

    match fn {
        get_type => || ffi::gtk_file_chooser_native_get_type(),
    }
}

impl FileChooserNative {
    #[doc(alias = "gtk_file_chooser_native_new")]
    pub fn new<P: IsA<Window>>(
        title: Option<&str>,
        parent: Option<&P>,
        action: FileChooserAction,
        accept_label: Option<&str>,
        cancel_label: Option<&str>,
    ) -> FileChooserNative {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_native_new(
                title.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                action.to_glib(),
                accept_label.to_glib_none().0,
                cancel_label.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_file_chooser_native_get_accept_label")]
    pub fn get_accept_label(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_native_get_accept_label(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_file_chooser_native_get_cancel_label")]
    pub fn get_cancel_label(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_native_get_cancel_label(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_file_chooser_native_set_accept_label")]
    pub fn set_accept_label(&self, accept_label: Option<&str>) {
        unsafe {
            ffi::gtk_file_chooser_native_set_accept_label(
                self.to_glib_none().0,
                accept_label.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_file_chooser_native_set_cancel_label")]
    pub fn set_cancel_label(&self, cancel_label: Option<&str>) {
        unsafe {
            ffi::gtk_file_chooser_native_set_cancel_label(
                self.to_glib_none().0,
                cancel_label.to_glib_none().0,
            );
        }
    }

    pub fn connect_property_accept_label_notify<F: Fn(&FileChooserNative) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accept_label_trampoline<F: Fn(&FileChooserNative) + 'static>(
            this: *mut ffi::GtkFileChooserNative,
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
                b"notify::accept-label\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accept_label_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_cancel_label_notify<F: Fn(&FileChooserNative) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_cancel_label_trampoline<F: Fn(&FileChooserNative) + 'static>(
            this: *mut ffi::GtkFileChooserNative,
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
                b"notify::cancel-label\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_cancel_label_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct FileChooserNativeBuilder {
    accept_label: Option<String>,
    cancel_label: Option<String>,
    modal: Option<bool>,
    title: Option<String>,
    transient_for: Option<Window>,
    visible: Option<bool>,
    action: Option<FileChooserAction>,
    create_folders: Option<bool>,
    filter: Option<FileFilter>,
    select_multiple: Option<bool>,
}

impl FileChooserNativeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> FileChooserNative {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref accept_label) = self.accept_label {
            properties.push(("accept-label", accept_label));
        }
        if let Some(ref cancel_label) = self.cancel_label {
            properties.push(("cancel-label", cancel_label));
        }
        if let Some(ref modal) = self.modal {
            properties.push(("modal", modal));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        if let Some(ref transient_for) = self.transient_for {
            properties.push(("transient-for", transient_for));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref action) = self.action {
            properties.push(("action", action));
        }
        if let Some(ref create_folders) = self.create_folders {
            properties.push(("create-folders", create_folders));
        }
        if let Some(ref filter) = self.filter {
            properties.push(("filter", filter));
        }
        if let Some(ref select_multiple) = self.select_multiple {
            properties.push(("select-multiple", select_multiple));
        }
        let ret = glib::Object::new::<FileChooserNative>(&properties).expect("object new");
        ret
    }

    pub fn accept_label(mut self, accept_label: &str) -> Self {
        self.accept_label = Some(accept_label.to_string());
        self
    }

    pub fn cancel_label(mut self, cancel_label: &str) -> Self {
        self.cancel_label = Some(cancel_label.to_string());
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn transient_for<P: IsA<Window>>(mut self, transient_for: &P) -> Self {
        self.transient_for = Some(transient_for.clone().upcast());
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn action(mut self, action: FileChooserAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn create_folders(mut self, create_folders: bool) -> Self {
        self.create_folders = Some(create_folders);
        self
    }

    pub fn filter(mut self, filter: &FileFilter) -> Self {
        self.filter = Some(filter.clone());
        self
    }

    pub fn select_multiple(mut self, select_multiple: bool) -> Self {
        self.select_multiple = Some(select_multiple);
        self
    }
}

impl fmt::Display for FileChooserNative {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileChooserNative")
    }
}
