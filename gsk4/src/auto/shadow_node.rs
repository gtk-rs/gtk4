// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RenderNode;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct ShadowNode(Object<ffi::GskShadowNode>) @extends RenderNode;

    match fn {
        get_type => || ffi::gsk_shadow_node_get_type(),
    }
}

impl ShadowNode {
    #[doc(alias = "gsk_shadow_node_get_child")]
    pub fn get_child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_shadow_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_shadow_node_get_n_shadows")]
    pub fn get_n_shadows(&self) -> usize {
        unsafe { ffi::gsk_shadow_node_get_n_shadows(self.to_glib_none().0) }
    }
}

impl fmt::Display for ShadowNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ShadowNode")
    }
}
