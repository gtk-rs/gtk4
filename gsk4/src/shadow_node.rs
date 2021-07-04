// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNode, RenderNodeType, Shadow};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GskShadowNode")]
    pub struct ShadowNode(Shared<ffi::GskShadowNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

define_render_node!(
    ShadowNode,
    ffi::GskShadowNode,
    ffi::gsk_shadow_node_get_type,
    RenderNodeType::ShadowNode
);

impl ShadowNode {
    #[doc(alias = "gsk_shadow_node_new")]
    pub fn new<P: AsRef<RenderNode>>(child: &P, shadows: &[Shadow]) -> Self {
        skip_assert_initialized!();
        let n_shadows = shadows.len() as usize;
        unsafe {
            from_glib_full(ffi::gsk_shadow_node_new(
                child.as_ref().to_glib_none().0,
                shadows.to_glib_none().0,
                n_shadows,
            ))
        }
    }

    #[doc(alias = "gsk_shadow_node_get_shadow")]
    #[doc(alias = "get_shadow")]
    pub fn shadow(&self, i: usize) -> Option<Shadow> {
        assert!(i < self.n_shadows());
        unsafe { from_glib_none(ffi::gsk_shadow_node_get_shadow(self.to_glib_none().0, i)) }
    }

    #[doc(alias = "gsk_shadow_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> RenderNode {
        unsafe { from_glib_none(ffi::gsk_shadow_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_shadow_node_get_n_shadows")]
    #[doc(alias = "get_n_shadows")]
    pub fn n_shadows(&self) -> usize {
        unsafe { ffi::gsk_shadow_node_get_n_shadows(self.to_glib_none().0) }
    }
}
