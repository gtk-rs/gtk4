// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::GLShader;
use crate::RenderNode;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct GLShaderNode(Object<ffi::GskGLShaderNode>) @extends RenderNode;

    match fn {
        get_type => || ffi::gsk_gl_shader_node_get_type(),
    }
}

impl GLShaderNode {
    #[doc(alias = "gsk_gl_shader_node_new")]
    pub fn new(
        shader: &GLShader,
        bounds: &graphene::Rect,
        args: &glib::Bytes,
        children: &[RenderNode],
    ) -> GLShaderNode {
        skip_assert_initialized!();
        let n_children = children.len() as u32;
        unsafe {
            from_glib_full(ffi::gsk_gl_shader_node_new(
                shader.to_glib_none().0,
                bounds.to_glib_none().0,
                args.to_glib_none().0,
                children.to_glib_none().0,
                n_children,
            ))
        }
    }

    #[doc(alias = "gsk_gl_shader_node_get_args")]
    pub fn get_args(&self) -> Option<glib::Bytes> {
        unsafe { from_glib_none(ffi::gsk_gl_shader_node_get_args(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_gl_shader_node_get_child")]
    pub fn get_child(&self, idx: u32) -> Option<RenderNode> {
        unsafe {
            from_glib_none(ffi::gsk_gl_shader_node_get_child(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    #[doc(alias = "gsk_gl_shader_node_get_n_children")]
    pub fn get_n_children(&self) -> u32 {
        unsafe { ffi::gsk_gl_shader_node_get_n_children(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_gl_shader_node_get_shader")]
    pub fn get_shader(&self) -> Option<GLShader> {
        unsafe { from_glib_none(ffi::gsk_gl_shader_node_get_shader(self.to_glib_none().0)) }
    }
}

impl fmt::Display for GLShaderNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GLShaderNode")
    }
}
