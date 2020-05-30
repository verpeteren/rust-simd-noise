use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 4] = [
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
];
#[doc = "create desktop-style surfaces\n\nThe xdg_wm_base interface is exposed as a global object enabling clients\nto turn their wl_surfaces into windows in a desktop environment. It\ndefines the basic functionality needed for clients and the compositor to\ncreate windows that can be dragged, resized, maximized, etc, as well as\ncreating transient windows such as popup menus."]
pub mod xdg_wm_base {
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
        #[doc = "given wl_surface has another role"]
        Role = 0,
        #[doc = "xdg_wm_base was destroyed before children"]
        DefunctSurfaces = 1,
        #[doc = "the client tried to map or destroy a non-topmost popup"]
        NotTheTopmostPopup = 2,
        #[doc = "the client specified an invalid popup parent surface"]
        InvalidPopupParent = 3,
        #[doc = "the client provided an invalid surface state"]
        InvalidSurfaceState = 4,
        #[doc = "the client provided an invalid positioner"]
        InvalidPositioner = 5,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::Role),
                1 => Some(Error::DefunctSurfaces),
                2 => Some(Error::NotTheTopmostPopup),
                3 => Some(Error::InvalidPopupParent),
                4 => Some(Error::InvalidSurfaceState),
                5 => Some(Error::InvalidPositioner),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy xdg_wm_base\n\nDestroy this xdg_wm_base object.\n\nDestroying a bound xdg_wm_base object while there are surfaces\nstill alive created by this xdg_wm_base object instance is illegal\nand will result in a protocol error.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "create a positioner object\n\nCreate a positioner object. A positioner object is used to position\nsurfaces relative to some parent surface. See the interface description\nand xdg_surface.get_popup for details."]
        CreatePositioner {},
        #[doc = "create a shell surface from a surface\n\nThis creates an xdg_surface for the given surface. While xdg_surface\nitself is not a role, the corresponding surface may only be assigned\na role extending xdg_surface, such as xdg_toplevel or xdg_popup.\n\nThis creates an xdg_surface for the given surface. An xdg_surface is\nused as basis to define a role to a given surface, such as xdg_toplevel\nor xdg_popup. It also manages functionality shared between xdg_surface\nbased surface roles.\n\nSee the documentation of xdg_surface for more details about what an\nxdg_surface is and how it is used."]
        GetXdgSurface {
            surface: super::wl_surface::WlSurface,
        },
        #[doc = "respond to a ping event\n\nA client must respond to a ping event with a pong request or\nthe client may be deemed unresponsive. See xdg_wm_base.ping."]
        Pong { serial: u32 },
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
                name: "create_positioner",
                since: 1,
                signature: &[super::ArgumentType::NewId],
                destructor: false,
            },
            super::MessageDesc {
                name: "get_xdg_surface",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "pong",
                since: 1,
                signature: &[super::ArgumentType::Uint],
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
                Request::CreatePositioner { .. } => 1,
                Request::GetXdgSurface { .. } => 2,
                Request::Pong { .. } => 3,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::CreatePositioner { .. } => 1,
                Request::GetXdgSurface { .. } => 1,
                Request::Pong { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(
                    Object::from_interface::<super::xdg_positioner::XdgPositioner>(
                        version,
                        meta.child(),
                    ),
                ),
                2 => Some(Object::from_interface::<super::xdg_surface::XdgSurface>(
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
                Request::CreatePositioner {} => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::NewId(0),],
                },
                Request::GetXdgSurface { surface } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![Argument::NewId(0), Argument::Object(surface.as_ref().id()),],
                },
                Request::Pong { serial } => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: smallvec![Argument::Uint(serial),],
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
                Request::CreatePositioner {} => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    f(1, &mut _args_array)
                }
                Request::GetXdgSurface { surface } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].o = surface.as_ref().c_ptr() as *mut _;
                    f(2, &mut _args_array)
                }
                Request::Pong { serial } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    f(3, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "check if the client is alive\n\nThe ping event asks the client if it's still alive. Pass the\nserial specified in the event back to the compositor by sending\na \"pong\" request back with the specified serial. See xdg_wm_base.pong.\n\nCompositors can use this to determine if the client is still\nalive. It's unspecified what will happen if the client doesn't\nrespond to the ping request, or in what timeframe. Clients should\ntry to respond in a reasonable amount of time.\n\nA compositor is free to ping in any way it wants, but a client must\nalways respond to any xdg_wm_base object it created."]
        Ping { serial: u32 },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "ping",
            since: 1,
            signature: &[super::ArgumentType::Uint],
            destructor: false,
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::Ping { .. } => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Ping { .. } => 1,
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
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Ping {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
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
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Ping { serial: _args[0].u })
                }
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
    pub struct XdgWmBase(Proxy<XdgWmBase>);
    impl AsRef<Proxy<XdgWmBase>> for XdgWmBase {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<XdgWmBase>> for XdgWmBase {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            XdgWmBase(value)
        }
    }
    impl From<XdgWmBase> for Proxy<XdgWmBase> {
        #[inline]
        fn from(value: XdgWmBase) -> Self {
            value.0
        }
    }
    impl Interface for XdgWmBase {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "xdg_wm_base";
        const VERSION: u32 = 2;
        fn c_interface() -> *const wl_interface {
            unsafe { &xdg_wm_base_interface }
        }
    }
    impl XdgWmBase {
        #[doc = "destroy xdg_wm_base\n\nDestroy this xdg_wm_base object.\n\nDestroying a bound xdg_wm_base object while there are surfaces\nstill alive created by this xdg_wm_base object instance is illegal\nand will result in a protocol error.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "create a positioner object\n\nCreate a positioner object. A positioner object is used to position\nsurfaces relative to some parent surface. See the interface description\nand xdg_surface.get_popup for details."]
        pub fn create_positioner(&self) -> Main<super::xdg_positioner::XdgPositioner> {
            let msg = Request::CreatePositioner {};
            self.0.send(msg, None).unwrap()
        }
        #[doc = "create a shell surface from a surface\n\nThis creates an xdg_surface for the given surface. While xdg_surface\nitself is not a role, the corresponding surface may only be assigned\na role extending xdg_surface, such as xdg_toplevel or xdg_popup.\n\nThis creates an xdg_surface for the given surface. An xdg_surface is\nused as basis to define a role to a given surface, such as xdg_toplevel\nor xdg_popup. It also manages functionality shared between xdg_surface\nbased surface roles.\n\nSee the documentation of xdg_surface for more details about what an\nxdg_surface is and how it is used."]
        pub fn get_xdg_surface(
            &self,
            surface: &super::wl_surface::WlSurface,
        ) -> Main<super::xdg_surface::XdgSurface> {
            let msg = Request::GetXdgSurface {
                surface: surface.clone(),
            };
            self.0.send(msg, None).unwrap()
        }
        #[doc = "respond to a ping event\n\nA client must respond to a ping event with a pong request or\nthe client may be deemed unresponsive. See xdg_wm_base.ping."]
        pub fn pong(&self, serial: u32) -> () {
            let msg = Request::Pong { serial: serial };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CREATE_POSITIONER_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_XDG_SURFACE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_PONG_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_PING_SINCE: u32 = 1u32;
    static mut xdg_wm_base_requests_create_positioner_types: [*const wl_interface; 1] =
        [unsafe { &super::xdg_positioner::xdg_positioner_interface as *const wl_interface }];
    static mut xdg_wm_base_requests_get_xdg_surface_types: [*const wl_interface; 2] = [
        unsafe { &super::xdg_surface::xdg_surface_interface as *const wl_interface },
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut xdg_wm_base_requests: [wl_message; 4] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"create_positioner\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &xdg_wm_base_requests_create_positioner_types as *const _ },
        },
        wl_message {
            name: b"get_xdg_surface\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe { &xdg_wm_base_requests_get_xdg_surface_types as *const _ },
        },
        wl_message {
            name: b"pong\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut xdg_wm_base_events: [wl_message; 1] = [wl_message {
        name: b"ping\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut xdg_wm_base_interface: wl_interface = wl_interface {
        name: b"xdg_wm_base\0" as *const u8 as *const c_char,
        version: 2,
        request_count: 4,
        requests: unsafe { &xdg_wm_base_requests as *const _ },
        event_count: 1,
        events: unsafe { &xdg_wm_base_events as *const _ },
    };
}
#[doc = "child surface positioner\n\nThe xdg_positioner provides a collection of rules for the placement of a\nchild surface relative to a parent surface. Rules can be defined to ensure\nthe child surface remains within the visible area's borders, and to\nspecify how the child surface changes its position, such as sliding along\nan axis, or flipping around a rectangle. These positioner-created rules are\nconstrained by the requirement that a child surface must intersect with or\nbe at least partially adjacent to its parent surface.\n\nSee the various requests for details about possible rules.\n\nAt the time of the request, the compositor makes a copy of the rules\nspecified by the xdg_positioner. Thus, after the request is complete the\nxdg_positioner object can be destroyed or reused; further changes to the\nobject will have no effect on previous usages.\n\nFor an xdg_positioner object to be considered complete, it must have a\nnon-zero size set by set_size, and a non-zero anchor rectangle set by\nset_anchor_rect. Passing an incomplete xdg_positioner object when\npositioning a surface raises an error."]
pub mod xdg_positioner {
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
        #[doc = "invalid input provided"]
        InvalidInput = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidInput),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Anchor {
        None = 0,
        Top = 1,
        Bottom = 2,
        Left = 3,
        Right = 4,
        TopLeft = 5,
        BottomLeft = 6,
        TopRight = 7,
        BottomRight = 8,
    }
    impl Anchor {
        pub fn from_raw(n: u32) -> Option<Anchor> {
            match n {
                0 => Some(Anchor::None),
                1 => Some(Anchor::Top),
                2 => Some(Anchor::Bottom),
                3 => Some(Anchor::Left),
                4 => Some(Anchor::Right),
                5 => Some(Anchor::TopLeft),
                6 => Some(Anchor::BottomLeft),
                7 => Some(Anchor::TopRight),
                8 => Some(Anchor::BottomRight),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Gravity {
        None = 0,
        Top = 1,
        Bottom = 2,
        Left = 3,
        Right = 4,
        TopLeft = 5,
        BottomLeft = 6,
        TopRight = 7,
        BottomRight = 8,
    }
    impl Gravity {
        pub fn from_raw(n: u32) -> Option<Gravity> {
            match n {
                0 => Some(Gravity::None),
                1 => Some(Gravity::Top),
                2 => Some(Gravity::Bottom),
                3 => Some(Gravity::Left),
                4 => Some(Gravity::Right),
                5 => Some(Gravity::TopLeft),
                6 => Some(Gravity::BottomLeft),
                7 => Some(Gravity::TopRight),
                8 => Some(Gravity::BottomRight),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    bitflags! { # [ doc = "constraint adjustments\n\nThe constraint adjustment value define ways the compositor will adjust\nthe position of the surface, if the unadjusted position would result\nin the surface being partly constrained.\n\nWhether a surface is considered 'constrained' is left to the compositor\nto determine. For example, the surface may be partly outside the\ncompositor's defined 'work area', thus necessitating the child surface's\nposition be adjusted until it is entirely inside the work area.\n\nThe adjustments can be combined, according to a defined precedence: 1)\nFlip, 2) Slide, 3) Resize." ] pub struct ConstraintAdjustment : u32 { # [ doc = "don't move the child surface when constrained\n\nDon't alter the surface position even if it is constrained on some\naxis, for example partially outside the edge of an output." ] const None = 0 ; # [ doc = "move along the x axis until unconstrained\n\nSlide the surface along the x axis until it is no longer constrained.\n\nFirst try to slide towards the direction of the gravity on the x axis\nuntil either the edge in the opposite direction of the gravity is\nunconstrained or the edge in the direction of the gravity is\nconstrained.\n\nThen try to slide towards the opposite direction of the gravity on the\nx axis until either the edge in the direction of the gravity is\nunconstrained or the edge in the opposite direction of the gravity is\nconstrained." ] const SlideX = 1 ; # [ doc = "move along the y axis until unconstrained\n\nSlide the surface along the y axis until it is no longer constrained.\n\nFirst try to slide towards the direction of the gravity on the y axis\nuntil either the edge in the opposite direction of the gravity is\nunconstrained or the edge in the direction of the gravity is\nconstrained.\n\nThen try to slide towards the opposite direction of the gravity on the\ny axis until either the edge in the direction of the gravity is\nunconstrained or the edge in the opposite direction of the gravity is\nconstrained." ] const SlideY = 2 ; # [ doc = "invert the anchor and gravity on the x axis\n\nInvert the anchor and gravity on the x axis if the surface is\nconstrained on the x axis. For example, if the left edge of the\nsurface is constrained, the gravity is 'left' and the anchor is\n'left', change the gravity to 'right' and the anchor to 'right'.\n\nIf the adjusted position also ends up being constrained, the resulting\nposition of the flip_x adjustment will be the one before the\nadjustment." ] const FlipX = 4 ; # [ doc = "invert the anchor and gravity on the y axis\n\nInvert the anchor and gravity on the y axis if the surface is\nconstrained on the y axis. For example, if the bottom edge of the\nsurface is constrained, the gravity is 'bottom' and the anchor is\n'bottom', change the gravity to 'top' and the anchor to 'top'.\n\nThe adjusted position is calculated given the original anchor\nrectangle and offset, but with the new flipped anchor and gravity\nvalues.\n\nIf the adjusted position also ends up being constrained, the resulting\nposition of the flip_y adjustment will be the one before the\nadjustment." ] const FlipY = 8 ; # [ doc = "horizontally resize the surface\n\nResize the surface horizontally so that it is completely\nunconstrained." ] const ResizeX = 16 ; # [ doc = "vertically resize the surface\n\nResize the surface vertically so that it is completely unconstrained." ] const ResizeY = 32 ; } }
    impl ConstraintAdjustment {
        pub fn from_raw(n: u32) -> Option<ConstraintAdjustment> {
            Some(ConstraintAdjustment::from_bits_truncate(n))
        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy the xdg_positioner object\n\nNotify the compositor that the xdg_positioner will no longer be used.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "set the size of the to-be positioned rectangle\n\nSet the size of the surface that is to be positioned with the positioner\nobject. The size is in surface-local coordinates and corresponds to the\nwindow geometry. See xdg_surface.set_window_geometry.\n\nIf a zero or negative size is set the invalid_input error is raised."]
        SetSize { width: i32, height: i32 },
        #[doc = "set the anchor rectangle within the parent surface\n\nSpecify the anchor rectangle within the parent surface that the child\nsurface will be placed relative to. The rectangle is relative to the\nwindow geometry as defined by xdg_surface.set_window_geometry of the\nparent surface.\n\nWhen the xdg_positioner object is used to position a child surface, the\nanchor rectangle may not extend outside the window geometry of the\npositioned child's parent surface.\n\nIf a negative size is set the invalid_input error is raised."]
        SetAnchorRect {
            x: i32,
            y: i32,
            width: i32,
            height: i32,
        },
        #[doc = "set anchor rectangle anchor\n\nDefines the anchor point for the anchor rectangle. The specified anchor\nis used derive an anchor point that the child surface will be\npositioned relative to. If a corner anchor is set (e.g. 'top_left' or\n'bottom_right'), the anchor point will be at the specified corner;\notherwise, the derived anchor point will be centered on the specified\nedge, or in the center of the anchor rectangle if no edge is specified."]
        SetAnchor { anchor: Anchor },
        #[doc = "set child surface gravity\n\nDefines in what direction a surface should be positioned, relative to\nthe anchor point of the parent surface. If a corner gravity is\nspecified (e.g. 'bottom_right' or 'top_left'), then the child surface\nwill be placed towards the specified gravity; otherwise, the child\nsurface will be centered over the anchor point on any axis that had no\ngravity specified."]
        SetGravity { gravity: Gravity },
        #[doc = "set the adjustment to be done when constrained\n\nSpecify how the window should be positioned if the originally intended\nposition caused the surface to be constrained, meaning at least\npartially outside positioning boundaries set by the compositor. The\nadjustment is set by constructing a bitmask describing the adjustment to\nbe made when the surface is constrained on that axis.\n\nIf no bit for one axis is set, the compositor will assume that the child\nsurface should not change its position on that axis when constrained.\n\nIf more than one bit for one axis is set, the order of how adjustments\nare applied is specified in the corresponding adjustment descriptions.\n\nThe default adjustment is none."]
        SetConstraintAdjustment { constraint_adjustment: u32 },
        #[doc = "set surface position offset\n\nSpecify the surface position offset relative to the position of the\nanchor on the anchor rectangle and the anchor on the surface. For\nexample if the anchor of the anchor rectangle is at (x, y), the surface\nhas the gravity bottom|right, and the offset is (ox, oy), the calculated\nsurface position will be (x + ox, y + oy). The offset position of the\nsurface is the one used for constraint testing. See\nset_constraint_adjustment.\n\nAn example use case is placing a popup menu on top of a user interface\nelement, while aligning the user interface element of the parent surface\nwith some user interface element placed somewhere in the popup surface."]
        SetOffset { x: i32, y: i32 },
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
                name: "set_size",
                since: 1,
                signature: &[super::ArgumentType::Int, super::ArgumentType::Int],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_anchor_rect",
                since: 1,
                signature: &[
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_anchor",
                since: 1,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_gravity",
                since: 1,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_constraint_adjustment",
                since: 1,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_offset",
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
                Request::SetSize { .. } => 1,
                Request::SetAnchorRect { .. } => 2,
                Request::SetAnchor { .. } => 3,
                Request::SetGravity { .. } => 4,
                Request::SetConstraintAdjustment { .. } => 5,
                Request::SetOffset { .. } => 6,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::SetSize { .. } => 1,
                Request::SetAnchorRect { .. } => 1,
                Request::SetAnchor { .. } => 1,
                Request::SetGravity { .. } => 1,
                Request::SetConstraintAdjustment { .. } => 1,
                Request::SetOffset { .. } => 1,
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
                Request::SetSize { width, height } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::Int(width), Argument::Int(height),],
                },
                Request::SetAnchorRect {
                    x,
                    y,
                    width,
                    height,
                } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![
                        Argument::Int(x),
                        Argument::Int(y),
                        Argument::Int(width),
                        Argument::Int(height),
                    ],
                },
                Request::SetAnchor { anchor } => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: smallvec![Argument::Uint(anchor.to_raw()),],
                },
                Request::SetGravity { gravity } => Message {
                    sender_id: sender_id,
                    opcode: 4,
                    args: smallvec![Argument::Uint(gravity.to_raw()),],
                },
                Request::SetConstraintAdjustment {
                    constraint_adjustment,
                } => Message {
                    sender_id: sender_id,
                    opcode: 5,
                    args: smallvec![Argument::Uint(constraint_adjustment),],
                },
                Request::SetOffset { x, y } => Message {
                    sender_id: sender_id,
                    opcode: 6,
                    args: smallvec![Argument::Int(x), Argument::Int(y),],
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
                Request::SetSize { width, height } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = width;
                    _args_array[1].i = height;
                    f(1, &mut _args_array)
                }
                Request::SetAnchorRect {
                    x,
                    y,
                    width,
                    height,
                } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = x;
                    _args_array[1].i = y;
                    _args_array[2].i = width;
                    _args_array[3].i = height;
                    f(2, &mut _args_array)
                }
                Request::SetAnchor { anchor } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = anchor.to_raw();
                    f(3, &mut _args_array)
                }
                Request::SetGravity { gravity } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = gravity.to_raw();
                    f(4, &mut _args_array)
                }
                Request::SetConstraintAdjustment {
                    constraint_adjustment,
                } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = constraint_adjustment;
                    f(5, &mut _args_array)
                }
                Request::SetOffset { x, y } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = x;
                    _args_array[1].i = y;
                    f(6, &mut _args_array)
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
    pub struct XdgPositioner(Proxy<XdgPositioner>);
    impl AsRef<Proxy<XdgPositioner>> for XdgPositioner {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<XdgPositioner>> for XdgPositioner {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            XdgPositioner(value)
        }
    }
    impl From<XdgPositioner> for Proxy<XdgPositioner> {
        #[inline]
        fn from(value: XdgPositioner) -> Self {
            value.0
        }
    }
    impl Interface for XdgPositioner {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "xdg_positioner";
        const VERSION: u32 = 2;
        fn c_interface() -> *const wl_interface {
            unsafe { &xdg_positioner_interface }
        }
    }
    impl XdgPositioner {
        #[doc = "destroy the xdg_positioner object\n\nNotify the compositor that the xdg_positioner will no longer be used.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the size of the to-be positioned rectangle\n\nSet the size of the surface that is to be positioned with the positioner\nobject. The size is in surface-local coordinates and corresponds to the\nwindow geometry. See xdg_surface.set_window_geometry.\n\nIf a zero or negative size is set the invalid_input error is raised."]
        pub fn set_size(&self, width: i32, height: i32) -> () {
            let msg = Request::SetSize {
                width: width,
                height: height,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the anchor rectangle within the parent surface\n\nSpecify the anchor rectangle within the parent surface that the child\nsurface will be placed relative to. The rectangle is relative to the\nwindow geometry as defined by xdg_surface.set_window_geometry of the\nparent surface.\n\nWhen the xdg_positioner object is used to position a child surface, the\nanchor rectangle may not extend outside the window geometry of the\npositioned child's parent surface.\n\nIf a negative size is set the invalid_input error is raised."]
        pub fn set_anchor_rect(&self, x: i32, y: i32, width: i32, height: i32) -> () {
            let msg = Request::SetAnchorRect {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set anchor rectangle anchor\n\nDefines the anchor point for the anchor rectangle. The specified anchor\nis used derive an anchor point that the child surface will be\npositioned relative to. If a corner anchor is set (e.g. 'top_left' or\n'bottom_right'), the anchor point will be at the specified corner;\notherwise, the derived anchor point will be centered on the specified\nedge, or in the center of the anchor rectangle if no edge is specified."]
        pub fn set_anchor(&self, anchor: Anchor) -> () {
            let msg = Request::SetAnchor { anchor: anchor };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set child surface gravity\n\nDefines in what direction a surface should be positioned, relative to\nthe anchor point of the parent surface. If a corner gravity is\nspecified (e.g. 'bottom_right' or 'top_left'), then the child surface\nwill be placed towards the specified gravity; otherwise, the child\nsurface will be centered over the anchor point on any axis that had no\ngravity specified."]
        pub fn set_gravity(&self, gravity: Gravity) -> () {
            let msg = Request::SetGravity { gravity: gravity };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the adjustment to be done when constrained\n\nSpecify how the window should be positioned if the originally intended\nposition caused the surface to be constrained, meaning at least\npartially outside positioning boundaries set by the compositor. The\nadjustment is set by constructing a bitmask describing the adjustment to\nbe made when the surface is constrained on that axis.\n\nIf no bit for one axis is set, the compositor will assume that the child\nsurface should not change its position on that axis when constrained.\n\nIf more than one bit for one axis is set, the order of how adjustments\nare applied is specified in the corresponding adjustment descriptions.\n\nThe default adjustment is none."]
        pub fn set_constraint_adjustment(&self, constraint_adjustment: u32) -> () {
            let msg = Request::SetConstraintAdjustment {
                constraint_adjustment: constraint_adjustment,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set surface position offset\n\nSpecify the surface position offset relative to the position of the\nanchor on the anchor rectangle and the anchor on the surface. For\nexample if the anchor of the anchor rectangle is at (x, y), the surface\nhas the gravity bottom|right, and the offset is (ox, oy), the calculated\nsurface position will be (x + ox, y + oy). The offset position of the\nsurface is the one used for constraint testing. See\nset_constraint_adjustment.\n\nAn example use case is placing a popup menu on top of a user interface\nelement, while aligning the user interface element of the parent surface\nwith some user interface element placed somewhere in the popup surface."]
        pub fn set_offset(&self, x: i32, y: i32) -> () {
            let msg = Request::SetOffset { x: x, y: y };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_SIZE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_ANCHOR_RECT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_ANCHOR_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_GRAVITY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_CONSTRAINT_ADJUSTMENT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_OFFSET_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut xdg_positioner_requests: [wl_message; 7] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_size\0" as *const u8 as *const c_char,
            signature: b"ii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_anchor_rect\0" as *const u8 as *const c_char,
            signature: b"iiii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_anchor\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_gravity\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_constraint_adjustment\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_offset\0" as *const u8 as *const c_char,
            signature: b"ii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut xdg_positioner_interface: wl_interface = wl_interface {
        name: b"xdg_positioner\0" as *const u8 as *const c_char,
        version: 2,
        request_count: 7,
        requests: unsafe { &xdg_positioner_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "desktop user interface surface base interface\n\nAn interface that may be implemented by a wl_surface, for\nimplementations that provide a desktop-style user interface.\n\nIt provides a base set of functionality required to construct user\ninterface elements requiring management by the compositor, such as\ntoplevel windows, menus, etc. The types of functionality are split into\nxdg_surface roles.\n\nCreating an xdg_surface does not set the role for a wl_surface. In order\nto map an xdg_surface, the client must create a role-specific object\nusing, e.g., get_toplevel, get_popup. The wl_surface for any given\nxdg_surface can have at most one role, and may not be assigned any role\nnot based on xdg_surface.\n\nA role must be assigned before any other requests are made to the\nxdg_surface object.\n\nThe client must call wl_surface.commit on the corresponding wl_surface\nfor the xdg_surface state to take effect.\n\nCreating an xdg_surface from a wl_surface which has a buffer attached or\ncommitted is a client error, and any attempts by a client to attach or\nmanipulate a buffer prior to the first xdg_surface.configure call must\nalso be treated as errors.\n\nMapping an xdg_surface-based role surface is defined as making it\npossible for the surface to be shown by the compositor. Note that\na mapped surface is not guaranteed to be visible once it is mapped.\n\nFor an xdg_surface to be mapped by the compositor, the following\nconditions must be met:\n(1) the client has assigned an xdg_surface-based role to the surface\n(2) the client has set and committed the xdg_surface state and the\nrole-dependent state to the surface\n(3) the client has committed a buffer to the surface\n\nA newly-unmapped surface is considered to have met condition (1) out\nof the 3 required conditions for mapping a surface if its role surface\nhas not been destroyed."]
pub mod xdg_surface {
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
        NotConstructed = 1,
        AlreadyConstructed = 2,
        UnconfiguredBuffer = 3,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                1 => Some(Error::NotConstructed),
                2 => Some(Error::AlreadyConstructed),
                3 => Some(Error::UnconfiguredBuffer),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy the xdg_surface\n\nDestroy the xdg_surface object. An xdg_surface must only be destroyed\nafter its role object has been destroyed.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "assign the xdg_toplevel surface role\n\nThis creates an xdg_toplevel object for the given xdg_surface and gives\nthe associated wl_surface the xdg_toplevel role.\n\nSee the documentation of xdg_toplevel for more details about what an\nxdg_toplevel is and how it is used."]
        GetToplevel {},
        #[doc = "assign the xdg_popup surface role\n\nThis creates an xdg_popup object for the given xdg_surface and gives\nthe associated wl_surface the xdg_popup role.\n\nIf null is passed as a parent, a parent surface must be specified using\nsome other protocol, before committing the initial state.\n\nSee the documentation of xdg_popup for more details about what an\nxdg_popup is and how it is used."]
        GetPopup {
            parent: Option<super::xdg_surface::XdgSurface>,
            positioner: super::xdg_positioner::XdgPositioner,
        },
        #[doc = "set the new window geometry\n\nThe window geometry of a surface is its \"visible bounds\" from the\nuser's perspective. Client-side decorations often have invisible\nportions like drop-shadows which should be ignored for the\npurposes of aligning, placing and constraining windows.\n\nThe window geometry is double buffered, and will be applied at the\ntime wl_surface.commit of the corresponding wl_surface is called.\n\nWhen maintaining a position, the compositor should treat the (x, y)\ncoordinate of the window geometry as the top left corner of the window.\nA client changing the (x, y) window geometry coordinate should in\ngeneral not alter the position of the window.\n\nOnce the window geometry of the surface is set, it is not possible to\nunset it, and it will remain the same until set_window_geometry is\ncalled again, even if a new subsurface or buffer is attached.\n\nIf never set, the value is the full bounds of the surface,\nincluding any subsurfaces. This updates dynamically on every\ncommit. This unset is meant for extremely simple clients.\n\nThe arguments are given in the surface-local coordinate space of\nthe wl_surface associated with this xdg_surface.\n\nThe width and height must be greater than zero. Setting an invalid size\nwill raise an error. When applied, the effective window geometry will be\nthe set window geometry clamped to the bounding rectangle of the\ncombined geometry of the surface of the xdg_surface and the associated\nsubsurfaces."]
        SetWindowGeometry {
            x: i32,
            y: i32,
            width: i32,
            height: i32,
        },
        #[doc = "ack a configure event\n\nWhen a configure event is received, if a client commits the\nsurface in response to the configure event, then the client\nmust make an ack_configure request sometime before the commit\nrequest, passing along the serial of the configure event.\n\nFor instance, for toplevel surfaces the compositor might use this\ninformation to move a surface to the top left only when the client has\ndrawn itself for the maximized or fullscreen state.\n\nIf the client receives multiple configure events before it\ncan respond to one, it only has to ack the last configure event.\n\nA client is not required to commit immediately after sending\nan ack_configure request - it may even ack_configure several times\nbefore its next surface commit.\n\nA client may send multiple ack_configure requests before committing, but\nonly the last request sent before a commit indicates which configure\nevent the client really is responding to."]
        AckConfigure { serial: u32 },
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
                name: "get_toplevel",
                since: 1,
                signature: &[super::ArgumentType::NewId],
                destructor: false,
            },
            super::MessageDesc {
                name: "get_popup",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                    super::ArgumentType::Object,
                    super::ArgumentType::Object,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_window_geometry",
                since: 1,
                signature: &[
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "ack_configure",
                since: 1,
                signature: &[super::ArgumentType::Uint],
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
                Request::GetToplevel { .. } => 1,
                Request::GetPopup { .. } => 2,
                Request::SetWindowGeometry { .. } => 3,
                Request::AckConfigure { .. } => 4,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::GetToplevel { .. } => 1,
                Request::GetPopup { .. } => 1,
                Request::SetWindowGeometry { .. } => 1,
                Request::AckConfigure { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<super::xdg_toplevel::XdgToplevel>(
                    version,
                    meta.child(),
                )),
                2 => Some(Object::from_interface::<super::xdg_popup::XdgPopup>(
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
                Request::GetToplevel {} => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::NewId(0),],
                },
                Request::GetPopup { parent, positioner } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![
                        Argument::NewId(0),
                        Argument::Object(parent.map(|o| o.as_ref().id()).unwrap_or(0)),
                        Argument::Object(positioner.as_ref().id()),
                    ],
                },
                Request::SetWindowGeometry {
                    x,
                    y,
                    width,
                    height,
                } => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: smallvec![
                        Argument::Int(x),
                        Argument::Int(y),
                        Argument::Int(width),
                        Argument::Int(height),
                    ],
                },
                Request::AckConfigure { serial } => Message {
                    sender_id: sender_id,
                    opcode: 4,
                    args: smallvec![Argument::Uint(serial),],
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
                Request::GetToplevel {} => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    f(1, &mut _args_array)
                }
                Request::GetPopup { parent, positioner } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].o = parent
                        .map(|o| o.as_ref().c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    _args_array[2].o = positioner.as_ref().c_ptr() as *mut _;
                    f(2, &mut _args_array)
                }
                Request::SetWindowGeometry {
                    x,
                    y,
                    width,
                    height,
                } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = x;
                    _args_array[1].i = y;
                    _args_array[2].i = width;
                    _args_array[3].i = height;
                    f(3, &mut _args_array)
                }
                Request::AckConfigure { serial } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    f(4, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "suggest a surface change\n\nThe configure event marks the end of a configure sequence. A configure\nsequence is a set of one or more events configuring the state of the\nxdg_surface, including the final xdg_surface.configure event.\n\nWhere applicable, xdg_surface surface roles will during a configure\nsequence extend this event as a latched state sent as events before the\nxdg_surface.configure event. Such events should be considered to make up\na set of atomically applied configuration states, where the\nxdg_surface.configure commits the accumulated state.\n\nClients should arrange their surface for the new states, and then send\nan ack_configure request with the serial sent in this configure event at\nsome point before committing the new surface.\n\nIf the client receives multiple configure events before it can respond\nto one, it is free to discard all but the last event it received."]
        Configure { serial: u32 },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "configure",
            since: 1,
            signature: &[super::ArgumentType::Uint],
            destructor: false,
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::Configure { .. } => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Configure { .. } => 1,
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
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Configure {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
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
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Configure { serial: _args[0].u })
                }
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
    pub struct XdgSurface(Proxy<XdgSurface>);
    impl AsRef<Proxy<XdgSurface>> for XdgSurface {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<XdgSurface>> for XdgSurface {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            XdgSurface(value)
        }
    }
    impl From<XdgSurface> for Proxy<XdgSurface> {
        #[inline]
        fn from(value: XdgSurface) -> Self {
            value.0
        }
    }
    impl Interface for XdgSurface {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "xdg_surface";
        const VERSION: u32 = 2;
        fn c_interface() -> *const wl_interface {
            unsafe { &xdg_surface_interface }
        }
    }
    impl XdgSurface {
        #[doc = "destroy the xdg_surface\n\nDestroy the xdg_surface object. An xdg_surface must only be destroyed\nafter its role object has been destroyed.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "assign the xdg_toplevel surface role\n\nThis creates an xdg_toplevel object for the given xdg_surface and gives\nthe associated wl_surface the xdg_toplevel role.\n\nSee the documentation of xdg_toplevel for more details about what an\nxdg_toplevel is and how it is used."]
        pub fn get_toplevel(&self) -> Main<super::xdg_toplevel::XdgToplevel> {
            let msg = Request::GetToplevel {};
            self.0.send(msg, None).unwrap()
        }
        #[doc = "assign the xdg_popup surface role\n\nThis creates an xdg_popup object for the given xdg_surface and gives\nthe associated wl_surface the xdg_popup role.\n\nIf null is passed as a parent, a parent surface must be specified using\nsome other protocol, before committing the initial state.\n\nSee the documentation of xdg_popup for more details about what an\nxdg_popup is and how it is used."]
        pub fn get_popup(
            &self,
            parent: Option<&super::xdg_surface::XdgSurface>,
            positioner: &super::xdg_positioner::XdgPositioner,
        ) -> Main<super::xdg_popup::XdgPopup> {
            let msg = Request::GetPopup {
                parent: parent.map(|o| o.clone()),
                positioner: positioner.clone(),
            };
            self.0.send(msg, None).unwrap()
        }
        #[doc = "set the new window geometry\n\nThe window geometry of a surface is its \"visible bounds\" from the\nuser's perspective. Client-side decorations often have invisible\nportions like drop-shadows which should be ignored for the\npurposes of aligning, placing and constraining windows.\n\nThe window geometry is double buffered, and will be applied at the\ntime wl_surface.commit of the corresponding wl_surface is called.\n\nWhen maintaining a position, the compositor should treat the (x, y)\ncoordinate of the window geometry as the top left corner of the window.\nA client changing the (x, y) window geometry coordinate should in\ngeneral not alter the position of the window.\n\nOnce the window geometry of the surface is set, it is not possible to\nunset it, and it will remain the same until set_window_geometry is\ncalled again, even if a new subsurface or buffer is attached.\n\nIf never set, the value is the full bounds of the surface,\nincluding any subsurfaces. This updates dynamically on every\ncommit. This unset is meant for extremely simple clients.\n\nThe arguments are given in the surface-local coordinate space of\nthe wl_surface associated with this xdg_surface.\n\nThe width and height must be greater than zero. Setting an invalid size\nwill raise an error. When applied, the effective window geometry will be\nthe set window geometry clamped to the bounding rectangle of the\ncombined geometry of the surface of the xdg_surface and the associated\nsubsurfaces."]
        pub fn set_window_geometry(&self, x: i32, y: i32, width: i32, height: i32) -> () {
            let msg = Request::SetWindowGeometry {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "ack a configure event\n\nWhen a configure event is received, if a client commits the\nsurface in response to the configure event, then the client\nmust make an ack_configure request sometime before the commit\nrequest, passing along the serial of the configure event.\n\nFor instance, for toplevel surfaces the compositor might use this\ninformation to move a surface to the top left only when the client has\ndrawn itself for the maximized or fullscreen state.\n\nIf the client receives multiple configure events before it\ncan respond to one, it only has to ack the last configure event.\n\nA client is not required to commit immediately after sending\nan ack_configure request - it may even ack_configure several times\nbefore its next surface commit.\n\nA client may send multiple ack_configure requests before committing, but\nonly the last request sent before a commit indicates which configure\nevent the client really is responding to."]
        pub fn ack_configure(&self, serial: u32) -> () {
            let msg = Request::AckConfigure { serial: serial };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_TOPLEVEL_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_POPUP_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_WINDOW_GEOMETRY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_ACK_CONFIGURE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CONFIGURE_SINCE: u32 = 1u32;
    static mut xdg_surface_requests_get_toplevel_types: [*const wl_interface; 1] =
        [unsafe { &super::xdg_toplevel::xdg_toplevel_interface as *const wl_interface }];
    static mut xdg_surface_requests_get_popup_types: [*const wl_interface; 3] = [
        unsafe { &super::xdg_popup::xdg_popup_interface as *const wl_interface },
        unsafe { &super::xdg_surface::xdg_surface_interface as *const wl_interface },
        unsafe { &super::xdg_positioner::xdg_positioner_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut xdg_surface_requests: [wl_message; 5] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"get_toplevel\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &xdg_surface_requests_get_toplevel_types as *const _ },
        },
        wl_message {
            name: b"get_popup\0" as *const u8 as *const c_char,
            signature: b"n?oo\0" as *const u8 as *const c_char,
            types: unsafe { &xdg_surface_requests_get_popup_types as *const _ },
        },
        wl_message {
            name: b"set_window_geometry\0" as *const u8 as *const c_char,
            signature: b"iiii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"ack_configure\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut xdg_surface_events: [wl_message; 1] = [wl_message {
        name: b"configure\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut xdg_surface_interface: wl_interface = wl_interface {
        name: b"xdg_surface\0" as *const u8 as *const c_char,
        version: 2,
        request_count: 5,
        requests: unsafe { &xdg_surface_requests as *const _ },
        event_count: 1,
        events: unsafe { &xdg_surface_events as *const _ },
    };
}
#[doc = "toplevel surface\n\nThis interface defines an xdg_surface role which allows a surface to,\namong other things, set window-like properties such as maximize,\nfullscreen, and minimize, set application-specific metadata like title and\nid, and well as trigger user interactive operations such as interactive\nresize and move.\n\nUnmapping an xdg_toplevel means that the surface cannot be shown\nby the compositor until it is explicitly mapped again.\nAll active operations (e.g., move, resize) are canceled and all\nattributes (e.g. title, state, stacking, ...) are discarded for\nan xdg_toplevel surface when it is unmapped.\n\nAttaching a null buffer to a toplevel unmaps the surface."]
pub mod xdg_toplevel {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[doc = "edge values for resizing\n\nThese values are used to indicate which edge of a surface\nis being dragged in a resize operation."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum ResizeEdge {
        None = 0,
        Top = 1,
        Bottom = 2,
        Left = 4,
        TopLeft = 5,
        BottomLeft = 6,
        Right = 8,
        TopRight = 9,
        BottomRight = 10,
    }
    impl ResizeEdge {
        pub fn from_raw(n: u32) -> Option<ResizeEdge> {
            match n {
                0 => Some(ResizeEdge::None),
                1 => Some(ResizeEdge::Top),
                2 => Some(ResizeEdge::Bottom),
                4 => Some(ResizeEdge::Left),
                5 => Some(ResizeEdge::TopLeft),
                6 => Some(ResizeEdge::BottomLeft),
                8 => Some(ResizeEdge::Right),
                9 => Some(ResizeEdge::TopRight),
                10 => Some(ResizeEdge::BottomRight),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[doc = "types of state on the surface\n\nThe different state values used on the surface. This is designed for\nstate values like maximized, fullscreen. It is paired with the\nconfigure event to ensure that both the client and the compositor\nsetting the state can be synchronized.\n\nStates set in this way are double-buffered. They will get applied on\nthe next commit."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum State {
        #[doc = "the surface is maximized\n\nThe surface is maximized. The window geometry specified in the configure\nevent must be obeyed by the client.\n\nThe client should draw without shadow or other\ndecoration outside of the window geometry."]
        Maximized = 1,
        #[doc = "the surface is fullscreen\n\nThe surface is fullscreen. The window geometry specified in the\nconfigure event is a maximum; the client cannot resize beyond it. For\na surface to cover the whole fullscreened area, the geometry\ndimensions must be obeyed by the client. For more details, see\nxdg_toplevel.set_fullscreen."]
        Fullscreen = 2,
        #[doc = "the surface is being resized\n\nThe surface is being resized. The window geometry specified in the\nconfigure event is a maximum; the client cannot resize beyond it.\nClients that have aspect ratio or cell sizing configuration can use\na smaller size, however."]
        Resizing = 3,
        #[doc = "the surface is now activated\n\nClient window decorations should be painted as if the window is\nactive. Do not assume this means that the window actually has\nkeyboard or pointer focus."]
        Activated = 4,
        #[doc = "the surface is tiled\n\nThe window is currently in a tiled layout and the left edge is\nconsidered to be adjacent to another part of the tiling grid."]
        TiledLeft = 5,
        #[doc = "the surface is tiled\n\nThe window is currently in a tiled layout and the right edge is\nconsidered to be adjacent to another part of the tiling grid."]
        TiledRight = 6,
        #[doc = "the surface is tiled\n\nThe window is currently in a tiled layout and the top edge is\nconsidered to be adjacent to another part of the tiling grid."]
        TiledTop = 7,
        #[doc = "the surface is tiled\n\nThe window is currently in a tiled layout and the bottom edge is\nconsidered to be adjacent to another part of the tiling grid."]
        TiledBottom = 8,
    }
    impl State {
        pub fn from_raw(n: u32) -> Option<State> {
            match n {
                1 => Some(State::Maximized),
                2 => Some(State::Fullscreen),
                3 => Some(State::Resizing),
                4 => Some(State::Activated),
                5 => Some(State::TiledLeft),
                6 => Some(State::TiledRight),
                7 => Some(State::TiledTop),
                8 => Some(State::TiledBottom),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy the xdg_toplevel\n\nThis request destroys the role surface and unmaps the surface;\nsee \"Unmapping\" behavior in interface section for details.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "set the parent of this surface\n\nSet the \"parent\" of this surface. This surface should be stacked\nabove the parent surface and all other ancestor surfaces.\n\nParent windows should be set on dialogs, toolboxes, or other\n\"auxiliary\" surfaces, so that the parent is raised when the dialog\nis raised.\n\nSetting a null parent for a child window removes any parent-child\nrelationship for the child. Setting a null parent for a window which\ncurrently has no parent is a no-op.\n\nIf the parent is unmapped then its children are managed as\nthough the parent of the now-unmapped parent has become the\nparent of this surface. If no parent exists for the now-unmapped\nparent then the children are managed as though they have no\nparent surface."]
        SetParent {
            parent: Option<super::xdg_toplevel::XdgToplevel>,
        },
        #[doc = "set surface title\n\nSet a short title for the surface.\n\nThis string may be used to identify the surface in a task bar,\nwindow list, or other user interface elements provided by the\ncompositor.\n\nThe string must be encoded in UTF-8."]
        SetTitle { title: String },
        #[doc = "set application ID\n\nSet an application identifier for the surface.\n\nThe app ID identifies the general class of applications to which\nthe surface belongs. The compositor can use this to group multiple\nsurfaces together, or to determine how to launch a new application.\n\nFor D-Bus activatable applications, the app ID is used as the D-Bus\nservice name.\n\nThe compositor shell will try to group application surfaces together\nby their app ID. As a best practice, it is suggested to select app\nID's that match the basename of the application's .desktop file.\nFor example, \"org.freedesktop.FooViewer\" where the .desktop file is\n\"org.freedesktop.FooViewer.desktop\".\n\nLike other properties, a set_app_id request can be sent after the\nxdg_toplevel has been mapped to update the property.\n\nSee the desktop-entry specification [0] for more details on\napplication identifiers and how they relate to well-known D-Bus\nnames and .desktop files.\n\n[0] http://standards.freedesktop.org/desktop-entry-spec/"]
        SetAppId { app_id: String },
        #[doc = "show the window menu\n\nClients implementing client-side decorations might want to show\na context menu when right-clicking on the decorations, giving the\nuser a menu that they can use to maximize or minimize the window.\n\nThis request asks the compositor to pop up such a window menu at\nthe given position, relative to the local surface coordinates of\nthe parent surface. There are no guarantees as to what menu items\nthe window menu contains.\n\nThis request must be used in response to some sort of user action\nlike a button press, key press, or touch down event."]
        ShowWindowMenu {
            seat: super::wl_seat::WlSeat,
            serial: u32,
            x: i32,
            y: i32,
        },
        #[doc = "start an interactive move\n\nStart an interactive, user-driven move of the surface.\n\nThis request must be used in response to some sort of user action\nlike a button press, key press, or touch down event. The passed\nserial is used to determine the type of interactive move (touch,\npointer, etc).\n\nThe server may ignore move requests depending on the state of\nthe surface (e.g. fullscreen or maximized), or if the passed serial\nis no longer valid.\n\nIf triggered, the surface will lose the focus of the device\n(wl_pointer, wl_touch, etc) used for the move. It is up to the\ncompositor to visually indicate that the move is taking place, such as\nupdating a pointer cursor, during the move. There is no guarantee\nthat the device focus will return when the move is completed."]
        Move {
            seat: super::wl_seat::WlSeat,
            serial: u32,
        },
        #[doc = "start an interactive resize\n\nStart a user-driven, interactive resize of the surface.\n\nThis request must be used in response to some sort of user action\nlike a button press, key press, or touch down event. The passed\nserial is used to determine the type of interactive resize (touch,\npointer, etc).\n\nThe server may ignore resize requests depending on the state of\nthe surface (e.g. fullscreen or maximized).\n\nIf triggered, the client will receive configure events with the\n\"resize\" state enum value and the expected sizes. See the \"resize\"\nenum value for more details about what is required. The client\nmust also acknowledge configure events using \"ack_configure\". After\nthe resize is completed, the client will receive another \"configure\"\nevent without the resize state.\n\nIf triggered, the surface also will lose the focus of the device\n(wl_pointer, wl_touch, etc) used for the resize. It is up to the\ncompositor to visually indicate that the resize is taking place,\nsuch as updating a pointer cursor, during the resize. There is no\nguarantee that the device focus will return when the resize is\ncompleted.\n\nThe edges parameter specifies how the surface should be resized,\nand is one of the values of the resize_edge enum. The compositor\nmay use this information to update the surface position for\nexample when dragging the top left corner. The compositor may also\nuse this information to adapt its behavior, e.g. choose an\nappropriate cursor image."]
        Resize {
            seat: super::wl_seat::WlSeat,
            serial: u32,
            edges: ResizeEdge,
        },
        #[doc = "set the maximum size\n\nSet a maximum size for the window.\n\nThe client can specify a maximum size so that the compositor does\nnot try to configure the window beyond this size.\n\nThe width and height arguments are in window geometry coordinates.\nSee xdg_surface.set_window_geometry.\n\nValues set in this way are double-buffered. They will get applied\non the next commit.\n\nThe compositor can use this information to allow or disallow\ndifferent states like maximize or fullscreen and draw accurate\nanimations.\n\nSimilarly, a tiling window manager may use this information to\nplace and resize client windows in a more effective way.\n\nThe client should not rely on the compositor to obey the maximum\nsize. The compositor may decide to ignore the values set by the\nclient and request a larger size.\n\nIf never set, or a value of zero in the request, means that the\nclient has no expected maximum size in the given dimension.\nAs a result, a client wishing to reset the maximum size\nto an unspecified state can use zero for width and height in the\nrequest.\n\nRequesting a maximum size to be smaller than the minimum size of\na surface is illegal and will result in a protocol error.\n\nThe width and height must be greater than or equal to zero. Using\nstrictly negative values for width and height will result in a\nprotocol error."]
        SetMaxSize { width: i32, height: i32 },
        #[doc = "set the minimum size\n\nSet a minimum size for the window.\n\nThe client can specify a minimum size so that the compositor does\nnot try to configure the window below this size.\n\nThe width and height arguments are in window geometry coordinates.\nSee xdg_surface.set_window_geometry.\n\nValues set in this way are double-buffered. They will get applied\non the next commit.\n\nThe compositor can use this information to allow or disallow\ndifferent states like maximize or fullscreen and draw accurate\nanimations.\n\nSimilarly, a tiling window manager may use this information to\nplace and resize client windows in a more effective way.\n\nThe client should not rely on the compositor to obey the minimum\nsize. The compositor may decide to ignore the values set by the\nclient and request a smaller size.\n\nIf never set, or a value of zero in the request, means that the\nclient has no expected minimum size in the given dimension.\nAs a result, a client wishing to reset the minimum size\nto an unspecified state can use zero for width and height in the\nrequest.\n\nRequesting a minimum size to be larger than the maximum size of\na surface is illegal and will result in a protocol error.\n\nThe width and height must be greater than or equal to zero. Using\nstrictly negative values for width and height will result in a\nprotocol error."]
        SetMinSize { width: i32, height: i32 },
        #[doc = "maximize the window\n\nMaximize the surface.\n\nAfter requesting that the surface should be maximized, the compositor\nwill respond by emitting a configure event. Whether this configure\nactually sets the window maximized is subject to compositor policies.\nThe client must then update its content, drawing in the configured\nstate. The client must also acknowledge the configure when committing\nthe new content (see ack_configure).\n\nIt is up to the compositor to decide how and where to maximize the\nsurface, for example which output and what region of the screen should\nbe used.\n\nIf the surface was already maximized, the compositor will still emit\na configure event with the \"maximized\" state.\n\nIf the surface is in a fullscreen state, this request has no direct\neffect. It may alter the state the surface is returned to when\nunmaximized unless overridden by the compositor."]
        SetMaximized,
        #[doc = "unmaximize the window\n\nUnmaximize the surface.\n\nAfter requesting that the surface should be unmaximized, the compositor\nwill respond by emitting a configure event. Whether this actually\nun-maximizes the window is subject to compositor policies.\nIf available and applicable, the compositor will include the window\ngeometry dimensions the window had prior to being maximized in the\nconfigure event. The client must then update its content, drawing it in\nthe configured state. The client must also acknowledge the configure\nwhen committing the new content (see ack_configure).\n\nIt is up to the compositor to position the surface after it was\nunmaximized; usually the position the surface had before maximizing, if\napplicable.\n\nIf the surface was already not maximized, the compositor will still\nemit a configure event without the \"maximized\" state.\n\nIf the surface is in a fullscreen state, this request has no direct\neffect. It may alter the state the surface is returned to when\nunmaximized unless overridden by the compositor."]
        UnsetMaximized,
        #[doc = "set the window as fullscreen on an output\n\nMake the surface fullscreen.\n\nAfter requesting that the surface should be fullscreened, the\ncompositor will respond by emitting a configure event. Whether the\nclient is actually put into a fullscreen state is subject to compositor\npolicies. The client must also acknowledge the configure when\ncommitting the new content (see ack_configure).\n\nThe output passed by the request indicates the client's preference as\nto which display it should be set fullscreen on. If this value is NULL,\nit's up to the compositor to choose which display will be used to map\nthis surface.\n\nIf the surface doesn't cover the whole output, the compositor will\nposition the surface in the center of the output and compensate with\nwith border fill covering the rest of the output. The content of the\nborder fill is undefined, but should be assumed to be in some way that\nattempts to blend into the surrounding area (e.g. solid black).\n\nIf the fullscreened surface is not opaque, the compositor must make\nsure that other screen content not part of the same surface tree (made\nup of subsurfaces, popups or similarly coupled surfaces) are not\nvisible below the fullscreened surface."]
        SetFullscreen {
            output: Option<super::wl_output::WlOutput>,
        },
        #[doc = "unset the window as fullscreen\n\nMake the surface no longer fullscreen.\n\nAfter requesting that the surface should be unfullscreened, the\ncompositor will respond by emitting a configure event.\nWhether this actually removes the fullscreen state of the client is\nsubject to compositor policies.\n\nMaking a surface unfullscreen sets states for the surface based on the following:\n* the state(s) it may have had before becoming fullscreen\n* any state(s) decided by the compositor\n* any state(s) requested by the client while the surface was fullscreen\n\nThe compositor may include the previous window geometry dimensions in\nthe configure event, if applicable.\n\nThe client must also acknowledge the configure when committing the new\ncontent (see ack_configure)."]
        UnsetFullscreen,
        #[doc = "set the window as minimized\n\nRequest that the compositor minimize your surface. There is no\nway to know if the surface is currently minimized, nor is there\nany way to unset minimization on this surface.\n\nIf you are looking to throttle redrawing when minimized, please\ninstead use the wl_surface.frame event for this, as this will\nalso work with live previews on windows in Alt-Tab, Expose or\nsimilar compositor features."]
        SetMinimized,
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
                name: "set_parent",
                since: 1,
                signature: &[super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_title",
                since: 1,
                signature: &[super::ArgumentType::Str],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_app_id",
                since: 1,
                signature: &[super::ArgumentType::Str],
                destructor: false,
            },
            super::MessageDesc {
                name: "show_window_menu",
                since: 1,
                signature: &[
                    super::ArgumentType::Object,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "move",
                since: 1,
                signature: &[super::ArgumentType::Object, super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "resize",
                since: 1,
                signature: &[
                    super::ArgumentType::Object,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_max_size",
                since: 1,
                signature: &[super::ArgumentType::Int, super::ArgumentType::Int],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_min_size",
                since: 1,
                signature: &[super::ArgumentType::Int, super::ArgumentType::Int],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_maximized",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "unset_maximized",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_fullscreen",
                since: 1,
                signature: &[super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "unset_fullscreen",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_minimized",
                since: 1,
                signature: &[],
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
                Request::SetParent { .. } => 1,
                Request::SetTitle { .. } => 2,
                Request::SetAppId { .. } => 3,
                Request::ShowWindowMenu { .. } => 4,
                Request::Move { .. } => 5,
                Request::Resize { .. } => 6,
                Request::SetMaxSize { .. } => 7,
                Request::SetMinSize { .. } => 8,
                Request::SetMaximized => 9,
                Request::UnsetMaximized => 10,
                Request::SetFullscreen { .. } => 11,
                Request::UnsetFullscreen => 12,
                Request::SetMinimized => 13,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::SetParent { .. } => 1,
                Request::SetTitle { .. } => 1,
                Request::SetAppId { .. } => 1,
                Request::ShowWindowMenu { .. } => 1,
                Request::Move { .. } => 1,
                Request::Resize { .. } => 1,
                Request::SetMaxSize { .. } => 1,
                Request::SetMinSize { .. } => 1,
                Request::SetMaximized => 1,
                Request::UnsetMaximized => 1,
                Request::SetFullscreen { .. } => 1,
                Request::UnsetFullscreen => 1,
                Request::SetMinimized => 1,
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
                Request::SetParent { parent } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::Object(
                        parent.map(|o| o.as_ref().id()).unwrap_or(0)
                    ),],
                },
                Request::SetTitle { title } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![Argument::Str(Box::new(unsafe {
                        ::std::ffi::CString::from_vec_unchecked(title.into())
                    })),],
                },
                Request::SetAppId { app_id } => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: smallvec![Argument::Str(Box::new(unsafe {
                        ::std::ffi::CString::from_vec_unchecked(app_id.into())
                    })),],
                },
                Request::ShowWindowMenu { seat, serial, x, y } => Message {
                    sender_id: sender_id,
                    opcode: 4,
                    args: smallvec![
                        Argument::Object(seat.as_ref().id()),
                        Argument::Uint(serial),
                        Argument::Int(x),
                        Argument::Int(y),
                    ],
                },
                Request::Move { seat, serial } => Message {
                    sender_id: sender_id,
                    opcode: 5,
                    args: smallvec![Argument::Object(seat.as_ref().id()), Argument::Uint(serial),],
                },
                Request::Resize {
                    seat,
                    serial,
                    edges,
                } => Message {
                    sender_id: sender_id,
                    opcode: 6,
                    args: smallvec![
                        Argument::Object(seat.as_ref().id()),
                        Argument::Uint(serial),
                        Argument::Uint(edges.to_raw()),
                    ],
                },
                Request::SetMaxSize { width, height } => Message {
                    sender_id: sender_id,
                    opcode: 7,
                    args: smallvec![Argument::Int(width), Argument::Int(height),],
                },
                Request::SetMinSize { width, height } => Message {
                    sender_id: sender_id,
                    opcode: 8,
                    args: smallvec![Argument::Int(width), Argument::Int(height),],
                },
                Request::SetMaximized => Message {
                    sender_id: sender_id,
                    opcode: 9,
                    args: smallvec![],
                },
                Request::UnsetMaximized => Message {
                    sender_id: sender_id,
                    opcode: 10,
                    args: smallvec![],
                },
                Request::SetFullscreen { output } => Message {
                    sender_id: sender_id,
                    opcode: 11,
                    args: smallvec![Argument::Object(
                        output.map(|o| o.as_ref().id()).unwrap_or(0)
                    ),],
                },
                Request::UnsetFullscreen => Message {
                    sender_id: sender_id,
                    opcode: 12,
                    args: smallvec![],
                },
                Request::SetMinimized => Message {
                    sender_id: sender_id,
                    opcode: 13,
                    args: smallvec![],
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
                Request::SetParent { parent } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = parent
                        .map(|o| o.as_ref().c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    f(1, &mut _args_array)
                }
                Request::SetTitle { title } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(title).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    f(2, &mut _args_array)
                }
                Request::SetAppId { app_id } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(app_id).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    f(3, &mut _args_array)
                }
                Request::ShowWindowMenu { seat, serial, x, y } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.as_ref().c_ptr() as *mut _;
                    _args_array[1].u = serial;
                    _args_array[2].i = x;
                    _args_array[3].i = y;
                    f(4, &mut _args_array)
                }
                Request::Move { seat, serial } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.as_ref().c_ptr() as *mut _;
                    _args_array[1].u = serial;
                    f(5, &mut _args_array)
                }
                Request::Resize {
                    seat,
                    serial,
                    edges,
                } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.as_ref().c_ptr() as *mut _;
                    _args_array[1].u = serial;
                    _args_array[2].u = edges.to_raw();
                    f(6, &mut _args_array)
                }
                Request::SetMaxSize { width, height } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = width;
                    _args_array[1].i = height;
                    f(7, &mut _args_array)
                }
                Request::SetMinSize { width, height } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = width;
                    _args_array[1].i = height;
                    f(8, &mut _args_array)
                }
                Request::SetMaximized => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(9, &mut _args_array)
                }
                Request::UnsetMaximized => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(10, &mut _args_array)
                }
                Request::SetFullscreen { output } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = output
                        .map(|o| o.as_ref().c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    f(11, &mut _args_array)
                }
                Request::UnsetFullscreen => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(12, &mut _args_array)
                }
                Request::SetMinimized => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(13, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "suggest a surface change\n\nThis configure event asks the client to resize its toplevel surface or\nto change its state. The configured state should not be applied\nimmediately. See xdg_surface.configure for details.\n\nThe width and height arguments specify a hint to the window\nabout how its surface should be resized in window geometry\ncoordinates. See set_window_geometry.\n\nIf the width or height arguments are zero, it means the client\nshould decide its own window dimension. This may happen when the\ncompositor needs to configure the state of the surface but doesn't\nhave any information about any previous or expected dimension.\n\nThe states listed in the event specify how the width/height\narguments should be interpreted, and possibly how it should be\ndrawn.\n\nClients must send an ack_configure in response to this event. See\nxdg_surface.configure and xdg_surface.ack_configure for details."]
        Configure {
            width: i32,
            height: i32,
            states: Vec<u8>,
        },
        #[doc = "surface wants to be closed\n\nThe close event is sent by the compositor when the user\nwants the surface to be closed. This should be equivalent to\nthe user clicking the close button in client-side decorations,\nif your application has any.\n\nThis is only a request that the user intends to close the\nwindow. The client may choose to ignore this request, or show\na dialog to ask the user to save their data, etc."]
        Close,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "configure",
                since: 1,
                signature: &[
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Array,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "close",
                since: 1,
                signature: &[],
                destructor: false,
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::Configure { .. } => 0,
                Event::Close => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Configure { .. } => 1,
                Event::Close => 1,
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
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Configure {
                        width: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        height: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        states: {
                            if let Some(Argument::Array(val)) = args.next() {
                                *val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => Ok(Event::Close),
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
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Configure {
                        width: _args[0].i,
                        height: _args[1].i,
                        states: {
                            let array = &*_args[2].a;
                            ::std::slice::from_raw_parts(array.data as *const u8, array.size)
                                .to_owned()
                        },
                    })
                }
                1 => Ok(Event::Close),
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
    pub struct XdgToplevel(Proxy<XdgToplevel>);
    impl AsRef<Proxy<XdgToplevel>> for XdgToplevel {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<XdgToplevel>> for XdgToplevel {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            XdgToplevel(value)
        }
    }
    impl From<XdgToplevel> for Proxy<XdgToplevel> {
        #[inline]
        fn from(value: XdgToplevel) -> Self {
            value.0
        }
    }
    impl Interface for XdgToplevel {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "xdg_toplevel";
        const VERSION: u32 = 2;
        fn c_interface() -> *const wl_interface {
            unsafe { &xdg_toplevel_interface }
        }
    }
    impl XdgToplevel {
        #[doc = "destroy the xdg_toplevel\n\nThis request destroys the role surface and unmaps the surface;\nsee \"Unmapping\" behavior in interface section for details.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the parent of this surface\n\nSet the \"parent\" of this surface. This surface should be stacked\nabove the parent surface and all other ancestor surfaces.\n\nParent windows should be set on dialogs, toolboxes, or other\n\"auxiliary\" surfaces, so that the parent is raised when the dialog\nis raised.\n\nSetting a null parent for a child window removes any parent-child\nrelationship for the child. Setting a null parent for a window which\ncurrently has no parent is a no-op.\n\nIf the parent is unmapped then its children are managed as\nthough the parent of the now-unmapped parent has become the\nparent of this surface. If no parent exists for the now-unmapped\nparent then the children are managed as though they have no\nparent surface."]
        pub fn set_parent(&self, parent: Option<&super::xdg_toplevel::XdgToplevel>) -> () {
            let msg = Request::SetParent {
                parent: parent.map(|o| o.clone()),
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set surface title\n\nSet a short title for the surface.\n\nThis string may be used to identify the surface in a task bar,\nwindow list, or other user interface elements provided by the\ncompositor.\n\nThe string must be encoded in UTF-8."]
        pub fn set_title(&self, title: String) -> () {
            let msg = Request::SetTitle { title: title };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set application ID\n\nSet an application identifier for the surface.\n\nThe app ID identifies the general class of applications to which\nthe surface belongs. The compositor can use this to group multiple\nsurfaces together, or to determine how to launch a new application.\n\nFor D-Bus activatable applications, the app ID is used as the D-Bus\nservice name.\n\nThe compositor shell will try to group application surfaces together\nby their app ID. As a best practice, it is suggested to select app\nID's that match the basename of the application's .desktop file.\nFor example, \"org.freedesktop.FooViewer\" where the .desktop file is\n\"org.freedesktop.FooViewer.desktop\".\n\nLike other properties, a set_app_id request can be sent after the\nxdg_toplevel has been mapped to update the property.\n\nSee the desktop-entry specification [0] for more details on\napplication identifiers and how they relate to well-known D-Bus\nnames and .desktop files.\n\n[0] http://standards.freedesktop.org/desktop-entry-spec/"]
        pub fn set_app_id(&self, app_id: String) -> () {
            let msg = Request::SetAppId { app_id: app_id };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "show the window menu\n\nClients implementing client-side decorations might want to show\na context menu when right-clicking on the decorations, giving the\nuser a menu that they can use to maximize or minimize the window.\n\nThis request asks the compositor to pop up such a window menu at\nthe given position, relative to the local surface coordinates of\nthe parent surface. There are no guarantees as to what menu items\nthe window menu contains.\n\nThis request must be used in response to some sort of user action\nlike a button press, key press, or touch down event."]
        pub fn show_window_menu(
            &self,
            seat: &super::wl_seat::WlSeat,
            serial: u32,
            x: i32,
            y: i32,
        ) -> () {
            let msg = Request::ShowWindowMenu {
                seat: seat.clone(),
                serial: serial,
                x: x,
                y: y,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "start an interactive move\n\nStart an interactive, user-driven move of the surface.\n\nThis request must be used in response to some sort of user action\nlike a button press, key press, or touch down event. The passed\nserial is used to determine the type of interactive move (touch,\npointer, etc).\n\nThe server may ignore move requests depending on the state of\nthe surface (e.g. fullscreen or maximized), or if the passed serial\nis no longer valid.\n\nIf triggered, the surface will lose the focus of the device\n(wl_pointer, wl_touch, etc) used for the move. It is up to the\ncompositor to visually indicate that the move is taking place, such as\nupdating a pointer cursor, during the move. There is no guarantee\nthat the device focus will return when the move is completed."]
        pub fn _move(&self, seat: &super::wl_seat::WlSeat, serial: u32) -> () {
            let msg = Request::Move {
                seat: seat.clone(),
                serial: serial,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "start an interactive resize\n\nStart a user-driven, interactive resize of the surface.\n\nThis request must be used in response to some sort of user action\nlike a button press, key press, or touch down event. The passed\nserial is used to determine the type of interactive resize (touch,\npointer, etc).\n\nThe server may ignore resize requests depending on the state of\nthe surface (e.g. fullscreen or maximized).\n\nIf triggered, the client will receive configure events with the\n\"resize\" state enum value and the expected sizes. See the \"resize\"\nenum value for more details about what is required. The client\nmust also acknowledge configure events using \"ack_configure\". After\nthe resize is completed, the client will receive another \"configure\"\nevent without the resize state.\n\nIf triggered, the surface also will lose the focus of the device\n(wl_pointer, wl_touch, etc) used for the resize. It is up to the\ncompositor to visually indicate that the resize is taking place,\nsuch as updating a pointer cursor, during the resize. There is no\nguarantee that the device focus will return when the resize is\ncompleted.\n\nThe edges parameter specifies how the surface should be resized,\nand is one of the values of the resize_edge enum. The compositor\nmay use this information to update the surface position for\nexample when dragging the top left corner. The compositor may also\nuse this information to adapt its behavior, e.g. choose an\nappropriate cursor image."]
        pub fn resize(&self, seat: &super::wl_seat::WlSeat, serial: u32, edges: ResizeEdge) -> () {
            let msg = Request::Resize {
                seat: seat.clone(),
                serial: serial,
                edges: edges,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the maximum size\n\nSet a maximum size for the window.\n\nThe client can specify a maximum size so that the compositor does\nnot try to configure the window beyond this size.\n\nThe width and height arguments are in window geometry coordinates.\nSee xdg_surface.set_window_geometry.\n\nValues set in this way are double-buffered. They will get applied\non the next commit.\n\nThe compositor can use this information to allow or disallow\ndifferent states like maximize or fullscreen and draw accurate\nanimations.\n\nSimilarly, a tiling window manager may use this information to\nplace and resize client windows in a more effective way.\n\nThe client should not rely on the compositor to obey the maximum\nsize. The compositor may decide to ignore the values set by the\nclient and request a larger size.\n\nIf never set, or a value of zero in the request, means that the\nclient has no expected maximum size in the given dimension.\nAs a result, a client wishing to reset the maximum size\nto an unspecified state can use zero for width and height in the\nrequest.\n\nRequesting a maximum size to be smaller than the minimum size of\na surface is illegal and will result in a protocol error.\n\nThe width and height must be greater than or equal to zero. Using\nstrictly negative values for width and height will result in a\nprotocol error."]
        pub fn set_max_size(&self, width: i32, height: i32) -> () {
            let msg = Request::SetMaxSize {
                width: width,
                height: height,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the minimum size\n\nSet a minimum size for the window.\n\nThe client can specify a minimum size so that the compositor does\nnot try to configure the window below this size.\n\nThe width and height arguments are in window geometry coordinates.\nSee xdg_surface.set_window_geometry.\n\nValues set in this way are double-buffered. They will get applied\non the next commit.\n\nThe compositor can use this information to allow or disallow\ndifferent states like maximize or fullscreen and draw accurate\nanimations.\n\nSimilarly, a tiling window manager may use this information to\nplace and resize client windows in a more effective way.\n\nThe client should not rely on the compositor to obey the minimum\nsize. The compositor may decide to ignore the values set by the\nclient and request a smaller size.\n\nIf never set, or a value of zero in the request, means that the\nclient has no expected minimum size in the given dimension.\nAs a result, a client wishing to reset the minimum size\nto an unspecified state can use zero for width and height in the\nrequest.\n\nRequesting a minimum size to be larger than the maximum size of\na surface is illegal and will result in a protocol error.\n\nThe width and height must be greater than or equal to zero. Using\nstrictly negative values for width and height will result in a\nprotocol error."]
        pub fn set_min_size(&self, width: i32, height: i32) -> () {
            let msg = Request::SetMinSize {
                width: width,
                height: height,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "maximize the window\n\nMaximize the surface.\n\nAfter requesting that the surface should be maximized, the compositor\nwill respond by emitting a configure event. Whether this configure\nactually sets the window maximized is subject to compositor policies.\nThe client must then update its content, drawing in the configured\nstate. The client must also acknowledge the configure when committing\nthe new content (see ack_configure).\n\nIt is up to the compositor to decide how and where to maximize the\nsurface, for example which output and what region of the screen should\nbe used.\n\nIf the surface was already maximized, the compositor will still emit\na configure event with the \"maximized\" state.\n\nIf the surface is in a fullscreen state, this request has no direct\neffect. It may alter the state the surface is returned to when\nunmaximized unless overridden by the compositor."]
        pub fn set_maximized(&self) -> () {
            let msg = Request::SetMaximized;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "unmaximize the window\n\nUnmaximize the surface.\n\nAfter requesting that the surface should be unmaximized, the compositor\nwill respond by emitting a configure event. Whether this actually\nun-maximizes the window is subject to compositor policies.\nIf available and applicable, the compositor will include the window\ngeometry dimensions the window had prior to being maximized in the\nconfigure event. The client must then update its content, drawing it in\nthe configured state. The client must also acknowledge the configure\nwhen committing the new content (see ack_configure).\n\nIt is up to the compositor to position the surface after it was\nunmaximized; usually the position the surface had before maximizing, if\napplicable.\n\nIf the surface was already not maximized, the compositor will still\nemit a configure event without the \"maximized\" state.\n\nIf the surface is in a fullscreen state, this request has no direct\neffect. It may alter the state the surface is returned to when\nunmaximized unless overridden by the compositor."]
        pub fn unset_maximized(&self) -> () {
            let msg = Request::UnsetMaximized;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the window as fullscreen on an output\n\nMake the surface fullscreen.\n\nAfter requesting that the surface should be fullscreened, the\ncompositor will respond by emitting a configure event. Whether the\nclient is actually put into a fullscreen state is subject to compositor\npolicies. The client must also acknowledge the configure when\ncommitting the new content (see ack_configure).\n\nThe output passed by the request indicates the client's preference as\nto which display it should be set fullscreen on. If this value is NULL,\nit's up to the compositor to choose which display will be used to map\nthis surface.\n\nIf the surface doesn't cover the whole output, the compositor will\nposition the surface in the center of the output and compensate with\nwith border fill covering the rest of the output. The content of the\nborder fill is undefined, but should be assumed to be in some way that\nattempts to blend into the surrounding area (e.g. solid black).\n\nIf the fullscreened surface is not opaque, the compositor must make\nsure that other screen content not part of the same surface tree (made\nup of subsurfaces, popups or similarly coupled surfaces) are not\nvisible below the fullscreened surface."]
        pub fn set_fullscreen(&self, output: Option<&super::wl_output::WlOutput>) -> () {
            let msg = Request::SetFullscreen {
                output: output.map(|o| o.clone()),
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "unset the window as fullscreen\n\nMake the surface no longer fullscreen.\n\nAfter requesting that the surface should be unfullscreened, the\ncompositor will respond by emitting a configure event.\nWhether this actually removes the fullscreen state of the client is\nsubject to compositor policies.\n\nMaking a surface unfullscreen sets states for the surface based on the following:\n* the state(s) it may have had before becoming fullscreen\n* any state(s) decided by the compositor\n* any state(s) requested by the client while the surface was fullscreen\n\nThe compositor may include the previous window geometry dimensions in\nthe configure event, if applicable.\n\nThe client must also acknowledge the configure when committing the new\ncontent (see ack_configure)."]
        pub fn unset_fullscreen(&self) -> () {
            let msg = Request::UnsetFullscreen;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the window as minimized\n\nRequest that the compositor minimize your surface. There is no\nway to know if the surface is currently minimized, nor is there\nany way to unset minimization on this surface.\n\nIf you are looking to throttle redrawing when minimized, please\ninstead use the wl_surface.frame event for this, as this will\nalso work with live previews on windows in Alt-Tab, Expose or\nsimilar compositor features."]
        pub fn set_minimized(&self) -> () {
            let msg = Request::SetMinimized;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_PARENT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_TITLE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_APP_ID_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SHOW_WINDOW_MENU_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_MOVE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_RESIZE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_MAX_SIZE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_MIN_SIZE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_MAXIMIZED_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_UNSET_MAXIMIZED_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_FULLSCREEN_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_UNSET_FULLSCREEN_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_MINIMIZED_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CONFIGURE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CLOSE_SINCE: u32 = 1u32;
    static mut xdg_toplevel_requests_set_parent_types: [*const wl_interface; 1] =
        [unsafe { &super::xdg_toplevel::xdg_toplevel_interface as *const wl_interface }];
    static mut xdg_toplevel_requests_show_window_menu_types: [*const wl_interface; 4] = [
        unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    static mut xdg_toplevel_requests_move_types: [*const wl_interface; 2] = [
        unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
    ];
    static mut xdg_toplevel_requests_resize_types: [*const wl_interface; 3] = [
        unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    static mut xdg_toplevel_requests_set_fullscreen_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_output::wl_output_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut xdg_toplevel_requests: [wl_message; 14] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_parent\0" as *const u8 as *const c_char,
            signature: b"?o\0" as *const u8 as *const c_char,
            types: unsafe { &xdg_toplevel_requests_set_parent_types as *const _ },
        },
        wl_message {
            name: b"set_title\0" as *const u8 as *const c_char,
            signature: b"s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_app_id\0" as *const u8 as *const c_char,
            signature: b"s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"show_window_menu\0" as *const u8 as *const c_char,
            signature: b"ouii\0" as *const u8 as *const c_char,
            types: unsafe { &xdg_toplevel_requests_show_window_menu_types as *const _ },
        },
        wl_message {
            name: b"move\0" as *const u8 as *const c_char,
            signature: b"ou\0" as *const u8 as *const c_char,
            types: unsafe { &xdg_toplevel_requests_move_types as *const _ },
        },
        wl_message {
            name: b"resize\0" as *const u8 as *const c_char,
            signature: b"ouu\0" as *const u8 as *const c_char,
            types: unsafe { &xdg_toplevel_requests_resize_types as *const _ },
        },
        wl_message {
            name: b"set_max_size\0" as *const u8 as *const c_char,
            signature: b"ii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_min_size\0" as *const u8 as *const c_char,
            signature: b"ii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_maximized\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"unset_maximized\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_fullscreen\0" as *const u8 as *const c_char,
            signature: b"?o\0" as *const u8 as *const c_char,
            types: unsafe { &xdg_toplevel_requests_set_fullscreen_types as *const _ },
        },
        wl_message {
            name: b"unset_fullscreen\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_minimized\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut xdg_toplevel_events: [wl_message; 2] = [
        wl_message {
            name: b"configure\0" as *const u8 as *const c_char,
            signature: b"iia\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"close\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut xdg_toplevel_interface: wl_interface = wl_interface {
        name: b"xdg_toplevel\0" as *const u8 as *const c_char,
        version: 2,
        request_count: 14,
        requests: unsafe { &xdg_toplevel_requests as *const _ },
        event_count: 2,
        events: unsafe { &xdg_toplevel_events as *const _ },
    };
}
#[doc = "short-lived, popup surfaces for menus\n\nA popup surface is a short-lived, temporary surface. It can be used to\nimplement for example menus, popovers, tooltips and other similar user\ninterface concepts.\n\nA popup can be made to take an explicit grab. See xdg_popup.grab for\ndetails.\n\nWhen the popup is dismissed, a popup_done event will be sent out, and at\nthe same time the surface will be unmapped. See the xdg_popup.popup_done\nevent for details.\n\nExplicitly destroying the xdg_popup object will also dismiss the popup and\nunmap the surface. Clients that want to dismiss the popup when another\nsurface of their own is clicked should dismiss the popup using the destroy\nrequest.\n\nA newly created xdg_popup will be stacked on top of all previously created\nxdg_popup surfaces associated with the same xdg_toplevel.\n\nThe parent of an xdg_popup must be mapped (see the xdg_surface\ndescription) before the xdg_popup itself.\n\nThe x and y arguments passed when creating the popup object specify\nwhere the top left of the popup should be placed, relative to the\nlocal surface coordinates of the parent surface. See\nxdg_surface.get_popup. An xdg_popup must intersect with or be at least\npartially adjacent to its parent surface.\n\nThe client must call wl_surface.commit on the corresponding wl_surface\nfor the xdg_popup state to take effect."]
pub mod xdg_popup {
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
        #[doc = "tried to grab after being mapped"]
        InvalidGrab = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidGrab),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "remove xdg_popup interface\n\nThis destroys the popup. Explicitly destroying the xdg_popup\nobject will also dismiss the popup, and unmap the surface.\n\nIf this xdg_popup is not the \"topmost\" popup, a protocol error\nwill be sent.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "make the popup take an explicit grab\n\nThis request makes the created popup take an explicit grab. An explicit\ngrab will be dismissed when the user dismisses the popup, or when the\nclient destroys the xdg_popup. This can be done by the user clicking\noutside the surface, using the keyboard, or even locking the screen\nthrough closing the lid or a timeout.\n\nIf the compositor denies the grab, the popup will be immediately\ndismissed.\n\nThis request must be used in response to some sort of user action like a\nbutton press, key press, or touch down event. The serial number of the\nevent should be passed as 'serial'.\n\nThe parent of a grabbing popup must either be an xdg_toplevel surface or\nanother xdg_popup with an explicit grab. If the parent is another\nxdg_popup it means that the popups are nested, with this popup now being\nthe topmost popup.\n\nNested popups must be destroyed in the reverse order they were created\nin, e.g. the only popup you are allowed to destroy at all times is the\ntopmost one.\n\nWhen compositors choose to dismiss a popup, they may dismiss every\nnested grabbing popup as well. When a compositor dismisses popups, it\nwill follow the same dismissing order as required from the client.\n\nThe parent of a grabbing popup must either be another xdg_popup with an\nactive explicit grab, or an xdg_popup or xdg_toplevel, if there are no\nexplicit grabs already taken.\n\nIf the topmost grabbing popup is destroyed, the grab will be returned to\nthe parent of the popup, if that parent previously had an explicit grab.\n\nIf the parent is a grabbing popup which has already been dismissed, this\npopup will be immediately dismissed. If the parent is a popup that did\nnot take an explicit grab, an error will be raised.\n\nDuring a popup grab, the client owning the grab will receive pointer\nand touch events for all their surfaces as normal (similar to an\n\"owner-events\" grab in X11 parlance), while the top most grabbing popup\nwill always have keyboard focus."]
        Grab {
            seat: super::wl_seat::WlSeat,
            serial: u32,
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
                name: "grab",
                since: 1,
                signature: &[super::ArgumentType::Object, super::ArgumentType::Uint],
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
                Request::Grab { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::Grab { .. } => 1,
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
                Request::Grab { seat, serial } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::Object(seat.as_ref().id()), Argument::Uint(serial),],
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
                Request::Grab { seat, serial } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.as_ref().c_ptr() as *mut _;
                    _args_array[1].u = serial;
                    f(1, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "configure the popup surface\n\nThis event asks the popup surface to configure itself given the\nconfiguration. The configured state should not be applied immediately.\nSee xdg_surface.configure for details.\n\nThe x and y arguments represent the position the popup was placed at\ngiven the xdg_positioner rule, relative to the upper left corner of the\nwindow geometry of the parent surface."]
        Configure {
            x: i32,
            y: i32,
            width: i32,
            height: i32,
        },
        #[doc = "popup interaction is done\n\nThe popup_done event is sent out when a popup is dismissed by the\ncompositor. The client should destroy the xdg_popup object at this\npoint."]
        PopupDone,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "configure",
                since: 1,
                signature: &[
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "popup_done",
                since: 1,
                signature: &[],
                destructor: false,
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::Configure { .. } => 0,
                Event::PopupDone => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Configure { .. } => 1,
                Event::PopupDone => 1,
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
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Configure {
                        x: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        y: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        width: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        height: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => Ok(Event::PopupDone),
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
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Event::Configure {
                        x: _args[0].i,
                        y: _args[1].i,
                        width: _args[2].i,
                        height: _args[3].i,
                    })
                }
                1 => Ok(Event::PopupDone),
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
    pub struct XdgPopup(Proxy<XdgPopup>);
    impl AsRef<Proxy<XdgPopup>> for XdgPopup {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<XdgPopup>> for XdgPopup {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            XdgPopup(value)
        }
    }
    impl From<XdgPopup> for Proxy<XdgPopup> {
        #[inline]
        fn from(value: XdgPopup) -> Self {
            value.0
        }
    }
    impl Interface for XdgPopup {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "xdg_popup";
        const VERSION: u32 = 2;
        fn c_interface() -> *const wl_interface {
            unsafe { &xdg_popup_interface }
        }
    }
    impl XdgPopup {
        #[doc = "remove xdg_popup interface\n\nThis destroys the popup. Explicitly destroying the xdg_popup\nobject will also dismiss the popup, and unmap the surface.\n\nIf this xdg_popup is not the \"topmost\" popup, a protocol error\nwill be sent.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "make the popup take an explicit grab\n\nThis request makes the created popup take an explicit grab. An explicit\ngrab will be dismissed when the user dismisses the popup, or when the\nclient destroys the xdg_popup. This can be done by the user clicking\noutside the surface, using the keyboard, or even locking the screen\nthrough closing the lid or a timeout.\n\nIf the compositor denies the grab, the popup will be immediately\ndismissed.\n\nThis request must be used in response to some sort of user action like a\nbutton press, key press, or touch down event. The serial number of the\nevent should be passed as 'serial'.\n\nThe parent of a grabbing popup must either be an xdg_toplevel surface or\nanother xdg_popup with an explicit grab. If the parent is another\nxdg_popup it means that the popups are nested, with this popup now being\nthe topmost popup.\n\nNested popups must be destroyed in the reverse order they were created\nin, e.g. the only popup you are allowed to destroy at all times is the\ntopmost one.\n\nWhen compositors choose to dismiss a popup, they may dismiss every\nnested grabbing popup as well. When a compositor dismisses popups, it\nwill follow the same dismissing order as required from the client.\n\nThe parent of a grabbing popup must either be another xdg_popup with an\nactive explicit grab, or an xdg_popup or xdg_toplevel, if there are no\nexplicit grabs already taken.\n\nIf the topmost grabbing popup is destroyed, the grab will be returned to\nthe parent of the popup, if that parent previously had an explicit grab.\n\nIf the parent is a grabbing popup which has already been dismissed, this\npopup will be immediately dismissed. If the parent is a popup that did\nnot take an explicit grab, an error will be raised.\n\nDuring a popup grab, the client owning the grab will receive pointer\nand touch events for all their surfaces as normal (similar to an\n\"owner-events\" grab in X11 parlance), while the top most grabbing popup\nwill always have keyboard focus."]
        pub fn grab(&self, seat: &super::wl_seat::WlSeat, serial: u32) -> () {
            let msg = Request::Grab {
                seat: seat.clone(),
                serial: serial,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GRAB_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CONFIGURE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_POPUP_DONE_SINCE: u32 = 1u32;
    static mut xdg_popup_requests_grab_types: [*const wl_interface; 2] = [
        unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut xdg_popup_requests: [wl_message; 2] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"grab\0" as *const u8 as *const c_char,
            signature: b"ou\0" as *const u8 as *const c_char,
            types: unsafe { &xdg_popup_requests_grab_types as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut xdg_popup_events: [wl_message; 2] = [
        wl_message {
            name: b"configure\0" as *const u8 as *const c_char,
            signature: b"iiii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"popup_done\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut xdg_popup_interface: wl_interface = wl_interface {
        name: b"xdg_popup\0" as *const u8 as *const c_char,
        version: 2,
        request_count: 2,
        requests: unsafe { &xdg_popup_requests as *const _ },
        event_count: 2,
        events: unsafe { &xdg_popup_events as *const _ },
    };
}
