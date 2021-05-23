// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    pub struct X11DeviceXI2(Object<ffi::GdkX11DeviceXI2, ffi::GdkX11DeviceXI2Class>) @extends gdk::Device;

    match fn {
        type_ => || ffi::gdk_x11_device_xi2_get_type(),
    }
}

impl X11DeviceXI2 {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`X11DeviceXI2`]
    /// This method returns an instance of [`X11DeviceXI2Builder`] which can be used to create a [`X11DeviceXI2`].
    pub fn builder() -> X11DeviceXI2Builder {
        X11DeviceXI2Builder::default()
    }

    #[doc(alias = "device-id")]
    pub fn device_id(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"device-id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `device-id` getter")
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`X11DeviceXI2`].
pub struct X11DeviceXI2Builder {
    device_id: Option<i32>,
    display: Option<gdk::Display>,
    has_cursor: Option<bool>,
    name: Option<String>,
    num_touches: Option<u32>,
    product_id: Option<String>,
    //seat: /*Unknown type*/,
    //source: /*Unknown type*/,
    vendor_id: Option<String>,
}

impl X11DeviceXI2Builder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`X11DeviceXI2Builder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`X11DeviceXI2`].
    pub fn build(self) -> X11DeviceXI2 {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref device_id) = self.device_id {
            properties.push(("device-id", device_id));
        }
        if let Some(ref display) = self.display {
            properties.push(("display", display));
        }
        if let Some(ref has_cursor) = self.has_cursor {
            properties.push(("has-cursor", has_cursor));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref num_touches) = self.num_touches {
            properties.push(("num-touches", num_touches));
        }
        if let Some(ref product_id) = self.product_id {
            properties.push(("product-id", product_id));
        }
        if let Some(ref vendor_id) = self.vendor_id {
            properties.push(("vendor-id", vendor_id));
        }
        glib::Object::new::<X11DeviceXI2>(&properties)
            .expect("Failed to create an instance of X11DeviceXI2")
    }

    pub fn device_id(mut self, device_id: i32) -> Self {
        self.device_id = Some(device_id);
        self
    }

    pub fn display(mut self, display: &impl IsA<gdk::Display>) -> Self {
        self.display = Some(display.clone().upcast());
        self
    }

    pub fn has_cursor(mut self, has_cursor: bool) -> Self {
        self.has_cursor = Some(has_cursor);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn num_touches(mut self, num_touches: u32) -> Self {
        self.num_touches = Some(num_touches);
        self
    }

    pub fn product_id(mut self, product_id: &str) -> Self {
        self.product_id = Some(product_id.to_string());
        self
    }

    pub fn vendor_id(mut self, vendor_id: &str) -> Self {
        self.vendor_id = Some(vendor_id.to_string());
        self
    }
}

impl fmt::Display for X11DeviceXI2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("X11DeviceXI2")
    }
}
