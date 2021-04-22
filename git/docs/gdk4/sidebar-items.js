initSidebarItems({"constant":[["ACTION_ALL",""],["BUTTON_MIDDLE","The middle button."],["BUTTON_PRIMARY","The primary button. This is typically the left mouse button, or the right button in a left-handed setup."],["BUTTON_SECONDARY","The secondary button. This is typically the right mouse button, or the left button in a left-handed setup."],["CURRENT_TIME",""],["EVENT_PROPAGATE",""],["EVENT_STOP",""],["MODIFIER_MASK",""],["NONE_CONTENT_PROVIDER",""],["NONE_DEVICE_PAD",""],["NONE_DRAG_SURFACE",""],["NONE_DRAW_CONTEXT",""],["NONE_EVENT",""],["NONE_PAINTABLE",""],["NONE_POPUP",""],["NONE_TEXTURE",""],["NONE_TOPLEVEL",""],["PRIORITY_REDRAW",""]],"enum":[["AxisUse","Defines how device axes are interpreted by GTK."],["CrossingMode","Specifies the crossing mode for enter and leave events."],["DevicePadFeature","A pad feature."],["DeviceToolType","Indicates the specific type of tool being used being a tablet. Such as an airbrush, pencil, etc."],["DragCancelReason","Used in `GdkDrag` to the reason of a cancelled DND operation."],["EventType","Specifies the type of the event."],["FullscreenMode","Indicates which monitor a surface should span over when in fullscreen mode."],["GLError","Error enumeration for `GdkGLContext`."],["Gravity","Defines the reference point of a surface and is used in `PopupLayout`."],["InputSource","An enumeration describing the type of an input device in general terms."],["KeyMatch","Describes how well an event matches a given keyval and modifiers."],["MemoryFormat","`GdkMemoryFormat` describes a format that bytes can have in memory."],["NotifyType","Specifies the kind of crossing for enter and leave events."],["ScrollDirection","Specifies the direction for scroll events."],["SubpixelLayout","This enumeration describes how the red, green and blue components of physical pixels on an output device are laid out."],["SurfaceEdge","Determines a surface edge or corner."],["TouchpadGesturePhase","Specifies the current state of a touchpad gesture."],["VulkanError","Error enumeration for `VulkanContext`."]],"fn":[["content_deserialize_async",""],["content_deserialize_async_future",""],["content_register_deserializer",""],["content_register_serializer",""],["content_serialize_async",""],["content_serialize_async_future",""],["intern_mime_type",""],["pango_layout_get_clip_region",""],["pango_layout_line_get_clip_region",""],["pixbuf_get_from_surface",""],["pixbuf_get_from_texture",""],["set_allowed_backends",""],["unicode_to_keyval",""]],"mod":[["functions",""],["keys",""],["prelude","Traits intended for blanket imports."],["subclass",""]],"struct":[["AnchorHints","Positioning hints for aligning a surface relative to a rectangle."],["AppLaunchContext","`GdkAppLaunchContext` handles launching an application in a graphical context."],["AppLaunchContextBuilder",""],["AxisFlags","Flags describing the current capabilities of a device/tool."],["ButtonEvent",""],["CairoContext","`GdkCairoContext` is an object representing the platform-specific draw context."],["Clipboard","The `GdkClipboard` object represents data shared between applications or inside an application."],["ClipboardBuilder",""],["ContentDeserializer","A `GdkContentDeserializer` is used to deserialize content received via inter-application data transfers."],["ContentFormats","The `GdkContentFormats` structure is used to advertise and negotiate the format of content."],["ContentFormatsBuilder","A `GdkContentFormatsBuilder` is an auxiliary struct used to create new `GdkContentFormats`, and should not be kept around."],["ContentProvider","A `GdkContentProvider` is used to provide content for the clipboard or for drag-and-drop operations in a number of formats."],["ContentSerializer","A `GdkContentSerializer` is used to serialize content for inter-application data transfers."],["CrossingEvent",""],["Cursor","`GdkCursor` is used to create and destroy cursors."],["CursorBuilder",""],["DNDEvent",""],["DeleteEvent",""],["Device","The `GdkDevice` object represents an input device, such as a keyboard, a mouse, or a touchpad."],["DevicePad","`GdkDevicePad` is an interface implemented by devices of type `InputSource::TabletPad`"],["DeviceTool","A physical tool associated to a `GdkDevice`."],["DeviceToolBuilder",""],["Display","`GdkDisplay` objects are the GDK representation of a workstation."],["DisplayManager","A singleton object that offers notification when displays appear or disappear."],["DisplayManagerBuilder",""],["Drag","The `GdkDrag` object represents the source of an ongoing DND operation."],["DragAction","Used in `GdkDrop` and `GdkDrag` to indicate the actions that the destination can and should do with the dropped data."],["DragSurface","A `DragSurface` is an interface for surfaces used during DND."],["DrawContext","Base class for objects implementing different rendering methods."],["Drop","The `GdkDrop` object represents the target of an ongoing DND operation."],["Event","`GdkEvent`s are immutable data structures, created by GDK to represent windowing system events."],["EventSequence","`GdkEventSequence` is an opaque type representing a sequence of related touch events."],["FocusEvent",""],["FrameClock","A `GdkFrameClock` tells the application when to update and repaint a surface."],["FrameClockPhase","Used to represent the different paint clock phases that can be requested."],["FrameTimings","A `GdkFrameTimings` object holds timing information for a single frame of the application’s displays."],["GLContext","`GdkGLContext` is an object representing a platform-specific OpenGL draw context."],["GLTexture","A `Texture` representing a GL texture object."],["GRange",""],["GrabBrokenEvent",""],["KeyEvent",""],["KeymapKey","A `GdkKeymapKey` is a hardware key that can be mapped to a keyval."],["MemoryTexture","A `GdkTexture` representing image data in memory."],["ModifierType","Flags to indicate the state of modifier keys and mouse buttons in events."],["Monitor","`GdkMonitor` objects represent the individual outputs that are associated with a `GdkDisplay`."],["MonitorBuilder",""],["MotionEvent",""],["PadEvent",""],["Paintable","`GdkPaintable` is a simple interface used by GTK to represent content that can be painted."],["PaintableFlags","Flags about a paintable object."],["Popup","A `GdkPopup` is a surface that is attached to another surface."],["PopupLayout","The `GdkPopupLayout` struct contains information that is necessary position a [interface`Gdk.Popup`] relative to its parent."],["ProximityEvent",""],["RGBA","A `GdkRGBA` is used to represent a color, in a way that is compatible with cairo’s notion of color."],["Rectangle","A `GdkRectangle` data type for representing rectangles."],["RgbaParseError",""],["ScrollEvent",""],["Seat","The `GdkSeat` object represents a collection of input devices that belong to a user."],["SeatCapabilities","Flags describing the seat capabilities."],["Snapshot","Base type for snapshot operations."],["Surface","A `GdkSurface` is a rectangular region on the screen."],["Texture","`GdkTexture` is the basic element used to refer to pixel data."],["TimeCoord","A `TimeCoord` stores a single event in a motion history."],["Toplevel","A `GdkToplevel` is a freestanding toplevel surface."],["ToplevelLayout","The `GdkToplevelLayout` struct contains information that is necessary to present a sovereign window on screen."],["ToplevelSize","The `GdkToplevelSize` struct contains information that is useful to compute the size of a toplevel."],["ToplevelState","Specifies the state of a toplevel surface."],["TouchEvent",""],["TouchpadEvent",""],["VulkanContext","`GdkVulkanContext` is an object representing the platform-specific Vulkan draw context."]],"trait":[["ContentProviderExt","Trait containing all `ContentProvider` methods."],["DevicePadExt","Trait containing all `DevicePad` methods."],["DragSurfaceExt","Trait containing all `DragSurface` methods."],["DrawContextExt","Trait containing all `DrawContext` methods."],["EventKind",""],["PaintableExt","Trait containing all `Paintable` methods."],["PopupExt","Trait containing all `Popup` methods."],["PopupLayoutExtManual",""],["SurfaceExtManual",""],["TextureExt","Trait containing all `Texture` methods."],["ToplevelExt","Trait containing all `Toplevel` methods."]],"type":[["Atom",""]]});