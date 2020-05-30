use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 4] = [
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
];
#[doc = "surface cropping and scaling\n\nThe global interface exposing surface cropping and scaling\ncapabilities is used to instantiate an interface extension for a\nwl_surface object. This extended interface will then allow\ncropping and scaling the surface contents, effectively\ndisconnecting the direct relationship between the buffer and the\nsurface size."]
pub mod wp_viewporter {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Error {
        #[doc = "the surface already has a viewport object associated"]
        ViewportExists = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::ViewportExists),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "unbind from the cropping and scaling interface\n\nInforms the server that the client will not be using this\nprotocol object anymore. This does not affect any other objects,\nwp_viewport objects included.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "extend surface interface for crop and scale\n\nInstantiate an interface extension for the given wl_surface to\ncrop and scale its content. If the given wl_surface already has\na wp_viewport object associated, the viewport_exists\nprotocol error is raised."]
        GetViewport {
            surface: super::wl_surface::WlSurface,
        },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
                destructor: true,
            },
            super::MessageDesc {
                name: "get_viewport",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Object],
                destructor: false,
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Destroy => 0,
                Request::GetViewport { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::GetViewport { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<super::wp_viewport::WpViewport>(
                    version,
                    meta.child(),
                )),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![],
                },
                Request::GetViewport { surface } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::NewId(0), Argument::Object(surface.as_ref().id()),],
                },
            }
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
                Request::GetViewport { surface } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].o = surface.as_ref().c_ptr() as *mut _;
                    f(1, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {}
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {}
        }
        fn opcode(&self) -> u16 {
            match *self {}
        }
        fn since(&self) -> u32 {
            match *self {}
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            match opcode {
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }
    #[derive(Clone, Eq, PartialEq)]
    pub struct WpViewporter(Proxy<WpViewporter>);
    impl AsRef<Proxy<WpViewporter>> for WpViewporter {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WpViewporter>> for WpViewporter {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WpViewporter(value)
        }
    }
    impl From<WpViewporter> for Proxy<WpViewporter> {
        #[inline]
        fn from(value: WpViewporter) -> Self {
            value.0
        }
    }
    impl Interface for WpViewporter {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wp_viewporter";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &wp_viewporter_interface }
        }
    }
    impl WpViewporter {
        #[doc = "unbind from the cropping and scaling interface\n\nInforms the server that the client will not be using this\nprotocol object anymore. This does not affect any other objects,\nwp_viewport objects included.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "extend surface interface for crop and scale\n\nInstantiate an interface extension for the given wl_surface to\ncrop and scale its content. If the given wl_surface already has\na wp_viewport object associated, the viewport_exists\nprotocol error is raised."]
        pub fn get_viewport(
            &self,
            surface: &super::wl_surface::WlSurface,
        ) -> Main<super::wp_viewport::WpViewport> {
            let msg = Request::GetViewport {
                surface: surface.clone(),
            };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_VIEWPORT_SINCE: u32 = 1u32;
    static mut wp_viewporter_requests_get_viewport_types: [*const wl_interface; 2] = [
        unsafe { &super::wp_viewport::wp_viewport_interface as *const wl_interface },
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wp_viewporter_requests: [wl_message; 2] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"get_viewport\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe { &wp_viewporter_requests_get_viewport_types as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wp_viewporter_interface: wl_interface = wl_interface {
        name: b"wp_viewporter\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &wp_viewporter_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "crop and scale interface to a wl_surface\n\nAn additional interface to a wl_surface object, which allows the\nclient to specify the cropping and scaling of the surface\ncontents.\n\nThis interface works with two concepts: the source rectangle (src_x,\nsrc_y, src_width, src_height), and the destination size (dst_width,\ndst_height). The contents of the source rectangle are scaled to the\ndestination size, and content outside the source rectangle is ignored.\nThis state is double-buffered, and is applied on the next\nwl_surface.commit.\n\nThe two parts of crop and scale state are independent: the source\nrectangle, and the destination size. Initially both are unset, that\nis, no scaling is applied. The whole of the current wl_buffer is\nused as the source, and the surface size is as defined in\nwl_surface.attach.\n\nIf the destination size is set, it causes the surface size to become\ndst_width, dst_height. The source (rectangle) is scaled to exactly\nthis size. This overrides whatever the attached wl_buffer size is,\nunless the wl_buffer is NULL. If the wl_buffer is NULL, the surface\nhas no content and therefore no size. Otherwise, the size is always\nat least 1x1 in surface local coordinates.\n\nIf the source rectangle is set, it defines what area of the wl_buffer is\ntaken as the source. If the source rectangle is set and the destination\nsize is not set, then src_width and src_height must be integers, and the\nsurface size becomes the source rectangle size. This results in cropping\nwithout scaling. If src_width or src_height are not integers and\ndestination size is not set, the bad_size protocol error is raised when\nthe surface state is applied.\n\nThe coordinate transformations from buffer pixel coordinates up to\nthe surface-local coordinates happen in the following order:\n1. buffer_transform (wl_surface.set_buffer_transform)\n2. buffer_scale (wl_surface.set_buffer_scale)\n3. crop and scale (wp_viewport.set*)\nThis means, that the source rectangle coordinates of crop and scale\nare given in the coordinates after the buffer transform and scale,\ni.e. in the coordinates that would be the surface-local coordinates\nif the crop and scale was not applied.\n\nIf src_x or src_y are negative, the bad_value protocol error is raised.\nOtherwise, if the source rectangle is partially or completely outside of\nthe non-NULL wl_buffer, then the out_of_buffer protocol error is raised\nwhen the surface state is applied. A NULL wl_buffer does not raise the\nout_of_buffer error.\n\nThe x, y arguments of wl_surface.attach are applied as normal to\nthe surface. They indicate how many pixels to remove from the\nsurface size from the left and the top. In other words, they are\nstill in the surface-local coordinate system, just like dst_width\nand dst_height are.\n\nIf the wl_surface associated with the wp_viewport is destroyed,\nall wp_viewport requests except 'destroy' raise the protocol error\nno_surface.\n\nIf the wp_viewport object is destroyed, the crop and scale\nstate is removed from the wl_surface. The change will be applied\non the next wl_surface.commit."]
pub mod wp_viewport {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Error {
        #[doc = "negative or zero values in width or height"]
        BadValue = 0,
        #[doc = "destination size is not integer"]
        BadSize = 1,
        #[doc = "source rectangle extends outside of the content area"]
        OutOfBuffer = 2,
        #[doc = "the wl_surface was destroyed"]
        NoSurface = 3,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::BadValue),
                1 => Some(Error::BadSize),
                2 => Some(Error::OutOfBuffer),
                3 => Some(Error::NoSurface),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "remove scaling and cropping from the surface\n\nThe associated wl_surface's crop and scale state is removed.\nThe change is applied on the next wl_surface.commit.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "set the source rectangle for cropping\n\nSet the source rectangle of the associated wl_surface. See\nwp_viewport for the description, and relation to the wl_buffer\nsize.\n\nIf all of x, y, width and height are -1.0, the source rectangle is\nunset instead. Any other set of values where width or height are zero\nor negative, or x or y are negative, raise the bad_value protocol\nerror.\n\nThe crop and scale state is double-buffered state, and will be\napplied on the next wl_surface.commit."]
        SetSource {
            x: f64,
            y: f64,
            width: f64,
            height: f64,
        },
        #[doc = "set the surface size for scaling\n\nSet the destination size of the associated wl_surface. See\nwp_viewport for the description, and relation to the wl_buffer\nsize.\n\nIf width is -1 and height is -1, the destination size is unset\ninstead. Any other pair of values for width and height that\ncontains zero or negative values raises the bad_value protocol\nerror.\n\nThe crop and scale state is double-buffered state, and will be\napplied on the next wl_surface.commit."]
        SetDestination { width: i32, height: i32 },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
                destructor: true,
            },
            super::MessageDesc {
                name: "set_source",
                since: 1,
                signature: &[
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Fixed,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_destination",
                since: 1,
                signature: &[super::ArgumentType::Int, super::ArgumentType::Int],
                destructor: false,
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Destroy => 0,
                Request::SetSource { .. } => 1,
                Request::SetDestination { .. } => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::SetSource { .. } => 1,
                Request::SetDestination { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![],
                },
                Request::SetSource {
                    x,
                    y,
                    width,
                    height,
                } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![
                        Argument::Fixed((x * 256.) as i32),
                        Argument::Fixed((y * 256.) as i32),
                        Argument::Fixed((width * 256.) as i32),
                        Argument::Fixed((height * 256.) as i32),
                    ],
                },
                Request::SetDestination { width, height } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![Argument::Int(width), Argument::Int(height),],
                },
            }
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
                Request::SetSource {
                    x,
                    y,
                    width,
                    height,
                } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].f = (x * 256.) as i32;
                    _args_array[1].f = (y * 256.) as i32;
                    _args_array[2].f = (width * 256.) as i32;
                    _args_array[3].f = (height * 256.) as i32;
                    f(1, &mut _args_array)
                }
                Request::SetDestination { width, height } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = width;
                    _args_array[1].i = height;
                    f(2, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {}
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {}
        }
        fn opcode(&self) -> u16 {
            match *self {}
        }
        fn since(&self) -> u32 {
            match *self {}
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            match opcode {
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }
    #[derive(Clone, Eq, PartialEq)]
    pub struct WpViewport(Proxy<WpViewport>);
    impl AsRef<Proxy<WpViewport>> for WpViewport {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WpViewport>> for WpViewport {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WpViewport(value)
        }
    }
    impl From<WpViewport> for Proxy<WpViewport> {
        #[inline]
        fn from(value: WpViewport) -> Self {
            value.0
        }
    }
    impl Interface for WpViewport {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wp_viewport";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &wp_viewport_interface }
        }
    }
    impl WpViewport {
        #[doc = "remove scaling and cropping from the surface\n\nThe associated wl_surface's crop and scale state is removed.\nThe change is applied on the next wl_surface.commit.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the source rectangle for cropping\n\nSet the source rectangle of the associated wl_surface. See\nwp_viewport for the description, and relation to the wl_buffer\nsize.\n\nIf all of x, y, width and height are -1.0, the source rectangle is\nunset instead. Any other set of values where width or height are zero\nor negative, or x or y are negative, raise the bad_value protocol\nerror.\n\nThe crop and scale state is double-buffered state, and will be\napplied on the next wl_surface.commit."]
        pub fn set_source(&self, x: f64, y: f64, width: f64, height: f64) -> () {
            let msg = Request::SetSource {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the surface size for scaling\n\nSet the destination size of the associated wl_surface. See\nwp_viewport for the description, and relation to the wl_buffer\nsize.\n\nIf width is -1 and height is -1, the destination size is unset\ninstead. Any other pair of values for width and height that\ncontains zero or negative values raises the bad_value protocol\nerror.\n\nThe crop and scale state is double-buffered state, and will be\napplied on the next wl_surface.commit."]
        pub fn set_destination(&self, width: i32, height: i32) -> () {
            let msg = Request::SetDestination {
                width: width,
                height: height,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_SOURCE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_DESTINATION_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wp_viewport_requests: [wl_message; 3] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_source\0" as *const u8 as *const c_char,
            signature: b"ffff\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_destination\0" as *const u8 as *const c_char,
            signature: b"ii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wp_viewport_interface: wl_interface = wl_interface {
        name: b"wp_viewport\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 3,
        requests: unsafe { &wp_viewport_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
