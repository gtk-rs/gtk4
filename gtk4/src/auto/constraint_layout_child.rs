// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::LayoutChild;
use crate::LayoutManager;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkConstraintLayoutChild")]
    pub struct ConstraintLayoutChild(Object<ffi::GtkConstraintLayoutChild, ffi::GtkConstraintLayoutChildClass>) @extends LayoutChild;

    match fn {
        type_ => || ffi::gtk_constraint_layout_child_get_type(),
    }
}

impl ConstraintLayoutChild {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ConstraintLayoutChild`] objects.
    ///
    /// This method returns an instance of [`ConstraintLayoutChildBuilder`] which can be used to create [`ConstraintLayoutChild`] objects.
    pub fn builder() -> ConstraintLayoutChildBuilder {
        ConstraintLayoutChildBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ConstraintLayoutChild`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct ConstraintLayoutChildBuilder {
    child_widget: Option<Widget>,
    layout_manager: Option<LayoutManager>,
}

impl ConstraintLayoutChildBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ConstraintLayoutChildBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ConstraintLayoutChild`].
    pub fn build(self) -> ConstraintLayoutChild {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref child_widget) = self.child_widget {
            properties.push(("child-widget", child_widget));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        glib::Object::new::<ConstraintLayoutChild>(&properties)
            .expect("Failed to create an instance of ConstraintLayoutChild")
    }

    pub fn child_widget<P: IsA<Widget>>(mut self, child_widget: &P) -> Self {
        self.child_widget = Some(child_widget.clone().upcast());
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }
}

impl fmt::Display for ConstraintLayoutChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ConstraintLayoutChild")
    }
}
