// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{BlendMode, RenderNode, RenderNodeType};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GskBlendNode")]
    pub struct BlendNode(Shared<ffi::GskBlendNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

define_render_node!(
    BlendNode,
    ffi::GskBlendNode,
    ffi::gsk_blend_node_get_type,
    RenderNodeType::BlendNode
);

impl BlendNode {
    #[doc(alias = "gsk_blend_node_new")]
    pub fn new<P: AsRef<RenderNode>, Q: AsRef<RenderNode>>(
        bottom: &P,
        top: &Q,
        blend_mode: BlendMode,
    ) -> Self {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_blend_node_new(
                bottom.as_ref().to_glib_none().0,
                top.as_ref().to_glib_none().0,
                blend_mode.into_glib(),
            ))
        }
    }

    #[doc(alias = "gsk_blend_node_get_blend_mode")]
    #[doc(alias = "get_blend_mode")]
    pub fn blend_mode(&self) -> BlendMode {
        unsafe { from_glib(ffi::gsk_blend_node_get_blend_mode(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_blend_node_get_bottom_child")]
    #[doc(alias = "get_bottom_child")]
    pub fn bottom_child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_blend_node_get_bottom_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_blend_node_get_top_child")]
    #[doc(alias = "get_top_child")]
    pub fn top_child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_blend_node_get_top_child(self.to_glib_none().0)) }
    }
}
