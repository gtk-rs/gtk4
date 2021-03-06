// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNode, RenderNodeType, RoundedRect};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GskRoundedClipNode")]
    pub struct RoundedClipNode(Shared<ffi::GskRoundedClipNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

define_render_node!(
    RoundedClipNode,
    ffi::GskRoundedClipNode,
    ffi::gsk_rounded_clip_node_get_type,
    RenderNodeType::RoundedClipNode
);

impl RoundedClipNode {
    #[doc(alias = "gsk_rounded_clip_node_new")]
    pub fn new<P: AsRef<RenderNode>>(child: &P, clip: &RoundedRect) -> Self {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gsk_rounded_clip_node_new(
                child.as_ref().to_glib_none().0,
                clip.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_rounded_clip_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_rounded_clip_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_rounded_clip_node_get_clip")]
    #[doc(alias = "get_clip")]
    pub fn clip(&self) -> Option<RoundedRect> {
        unsafe { from_glib_none(ffi::gsk_rounded_clip_node_get_clip(self.to_glib_none().0)) }
    }
}
