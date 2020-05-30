use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 4] = [
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
];
#[doc = "create desktop-style surfaces\n\nxdg_shell allows clients to turn a wl_surface into a \"real window\"\nwhich can be dragged, resized, stacked, and moved around by the\nuser. Everything about this interface is suited towards traditional\ndesktop environments."]
pub mod xdg_shell {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[doc = "latest protocol version\n\nThe 'current' member of this enum gives the version of the\nprotocol.  Implementations can compare this to the version\nthey implement using static_assert to ensure the protocol and\nimplementation versions match."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Version {
        #[doc = "Always the latest version"]
        Current = 5,
    }
    impl Version {
        pub fn from_raw(n: u32) -> Option<Version> {
            match n {
                5 => Some(Version::Current),
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
    pub enum Error {
        #[doc = "given wl_surface has another role"]
        Role = 0,
        #[doc = "xdg_shell was destroyed before children"]
        DefunctSurfaces = 1,
        #[doc = "the client tried to map or destroy a non-topmost popup"]
        NotTheTopmostPopup = 2,
        #[doc = "the client specified an invalid popup parent surface"]
        InvalidPopupParent = 3,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::Role),
                1 => Some(Error::DefunctSurfaces),
                2 => Some(Error::NotTheTopmostPopup),
                3 => Some(Error::InvalidPopupParent),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy xdg_shell\n\nDestroy this xdg_shell object.\n\nDestroying a bound xdg_shell object while there are surfaces\nstill alive created by this xdg_shell object instance is illegal\nand will result in a protocol error.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "enable use of this unstable version\n\nNegotiate the unstable version of the interface.  This\nmechanism is in place to ensure client and server agree on the\nunstable versions of the protocol that they speak or exit\ncleanly if they don't agree.  This request will go away once\nthe xdg-shell protocol is stable."]
        UseUnstableVersion { version: i32 },
        #[doc = "create a shell surface from a surface\n\nThis creates an xdg_surface for the given surface and gives it the\nxdg_surface role. A wl_surface can only be given an xdg_surface role\nonce. If get_xdg_surface is called with a wl_surface that already has\nan active xdg_surface associated with it, or if it had any other role,\nan error is raised.\n\nSee the documentation of xdg_surface for more details about what an\nxdg_surface is and how it is used."]
        GetXdgSurface {
            surface: super::wl_surface::WlSurface,
        },
        #[doc = "create a popup for a surface\n\nThis creates an xdg_popup for the given surface and gives it the\nxdg_popup role. A wl_surface can only be given an xdg_popup role\nonce. If get_xdg_popup is called with a wl_surface that already has\nan active xdg_popup associated with it, or if it had any other role,\nan error is raised.\n\nThis request must be used in response to some sort of user action\nlike a button press, key press, or touch down event.\n\nSee the documentation of xdg_popup for more details about what an\nxdg_popup is and how it is used."]
        GetXdgPopup {
            surface: super::wl_surface::WlSurface,
            parent: super::wl_surface::WlSurface,
            seat: super::wl_seat::WlSeat,
            serial: u32,
            x: i32,
            y: i32,
        },
        #[doc = "respond to a ping event\n\nA client must respond to a ping event with a pong request or\nthe client may be deemed unresponsive."]
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
                name: "use_unstable_version",
                since: 1,
                signature: &[super::ArgumentType::Int],
                destructor: false,
            },
            super::MessageDesc {
                name: "get_xdg_surface",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "get_xdg_popup",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                    super::ArgumentType::Object,
                    super::ArgumentType::Object,
                    super::ArgumentType::Object,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
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
                Request::UseUnstableVersion { .. } => 1,
                Request::GetXdgSurface { .. } => 2,
                Request::GetXdgPopup { .. } => 3,
                Request::Pong { .. } => 4,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::UseUnstableVersion { .. } => 1,
                Request::GetXdgSurface { .. } => 1,
                Request::GetXdgPopup { .. } => 1,
                Request::Pong { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                2 => Some(Object::from_interface::<super::xdg_surface::XdgSurface>(
                    version,
                    meta.child(),
                )),
                3 => Some(Object::from_interface::<super::xdg_popup::XdgPopup>(
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
                Request::UseUnstableVersion { version } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::Int(version),],
                },
                Request::GetXdgSurface { surface } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![Argument::NewId(0), Argument::Object(surface.as_ref().id()),],
                },
                Request::GetXdgPopup {
                    surface,
                    parent,
                    seat,
                    serial,
                    x,
                    y,
                } => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: smallvec![
                        Argument::NewId(0),
                        Argument::Object(surface.as_ref().id()),
                        Argument::Object(parent.as_ref().id()),
                        Argument::Object(seat.as_ref().id()),
                        Argument::Uint(serial),
                        Argument::Int(x),
                        Argument::Int(y),
                    ],
                },
                Request::Pong { serial } => Message {
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
                Request::UseUnstableVersion { version } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = version;
                    f(1, &mut _args_array)
                }
                Request::GetXdgSurface { surface } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].o = surface.as_ref().c_ptr() as *mut _;
                    f(2, &mut _args_array)
                }
                Request::GetXdgPopup {
                    surface,
                    parent,
                    seat,
                    serial,
                    x,
                    y,
                } => {
                    let mut _args_array: [wl_argument; 7] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].o = surface.as_ref().c_ptr() as *mut _;
                    _args_array[2].o = parent.as_ref().c_ptr() as *mut _;
                    _args_array[3].o = seat.as_ref().c_ptr() as *mut _;
                    _args_array[4].u = serial;
                    _args_array[5].i = x;
                    _args_array[6].i = y;
                    f(3, &mut _args_array)
                }
                Request::Pong { serial } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    f(4, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "check if the client is alive\n\nThe ping event asks the client if it's still alive. Pass the\nserial specified in the event back to the compositor by sending\na \"pong\" request back with the specified serial.\n\nCompositors can use this to determine if the client is still\nalive. It's unspecified what will happen if the client doesn't\nrespond to the ping request, or in what timeframe. Clients should\ntry to respond in a reasonable amount of time.\n\nA compositor is free to ping in any way it wants, but a client must\nalways respond to any xdg_shell object it created."]
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
    pub struct XdgShell(Proxy<XdgShell>);
    impl AsRef<Proxy<XdgShell>> for XdgShell {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<XdgShell>> for XdgShell {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            XdgShell(value)
        }
    }
    impl From<XdgShell> for Proxy<XdgShell> {
        #[inline]
        fn from(value: XdgShell) -> Self {
            value.0
        }
    }
    impl Interface for XdgShell {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "xdg_shell";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &xdg_shell_interface }
        }
    }
    impl XdgShell {
        #[doc = "destroy xdg_shell\n\nDestroy this xdg_shell object.\n\nDestroying a bound xdg_shell object while there are surfaces\nstill alive created by this xdg_shell object instance is illegal\nand will result in a protocol error.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "enable use of this unstable version\n\nNegotiate the unstable version of the interface.  This\nmechanism is in place to ensure client and server agree on the\nunstable versions of the protocol that they speak or exit\ncleanly if they don't agree.  This request will go away once\nthe xdg-shell protocol is stable."]
        pub fn use_unstable_version(&self, version: i32) -> () {
            let msg = Request::UseUnstableVersion { version: version };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "create a shell surface from a surface\n\nThis creates an xdg_surface for the given surface and gives it the\nxdg_surface role. A wl_surface can only be given an xdg_surface role\nonce. If get_xdg_surface is called with a wl_surface that already has\nan active xdg_surface associated with it, or if it had any other role,\nan error is raised.\n\nSee the documentation of xdg_surface for more details about what an\nxdg_surface is and how it is used."]
        pub fn get_xdg_surface(
            &self,
            surface: &super::wl_surface::WlSurface,
        ) -> Main<super::xdg_surface::XdgSurface> {
            let msg = Request::GetXdgSurface {
                surface: surface.clone(),
            };
            self.0.send(msg, None).unwrap()
        }
        #[doc = "create a popup for a surface\n\nThis creates an xdg_popup for the given surface and gives it the\nxdg_popup role. A wl_surface can only be given an xdg_popup role\nonce. If get_xdg_popup is called with a wl_surface that already has\nan active xdg_popup associated with it, or if it had any other role,\nan error is raised.\n\nThis request must be used in response to some sort of user action\nlike a button press, key press, or touch down event.\n\nSee the documentation of xdg_popup for more details about what an\nxdg_popup is and how it is used."]
        pub fn get_xdg_popup(
            &self,
            surface: &super::wl_surface::WlSurface,
            parent: &super::wl_surface::WlSurface,
            seat: &super::wl_seat::WlSeat,
            serial: u32,
            x: i32,
            y: i32,
        ) -> Main<super::xdg_popup::XdgPopup> {
            let msg = Request::GetXdgPopup {
                surface: surface.clone(),
                parent: parent.clone(),
                seat: seat.clone(),
                serial: serial,
                x: x,
                y: y,
            };
            self.0.send(msg, None).unwrap()
        }
        #[doc = "respond to a ping event\n\nA client must respond to a ping event with a pong request or\nthe client may be deemed unresponsive."]
        pub fn pong(&self, serial: u32) -> () {
            let msg = Request::Pong { serial: serial };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_USE_UNSTABLE_VERSION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_XDG_SURFACE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_XDG_POPUP_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_PONG_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_PING_SINCE: u32 = 1u32;
    static mut xdg_shell_requests_get_xdg_surface_types: [*const wl_interface; 2] = [
        unsafe { &super::xdg_surface::xdg_surface_interface as *const wl_interface },
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
    ];
    static mut xdg_shell_requests_get_xdg_popup_types: [*const wl_interface; 7] = [
        unsafe { &super::xdg_popup::xdg_popup_interface as *const wl_interface },
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut xdg_shell_requests: [wl_message; 5] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"use_unstable_version\0" as *const u8 as *const c_char,
            signature: b"i\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"get_xdg_surface\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe { &xdg_shell_requests_get_xdg_surface_types as *const _ },
        },
        wl_message {
            name: b"get_xdg_popup\0" as *const u8 as *const c_char,
            signature: b"nooouii\0" as *const u8 as *const c_char,
            types: unsafe { &xdg_shell_requests_get_xdg_popup_types as *const _ },
        },
        wl_message {
            name: b"pong\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut xdg_shell_events: [wl_message; 1] = [wl_message {
        name: b"ping\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut xdg_shell_interface: wl_interface = wl_interface {
        name: b"xdg_shell\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 5,
        requests: unsafe { &xdg_shell_requests as *const _ },
        event_count: 1,
        events: unsafe { &xdg_shell_events as *const _ },
    };
}
#[doc = "A desktop window\n\nAn interface that may be implemented by a wl_surface, for\nimplementations that provide a desktop-style user interface.\n\nIt provides requests to treat surfaces like windows, allowing to set\nproperties like maximized, fullscreen, minimized, and to move and resize\nthem, and associate metadata like title and app id.\n\nThe client must call wl_surface.commit on the corresponding wl_surface\nfor the xdg_surface state to take effect. Prior to committing the new\nstate, it can set up initial configuration, such as maximizing or setting\na window geometry.\n\nEven without attaching a buffer the compositor must respond to initial\ncommitted configuration, for instance sending a configure event with\nexpected window geometry if the client maximized its surface during\ninitialization.\n\nFor a surface to be mapped by the compositor the client must have\ncommitted both an xdg_surface state and a buffer."]
pub mod xdg_surface {
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
    #[doc = "types of state on the surface\n\nThe different state values used on the surface. This is designed for\nstate values like maximized, fullscreen. It is paired with the\nconfigure event to ensure that both the client and the compositor\nsetting the state can be synchronized.\n\nStates set in this way are double-buffered. They will get applied on\nthe next commit.\n\nDesktop environments may extend this enum by taking up a range of\nvalues and documenting the range they chose in this description.\nThey are not required to document the values for the range that they\nchose. Ideally, any good extensions from a desktop environment should\nmake its way into standardization into this enum.\n\nThe current reserved ranges are:\n\n0x0000 - 0x0FFF: xdg-shell core values, documented below.\n0x1000 - 0x1FFF: GNOME\n0x2000 - 0x2FFF: EFL"]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum State {
        #[doc = "the surface is maximized\n\nThe surface is maximized. The window geometry specified in the configure\nevent must be obeyed by the client."]
        Maximized = 1,
        #[doc = "the surface is fullscreen\n\nThe surface is fullscreen. The window geometry specified in the configure\nevent must be obeyed by the client."]
        Fullscreen = 2,
        #[doc = "the surface is being resized\n\nThe surface is being resized. The window geometry specified in the\nconfigure event is a maximum; the client cannot resize beyond it.\nClients that have aspect ratio or cell sizing configuration can use\na smaller size, however."]
        Resizing = 3,
        #[doc = "the surface is now activated\n\nClient window decorations should be painted as if the window is\nactive. Do not assume this means that the window actually has\nkeyboard or pointer focus."]
        Activated = 4,
    }
    impl State {
        pub fn from_raw(n: u32) -> Option<State> {
            match n {
                1 => Some(State::Maximized),
                2 => Some(State::Fullscreen),
                3 => Some(State::Resizing),
                4 => Some(State::Activated),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "Destroy the xdg_surface\n\nUnmap and destroy the window. The window will be effectively\nhidden from the user's point of view, and all state like\nmaximization, fullscreen, and so on, will be lost.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "set the parent of this surface\n\nSet the \"parent\" of this surface. This window should be stacked\nabove a parent. The parent surface must be mapped as long as this\nsurface is mapped.\n\nParent windows should be set on dialogs, toolboxes, or other\n\"auxiliary\" surfaces, so that the parent is raised when the dialog\nis raised."]
        SetParent {
            parent: Option<super::xdg_surface::XdgSurface>,
        },
        #[doc = "set surface title\n\nSet a short title for the surface.\n\nThis string may be used to identify the surface in a task bar,\nwindow list, or other user interface elements provided by the\ncompositor.\n\nThe string must be encoded in UTF-8."]
        SetTitle { title: String },
        #[doc = "set application ID\n\nSet an application identifier for the surface.\n\nThe app ID identifies the general class of applications to which\nthe surface belongs. The compositor can use this to group multiple\nsurfaces together, or to determine how to launch a new application.\n\nFor D-Bus activatable applications, the app ID is used as the D-Bus\nservice name.\n\nThe compositor shell will try to group application surfaces together\nby their app ID.  As a best practice, it is suggested to select app\nID's that match the basename of the application's .desktop file.\nFor example, \"org.freedesktop.FooViewer\" where the .desktop file is\n\"org.freedesktop.FooViewer.desktop\".\n\nSee the desktop-entry specification [0] for more details on\napplication identifiers and how they relate to well-known D-Bus\nnames and .desktop files.\n\n[0] http://standards.freedesktop.org/desktop-entry-spec/"]
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
            edges: u32,
        },
        #[doc = "ack a configure event\n\nWhen a configure event is received, if a client commits the\nsurface in response to the configure event, then the client\nmust make an ack_configure request sometime before the commit\nrequest, passing along the serial of the configure event.\n\nFor instance, the compositor might use this information to move\na surface to the top left only when the client has drawn itself\nfor the maximized or fullscreen state.\n\nIf the client receives multiple configure events before it\ncan respond to one, it only has to ack the last configure event.\n\nA client is not required to commit immediately after sending\nan ack_configure request - it may even ack_configure several times\nbefore its next surface commit.\n\nThe compositor expects that the most recently received\nack_configure request at the time of a commit indicates which\nconfigure event the client is responding to."]
        AckConfigure { serial: u32 },
        #[doc = "set the new window geometry\n\nThe window geometry of a window is its \"visible bounds\" from the\nuser's perspective. Client-side decorations often have invisible\nportions like drop-shadows which should be ignored for the\npurposes of aligning, placing and constraining windows.\n\nThe window geometry is double buffered, and will be applied at the\ntime wl_surface.commit of the corresponding wl_surface is called.\n\nOnce the window geometry of the surface is set once, it is not\npossible to unset it, and it will remain the same until\nset_window_geometry is called again, even if a new subsurface or\nbuffer is attached.\n\nIf never set, the value is the full bounds of the surface,\nincluding any subsurfaces. This updates dynamically on every\ncommit. This unset mode is meant for extremely simple clients.\n\nIf responding to a configure event, the window geometry in here\nmust respect the sizing negotiations specified by the states in\nthe configure event.\n\nThe arguments are given in the surface local coordinate space of\nthe wl_surface associated with this xdg_surface.\n\nThe width and height must be greater than zero."]
        SetWindowGeometry {
            x: i32,
            y: i32,
            width: i32,
            height: i32,
        },
        #[doc = "maximize the window\n\nMaximize the surface.\n\nAfter requesting that the surface should be maximized, the compositor\nwill respond by emitting a configure event with the \"maximized\" state\nand the required window geometry. The client should then update its\ncontent, drawing it in a maximized state, i.e. without shadow or other\ndecoration outside of the window geometry. The client must also\nacknowledge the configure when committing the new content (see\nack_configure).\n\nIt is up to the compositor to decide how and where to maximize the\nsurface, for example which output and what region of the screen should\nbe used.\n\nIf the surface was already maximized, the compositor will still emit\na configure event with the \"maximized\" state."]
        SetMaximized,
        #[doc = "unmaximize the window\n\nUnmaximize the surface.\n\nAfter requesting that the surface should be unmaximized, the compositor\nwill respond by emitting a configure event without the \"maximized\"\nstate. If available, the compositor will include the window geometry\ndimensions the window had prior to being maximized in the configure\nrequest. The client must then update its content, drawing it in a\nregular state, i.e. potentially with shadow, etc. The client must also\nacknowledge the configure when committing the new content (see\nack_configure).\n\nIt is up to the compositor to position the surface after it was\nunmaximized; usually the position the surface had before maximizing, if\napplicable.\n\nIf the surface was already not maximized, the compositor will still\nemit a configure event without the \"maximized\" state."]
        UnsetMaximized,
        #[doc = "set the window as fullscreen on a monitor\n\nMake the surface fullscreen.\n\nYou can specify an output that you would prefer to be fullscreen.\nIf this value is NULL, it's up to the compositor to choose which\ndisplay will be used to map this surface.\n\nIf the surface doesn't cover the whole output, the compositor will\nposition the surface in the center of the output and compensate with\nblack borders filling the rest of the output."]
        SetFullscreen {
            output: Option<super::wl_output::WlOutput>,
        },
        #[doc = ""]
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
                name: "ack_configure",
                since: 1,
                signature: &[super::ArgumentType::Uint],
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
                Request::AckConfigure { .. } => 7,
                Request::SetWindowGeometry { .. } => 8,
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
                Request::AckConfigure { .. } => 1,
                Request::SetWindowGeometry { .. } => 1,
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
                        Argument::Uint(edges),
                    ],
                },
                Request::AckConfigure { serial } => Message {
                    sender_id: sender_id,
                    opcode: 7,
                    args: smallvec![Argument::Uint(serial),],
                },
                Request::SetWindowGeometry {
                    x,
                    y,
                    width,
                    height,
                } => Message {
                    sender_id: sender_id,
                    opcode: 8,
                    args: smallvec![
                        Argument::Int(x),
                        Argument::Int(y),
                        Argument::Int(width),
                        Argument::Int(height),
                    ],
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
                    _args_array[2].u = edges;
                    f(6, &mut _args_array)
                }
                Request::AckConfigure { serial } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    f(7, &mut _args_array)
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
        #[doc = "suggest a surface change\n\nThe configure event asks the client to resize its surface or to\nchange its state.\n\nThe width and height arguments specify a hint to the window\nabout how its surface should be resized in window geometry\ncoordinates. See set_window_geometry.\n\nIf the width or height arguments are zero, it means the client\nshould decide its own window dimension. This may happen when the\ncompositor need to configure the state of the surface but doesn't\nhave any information about any previous or expected dimension.\n\nThe states listed in the event specify how the width/height\narguments should be interpreted, and possibly how it should be\ndrawn.\n\nClients should arrange their surface for the new size and\nstates, and then send a ack_configure request with the serial\nsent in this configure event at some point before committing\nthe new surface.\n\nIf the client receives multiple configure events before it\ncan respond to one, it is free to discard all but the last\nevent it received."]
        Configure {
            width: i32,
            height: i32,
            states: Vec<u8>,
            serial: u32,
        },
        #[doc = "surface wants to be closed\n\nThe close event is sent by the compositor when the user\nwants the surface to be closed. This should be equivalent to\nthe user clicking the close button in client-side decorations,\nif your application has any...\n\nThis is only a request that the user intends to close your\nwindow. The client may choose to ignore this request, or show\na dialog to ask the user to save their data..."]
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
                    super::ArgumentType::Uint,
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
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
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
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Event::Configure {
                        width: _args[0].i,
                        height: _args[1].i,
                        states: {
                            let array = &*_args[2].a;
                            ::std::slice::from_raw_parts(array.data as *const u8, array.size)
                                .to_owned()
                        },
                        serial: _args[3].u,
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
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &xdg_surface_interface }
        }
    }
    impl XdgSurface {
        #[doc = "Destroy the xdg_surface\n\nUnmap and destroy the window. The window will be effectively\nhidden from the user's point of view, and all state like\nmaximization, fullscreen, and so on, will be lost.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the parent of this surface\n\nSet the \"parent\" of this surface. This window should be stacked\nabove a parent. The parent surface must be mapped as long as this\nsurface is mapped.\n\nParent windows should be set on dialogs, toolboxes, or other\n\"auxiliary\" surfaces, so that the parent is raised when the dialog\nis raised."]
        pub fn set_parent(&self, parent: Option<&super::xdg_surface::XdgSurface>) -> () {
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
        #[doc = "set application ID\n\nSet an application identifier for the surface.\n\nThe app ID identifies the general class of applications to which\nthe surface belongs. The compositor can use this to group multiple\nsurfaces together, or to determine how to launch a new application.\n\nFor D-Bus activatable applications, the app ID is used as the D-Bus\nservice name.\n\nThe compositor shell will try to group application surfaces together\nby their app ID.  As a best practice, it is suggested to select app\nID's that match the basename of the application's .desktop file.\nFor example, \"org.freedesktop.FooViewer\" where the .desktop file is\n\"org.freedesktop.FooViewer.desktop\".\n\nSee the desktop-entry specification [0] for more details on\napplication identifiers and how they relate to well-known D-Bus\nnames and .desktop files.\n\n[0] http://standards.freedesktop.org/desktop-entry-spec/"]
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
        pub fn resize(&self, seat: &super::wl_seat::WlSeat, serial: u32, edges: u32) -> () {
            let msg = Request::Resize {
                seat: seat.clone(),
                serial: serial,
                edges: edges,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "ack a configure event\n\nWhen a configure event is received, if a client commits the\nsurface in response to the configure event, then the client\nmust make an ack_configure request sometime before the commit\nrequest, passing along the serial of the configure event.\n\nFor instance, the compositor might use this information to move\na surface to the top left only when the client has drawn itself\nfor the maximized or fullscreen state.\n\nIf the client receives multiple configure events before it\ncan respond to one, it only has to ack the last configure event.\n\nA client is not required to commit immediately after sending\nan ack_configure request - it may even ack_configure several times\nbefore its next surface commit.\n\nThe compositor expects that the most recently received\nack_configure request at the time of a commit indicates which\nconfigure event the client is responding to."]
        pub fn ack_configure(&self, serial: u32) -> () {
            let msg = Request::AckConfigure { serial: serial };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the new window geometry\n\nThe window geometry of a window is its \"visible bounds\" from the\nuser's perspective. Client-side decorations often have invisible\nportions like drop-shadows which should be ignored for the\npurposes of aligning, placing and constraining windows.\n\nThe window geometry is double buffered, and will be applied at the\ntime wl_surface.commit of the corresponding wl_surface is called.\n\nOnce the window geometry of the surface is set once, it is not\npossible to unset it, and it will remain the same until\nset_window_geometry is called again, even if a new subsurface or\nbuffer is attached.\n\nIf never set, the value is the full bounds of the surface,\nincluding any subsurfaces. This updates dynamically on every\ncommit. This unset mode is meant for extremely simple clients.\n\nIf responding to a configure event, the window geometry in here\nmust respect the sizing negotiations specified by the states in\nthe configure event.\n\nThe arguments are given in the surface local coordinate space of\nthe wl_surface associated with this xdg_surface.\n\nThe width and height must be greater than zero."]
        pub fn set_window_geometry(&self, x: i32, y: i32, width: i32, height: i32) -> () {
            let msg = Request::SetWindowGeometry {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "maximize the window\n\nMaximize the surface.\n\nAfter requesting that the surface should be maximized, the compositor\nwill respond by emitting a configure event with the \"maximized\" state\nand the required window geometry. The client should then update its\ncontent, drawing it in a maximized state, i.e. without shadow or other\ndecoration outside of the window geometry. The client must also\nacknowledge the configure when committing the new content (see\nack_configure).\n\nIt is up to the compositor to decide how and where to maximize the\nsurface, for example which output and what region of the screen should\nbe used.\n\nIf the surface was already maximized, the compositor will still emit\na configure event with the \"maximized\" state."]
        pub fn set_maximized(&self) -> () {
            let msg = Request::SetMaximized;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "unmaximize the window\n\nUnmaximize the surface.\n\nAfter requesting that the surface should be unmaximized, the compositor\nwill respond by emitting a configure event without the \"maximized\"\nstate. If available, the compositor will include the window geometry\ndimensions the window had prior to being maximized in the configure\nrequest. The client must then update its content, drawing it in a\nregular state, i.e. potentially with shadow, etc. The client must also\nacknowledge the configure when committing the new content (see\nack_configure).\n\nIt is up to the compositor to position the surface after it was\nunmaximized; usually the position the surface had before maximizing, if\napplicable.\n\nIf the surface was already not maximized, the compositor will still\nemit a configure event without the \"maximized\" state."]
        pub fn unset_maximized(&self) -> () {
            let msg = Request::UnsetMaximized;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the window as fullscreen on a monitor\n\nMake the surface fullscreen.\n\nYou can specify an output that you would prefer to be fullscreen.\nIf this value is NULL, it's up to the compositor to choose which\ndisplay will be used to map this surface.\n\nIf the surface doesn't cover the whole output, the compositor will\nposition the surface in the center of the output and compensate with\nblack borders filling the rest of the output."]
        pub fn set_fullscreen(&self, output: Option<&super::wl_output::WlOutput>) -> () {
            let msg = Request::SetFullscreen {
                output: output.map(|o| o.clone()),
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = ""]
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
    pub const REQ_ACK_CONFIGURE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_WINDOW_GEOMETRY_SINCE: u32 = 1u32;
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
    static mut xdg_surface_requests_set_parent_types: [*const wl_interface; 1] =
        [unsafe { &super::xdg_surface::xdg_surface_interface as *const wl_interface }];
    static mut xdg_surface_requests_show_window_menu_types: [*const wl_interface; 4] = [
        unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    static mut xdg_surface_requests_move_types: [*const wl_interface; 2] = [
        unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
    ];
    static mut xdg_surface_requests_resize_types: [*const wl_interface; 3] = [
        unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    static mut xdg_surface_requests_set_fullscreen_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_output::wl_output_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut xdg_surface_requests: [wl_message; 14] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_parent\0" as *const u8 as *const c_char,
            signature: b"?o\0" as *const u8 as *const c_char,
            types: unsafe { &xdg_surface_requests_set_parent_types as *const _ },
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
            types: unsafe { &xdg_surface_requests_show_window_menu_types as *const _ },
        },
        wl_message {
            name: b"move\0" as *const u8 as *const c_char,
            signature: b"ou\0" as *const u8 as *const c_char,
            types: unsafe { &xdg_surface_requests_move_types as *const _ },
        },
        wl_message {
            name: b"resize\0" as *const u8 as *const c_char,
            signature: b"ouu\0" as *const u8 as *const c_char,
            types: unsafe { &xdg_surface_requests_resize_types as *const _ },
        },
        wl_message {
            name: b"ack_configure\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_window_geometry\0" as *const u8 as *const c_char,
            signature: b"iiii\0" as *const u8 as *const c_char,
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
            types: unsafe { &xdg_surface_requests_set_fullscreen_types as *const _ },
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
    pub static mut xdg_surface_events: [wl_message; 2] = [
        wl_message {
            name: b"configure\0" as *const u8 as *const c_char,
            signature: b"iiau\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"close\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut xdg_surface_interface: wl_interface = wl_interface {
        name: b"xdg_surface\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 14,
        requests: unsafe { &xdg_surface_requests as *const _ },
        event_count: 2,
        events: unsafe { &xdg_surface_events as *const _ },
    };
}
#[doc = "short-lived, popup surfaces for menus\n\nA popup surface is a short-lived, temporary surface that can be\nused to implement menus. It takes an explicit grab on the surface\nthat will be dismissed when the user dismisses the popup. This can\nbe done by the user clicking outside the surface, using the keyboard,\nor even locking the screen through closing the lid or a timeout.\n\nWhen the popup is dismissed, a popup_done event will be sent out,\nand at the same time the surface will be unmapped. The xdg_popup\nobject is now inert and cannot be reactivated, so clients should\ndestroy it. Explicitly destroying the xdg_popup object will also\ndismiss the popup and unmap the surface.\n\nClients will receive events for all their surfaces during this\ngrab (which is an \"owner-events\" grab in X11 parlance). This is\ndone so that users can navigate through submenus and other\n\"nested\" popup windows without having to dismiss the topmost\npopup.\n\nClients that want to dismiss the popup when another surface of\ntheir own is clicked should dismiss the popup using the destroy\nrequest.\n\nThe parent surface must have either an xdg_surface or xdg_popup\nrole.\n\nSpecifying an xdg_popup for the parent means that the popups are\nnested, with this popup now being the topmost popup. Nested\npopups must be destroyed in the reverse order they were created\nin, e.g. the only popup you are allowed to destroy at all times\nis the topmost one.\n\nIf there is an existing popup when creating a new popup, the\nparent must be the current topmost popup.\n\nA parent surface must be mapped before the new popup is mapped.\n\nWhen compositors choose to dismiss a popup, they will likely\ndismiss every nested popup as well. When a compositor dismisses\npopups, it will follow the same dismissing order as required\nfrom the client.\n\nThe x and y arguments passed when creating the popup object specify\nwhere the top left of the popup should be placed, relative to the\nlocal surface coordinates of the parent surface. See\nxdg_shell.get_xdg_popup.\n\nThe client must call wl_surface.commit on the corresponding wl_surface\nfor the xdg_popup state to take effect.\n\nFor a surface to be mapped by the compositor the client must have\ncommitted both the xdg_popup state and a buffer."]
pub mod xdg_popup {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "remove xdg_popup interface\n\nThis destroys the popup. Explicitly destroying the xdg_popup\nobject will also dismiss the popup, and unmap the surface.\n\nIf this xdg_popup is not the \"topmost\" popup, a protocol error\nwill be sent.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "destroy",
            since: 1,
            signature: &[],
            destructor: true,
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Destroy => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
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
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "popup interaction is done\n\nThe popup_done event is sent out when a popup is dismissed by the\ncompositor. The client should destroy the xdg_popup object at this\npoint."]
        PopupDone,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "popup_done",
            since: 1,
            signature: &[],
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
                Event::PopupDone => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
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
                0 => Ok(Event::PopupDone),
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
                0 => Ok(Event::PopupDone),
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
        const VERSION: u32 = 1;
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
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_POPUP_DONE_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut xdg_popup_requests: [wl_message; 1] = [wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut xdg_popup_events: [wl_message; 1] = [wl_message {
        name: b"popup_done\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut xdg_popup_interface: wl_interface = wl_interface {
        name: b"xdg_popup\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 1,
        requests: unsafe { &xdg_popup_requests as *const _ },
        event_count: 1,
        events: unsafe { &xdg_popup_events as *const _ },
    };
}
