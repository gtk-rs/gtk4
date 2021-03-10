// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gtk4-gir-files
// DO NOT EDIT

use crate::RenderNode;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct TextureNode(Object<ffi::GskTextureNode>) @extends RenderNode;

    match fn {
        get_type => || ffi::gsk_texture_node_get_type(),
    }
}

impl TextureNode {
    #[doc(alias = "gsk_texture_node_new")]
    pub fn new<P: IsA<gdk::Texture>>(texture: &P, bounds: &graphene::Rect) -> TextureNode {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_texture_node_new(
                texture.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_texture_node_get_texture")]
    pub fn get_texture(&self) -> Option<gdk::Texture> {
        unsafe { from_glib_none(ffi::gsk_texture_node_get_texture(self.to_glib_none().0)) }
    }
}

impl fmt::Display for TextureNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TextureNode")
    }
}
