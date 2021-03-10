// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gtk4-gir-files
// DO NOT EDIT

use crate::FontChooserLevel;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct FontChooser(Interface<ffi::GtkFontChooser, ffi::GtkFontChooserIface>);

    match fn {
        get_type => || ffi::gtk_font_chooser_get_type(),
    }
}

pub const NONE_FONT_CHOOSER: Option<&FontChooser> = None;

pub trait FontChooserExt: 'static {
    #[doc(alias = "gtk_font_chooser_get_font")]
    fn get_font(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_font_chooser_get_font_desc")]
    fn get_font_desc(&self) -> Option<pango::FontDescription>;

    #[doc(alias = "gtk_font_chooser_get_font_face")]
    fn get_font_face(&self) -> Option<pango::FontFace>;

    #[doc(alias = "gtk_font_chooser_get_font_family")]
    fn get_font_family(&self) -> Option<pango::FontFamily>;

    #[doc(alias = "gtk_font_chooser_get_font_features")]
    fn get_font_features(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_font_chooser_get_font_map")]
    fn get_font_map(&self) -> Option<pango::FontMap>;

    #[doc(alias = "gtk_font_chooser_get_font_size")]
    fn get_font_size(&self) -> i32;

    #[doc(alias = "gtk_font_chooser_get_language")]
    fn get_language(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_font_chooser_get_level")]
    fn get_level(&self) -> FontChooserLevel;

    #[doc(alias = "gtk_font_chooser_get_preview_text")]
    fn get_preview_text(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_font_chooser_get_show_preview_entry")]
    fn get_show_preview_entry(&self) -> bool;

    #[doc(alias = "gtk_font_chooser_set_filter_func")]
    fn set_filter_func(
        &self,
        filter: Option<Box_<dyn Fn(&pango::FontFamily, &pango::FontFace) -> bool + 'static>>,
    );

    #[doc(alias = "gtk_font_chooser_set_font")]
    fn set_font(&self, fontname: &str);

    #[doc(alias = "gtk_font_chooser_set_font_desc")]
    fn set_font_desc(&self, font_desc: &pango::FontDescription);

    #[doc(alias = "gtk_font_chooser_set_font_map")]
    fn set_font_map<P: IsA<pango::FontMap>>(&self, fontmap: Option<&P>);

    #[doc(alias = "gtk_font_chooser_set_language")]
    fn set_language(&self, language: &str);

    #[doc(alias = "gtk_font_chooser_set_level")]
    fn set_level(&self, level: FontChooserLevel);

    #[doc(alias = "gtk_font_chooser_set_preview_text")]
    fn set_preview_text(&self, text: &str);

    #[doc(alias = "gtk_font_chooser_set_show_preview_entry")]
    fn set_show_preview_entry(&self, show_preview_entry: bool);

    fn connect_font_activated<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_font_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_font_features_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_language_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_preview_text_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_show_preview_entry_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<FontChooser>> FontChooserExt for O {
    fn get_font(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_font_desc(&self) -> Option<pango::FontDescription> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font_desc(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_font_face(&self) -> Option<pango::FontFace> {
        unsafe {
            from_glib_none(ffi::gtk_font_chooser_get_font_face(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_font_family(&self) -> Option<pango::FontFamily> {
        unsafe {
            from_glib_none(ffi::gtk_font_chooser_get_font_family(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_font_features(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font_features(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_font_map(&self) -> Option<pango::FontMap> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font_map(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_font_size(&self) -> i32 {
        unsafe { ffi::gtk_font_chooser_get_font_size(self.as_ref().to_glib_none().0) }
    }

    fn get_language(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_language(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_level(&self) -> FontChooserLevel {
        unsafe {
            from_glib(ffi::gtk_font_chooser_get_level(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_preview_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_preview_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_show_preview_entry(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_chooser_get_show_preview_entry(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_filter_func(
        &self,
        filter: Option<Box_<dyn Fn(&pango::FontFamily, &pango::FontFace) -> bool + 'static>>,
    ) {
        let filter_data: Box_<
            Option<Box_<dyn Fn(&pango::FontFamily, &pango::FontFace) -> bool + 'static>>,
        > = Box_::new(filter);
        unsafe extern "C" fn filter_func(
            family: *const pango::ffi::PangoFontFamily,
            face: *const pango::ffi::PangoFontFace,
            data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let family = from_glib_borrow(family);
            let face = from_glib_borrow(face);
            let callback: &Option<
                Box_<dyn Fn(&pango::FontFamily, &pango::FontFace) -> bool + 'static>,
            > = &*(data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&family, &face)
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib()
        }
        let filter = if filter_data.is_some() {
            Some(filter_func as _)
        } else {
            None
        };
        unsafe extern "C" fn destroy_func(data: glib::ffi::gpointer) {
            let _callback: Box_<
                Option<Box_<dyn Fn(&pango::FontFamily, &pango::FontFace) -> bool + 'static>>,
            > = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func as _);
        let super_callback0: Box_<
            Option<Box_<dyn Fn(&pango::FontFamily, &pango::FontFace) -> bool + 'static>>,
        > = filter_data;
        unsafe {
            ffi::gtk_font_chooser_set_filter_func(
                self.as_ref().to_glib_none().0,
                filter,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn set_font(&self, fontname: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_font(
                self.as_ref().to_glib_none().0,
                fontname.to_glib_none().0,
            );
        }
    }

    fn set_font_desc(&self, font_desc: &pango::FontDescription) {
        unsafe {
            ffi::gtk_font_chooser_set_font_desc(
                self.as_ref().to_glib_none().0,
                font_desc.to_glib_none().0,
            );
        }
    }

    fn set_font_map<P: IsA<pango::FontMap>>(&self, fontmap: Option<&P>) {
        unsafe {
            ffi::gtk_font_chooser_set_font_map(
                self.as_ref().to_glib_none().0,
                fontmap.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_language(&self, language: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_language(
                self.as_ref().to_glib_none().0,
                language.to_glib_none().0,
            );
        }
    }

    fn set_level(&self, level: FontChooserLevel) {
        unsafe {
            ffi::gtk_font_chooser_set_level(self.as_ref().to_glib_none().0, level.to_glib());
        }
    }

    fn set_preview_text(&self, text: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_preview_text(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    fn set_show_preview_entry(&self, show_preview_entry: bool) {
        unsafe {
            ffi::gtk_font_chooser_set_show_preview_entry(
                self.as_ref().to_glib_none().0,
                show_preview_entry.to_glib(),
            );
        }
    }

    fn connect_font_activated<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn font_activated_trampoline<P, F: Fn(&P, &str) + 'static>(
            this: *mut ffi::GtkFontChooser,
            fontname: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FontChooser>,
        {
            let f: &F = &*(f as *const F);
            f(
                &FontChooser::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(fontname),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"font-activated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    font_activated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_font_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFontChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FontChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FontChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::font\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_font_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_desc_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFontChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FontChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FontChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::font-desc\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_font_desc_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_font_features_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_features_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFontChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FontChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FontChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::font-features\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_font_features_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_language_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_language_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFontChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FontChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FontChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::language\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_language_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_level_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFontChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FontChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FontChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::level\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_level_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_preview_text_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_preview_text_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFontChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FontChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FontChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::preview-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_preview_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_show_preview_entry_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_preview_entry_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFontChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FontChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FontChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-preview-entry\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_preview_entry_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FontChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FontChooser")
    }
}
