use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 2] = [
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
];
#[doc = "manage xdg_output objects\n\nA global factory interface for xdg_output objects."]
pub mod zxdg_output_manager_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy the xdg_output_manager object\n\nUsing this request a client can tell the server that it is not\ngoing to use the xdg_output_manager object anymore.\n\nAny objects already created through this instance are not affected.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "create an xdg output from a wl_output\n\nThis creates a new xdg_output object for the given wl_output."]
        GetXdgOutput { output: super::wl_output::WlOutput },
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
                name: "get_xdg_output",
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
                Request::GetXdgOutput { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::GetXdgOutput { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(
                    Object::from_interface::<super::zxdg_output_v1::ZxdgOutputV1>(
                        version,
                        meta.child(),
                    ),
                ),
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
                Request::GetXdgOutput { output } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::NewId(0), Argument::Object(output.as_ref().id()),],
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
                Request::GetXdgOutput { output } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].o = output.as_ref().c_ptr() as *mut _;
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
    pub struct ZxdgOutputManagerV1(Proxy<ZxdgOutputManagerV1>);
    impl AsRef<Proxy<ZxdgOutputManagerV1>> for ZxdgOutputManagerV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZxdgOutputManagerV1>> for ZxdgOutputManagerV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZxdgOutputManagerV1(value)
        }
    }
    impl From<ZxdgOutputManagerV1> for Proxy<ZxdgOutputManagerV1> {
        #[inline]
        fn from(value: ZxdgOutputManagerV1) -> Self {
            value.0
        }
    }
    impl Interface for ZxdgOutputManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_output_manager_v1";
        const VERSION: u32 = 3;
        fn c_interface() -> *const wl_interface {
            unsafe { &zxdg_output_manager_v1_interface }
        }
    }
    impl ZxdgOutputManagerV1 {
        #[doc = "destroy the xdg_output_manager object\n\nUsing this request a client can tell the server that it is not\ngoing to use the xdg_output_manager object anymore.\n\nAny objects already created through this instance are not affected.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "create an xdg output from a wl_output\n\nThis creates a new xdg_output object for the given wl_output."]
        pub fn get_xdg_output(
            &self,
            output: &super::wl_output::WlOutput,
        ) -> Main<super::zxdg_output_v1::ZxdgOutputV1> {
            let msg = Request::GetXdgOutput {
                output: output.clone(),
            };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_XDG_OUTPUT_SINCE: u32 = 1u32;
    static mut zxdg_output_manager_v1_requests_get_xdg_output_types: [*const wl_interface; 2] = [
        unsafe { &super::zxdg_output_v1::zxdg_output_v1_interface as *const wl_interface },
        unsafe { &super::wl_output::wl_output_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zxdg_output_manager_v1_requests: [wl_message; 2] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"get_xdg_output\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe { &zxdg_output_manager_v1_requests_get_xdg_output_types as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zxdg_output_manager_v1_interface: wl_interface = wl_interface {
        name: b"zxdg_output_manager_v1\0" as *const u8 as *const c_char,
        version: 3,
        request_count: 2,
        requests: unsafe { &zxdg_output_manager_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "compositor logical output region\n\nAn xdg_output describes part of the compositor geometry.\n\nThis typically corresponds to a monitor that displays part of the\ncompositor space.\n\nFor objects version 3 onwards, after all xdg_output properties have been\nsent (when the object is created and when properties are updated), a\nwl_output.done event is sent. This allows changes to the output\nproperties to be seen as atomic, even if they happen via multiple events."]
pub mod zxdg_output_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy the xdg_output object\n\nUsing this request a client can tell the server that it is not\ngoing to use the xdg_output object anymore.\n\nThis is a destructor, once sent this object cannot be used any longer."]
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
        #[doc = "position of the output within the global compositor space\n\nThe position event describes the location of the wl_output within\nthe global compositor space.\n\nThe logical_position event is sent after creating an xdg_output\n(see xdg_output_manager.get_xdg_output) and whenever the location\nof the output changes within the global compositor space."]
        LogicalPosition { x: i32, y: i32 },
        #[doc = "size of the output in the global compositor space\n\nThe logical_size event describes the size of the output in the\nglobal compositor space.\n\nFor example, a surface without any buffer scale, transformation\nnor rotation set, with the size matching the logical_size will\nhave the same size as the corresponding output when displayed.\n\nMost regular Wayland clients should not pay attention to the\nlogical size and would rather rely on xdg_shell interfaces.\n\nSome clients such as Xwayland, however, need this to configure\ntheir surfaces in the global compositor space as the compositor\nmay apply a different scale from what is advertised by the output\nscaling property (to achieve fractional scaling, for example).\n\nFor example, for a wl_output mode 3840×2160 and a scale factor 2:\n\n- A compositor not scaling the surface buffers will advertise a\nlogical size of 3840×2160,\n\n- A compositor automatically scaling the surface buffers will\nadvertise a logical size of 1920×1080,\n\n- A compositor using a fractional scale of 1.5 will advertise a\nlogical size to 2560×1620.\n\nFor example, for a wl_output mode 1920×1080 and a 90 degree rotation,\nthe compositor will advertise a logical size of 1080x1920.\n\nThe logical_size event is sent after creating an xdg_output\n(see xdg_output_manager.get_xdg_output) and whenever the logical\nsize of the output changes, either as a result of a change in the\napplied scale or because of a change in the corresponding output\nmode(see wl_output.mode) or transform (see wl_output.transform)."]
        LogicalSize { width: i32, height: i32 },
        #[doc = "all information about the output have been sent\n\nThis event is sent after all other properties of an xdg_output\nhave been sent.\n\nThis allows changes to the xdg_output properties to be seen as\natomic, even if they happen via multiple events.\n\nFor objects version 3 onwards, this event is deprecated. Compositors\nare not required to send it anymore and must send wl_output.done\ninstead."]
        Done,
        #[doc = "name of this output\n\nMany compositors will assign names to their outputs, show them to the\nuser, allow them to be configured by name, etc. The client may wish to\nknow this name as well to offer the user similar behaviors.\n\nThe naming convention is compositor defined, but limited to\nalphanumeric characters and dashes (-). Each name is unique among all\nwl_output globals, but if a wl_output global is destroyed the same name\nmay be reused later. The names will also remain consistent across\nsessions with the same hardware and software configuration.\n\nExamples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do\nnot assume that the name is a reflection of an underlying DRM\nconnector, X11 connection, etc.\n\nThe name event is sent after creating an xdg_output (see\nxdg_output_manager.get_xdg_output). This event is only sent once per\nxdg_output, and the name does not change over the lifetime of the\nwl_output global.\n\nOnly available since version 2 of the interface"]
        Name { name: String },
        #[doc = "human-readable description of this output\n\nMany compositors can produce human-readable descriptions of their\noutputs.  The client may wish to know this description as well, to\ncommunicate the user for various purposes.\n\nThe description is a UTF-8 string with no convention defined for its\ncontents. Examples might include 'Foocorp 11\" Display' or 'Virtual X11\noutput via :1'.\n\nThe description event is sent after creating an xdg_output (see\nxdg_output_manager.get_xdg_output) and whenever the description\nchanges. The description is optional, and may not be sent at all.\n\nFor objects of version 2 and lower, this event is only sent once per\nxdg_output, and the description does not change over the lifetime of\nthe wl_output global.\n\nOnly available since version 2 of the interface"]
        Description { description: String },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "logical_position",
                since: 1,
                signature: &[super::ArgumentType::Int, super::ArgumentType::Int],
                destructor: false,
            },
            super::MessageDesc {
                name: "logical_size",
                since: 1,
                signature: &[super::ArgumentType::Int, super::ArgumentType::Int],
                destructor: false,
            },
            super::MessageDesc {
                name: "done",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "name",
                since: 2,
                signature: &[super::ArgumentType::Str],
                destructor: false,
            },
            super::MessageDesc {
                name: "description",
                since: 2,
                signature: &[super::ArgumentType::Str],
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
                Event::LogicalPosition { .. } => 0,
                Event::LogicalSize { .. } => 1,
                Event::Done => 2,
                Event::Name { .. } => 3,
                Event::Description { .. } => 4,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::LogicalPosition { .. } => 1,
                Event::LogicalSize { .. } => 1,
                Event::Done => 1,
                Event::Name { .. } => 2,
                Event::Description { .. } => 2,
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
                    Ok(Event::LogicalPosition {
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
                    })
                }
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::LogicalSize {
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
                2 => Ok(Event::Done),
                3 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Name {
                        name: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                4 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Description {
                        description: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
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
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::LogicalPosition {
                        x: _args[0].i,
                        y: _args[1].i,
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::LogicalSize {
                        width: _args[0].i,
                        height: _args[1].i,
                    })
                }
                2 => Ok(Event::Done),
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Name {
                        name: ::std::ffi::CStr::from_ptr(_args[0].s)
                            .to_string_lossy()
                            .into_owned(),
                    })
                }
                4 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Description {
                        description: ::std::ffi::CStr::from_ptr(_args[0].s)
                            .to_string_lossy()
                            .into_owned(),
                    })
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
    pub struct ZxdgOutputV1(Proxy<ZxdgOutputV1>);
    impl AsRef<Proxy<ZxdgOutputV1>> for ZxdgOutputV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZxdgOutputV1>> for ZxdgOutputV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZxdgOutputV1(value)
        }
    }
    impl From<ZxdgOutputV1> for Proxy<ZxdgOutputV1> {
        #[inline]
        fn from(value: ZxdgOutputV1) -> Self {
            value.0
        }
    }
    impl Interface for ZxdgOutputV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_output_v1";
        const VERSION: u32 = 3;
        fn c_interface() -> *const wl_interface {
            unsafe { &zxdg_output_v1_interface }
        }
    }
    impl ZxdgOutputV1 {
        #[doc = "destroy the xdg_output object\n\nUsing this request a client can tell the server that it is not\ngoing to use the xdg_output object anymore.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_LOGICAL_POSITION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_LOGICAL_SIZE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DONE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_NAME_SINCE: u32 = 2u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DESCRIPTION_SINCE: u32 = 2u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zxdg_output_v1_requests: [wl_message; 1] = [wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zxdg_output_v1_events: [wl_message; 5] = [
        wl_message {
            name: b"logical_position\0" as *const u8 as *const c_char,
            signature: b"ii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"logical_size\0" as *const u8 as *const c_char,
            signature: b"ii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"done\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"name\0" as *const u8 as *const c_char,
            signature: b"2s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"description\0" as *const u8 as *const c_char,
            signature: b"2s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zxdg_output_v1_interface: wl_interface = wl_interface {
        name: b"zxdg_output_v1\0" as *const u8 as *const c_char,
        version: 3,
        request_count: 1,
        requests: unsafe { &zxdg_output_v1_requests as *const _ },
        event_count: 5,
        events: unsafe { &zxdg_output_v1_events as *const _ },
    };
}
