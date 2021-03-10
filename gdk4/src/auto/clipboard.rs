// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gtk4-gir-files
// DO NOT EDIT

use crate::ContentFormats;
use crate::ContentProvider;
use crate::Display;
use crate::Texture;
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
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    pub struct Clipboard(Object<ffi::GdkClipboard>);

    match fn {
        get_type => || ffi::gdk_clipboard_get_type(),
    }
}

impl Clipboard {
    #[doc(alias = "gdk_clipboard_get_content")]
    pub fn get_content(&self) -> Option<ContentProvider> {
        unsafe { from_glib_none(ffi::gdk_clipboard_get_content(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_clipboard_get_display")]
    pub fn get_display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_clipboard_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_clipboard_get_formats")]
    pub fn get_formats(&self) -> Option<ContentFormats> {
        unsafe { from_glib_none(ffi::gdk_clipboard_get_formats(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_clipboard_is_local")]
    pub fn is_local(&self) -> bool {
        unsafe { from_glib(ffi::gdk_clipboard_is_local(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_clipboard_read_text_async")]
    pub fn read_text_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<Option<glib::GString>, glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn read_text_async_trampoline<
            Q: FnOnce(Result<Option<glib::GString>, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::gdk_clipboard_read_text_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_text_async_trampoline::<Q>;
        unsafe {
            ffi::gdk_clipboard_read_text_async(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn read_text_async_future(
        &self,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<Option<glib::GString>, glib::Error>> + 'static,
        >,
    > {
        Box_::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.read_text_async(Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    #[doc(alias = "gdk_clipboard_read_texture_async")]
    pub fn read_texture_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<Option<Texture>, glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn read_texture_async_trampoline<
            Q: FnOnce(Result<Option<Texture>, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::gdk_clipboard_read_texture_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_texture_async_trampoline::<Q>;
        unsafe {
            ffi::gdk_clipboard_read_texture_async(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn read_texture_async_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Option<Texture>, glib::Error>> + 'static>>
    {
        Box_::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.read_texture_async(Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    #[doc(alias = "gdk_clipboard_read_value_async")]
    pub fn read_value_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<glib::Value, glib::Error>) + Send + 'static,
    >(
        &self,
        type_: glib::types::Type,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn read_value_async_trampoline<
            Q: FnOnce(Result<glib::Value, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::gdk_clipboard_read_value_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_value_async_trampoline::<Q>;
        unsafe {
            ffi::gdk_clipboard_read_value_async(
                self.to_glib_none().0,
                type_.to_glib(),
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn read_value_async_future(
        &self,
        type_: glib::types::Type,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::Value, glib::Error>> + 'static>>
    {
        Box_::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.read_value_async(type_, io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    //#[doc(alias = "gdk_clipboard_set")]
    //pub fn set(&self, type_: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gdk_clipboard_set() }
    //}

    #[doc(alias = "gdk_clipboard_set_content")]
    pub fn set_content<P: IsA<ContentProvider>>(&self, provider: Option<&P>) -> bool {
        unsafe {
            from_glib(ffi::gdk_clipboard_set_content(
                self.to_glib_none().0,
                provider.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_clipboard_set_text")]
    pub fn set_text(&self, text: &str) {
        unsafe {
            ffi::gdk_clipboard_set_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_clipboard_set_texture")]
    pub fn set_texture<P: IsA<Texture>>(&self, texture: &P) {
        unsafe {
            ffi::gdk_clipboard_set_texture(
                self.to_glib_none().0,
                texture.as_ref().to_glib_none().0,
            );
        }
    }

    //#[doc(alias = "gdk_clipboard_set_valist")]
    //pub fn set_valist(&self, type_: glib::types::Type, args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:gdk_clipboard_set_valist() }
    //}

    #[doc(alias = "gdk_clipboard_set_value")]
    pub fn set_value(&self, value: &glib::Value) {
        unsafe {
            ffi::gdk_clipboard_set_value(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_clipboard_store_async")]
    pub fn store_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn store_async_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::gdk_clipboard_store_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = store_async_trampoline::<Q>;
        unsafe {
            ffi::gdk_clipboard_store_async(
                self.to_glib_none().0,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn store_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.store_async(io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    pub fn get_property_local(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"local\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `local` getter")
                .unwrap()
        }
    }

    pub fn connect_changed<F: Fn(&Clipboard) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<F: Fn(&Clipboard) + 'static>(
            this: *mut ffi::GdkClipboard,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_content_notify<F: Fn(&Clipboard) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_trampoline<F: Fn(&Clipboard) + 'static>(
            this: *mut ffi::GdkClipboard,
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
                b"notify::content\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_content_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_formats_notify<F: Fn(&Clipboard) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_formats_trampoline<F: Fn(&Clipboard) + 'static>(
            this: *mut ffi::GdkClipboard,
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
                b"notify::formats\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_formats_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_local_notify<F: Fn(&Clipboard) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_trampoline<F: Fn(&Clipboard) + 'static>(
            this: *mut ffi::GdkClipboard,
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
                b"notify::local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_local_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct ClipboardBuilder {
    display: Option<Display>,
}

impl ClipboardBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Clipboard {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref display) = self.display {
            properties.push(("display", display));
        }
        let ret = glib::Object::new::<Clipboard>(&properties).expect("object new");
        ret
    }

    pub fn display(mut self, display: &Display) -> Self {
        self.display = Some(display.clone());
        self
    }
}

impl fmt::Display for Clipboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Clipboard")
    }
}
