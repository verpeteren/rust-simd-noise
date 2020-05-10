use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 8] = [
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
];
#[doc = "core global object\n\nThe core global object.  This is a special singleton object.  It\nis used for internal Wayland protocol features."]
pub mod wl_display {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[doc = "global error values\n\nThese errors are global and can be emitted in response to any\nserver request."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Error {
        #[doc = "server couldn't find object"]
        InvalidObject = 0,
        #[doc = "method doesn't exist on the specified interface"]
        InvalidMethod = 1,
        #[doc = "server is out of memory"]
        NoMemory = 2,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidObject),
                1 => Some(Error::InvalidMethod),
                2 => Some(Error::NoMemory),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "asynchronous roundtrip\n\nThe sync request asks the server to emit the 'done' event\non the returned wl_callback object.  Since requests are\nhandled in-order and events are delivered in-order, this can\nbe used as a barrier to ensure all previous requests and the\nresulting events have been handled.\n\nThe object returned by this request will be destroyed by the\ncompositor after the callback is fired and as such the client must not\nattempt to use it after that point.\n\nThe callback_data passed in the callback is the event serial."]
        Sync {},
        #[doc = "get global registry object\n\nThis request creates a registry object that allows the client\nto list and bind the global objects available from the\ncompositor.\n\nIt should be noted that the server side resources consumed in\nresponse to a get_registry request can only be released when the\nclient disconnects, not when the client side proxy is destroyed.\nTherefore, clients should invoke get_registry as infrequently as\npossible to avoid wasting memory."]
        GetRegistry {},
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "sync",
                since: 1,
                signature: &[super::ArgumentType::NewId],
                destructor: false,
            },
            super::MessageDesc {
                name: "get_registry",
                since: 1,
                signature: &[super::ArgumentType::NewId],
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
                Request::Sync { .. } => 0,
                Request::GetRegistry { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Sync { .. } => 1,
                Request::GetRegistry { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<super::wl_callback::WlCallback>(
                    version,
                    meta.child(),
                )),
                1 => Some(Object::from_interface::<super::wl_registry::WlRegistry>(
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
                Request::Sync {} => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![Argument::NewId(0),],
                },
                Request::GetRegistry {} => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::NewId(0),],
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
                Request::Sync {} => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    f(0, &mut _args_array)
                }
                Request::GetRegistry {} => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    f(1, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "fatal error event\n\nThe error event is sent out when a fatal (non-recoverable)\nerror has occurred.  The object_id argument is the object\nwhere the error occurred, most often in response to a request\nto that object.  The code identifies the error and is defined\nby the object interface.  As such, each interface defines its\nown set of error codes.  The message is a brief description\nof the error, for (debugging) convenience."]
        Error {
            object_id: AnonymousObject,
            code: u32,
            message: String,
        },
        #[doc = "acknowledge object ID deletion\n\nThis event is used internally by the object ID management\nlogic.  When a client deletes an object, the server will send\nthis event to acknowledge that it has seen the delete request.\nWhen the client receives this event, it will know that it can\nsafely reuse the object ID."]
        DeleteId { id: u32 },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "error",
                since: 1,
                signature: &[
                    super::ArgumentType::Object,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Str,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "delete_id",
                since: 1,
                signature: &[super::ArgumentType::Uint],
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
                Event::Error { .. } => 0,
                Event::DeleteId { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Error { .. } => 1,
                Event::DeleteId { .. } => 1,
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
                    Ok(Event::Error {
                        object_id: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?.into()
                            } else {
                                return Err(());
                            }
                        },
                        code: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        message: {
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
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::DeleteId {
                        id: {
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
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Error {
                        object_id: Proxy::<AnonymousObject>::from_c_ptr(_args[0].o as *mut _)
                            .into(),
                        code: _args[1].u,
                        message: ::std::ffi::CStr::from_ptr(_args[2].s)
                            .to_string_lossy()
                            .into_owned(),
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::DeleteId { id: _args[0].u })
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
    pub struct WlDisplay(Proxy<WlDisplay>);
    impl AsRef<Proxy<WlDisplay>> for WlDisplay {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlDisplay>> for WlDisplay {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlDisplay(value)
        }
    }
    impl From<WlDisplay> for Proxy<WlDisplay> {
        #[inline]
        fn from(value: WlDisplay) -> Self {
            value.0
        }
    }
    impl Interface for WlDisplay {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_display";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_display_interface }
        }
    }
    impl WlDisplay {
        #[doc = "asynchronous roundtrip\n\nThe sync request asks the server to emit the 'done' event\non the returned wl_callback object.  Since requests are\nhandled in-order and events are delivered in-order, this can\nbe used as a barrier to ensure all previous requests and the\nresulting events have been handled.\n\nThe object returned by this request will be destroyed by the\ncompositor after the callback is fired and as such the client must not\nattempt to use it after that point.\n\nThe callback_data passed in the callback is the event serial."]
        pub fn sync(&self) -> Main<super::wl_callback::WlCallback> {
            let msg = Request::Sync {};
            self.0.send(msg, None).unwrap()
        }
        #[doc = "get global registry object\n\nThis request creates a registry object that allows the client\nto list and bind the global objects available from the\ncompositor.\n\nIt should be noted that the server side resources consumed in\nresponse to a get_registry request can only be released when the\nclient disconnects, not when the client side proxy is destroyed.\nTherefore, clients should invoke get_registry as infrequently as\npossible to avoid wasting memory."]
        pub fn get_registry(&self) -> Main<super::wl_registry::WlRegistry> {
            let msg = Request::GetRegistry {};
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SYNC_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_REGISTRY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_ERROR_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DELETE_ID_SINCE: u32 = 1u32;
    static mut wl_display_requests_sync_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_callback::wl_callback_interface as *const wl_interface }];
    static mut wl_display_requests_get_registry_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_registry::wl_registry_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_display_requests: [wl_message; 2] = [
        wl_message {
            name: b"sync\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &wl_display_requests_sync_types as *const _ },
        },
        wl_message {
            name: b"get_registry\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &wl_display_requests_get_registry_types as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_display_events: [wl_message; 2] = [
        wl_message {
            name: b"error\0" as *const u8 as *const c_char,
            signature: b"ous\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"delete_id\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_display_interface: wl_interface = wl_interface {
        name: b"wl_display\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &wl_display_requests as *const _ },
        event_count: 2,
        events: unsafe { &wl_display_events as *const _ },
    };
}
#[doc = "global registry object\n\nThe singleton global registry object.  The server has a number of\nglobal objects that are available to all clients.  These objects\ntypically represent an actual object in the server (for example,\nan input device) or they are singleton objects that provide\nextension functionality.\n\nWhen a client creates a registry object, the registry object\nwill emit a global event for each global currently in the\nregistry.  Globals come and go as a result of device or\nmonitor hotplugs, reconfiguration or other events, and the\nregistry will send out global and global_remove events to\nkeep the client up to date with the changes.  To mark the end\nof the initial burst of events, the client can use the\nwl_display.sync request immediately after calling\nwl_display.get_registry.\n\nA client can bind to a global object by using the bind\nrequest.  This creates a client-side handle that lets the object\nemit events to the client and lets the client invoke requests on\nthe object."]
pub mod wl_registry {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "bind an object to the display\n\nBinds a new, client-created object to the server using the\nspecified name as the identifier."]
        Bind { name: u32, id: (String, u32) },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "bind",
            since: 1,
            signature: &[super::ArgumentType::Uint, super::ArgumentType::NewId],
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
                Request::Bind { .. } => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Bind { .. } => 1,
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
                Request::Bind { name, id } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![
                        Argument::Uint(name),
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(id.0.into())
                        })),
                        Argument::Uint(id.1),
                        Argument::NewId(0),
                    ],
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
                Request::Bind { name, id } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = name;
                    let _arg_1_s = ::std::ffi::CString::new(id.0).unwrap();
                    _args_array[1].s = _arg_1_s.as_ptr();
                    _args_array[2].u = id.1;
                    _args_array[3].o = ::std::ptr::null_mut();
                    f(0, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "announce global object\n\nNotify the client of global objects.\n\nThe event notifies the client that a global object with\nthe given name is now available, and it implements the\ngiven version of the given interface."]
        Global {
            name: u32,
            interface: String,
            version: u32,
        },
        #[doc = "announce removal of global object\n\nNotify the client of removed global objects.\n\nThis event notifies the client that the global identified\nby name is no longer available.  If the client bound to\nthe global using the bind request, the client should now\ndestroy that object.\n\nThe object remains valid and requests to the object will be\nignored until the client destroys it, to avoid races between\nthe global going away and a client sending a request to it."]
        GlobalRemove { name: u32 },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "global",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Str,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "global_remove",
                since: 1,
                signature: &[super::ArgumentType::Uint],
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
                Event::Global { .. } => 0,
                Event::GlobalRemove { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Global { .. } => 1,
                Event::GlobalRemove { .. } => 1,
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
                    Ok(Event::Global {
                        name: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        interface: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                        version: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::GlobalRemove {
                        name: {
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
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Global {
                        name: _args[0].u,
                        interface: ::std::ffi::CStr::from_ptr(_args[1].s)
                            .to_string_lossy()
                            .into_owned(),
                        version: _args[2].u,
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::GlobalRemove { name: _args[0].u })
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
    pub struct WlRegistry(Proxy<WlRegistry>);
    impl AsRef<Proxy<WlRegistry>> for WlRegistry {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlRegistry>> for WlRegistry {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlRegistry(value)
        }
    }
    impl From<WlRegistry> for Proxy<WlRegistry> {
        #[inline]
        fn from(value: WlRegistry) -> Self {
            value.0
        }
    }
    impl Interface for WlRegistry {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_registry";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_registry_interface }
        }
    }
    impl WlRegistry {
        #[doc = "bind an object to the display\n\nBinds a new, client-created object to the server using the\nspecified name as the identifier."]
        pub fn bind<T: Interface + From<Proxy<T>> + AsRef<Proxy<T>>>(
            &self,
            version: u32,
            name: u32,
        ) -> Main<T> {
            let msg = Request::Bind {
                name: name,
                id: (T::NAME.into(), version),
            };
            self.0.send(msg, Some(version)).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_BIND_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_GLOBAL_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_GLOBAL_REMOVE_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_registry_requests: [wl_message; 1] = [wl_message {
        name: b"bind\0" as *const u8 as *const c_char,
        signature: b"usun\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_registry_events: [wl_message; 2] = [
        wl_message {
            name: b"global\0" as *const u8 as *const c_char,
            signature: b"usu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"global_remove\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_registry_interface: wl_interface = wl_interface {
        name: b"wl_registry\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 1,
        requests: unsafe { &wl_registry_requests as *const _ },
        event_count: 2,
        events: unsafe { &wl_registry_events as *const _ },
    };
}
#[doc = "callback object\n\nClients can handle the 'done' event to get notified when\nthe related request is done."]
pub mod wl_callback {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {}
    impl super::MessageGroup for Request {
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
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {}
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
            match self {}
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "done event\n\nNotify the client when the related request is done.\n\nThis is a destructor, once received this object cannot be used any longer."]
        Done { callback_data: u32 },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "done",
            since: 1,
            signature: &[super::ArgumentType::Uint],
            destructor: true,
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Event::Done { .. } => true,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::Done { .. } => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Done { .. } => 1,
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
                    Ok(Event::Done {
                        callback_data: {
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
                    Ok(Event::Done {
                        callback_data: _args[0].u,
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
    pub struct WlCallback(Proxy<WlCallback>);
    impl AsRef<Proxy<WlCallback>> for WlCallback {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlCallback>> for WlCallback {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlCallback(value)
        }
    }
    impl From<WlCallback> for Proxy<WlCallback> {
        #[inline]
        fn from(value: WlCallback) -> Self {
            value.0
        }
    }
    impl Interface for WlCallback {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_callback";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_callback_interface }
        }
    }
    impl WlCallback {}
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DONE_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_callback_events: [wl_message; 1] = [wl_message {
        name: b"done\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_callback_interface: wl_interface = wl_interface {
        name: b"wl_callback\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 0,
        requests: NULLPTR as *const wl_message,
        event_count: 1,
        events: unsafe { &wl_callback_events as *const _ },
    };
}
#[doc = "the compositor singleton\n\nA compositor.  This object is a singleton global.  The\ncompositor is in charge of combining the contents of multiple\nsurfaces into one displayable output."]
pub mod wl_compositor {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "create new surface\n\nAsk the compositor to create a new surface."]
        CreateSurface {},
        #[doc = "create new region\n\nAsk the compositor to create a new region."]
        CreateRegion {},
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "create_surface",
                since: 1,
                signature: &[super::ArgumentType::NewId],
                destructor: false,
            },
            super::MessageDesc {
                name: "create_region",
                since: 1,
                signature: &[super::ArgumentType::NewId],
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
                Request::CreateSurface { .. } => 0,
                Request::CreateRegion { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::CreateSurface { .. } => 1,
                Request::CreateRegion { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<super::wl_surface::WlSurface>(
                    version,
                    meta.child(),
                )),
                1 => Some(Object::from_interface::<super::wl_region::WlRegion>(
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
                Request::CreateSurface {} => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![Argument::NewId(0),],
                },
                Request::CreateRegion {} => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::NewId(0),],
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
                Request::CreateSurface {} => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    f(0, &mut _args_array)
                }
                Request::CreateRegion {} => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
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
    pub struct WlCompositor(Proxy<WlCompositor>);
    impl AsRef<Proxy<WlCompositor>> for WlCompositor {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlCompositor>> for WlCompositor {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlCompositor(value)
        }
    }
    impl From<WlCompositor> for Proxy<WlCompositor> {
        #[inline]
        fn from(value: WlCompositor) -> Self {
            value.0
        }
    }
    impl Interface for WlCompositor {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_compositor";
        const VERSION: u32 = 4;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_compositor_interface }
        }
    }
    impl WlCompositor {
        #[doc = "create new surface\n\nAsk the compositor to create a new surface."]
        pub fn create_surface(&self) -> Main<super::wl_surface::WlSurface> {
            let msg = Request::CreateSurface {};
            self.0.send(msg, None).unwrap()
        }
        #[doc = "create new region\n\nAsk the compositor to create a new region."]
        pub fn create_region(&self) -> Main<super::wl_region::WlRegion> {
            let msg = Request::CreateRegion {};
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CREATE_SURFACE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CREATE_REGION_SINCE: u32 = 1u32;
    static mut wl_compositor_requests_create_surface_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface }];
    static mut wl_compositor_requests_create_region_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_region::wl_region_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_compositor_requests: [wl_message; 2] = [
        wl_message {
            name: b"create_surface\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &wl_compositor_requests_create_surface_types as *const _ },
        },
        wl_message {
            name: b"create_region\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &wl_compositor_requests_create_region_types as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_compositor_interface: wl_interface = wl_interface {
        name: b"wl_compositor\0" as *const u8 as *const c_char,
        version: 4,
        request_count: 2,
        requests: unsafe { &wl_compositor_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "a shared memory pool\n\nThe wl_shm_pool object encapsulates a piece of memory shared\nbetween the compositor and client.  Through the wl_shm_pool\nobject, the client can allocate shared memory wl_buffer objects.\nAll objects created through the same pool share the same\nunderlying mapped memory. Reusing the mapped memory avoids the\nsetup/teardown overhead and is useful when interactively resizing\na surface or for many small buffers."]
pub mod wl_shm_pool {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "create a buffer from the pool\n\nCreate a wl_buffer object from the pool.\n\nThe buffer is created offset bytes into the pool and has\nwidth and height as specified.  The stride argument specifies\nthe number of bytes from the beginning of one row to the beginning\nof the next.  The format is the pixel format of the buffer and\nmust be one of those advertised through the wl_shm.format event.\n\nA buffer will keep a reference to the pool it was created from\nso it is valid to destroy the pool immediately after creating\na buffer from it."]
        CreateBuffer {
            offset: i32,
            width: i32,
            height: i32,
            stride: i32,
            format: super::wl_shm::Format,
        },
        #[doc = "destroy the pool\n\nDestroy the shared memory pool.\n\nThe mmapped memory will be released when all\nbuffers that have been created from this pool\nare gone.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "change the size of the pool mapping\n\nThis request will cause the server to remap the backing memory\nfor the pool from the file descriptor passed when the pool was\ncreated, but using the new size.  This request can only be\nused to make the pool bigger."]
        Resize { size: i32 },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "create_buffer",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
                destructor: true,
            },
            super::MessageDesc {
                name: "resize",
                since: 1,
                signature: &[super::ArgumentType::Int],
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
                Request::CreateBuffer { .. } => 0,
                Request::Destroy => 1,
                Request::Resize { .. } => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::CreateBuffer { .. } => 1,
                Request::Destroy => 1,
                Request::Resize { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<super::wl_buffer::WlBuffer>(
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
                Request::CreateBuffer {
                    offset,
                    width,
                    height,
                    stride,
                    format,
                } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![
                        Argument::NewId(0),
                        Argument::Int(offset),
                        Argument::Int(width),
                        Argument::Int(height),
                        Argument::Int(stride),
                        Argument::Uint(format.to_raw()),
                    ],
                },
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![],
                },
                Request::Resize { size } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![Argument::Int(size),],
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
                Request::CreateBuffer {
                    offset,
                    width,
                    height,
                    stride,
                    format,
                } => {
                    let mut _args_array: [wl_argument; 6] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].i = offset;
                    _args_array[2].i = width;
                    _args_array[3].i = height;
                    _args_array[4].i = stride;
                    _args_array[5].u = format.to_raw();
                    f(0, &mut _args_array)
                }
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(1, &mut _args_array)
                }
                Request::Resize { size } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = size;
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
    pub struct WlShmPool(Proxy<WlShmPool>);
    impl AsRef<Proxy<WlShmPool>> for WlShmPool {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlShmPool>> for WlShmPool {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlShmPool(value)
        }
    }
    impl From<WlShmPool> for Proxy<WlShmPool> {
        #[inline]
        fn from(value: WlShmPool) -> Self {
            value.0
        }
    }
    impl Interface for WlShmPool {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_shm_pool";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_shm_pool_interface }
        }
    }
    impl WlShmPool {
        #[doc = "create a buffer from the pool\n\nCreate a wl_buffer object from the pool.\n\nThe buffer is created offset bytes into the pool and has\nwidth and height as specified.  The stride argument specifies\nthe number of bytes from the beginning of one row to the beginning\nof the next.  The format is the pixel format of the buffer and\nmust be one of those advertised through the wl_shm.format event.\n\nA buffer will keep a reference to the pool it was created from\nso it is valid to destroy the pool immediately after creating\na buffer from it."]
        pub fn create_buffer(
            &self,
            offset: i32,
            width: i32,
            height: i32,
            stride: i32,
            format: super::wl_shm::Format,
        ) -> Main<super::wl_buffer::WlBuffer> {
            let msg = Request::CreateBuffer {
                offset: offset,
                width: width,
                height: height,
                stride: stride,
                format: format,
            };
            self.0.send(msg, None).unwrap()
        }
        #[doc = "destroy the pool\n\nDestroy the shared memory pool.\n\nThe mmapped memory will be released when all\nbuffers that have been created from this pool\nare gone.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "change the size of the pool mapping\n\nThis request will cause the server to remap the backing memory\nfor the pool from the file descriptor passed when the pool was\ncreated, but using the new size.  This request can only be\nused to make the pool bigger."]
        pub fn resize(&self, size: i32) -> () {
            let msg = Request::Resize { size: size };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CREATE_BUFFER_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_RESIZE_SINCE: u32 = 1u32;
    static mut wl_shm_pool_requests_create_buffer_types: [*const wl_interface; 6] = [
        unsafe { &super::wl_buffer::wl_buffer_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_shm_pool_requests: [wl_message; 3] = [
        wl_message {
            name: b"create_buffer\0" as *const u8 as *const c_char,
            signature: b"niiiiu\0" as *const u8 as *const c_char,
            types: unsafe { &wl_shm_pool_requests_create_buffer_types as *const _ },
        },
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"resize\0" as *const u8 as *const c_char,
            signature: b"i\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_shm_pool_interface: wl_interface = wl_interface {
        name: b"wl_shm_pool\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 3,
        requests: unsafe { &wl_shm_pool_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "shared memory support\n\nA singleton global object that provides support for shared\nmemory.\n\nClients can create wl_shm_pool objects using the create_pool\nrequest.\n\nAt connection setup time, the wl_shm object emits one or more\nformat events to inform clients about the valid pixel formats\nthat can be used for buffers."]
pub mod wl_shm {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[doc = "wl_shm error values\n\nThese errors can be emitted in response to wl_shm requests."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Error {
        #[doc = "buffer format is not known"]
        InvalidFormat = 0,
        #[doc = "invalid size or stride during pool or buffer creation"]
        InvalidStride = 1,
        #[doc = "mmapping the file descriptor failed"]
        InvalidFd = 2,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidFormat),
                1 => Some(Error::InvalidStride),
                2 => Some(Error::InvalidFd),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[doc = "pixel formats\n\nThis describes the memory layout of an individual pixel.\n\nAll renderers should support argb8888 and xrgb8888 but any other\nformats are optional and may not be supported by the particular\nrenderer in use.\n\nThe drm format codes match the macros defined in drm_fourcc.h.\nThe formats actually supported by the compositor will be\nreported by the format event."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Format {
        #[doc = "32-bit ARGB format, [31:0] A:R:G:B 8:8:8:8 little endian"]
        Argb8888 = 0,
        #[doc = "32-bit RGB format, [31:0] x:R:G:B 8:8:8:8 little endian"]
        Xrgb8888 = 1,
        #[doc = "8-bit color index format, [7:0] C"]
        C8 = 538982467,
        #[doc = "8-bit RGB format, [7:0] R:G:B 3:3:2"]
        Rgb332 = 943867730,
        #[doc = "8-bit BGR format, [7:0] B:G:R 2:3:3"]
        Bgr233 = 944916290,
        #[doc = "16-bit xRGB format, [15:0] x:R:G:B 4:4:4:4 little endian"]
        Xrgb4444 = 842093144,
        #[doc = "16-bit xBGR format, [15:0] x:B:G:R 4:4:4:4 little endian"]
        Xbgr4444 = 842089048,
        #[doc = "16-bit RGBx format, [15:0] R:G:B:x 4:4:4:4 little endian"]
        Rgbx4444 = 842094674,
        #[doc = "16-bit BGRx format, [15:0] B:G:R:x 4:4:4:4 little endian"]
        Bgrx4444 = 842094658,
        #[doc = "16-bit ARGB format, [15:0] A:R:G:B 4:4:4:4 little endian"]
        Argb4444 = 842093121,
        #[doc = "16-bit ABGR format, [15:0] A:B:G:R 4:4:4:4 little endian"]
        Abgr4444 = 842089025,
        #[doc = "16-bit RBGA format, [15:0] R:G:B:A 4:4:4:4 little endian"]
        Rgba4444 = 842088786,
        #[doc = "16-bit BGRA format, [15:0] B:G:R:A 4:4:4:4 little endian"]
        Bgra4444 = 842088770,
        #[doc = "16-bit xRGB format, [15:0] x:R:G:B 1:5:5:5 little endian"]
        Xrgb1555 = 892424792,
        #[doc = "16-bit xBGR 1555 format, [15:0] x:B:G:R 1:5:5:5 little endian"]
        Xbgr1555 = 892420696,
        #[doc = "16-bit RGBx 5551 format, [15:0] R:G:B:x 5:5:5:1 little endian"]
        Rgbx5551 = 892426322,
        #[doc = "16-bit BGRx 5551 format, [15:0] B:G:R:x 5:5:5:1 little endian"]
        Bgrx5551 = 892426306,
        #[doc = "16-bit ARGB 1555 format, [15:0] A:R:G:B 1:5:5:5 little endian"]
        Argb1555 = 892424769,
        #[doc = "16-bit ABGR 1555 format, [15:0] A:B:G:R 1:5:5:5 little endian"]
        Abgr1555 = 892420673,
        #[doc = "16-bit RGBA 5551 format, [15:0] R:G:B:A 5:5:5:1 little endian"]
        Rgba5551 = 892420434,
        #[doc = "16-bit BGRA 5551 format, [15:0] B:G:R:A 5:5:5:1 little endian"]
        Bgra5551 = 892420418,
        #[doc = "16-bit RGB 565 format, [15:0] R:G:B 5:6:5 little endian"]
        Rgb565 = 909199186,
        #[doc = "16-bit BGR 565 format, [15:0] B:G:R 5:6:5 little endian"]
        Bgr565 = 909199170,
        #[doc = "24-bit RGB format, [23:0] R:G:B little endian"]
        Rgb888 = 875710290,
        #[doc = "24-bit BGR format, [23:0] B:G:R little endian"]
        Bgr888 = 875710274,
        #[doc = "32-bit xBGR format, [31:0] x:B:G:R 8:8:8:8 little endian"]
        Xbgr8888 = 875709016,
        #[doc = "32-bit RGBx format, [31:0] R:G:B:x 8:8:8:8 little endian"]
        Rgbx8888 = 875714642,
        #[doc = "32-bit BGRx format, [31:0] B:G:R:x 8:8:8:8 little endian"]
        Bgrx8888 = 875714626,
        #[doc = "32-bit ABGR format, [31:0] A:B:G:R 8:8:8:8 little endian"]
        Abgr8888 = 875708993,
        #[doc = "32-bit RGBA format, [31:0] R:G:B:A 8:8:8:8 little endian"]
        Rgba8888 = 875708754,
        #[doc = "32-bit BGRA format, [31:0] B:G:R:A 8:8:8:8 little endian"]
        Bgra8888 = 875708738,
        #[doc = "32-bit xRGB format, [31:0] x:R:G:B 2:10:10:10 little endian"]
        Xrgb2101010 = 808669784,
        #[doc = "32-bit xBGR format, [31:0] x:B:G:R 2:10:10:10 little endian"]
        Xbgr2101010 = 808665688,
        #[doc = "32-bit RGBx format, [31:0] R:G:B:x 10:10:10:2 little endian"]
        Rgbx1010102 = 808671314,
        #[doc = "32-bit BGRx format, [31:0] B:G:R:x 10:10:10:2 little endian"]
        Bgrx1010102 = 808671298,
        #[doc = "32-bit ARGB format, [31:0] A:R:G:B 2:10:10:10 little endian"]
        Argb2101010 = 808669761,
        #[doc = "32-bit ABGR format, [31:0] A:B:G:R 2:10:10:10 little endian"]
        Abgr2101010 = 808665665,
        #[doc = "32-bit RGBA format, [31:0] R:G:B:A 10:10:10:2 little endian"]
        Rgba1010102 = 808665426,
        #[doc = "32-bit BGRA format, [31:0] B:G:R:A 10:10:10:2 little endian"]
        Bgra1010102 = 808665410,
        #[doc = "packed YCbCr format, [31:0] Cr0:Y1:Cb0:Y0 8:8:8:8 little endian"]
        Yuyv = 1448695129,
        #[doc = "packed YCbCr format, [31:0] Cb0:Y1:Cr0:Y0 8:8:8:8 little endian"]
        Yvyu = 1431918169,
        #[doc = "packed YCbCr format, [31:0] Y1:Cr0:Y0:Cb0 8:8:8:8 little endian"]
        Uyvy = 1498831189,
        #[doc = "packed YCbCr format, [31:0] Y1:Cb0:Y0:Cr0 8:8:8:8 little endian"]
        Vyuy = 1498765654,
        #[doc = "packed AYCbCr format, [31:0] A:Y:Cb:Cr 8:8:8:8 little endian"]
        Ayuv = 1448433985,
        #[doc = "2 plane YCbCr Cr:Cb format, 2x2 subsampled Cr:Cb plane"]
        Nv12 = 842094158,
        #[doc = "2 plane YCbCr Cb:Cr format, 2x2 subsampled Cb:Cr plane"]
        Nv21 = 825382478,
        #[doc = "2 plane YCbCr Cr:Cb format, 2x1 subsampled Cr:Cb plane"]
        Nv16 = 909203022,
        #[doc = "2 plane YCbCr Cb:Cr format, 2x1 subsampled Cb:Cr plane"]
        Nv61 = 825644622,
        #[doc = "3 plane YCbCr format, 4x4 subsampled Cb (1) and Cr (2) planes"]
        Yuv410 = 961959257,
        #[doc = "3 plane YCbCr format, 4x4 subsampled Cr (1) and Cb (2) planes"]
        Yvu410 = 961893977,
        #[doc = "3 plane YCbCr format, 4x1 subsampled Cb (1) and Cr (2) planes"]
        Yuv411 = 825316697,
        #[doc = "3 plane YCbCr format, 4x1 subsampled Cr (1) and Cb (2) planes"]
        Yvu411 = 825316953,
        #[doc = "3 plane YCbCr format, 2x2 subsampled Cb (1) and Cr (2) planes"]
        Yuv420 = 842093913,
        #[doc = "3 plane YCbCr format, 2x2 subsampled Cr (1) and Cb (2) planes"]
        Yvu420 = 842094169,
        #[doc = "3 plane YCbCr format, 2x1 subsampled Cb (1) and Cr (2) planes"]
        Yuv422 = 909202777,
        #[doc = "3 plane YCbCr format, 2x1 subsampled Cr (1) and Cb (2) planes"]
        Yvu422 = 909203033,
        #[doc = "3 plane YCbCr format, non-subsampled Cb (1) and Cr (2) planes"]
        Yuv444 = 875713881,
        #[doc = "3 plane YCbCr format, non-subsampled Cr (1) and Cb (2) planes"]
        Yvu444 = 875714137,
    }
    impl Format {
        pub fn from_raw(n: u32) -> Option<Format> {
            match n {
                0 => Some(Format::Argb8888),
                1 => Some(Format::Xrgb8888),
                538982467 => Some(Format::C8),
                943867730 => Some(Format::Rgb332),
                944916290 => Some(Format::Bgr233),
                842093144 => Some(Format::Xrgb4444),
                842089048 => Some(Format::Xbgr4444),
                842094674 => Some(Format::Rgbx4444),
                842094658 => Some(Format::Bgrx4444),
                842093121 => Some(Format::Argb4444),
                842089025 => Some(Format::Abgr4444),
                842088786 => Some(Format::Rgba4444),
                842088770 => Some(Format::Bgra4444),
                892424792 => Some(Format::Xrgb1555),
                892420696 => Some(Format::Xbgr1555),
                892426322 => Some(Format::Rgbx5551),
                892426306 => Some(Format::Bgrx5551),
                892424769 => Some(Format::Argb1555),
                892420673 => Some(Format::Abgr1555),
                892420434 => Some(Format::Rgba5551),
                892420418 => Some(Format::Bgra5551),
                909199186 => Some(Format::Rgb565),
                909199170 => Some(Format::Bgr565),
                875710290 => Some(Format::Rgb888),
                875710274 => Some(Format::Bgr888),
                875709016 => Some(Format::Xbgr8888),
                875714642 => Some(Format::Rgbx8888),
                875714626 => Some(Format::Bgrx8888),
                875708993 => Some(Format::Abgr8888),
                875708754 => Some(Format::Rgba8888),
                875708738 => Some(Format::Bgra8888),
                808669784 => Some(Format::Xrgb2101010),
                808665688 => Some(Format::Xbgr2101010),
                808671314 => Some(Format::Rgbx1010102),
                808671298 => Some(Format::Bgrx1010102),
                808669761 => Some(Format::Argb2101010),
                808665665 => Some(Format::Abgr2101010),
                808665426 => Some(Format::Rgba1010102),
                808665410 => Some(Format::Bgra1010102),
                1448695129 => Some(Format::Yuyv),
                1431918169 => Some(Format::Yvyu),
                1498831189 => Some(Format::Uyvy),
                1498765654 => Some(Format::Vyuy),
                1448433985 => Some(Format::Ayuv),
                842094158 => Some(Format::Nv12),
                825382478 => Some(Format::Nv21),
                909203022 => Some(Format::Nv16),
                825644622 => Some(Format::Nv61),
                961959257 => Some(Format::Yuv410),
                961893977 => Some(Format::Yvu410),
                825316697 => Some(Format::Yuv411),
                825316953 => Some(Format::Yvu411),
                842093913 => Some(Format::Yuv420),
                842094169 => Some(Format::Yvu420),
                909202777 => Some(Format::Yuv422),
                909203033 => Some(Format::Yvu422),
                875713881 => Some(Format::Yuv444),
                875714137 => Some(Format::Yvu444),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "create a shm pool\n\nCreate a new wl_shm_pool object.\n\nThe pool can be used to create shared memory based buffer\nobjects.  The server will mmap size bytes of the passed file\ndescriptor, to use as backing memory for the pool."]
        CreatePool {
            fd: ::std::os::unix::io::RawFd,
            size: i32,
        },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "create_pool",
            since: 1,
            signature: &[
                super::ArgumentType::NewId,
                super::ArgumentType::Fd,
                super::ArgumentType::Int,
            ],
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
                Request::CreatePool { .. } => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::CreatePool { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<super::wl_shm_pool::WlShmPool>(
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
                Request::CreatePool { fd, size } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![Argument::NewId(0), Argument::Fd(fd), Argument::Int(size),],
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
                Request::CreatePool { fd, size } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].h = fd;
                    _args_array[2].i = size;
                    f(0, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "pixel format description\n\nInforms the client about a valid pixel format that\ncan be used for buffers. Known formats include\nargb8888 and xrgb8888."]
        Format { format: Format },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "format",
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
                Event::Format { .. } => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Format { .. } => 1,
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
                    Ok(Event::Format {
                        format: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                Format::from_raw(val).ok_or(())?
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
                    Ok(Event::Format {
                        format: Format::from_raw(_args[0].u).ok_or(())?,
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
    pub struct WlShm(Proxy<WlShm>);
    impl AsRef<Proxy<WlShm>> for WlShm {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlShm>> for WlShm {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlShm(value)
        }
    }
    impl From<WlShm> for Proxy<WlShm> {
        #[inline]
        fn from(value: WlShm) -> Self {
            value.0
        }
    }
    impl Interface for WlShm {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_shm";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_shm_interface }
        }
    }
    impl WlShm {
        #[doc = "create a shm pool\n\nCreate a new wl_shm_pool object.\n\nThe pool can be used to create shared memory based buffer\nobjects.  The server will mmap size bytes of the passed file\ndescriptor, to use as backing memory for the pool."]
        pub fn create_pool(
            &self,
            fd: ::std::os::unix::io::RawFd,
            size: i32,
        ) -> Main<super::wl_shm_pool::WlShmPool> {
            let msg = Request::CreatePool { fd: fd, size: size };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CREATE_POOL_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FORMAT_SINCE: u32 = 1u32;
    static mut wl_shm_requests_create_pool_types: [*const wl_interface; 3] = [
        unsafe { &super::wl_shm_pool::wl_shm_pool_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_shm_requests: [wl_message; 1] = [wl_message {
        name: b"create_pool\0" as *const u8 as *const c_char,
        signature: b"nhi\0" as *const u8 as *const c_char,
        types: unsafe { &wl_shm_requests_create_pool_types as *const _ },
    }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_shm_events: [wl_message; 1] = [wl_message {
        name: b"format\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_shm_interface: wl_interface = wl_interface {
        name: b"wl_shm\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 1,
        requests: unsafe { &wl_shm_requests as *const _ },
        event_count: 1,
        events: unsafe { &wl_shm_events as *const _ },
    };
}
#[doc = "content for a wl_surface\n\nA buffer provides the content for a wl_surface. Buffers are\ncreated through factory interfaces such as wl_drm, wl_shm or\nsimilar. It has a width and a height and can be attached to a\nwl_surface, but the mechanism by which a client provides and\nupdates the contents is defined by the buffer factory interface."]
pub mod wl_buffer {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy a buffer\n\nDestroy a buffer. If and how you need to release the backing\nstorage is defined by the buffer factory interface.\n\nFor possible side-effects to a surface, see wl_surface.attach.\n\nThis is a destructor, once sent this object cannot be used any longer."]
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
        #[doc = "compositor releases buffer\n\nSent when this wl_buffer is no longer used by the compositor.\nThe client is now free to reuse or destroy this buffer and its\nbacking storage.\n\nIf a client receives a release event before the frame callback\nrequested in the same wl_surface.commit that attaches this\nwl_buffer to a surface, then the client is immediately free to\nreuse the buffer and its backing storage, and does not need a\nsecond buffer for the next surface content update. Typically\nthis is possible, when the compositor maintains a copy of the\nwl_surface contents, e.g. as a GL texture. This is an important\noptimization for GL(ES) compositors with wl_shm clients."]
        Release,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "release",
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
                Event::Release => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Release => 1,
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
                0 => Ok(Event::Release),
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
                0 => Ok(Event::Release),
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
    pub struct WlBuffer(Proxy<WlBuffer>);
    impl AsRef<Proxy<WlBuffer>> for WlBuffer {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlBuffer>> for WlBuffer {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlBuffer(value)
        }
    }
    impl From<WlBuffer> for Proxy<WlBuffer> {
        #[inline]
        fn from(value: WlBuffer) -> Self {
            value.0
        }
    }
    impl Interface for WlBuffer {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_buffer";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_buffer_interface }
        }
    }
    impl WlBuffer {
        #[doc = "destroy a buffer\n\nDestroy a buffer. If and how you need to release the backing\nstorage is defined by the buffer factory interface.\n\nFor possible side-effects to a surface, see wl_surface.attach.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_RELEASE_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_buffer_requests: [wl_message; 1] = [wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_buffer_events: [wl_message; 1] = [wl_message {
        name: b"release\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_buffer_interface: wl_interface = wl_interface {
        name: b"wl_buffer\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 1,
        requests: unsafe { &wl_buffer_requests as *const _ },
        event_count: 1,
        events: unsafe { &wl_buffer_events as *const _ },
    };
}
#[doc = "offer to transfer data\n\nA wl_data_offer represents a piece of data offered for transfer\nby another client (the source client).  It is used by the\ncopy-and-paste and drag-and-drop mechanisms.  The offer\ndescribes the different mime types that the data can be\nconverted to and provides the mechanism for transferring the\ndata directly from the source client."]
pub mod wl_data_offer {
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
        #[doc = "finish request was called untimely"]
        InvalidFinish = 0,
        #[doc = "action mask contains invalid values"]
        InvalidActionMask = 1,
        #[doc = "action argument has an invalid value"]
        InvalidAction = 2,
        #[doc = "offer doesn't accept this request"]
        InvalidOffer = 3,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidFinish),
                1 => Some(Error::InvalidActionMask),
                2 => Some(Error::InvalidAction),
                3 => Some(Error::InvalidOffer),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "accept one of the offered mime types\n\nIndicate that the client can accept the given mime type, or\nNULL for not accepted.\n\nFor objects of version 2 or older, this request is used by the\nclient to give feedback whether the client can receive the given\nmime type, or NULL if none is accepted; the feedback does not\ndetermine whether the drag-and-drop operation succeeds or not.\n\nFor objects of version 3 or newer, this request determines the\nfinal result of the drag-and-drop operation. If the end result\nis that no mime types were accepted, the drag-and-drop operation\nwill be cancelled and the corresponding drag source will receive\nwl_data_source.cancelled. Clients may still use this event in\nconjunction with wl_data_source.action for feedback."]
        Accept {
            serial: u32,
            mime_type: Option<String>,
        },
        #[doc = "request that the data is transferred\n\nTo transfer the offered data, the client issues this request\nand indicates the mime type it wants to receive.  The transfer\nhappens through the passed file descriptor (typically created\nwith the pipe system call).  The source client writes the data\nin the mime type representation requested and then closes the\nfile descriptor.\n\nThe receiving client reads from the read end of the pipe until\nEOF and then closes its end, at which point the transfer is\ncomplete.\n\nThis request may happen multiple times for different mime types,\nboth before and after wl_data_device.drop. Drag-and-drop destination\nclients may preemptively fetch data or examine it more closely to\ndetermine acceptance."]
        Receive {
            mime_type: String,
            fd: ::std::os::unix::io::RawFd,
        },
        #[doc = "destroy data offer\n\nDestroy the data offer.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "the offer will no longer be used\n\nNotifies the compositor that the drag destination successfully\nfinished the drag-and-drop operation.\n\nUpon receiving this request, the compositor will emit\nwl_data_source.dnd_finished on the drag source client.\n\nIt is a client error to perform other requests than\nwl_data_offer.destroy after this one. It is also an error to perform\nthis request after a NULL mime type has been set in\nwl_data_offer.accept or no action was received through\nwl_data_offer.action.\n\nOnly available since version 3 of the interface"]
        Finish,
        #[doc = "set the available/preferred drag-and-drop actions\n\nSets the actions that the destination side client supports for\nthis operation. This request may trigger the emission of\nwl_data_source.action and wl_data_offer.action events if the compositor\nneeds to change the selected action.\n\nThis request can be called multiple times throughout the\ndrag-and-drop operation, typically in response to wl_data_device.enter\nor wl_data_device.motion events.\n\nThis request determines the final result of the drag-and-drop\noperation. If the end result is that no action is accepted,\nthe drag source will receive wl_drag_source.cancelled.\n\nThe dnd_actions argument must contain only values expressed in the\nwl_data_device_manager.dnd_actions enum, and the preferred_action\nargument must only contain one of those values set, otherwise it\nwill result in a protocol error.\n\nWhile managing an \"ask\" action, the destination drag-and-drop client\nmay perform further wl_data_offer.receive requests, and is expected\nto perform one last wl_data_offer.set_actions request with a preferred\naction other than \"ask\" (and optionally wl_data_offer.accept) before\nrequesting wl_data_offer.finish, in order to convey the action selected\nby the user. If the preferred action is not in the\nwl_data_offer.source_actions mask, an error will be raised.\n\nIf the \"ask\" action is dismissed (e.g. user cancellation), the client\nis expected to perform wl_data_offer.destroy right away.\n\nThis request can only be made on drag-and-drop offers, a protocol error\nwill be raised otherwise.\n\nOnly available since version 3 of the interface"]
        SetActions {
            dnd_actions: u32,
            preferred_action: u32,
        },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "accept",
                since: 1,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Str],
                destructor: false,
            },
            super::MessageDesc {
                name: "receive",
                since: 1,
                signature: &[super::ArgumentType::Str, super::ArgumentType::Fd],
                destructor: false,
            },
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
                destructor: true,
            },
            super::MessageDesc {
                name: "finish",
                since: 3,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_actions",
                since: 3,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Uint],
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
                Request::Accept { .. } => 0,
                Request::Receive { .. } => 1,
                Request::Destroy => 2,
                Request::Finish => 3,
                Request::SetActions { .. } => 4,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Accept { .. } => 1,
                Request::Receive { .. } => 1,
                Request::Destroy => 1,
                Request::Finish => 3,
                Request::SetActions { .. } => 3,
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
                Request::Accept { serial, mime_type } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![
                        Argument::Uint(serial),
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(
                                mime_type.map(Into::into).unwrap_or_else(Vec::new),
                            )
                        })),
                    ],
                },
                Request::Receive { mime_type, fd } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(mime_type.into())
                        })),
                        Argument::Fd(fd),
                    ],
                },
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![],
                },
                Request::Finish => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: smallvec![],
                },
                Request::SetActions {
                    dnd_actions,
                    preferred_action,
                } => Message {
                    sender_id: sender_id,
                    opcode: 4,
                    args: smallvec![
                        Argument::Uint(dnd_actions),
                        Argument::Uint(preferred_action),
                    ],
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
                Request::Accept { serial, mime_type } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    let _arg_1 = mime_type.map(|s| ::std::ffi::CString::new(s).unwrap());
                    _args_array[1].s = _arg_1.map(|s| s.as_ptr()).unwrap_or(::std::ptr::null());
                    f(0, &mut _args_array)
                }
                Request::Receive { mime_type, fd } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(mime_type).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    _args_array[1].h = fd;
                    f(1, &mut _args_array)
                }
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(2, &mut _args_array)
                }
                Request::Finish => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(3, &mut _args_array)
                }
                Request::SetActions {
                    dnd_actions,
                    preferred_action,
                } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = dnd_actions;
                    _args_array[1].u = preferred_action;
                    f(4, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "advertise offered mime type\n\nSent immediately after creating the wl_data_offer object.  One\nevent per offered mime type."]
        Offer { mime_type: String },
        #[doc = "notify the source-side available actions\n\nThis event indicates the actions offered by the data source. It\nwill be sent right after wl_data_device.enter, or anytime the source\nside changes its offered actions through wl_data_source.set_actions.\n\nOnly available since version 3 of the interface"]
        SourceActions { source_actions: u32 },
        #[doc = "notify the selected action\n\nThis event indicates the action selected by the compositor after\nmatching the source/destination side actions. Only one action (or\nnone) will be offered here.\n\nThis event can be emitted multiple times during the drag-and-drop\noperation in response to destination side action changes through\nwl_data_offer.set_actions.\n\nThis event will no longer be emitted after wl_data_device.drop\nhappened on the drag-and-drop destination, the client must\nhonor the last action received, or the last preferred one set\nthrough wl_data_offer.set_actions when handling an \"ask\" action.\n\nCompositors may also change the selected action on the fly, mainly\nin response to keyboard modifier changes during the drag-and-drop\noperation.\n\nThe most recent action received is always the valid one. Prior to\nreceiving wl_data_device.drop, the chosen action may change (e.g.\ndue to keyboard modifiers being pressed). At the time of receiving\nwl_data_device.drop the drag-and-drop destination must honor the\nlast action received.\n\nAction changes may still happen after wl_data_device.drop,\nespecially on \"ask\" actions, where the drag-and-drop destination\nmay choose another action afterwards. Action changes happening\nat this stage are always the result of inter-client negotiation, the\ncompositor shall no longer be able to induce a different action.\n\nUpon \"ask\" actions, it is expected that the drag-and-drop destination\nmay potentially choose a different action and/or mime type,\nbased on wl_data_offer.source_actions and finally chosen by the\nuser (e.g. popping up a menu with the available options). The\nfinal wl_data_offer.set_actions and wl_data_offer.accept requests\nmust happen before the call to wl_data_offer.finish.\n\nOnly available since version 3 of the interface"]
        Action { dnd_action: u32 },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "offer",
                since: 1,
                signature: &[super::ArgumentType::Str],
                destructor: false,
            },
            super::MessageDesc {
                name: "source_actions",
                since: 3,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "action",
                since: 3,
                signature: &[super::ArgumentType::Uint],
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
                Event::Offer { .. } => 0,
                Event::SourceActions { .. } => 1,
                Event::Action { .. } => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Offer { .. } => 1,
                Event::SourceActions { .. } => 3,
                Event::Action { .. } => 3,
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
                    Ok(Event::Offer {
                        mime_type: {
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
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::SourceActions {
                        source_actions: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Action {
                        dnd_action: {
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
                    Ok(Event::Offer {
                        mime_type: ::std::ffi::CStr::from_ptr(_args[0].s)
                            .to_string_lossy()
                            .into_owned(),
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::SourceActions {
                        source_actions: _args[0].u,
                    })
                }
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Action {
                        dnd_action: _args[0].u,
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
    pub struct WlDataOffer(Proxy<WlDataOffer>);
    impl AsRef<Proxy<WlDataOffer>> for WlDataOffer {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlDataOffer>> for WlDataOffer {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlDataOffer(value)
        }
    }
    impl From<WlDataOffer> for Proxy<WlDataOffer> {
        #[inline]
        fn from(value: WlDataOffer) -> Self {
            value.0
        }
    }
    impl Interface for WlDataOffer {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_data_offer";
        const VERSION: u32 = 3;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_data_offer_interface }
        }
    }
    impl WlDataOffer {
        #[doc = "accept one of the offered mime types\n\nIndicate that the client can accept the given mime type, or\nNULL for not accepted.\n\nFor objects of version 2 or older, this request is used by the\nclient to give feedback whether the client can receive the given\nmime type, or NULL if none is accepted; the feedback does not\ndetermine whether the drag-and-drop operation succeeds or not.\n\nFor objects of version 3 or newer, this request determines the\nfinal result of the drag-and-drop operation. If the end result\nis that no mime types were accepted, the drag-and-drop operation\nwill be cancelled and the corresponding drag source will receive\nwl_data_source.cancelled. Clients may still use this event in\nconjunction with wl_data_source.action for feedback."]
        pub fn accept(&self, serial: u32, mime_type: Option<String>) -> () {
            let msg = Request::Accept {
                serial: serial,
                mime_type: mime_type,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "request that the data is transferred\n\nTo transfer the offered data, the client issues this request\nand indicates the mime type it wants to receive.  The transfer\nhappens through the passed file descriptor (typically created\nwith the pipe system call).  The source client writes the data\nin the mime type representation requested and then closes the\nfile descriptor.\n\nThe receiving client reads from the read end of the pipe until\nEOF and then closes its end, at which point the transfer is\ncomplete.\n\nThis request may happen multiple times for different mime types,\nboth before and after wl_data_device.drop. Drag-and-drop destination\nclients may preemptively fetch data or examine it more closely to\ndetermine acceptance."]
        pub fn receive(&self, mime_type: String, fd: ::std::os::unix::io::RawFd) -> () {
            let msg = Request::Receive {
                mime_type: mime_type,
                fd: fd,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "destroy data offer\n\nDestroy the data offer.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "the offer will no longer be used\n\nNotifies the compositor that the drag destination successfully\nfinished the drag-and-drop operation.\n\nUpon receiving this request, the compositor will emit\nwl_data_source.dnd_finished on the drag source client.\n\nIt is a client error to perform other requests than\nwl_data_offer.destroy after this one. It is also an error to perform\nthis request after a NULL mime type has been set in\nwl_data_offer.accept or no action was received through\nwl_data_offer.action.\n\nOnly available since version 3 of the interface."]
        pub fn finish(&self) -> () {
            let msg = Request::Finish;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the available/preferred drag-and-drop actions\n\nSets the actions that the destination side client supports for\nthis operation. This request may trigger the emission of\nwl_data_source.action and wl_data_offer.action events if the compositor\nneeds to change the selected action.\n\nThis request can be called multiple times throughout the\ndrag-and-drop operation, typically in response to wl_data_device.enter\nor wl_data_device.motion events.\n\nThis request determines the final result of the drag-and-drop\noperation. If the end result is that no action is accepted,\nthe drag source will receive wl_drag_source.cancelled.\n\nThe dnd_actions argument must contain only values expressed in the\nwl_data_device_manager.dnd_actions enum, and the preferred_action\nargument must only contain one of those values set, otherwise it\nwill result in a protocol error.\n\nWhile managing an \"ask\" action, the destination drag-and-drop client\nmay perform further wl_data_offer.receive requests, and is expected\nto perform one last wl_data_offer.set_actions request with a preferred\naction other than \"ask\" (and optionally wl_data_offer.accept) before\nrequesting wl_data_offer.finish, in order to convey the action selected\nby the user. If the preferred action is not in the\nwl_data_offer.source_actions mask, an error will be raised.\n\nIf the \"ask\" action is dismissed (e.g. user cancellation), the client\nis expected to perform wl_data_offer.destroy right away.\n\nThis request can only be made on drag-and-drop offers, a protocol error\nwill be raised otherwise.\n\nOnly available since version 3 of the interface."]
        pub fn set_actions(&self, dnd_actions: u32, preferred_action: u32) -> () {
            let msg = Request::SetActions {
                dnd_actions: dnd_actions,
                preferred_action: preferred_action,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_ACCEPT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_RECEIVE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_FINISH_SINCE: u32 = 3u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_ACTIONS_SINCE: u32 = 3u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_OFFER_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_SOURCE_ACTIONS_SINCE: u32 = 3u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_ACTION_SINCE: u32 = 3u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_data_offer_requests: [wl_message; 5] = [
        wl_message {
            name: b"accept\0" as *const u8 as *const c_char,
            signature: b"u?s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"receive\0" as *const u8 as *const c_char,
            signature: b"sh\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"finish\0" as *const u8 as *const c_char,
            signature: b"3\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_actions\0" as *const u8 as *const c_char,
            signature: b"3uu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_data_offer_events: [wl_message; 3] = [
        wl_message {
            name: b"offer\0" as *const u8 as *const c_char,
            signature: b"s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"source_actions\0" as *const u8 as *const c_char,
            signature: b"3u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"action\0" as *const u8 as *const c_char,
            signature: b"3u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_data_offer_interface: wl_interface = wl_interface {
        name: b"wl_data_offer\0" as *const u8 as *const c_char,
        version: 3,
        request_count: 5,
        requests: unsafe { &wl_data_offer_requests as *const _ },
        event_count: 3,
        events: unsafe { &wl_data_offer_events as *const _ },
    };
}
#[doc = "offer to transfer data\n\nThe wl_data_source object is the source side of a wl_data_offer.\nIt is created by the source client in a data transfer and\nprovides a way to describe the offered data and a way to respond\nto requests to transfer the data."]
pub mod wl_data_source {
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
        #[doc = "action mask contains invalid values"]
        InvalidActionMask = 0,
        #[doc = "source doesn't accept this request"]
        InvalidSource = 1,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidActionMask),
                1 => Some(Error::InvalidSource),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "add an offered mime type\n\nThis request adds a mime type to the set of mime types\nadvertised to targets.  Can be called several times to offer\nmultiple types."]
        Offer { mime_type: String },
        #[doc = "destroy the data source\n\nDestroy the data source.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "set the available drag-and-drop actions\n\nSets the actions that the source side client supports for this\noperation. This request may trigger wl_data_source.action and\nwl_data_offer.action events if the compositor needs to change the\nselected action.\n\nThe dnd_actions argument must contain only values expressed in the\nwl_data_device_manager.dnd_actions enum, otherwise it will result\nin a protocol error.\n\nThis request must be made once only, and can only be made on sources\nused in drag-and-drop, so it must be performed before\nwl_data_device.start_drag. Attempting to use the source other than\nfor drag-and-drop will raise a protocol error.\n\nOnly available since version 3 of the interface"]
        SetActions { dnd_actions: u32 },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "offer",
                since: 1,
                signature: &[super::ArgumentType::Str],
                destructor: false,
            },
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
                destructor: true,
            },
            super::MessageDesc {
                name: "set_actions",
                since: 3,
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
                Request::Offer { .. } => 0,
                Request::Destroy => 1,
                Request::SetActions { .. } => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Offer { .. } => 1,
                Request::Destroy => 1,
                Request::SetActions { .. } => 3,
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
                Request::Offer { mime_type } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![Argument::Str(Box::new(unsafe {
                        ::std::ffi::CString::from_vec_unchecked(mime_type.into())
                    })),],
                },
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![],
                },
                Request::SetActions { dnd_actions } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![Argument::Uint(dnd_actions),],
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
                Request::Offer { mime_type } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(mime_type).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    f(0, &mut _args_array)
                }
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(1, &mut _args_array)
                }
                Request::SetActions { dnd_actions } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = dnd_actions;
                    f(2, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "a target accepts an offered mime type\n\nSent when a target accepts pointer_focus or motion events.  If\na target does not accept any of the offered types, type is NULL.\n\nUsed for feedback during drag-and-drop."]
        Target { mime_type: Option<String> },
        #[doc = "send the data\n\nRequest for data from the client.  Send the data as the\nspecified mime type over the passed file descriptor, then\nclose it."]
        Send {
            mime_type: String,
            fd: ::std::os::unix::io::RawFd,
        },
        #[doc = "selection was cancelled\n\nThis data source is no longer valid. There are several reasons why\nthis could happen:\n\n- The data source has been replaced by another data source.\n- The drag-and-drop operation was performed, but the drop destination\ndid not accept any of the mime types offered through\nwl_data_source.target.\n- The drag-and-drop operation was performed, but the drop destination\ndid not select any of the actions present in the mask offered through\nwl_data_source.action.\n- The drag-and-drop operation was performed but didn't happen over a\nsurface.\n- The compositor cancelled the drag-and-drop operation (e.g. compositor\ndependent timeouts to avoid stale drag-and-drop transfers).\n\nThe client should clean up and destroy this data source.\n\nFor objects of version 2 or older, wl_data_source.cancelled will\nonly be emitted if the data source was replaced by another data\nsource."]
        Cancelled,
        #[doc = "the drag-and-drop operation physically finished\n\nThe user performed the drop action. This event does not indicate\nacceptance, wl_data_source.cancelled may still be emitted afterwards\nif the drop destination does not accept any mime type.\n\nHowever, this event might however not be received if the compositor\ncancelled the drag-and-drop operation before this event could happen.\n\nNote that the data_source may still be used in the future and should\nnot be destroyed here.\n\nOnly available since version 3 of the interface"]
        DndDropPerformed,
        #[doc = "the drag-and-drop operation concluded\n\nThe drop destination finished interoperating with this data\nsource, so the client is now free to destroy this data source and\nfree all associated data.\n\nIf the action used to perform the operation was \"move\", the\nsource can now delete the transferred data.\n\nOnly available since version 3 of the interface"]
        DndFinished,
        #[doc = "notify the selected action\n\nThis event indicates the action selected by the compositor after\nmatching the source/destination side actions. Only one action (or\nnone) will be offered here.\n\nThis event can be emitted multiple times during the drag-and-drop\noperation, mainly in response to destination side changes through\nwl_data_offer.set_actions, and as the data device enters/leaves\nsurfaces.\n\nIt is only possible to receive this event after\nwl_data_source.dnd_drop_performed if the drag-and-drop operation\nended in an \"ask\" action, in which case the final wl_data_source.action\nevent will happen immediately before wl_data_source.dnd_finished.\n\nCompositors may also change the selected action on the fly, mainly\nin response to keyboard modifier changes during the drag-and-drop\noperation.\n\nThe most recent action received is always the valid one. The chosen\naction may change alongside negotiation (e.g. an \"ask\" action can turn\ninto a \"move\" operation), so the effects of the final action must\nalways be applied in wl_data_offer.dnd_finished.\n\nClients can trigger cursor surface changes from this point, so\nthey reflect the current action.\n\nOnly available since version 3 of the interface"]
        Action { dnd_action: u32 },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "target",
                since: 1,
                signature: &[super::ArgumentType::Str],
                destructor: false,
            },
            super::MessageDesc {
                name: "send",
                since: 1,
                signature: &[super::ArgumentType::Str, super::ArgumentType::Fd],
                destructor: false,
            },
            super::MessageDesc {
                name: "cancelled",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "dnd_drop_performed",
                since: 3,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "dnd_finished",
                since: 3,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "action",
                since: 3,
                signature: &[super::ArgumentType::Uint],
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
                Event::Target { .. } => 0,
                Event::Send { .. } => 1,
                Event::Cancelled => 2,
                Event::DndDropPerformed => 3,
                Event::DndFinished => 4,
                Event::Action { .. } => 5,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Target { .. } => 1,
                Event::Send { .. } => 1,
                Event::Cancelled => 1,
                Event::DndDropPerformed => 3,
                Event::DndFinished => 3,
                Event::Action { .. } => 3,
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
                    Ok(Event::Target {
                        mime_type: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                if s.len() == 0 {
                                    None
                                } else {
                                    Some(s)
                                }
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Send {
                        mime_type: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                        fd: {
                            if let Some(Argument::Fd(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                2 => Ok(Event::Cancelled),
                3 => Ok(Event::DndDropPerformed),
                4 => Ok(Event::DndFinished),
                5 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Action {
                        dnd_action: {
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
                    Ok(Event::Target {
                        mime_type: if _args[0].s.is_null() {
                            None
                        } else {
                            Some(
                                ::std::ffi::CStr::from_ptr(_args[0].s)
                                    .to_string_lossy()
                                    .into_owned(),
                            )
                        },
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Send {
                        mime_type: ::std::ffi::CStr::from_ptr(_args[0].s)
                            .to_string_lossy()
                            .into_owned(),
                        fd: _args[1].h,
                    })
                }
                2 => Ok(Event::Cancelled),
                3 => Ok(Event::DndDropPerformed),
                4 => Ok(Event::DndFinished),
                5 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Action {
                        dnd_action: _args[0].u,
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
    pub struct WlDataSource(Proxy<WlDataSource>);
    impl AsRef<Proxy<WlDataSource>> for WlDataSource {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlDataSource>> for WlDataSource {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlDataSource(value)
        }
    }
    impl From<WlDataSource> for Proxy<WlDataSource> {
        #[inline]
        fn from(value: WlDataSource) -> Self {
            value.0
        }
    }
    impl Interface for WlDataSource {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_data_source";
        const VERSION: u32 = 3;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_data_source_interface }
        }
    }
    impl WlDataSource {
        #[doc = "add an offered mime type\n\nThis request adds a mime type to the set of mime types\nadvertised to targets.  Can be called several times to offer\nmultiple types."]
        pub fn offer(&self, mime_type: String) -> () {
            let msg = Request::Offer {
                mime_type: mime_type,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "destroy the data source\n\nDestroy the data source.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the available drag-and-drop actions\n\nSets the actions that the source side client supports for this\noperation. This request may trigger wl_data_source.action and\nwl_data_offer.action events if the compositor needs to change the\nselected action.\n\nThe dnd_actions argument must contain only values expressed in the\nwl_data_device_manager.dnd_actions enum, otherwise it will result\nin a protocol error.\n\nThis request must be made once only, and can only be made on sources\nused in drag-and-drop, so it must be performed before\nwl_data_device.start_drag. Attempting to use the source other than\nfor drag-and-drop will raise a protocol error.\n\nOnly available since version 3 of the interface."]
        pub fn set_actions(&self, dnd_actions: u32) -> () {
            let msg = Request::SetActions {
                dnd_actions: dnd_actions,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_OFFER_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_ACTIONS_SINCE: u32 = 3u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_TARGET_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_SEND_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CANCELLED_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DND_DROP_PERFORMED_SINCE: u32 = 3u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DND_FINISHED_SINCE: u32 = 3u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_ACTION_SINCE: u32 = 3u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_data_source_requests: [wl_message; 3] = [
        wl_message {
            name: b"offer\0" as *const u8 as *const c_char,
            signature: b"s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_actions\0" as *const u8 as *const c_char,
            signature: b"3u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_data_source_events: [wl_message; 6] = [
        wl_message {
            name: b"target\0" as *const u8 as *const c_char,
            signature: b"?s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"send\0" as *const u8 as *const c_char,
            signature: b"sh\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"cancelled\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"dnd_drop_performed\0" as *const u8 as *const c_char,
            signature: b"3\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"dnd_finished\0" as *const u8 as *const c_char,
            signature: b"3\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"action\0" as *const u8 as *const c_char,
            signature: b"3u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_data_source_interface: wl_interface = wl_interface {
        name: b"wl_data_source\0" as *const u8 as *const c_char,
        version: 3,
        request_count: 3,
        requests: unsafe { &wl_data_source_requests as *const _ },
        event_count: 6,
        events: unsafe { &wl_data_source_events as *const _ },
    };
}
#[doc = "data transfer device\n\nThere is one wl_data_device per seat which can be obtained\nfrom the global wl_data_device_manager singleton.\n\nA wl_data_device provides access to inter-client data transfer\nmechanisms such as copy-and-paste and drag-and-drop."]
pub mod wl_data_device {
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
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::Role),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "start drag-and-drop operation\n\nThis request asks the compositor to start a drag-and-drop\noperation on behalf of the client.\n\nThe source argument is the data source that provides the data\nfor the eventual data transfer. If source is NULL, enter, leave\nand motion events are sent only to the client that initiated the\ndrag and the client is expected to handle the data passing\ninternally.\n\nThe origin surface is the surface where the drag originates and\nthe client must have an active implicit grab that matches the\nserial.\n\nThe icon surface is an optional (can be NULL) surface that\nprovides an icon to be moved around with the cursor.  Initially,\nthe top-left corner of the icon surface is placed at the cursor\nhotspot, but subsequent wl_surface.attach request can move the\nrelative position. Attach requests must be confirmed with\nwl_surface.commit as usual. The icon surface is given the role of\na drag-and-drop icon. If the icon surface already has another role,\nit raises a protocol error.\n\nThe current and pending input regions of the icon wl_surface are\ncleared, and wl_surface.set_input_region is ignored until the\nwl_surface is no longer used as the icon surface. When the use\nas an icon ends, the current and pending input regions become\nundefined, and the wl_surface is unmapped."]
        StartDrag {
            source: Option<super::wl_data_source::WlDataSource>,
            origin: super::wl_surface::WlSurface,
            icon: Option<super::wl_surface::WlSurface>,
            serial: u32,
        },
        #[doc = "copy data to the selection\n\nThis request asks the compositor to set the selection\nto the data from the source on behalf of the client.\n\nTo unset the selection, set the source to NULL."]
        SetSelection {
            source: Option<super::wl_data_source::WlDataSource>,
            serial: u32,
        },
        #[doc = "destroy data device\n\nThis request destroys the data device.\n\nThis is a destructor, once sent this object cannot be used any longer.\nOnly available since version 2 of the interface"]
        Release,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "start_drag",
                since: 1,
                signature: &[
                    super::ArgumentType::Object,
                    super::ArgumentType::Object,
                    super::ArgumentType::Object,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_selection",
                since: 1,
                signature: &[super::ArgumentType::Object, super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "release",
                since: 2,
                signature: &[],
                destructor: true,
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Release => true,
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::StartDrag { .. } => 0,
                Request::SetSelection { .. } => 1,
                Request::Release => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::StartDrag { .. } => 1,
                Request::SetSelection { .. } => 1,
                Request::Release => 2,
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
                Request::StartDrag {
                    source,
                    origin,
                    icon,
                    serial,
                } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![
                        Argument::Object(source.map(|o| o.as_ref().id()).unwrap_or(0)),
                        Argument::Object(origin.as_ref().id()),
                        Argument::Object(icon.map(|o| o.as_ref().id()).unwrap_or(0)),
                        Argument::Uint(serial),
                    ],
                },
                Request::SetSelection { source, serial } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![
                        Argument::Object(source.map(|o| o.as_ref().id()).unwrap_or(0)),
                        Argument::Uint(serial),
                    ],
                },
                Request::Release => Message {
                    sender_id: sender_id,
                    opcode: 2,
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
                Request::StartDrag {
                    source,
                    origin,
                    icon,
                    serial,
                } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = source
                        .map(|o| o.as_ref().c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    _args_array[1].o = origin.as_ref().c_ptr() as *mut _;
                    _args_array[2].o = icon
                        .map(|o| o.as_ref().c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    _args_array[3].u = serial;
                    f(0, &mut _args_array)
                }
                Request::SetSelection { source, serial } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = source
                        .map(|o| o.as_ref().c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    _args_array[1].u = serial;
                    f(1, &mut _args_array)
                }
                Request::Release => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(2, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "introduce a new wl_data_offer\n\nThe data_offer event introduces a new wl_data_offer object,\nwhich will subsequently be used in either the\ndata_device.enter event (for drag-and-drop) or the\ndata_device.selection event (for selections).  Immediately\nfollowing the data_device_data_offer event, the new data_offer\nobject will send out data_offer.offer events to describe the\nmime types it offers."]
        DataOffer {
            id: Main<super::wl_data_offer::WlDataOffer>,
        },
        #[doc = "initiate drag-and-drop session\n\nThis event is sent when an active drag-and-drop pointer enters\na surface owned by the client.  The position of the pointer at\nenter time is provided by the x and y arguments, in surface-local\ncoordinates."]
        Enter {
            serial: u32,
            surface: super::wl_surface::WlSurface,
            x: f64,
            y: f64,
            id: Option<super::wl_data_offer::WlDataOffer>,
        },
        #[doc = "end drag-and-drop session\n\nThis event is sent when the drag-and-drop pointer leaves the\nsurface and the session ends.  The client must destroy the\nwl_data_offer introduced at enter time at this point."]
        Leave,
        #[doc = "drag-and-drop session motion\n\nThis event is sent when the drag-and-drop pointer moves within\nthe currently focused surface. The new position of the pointer\nis provided by the x and y arguments, in surface-local\ncoordinates."]
        Motion { time: u32, x: f64, y: f64 },
        #[doc = "end drag-and-drop session successfully\n\nThe event is sent when a drag-and-drop operation is ended\nbecause the implicit grab is removed.\n\nThe drag-and-drop destination is expected to honor the last action\nreceived through wl_data_offer.action, if the resulting action is\n\"copy\" or \"move\", the destination can still perform\nwl_data_offer.receive requests, and is expected to end all\ntransfers with a wl_data_offer.finish request.\n\nIf the resulting action is \"ask\", the action will not be considered\nfinal. The drag-and-drop destination is expected to perform one last\nwl_data_offer.set_actions request, or wl_data_offer.destroy in order\nto cancel the operation."]
        Drop,
        #[doc = "advertise new selection\n\nThe selection event is sent out to notify the client of a new\nwl_data_offer for the selection for this device.  The\ndata_device.data_offer and the data_offer.offer events are\nsent out immediately before this event to introduce the data\noffer object.  The selection event is sent to a client\nimmediately before receiving keyboard focus and when a new\nselection is set while the client has keyboard focus.  The\ndata_offer is valid until a new data_offer or NULL is received\nor until the client loses keyboard focus.  The client must\ndestroy the previous selection data_offer, if any, upon receiving\nthis event."]
        Selection {
            id: Option<super::wl_data_offer::WlDataOffer>,
        },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "data_offer",
                since: 1,
                signature: &[super::ArgumentType::NewId],
                destructor: false,
            },
            super::MessageDesc {
                name: "enter",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Object,
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Object,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "leave",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "motion",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Fixed,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "drop",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "selection",
                since: 1,
                signature: &[super::ArgumentType::Object],
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
                Event::DataOffer { .. } => 0,
                Event::Enter { .. } => 1,
                Event::Leave => 2,
                Event::Motion { .. } => 3,
                Event::Drop => 4,
                Event::Selection { .. } => 5,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::DataOffer { .. } => 1,
                Event::Enter { .. } => 1,
                Event::Leave => 1,
                Event::Motion { .. } => 1,
                Event::Drop => 1,
                Event::Selection { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<super::wl_data_offer::WlDataOffer>(
                    version,
                    meta.child(),
                )),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::DataOffer {
                        id: {
                            if let Some(Argument::NewId(val)) = args.next() {
                                map.get_new(val).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Enter {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        surface: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?.into()
                            } else {
                                return Err(());
                            }
                        },
                        x: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                        y: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                        id: {
                            if let Some(Argument::Object(val)) = args.next() {
                                if val == 0 {
                                    None
                                } else {
                                    Some(map.get(val).ok_or(())?.into())
                                }
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                2 => Ok(Event::Leave),
                3 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Motion {
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        x: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                        y: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                4 => Ok(Event::Drop),
                5 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Selection {
                        id: {
                            if let Some(Argument::Object(val)) = args.next() {
                                if val == 0 {
                                    None
                                } else {
                                    Some(map.get(val).ok_or(())?.into())
                                }
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
                    Ok(Event::DataOffer {
                        id: Main::<super::wl_data_offer::WlDataOffer>::from_c_ptr(
                            _args[0].o as *mut _,
                        ),
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 5);
                    Ok(Event::Enter {
                        serial: _args[0].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(
                            _args[1].o as *mut _,
                        )
                        .into(),
                        x: (_args[2].f as f64) / 256.,
                        y: (_args[3].f as f64) / 256.,
                        id: if _args[4].o.is_null() {
                            None
                        } else {
                            Some(
                                Proxy::<super::wl_data_offer::WlDataOffer>::from_c_ptr(
                                    _args[4].o as *mut _,
                                )
                                .into(),
                            )
                        },
                    })
                }
                2 => Ok(Event::Leave),
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Motion {
                        time: _args[0].u,
                        x: (_args[1].f as f64) / 256.,
                        y: (_args[2].f as f64) / 256.,
                    })
                }
                4 => Ok(Event::Drop),
                5 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Selection {
                        id: if _args[0].o.is_null() {
                            None
                        } else {
                            Some(
                                Proxy::<super::wl_data_offer::WlDataOffer>::from_c_ptr(
                                    _args[0].o as *mut _,
                                )
                                .into(),
                            )
                        },
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
    pub struct WlDataDevice(Proxy<WlDataDevice>);
    impl AsRef<Proxy<WlDataDevice>> for WlDataDevice {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlDataDevice>> for WlDataDevice {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlDataDevice(value)
        }
    }
    impl From<WlDataDevice> for Proxy<WlDataDevice> {
        #[inline]
        fn from(value: WlDataDevice) -> Self {
            value.0
        }
    }
    impl Interface for WlDataDevice {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_data_device";
        const VERSION: u32 = 3;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_data_device_interface }
        }
    }
    impl WlDataDevice {
        #[doc = "start drag-and-drop operation\n\nThis request asks the compositor to start a drag-and-drop\noperation on behalf of the client.\n\nThe source argument is the data source that provides the data\nfor the eventual data transfer. If source is NULL, enter, leave\nand motion events are sent only to the client that initiated the\ndrag and the client is expected to handle the data passing\ninternally.\n\nThe origin surface is the surface where the drag originates and\nthe client must have an active implicit grab that matches the\nserial.\n\nThe icon surface is an optional (can be NULL) surface that\nprovides an icon to be moved around with the cursor.  Initially,\nthe top-left corner of the icon surface is placed at the cursor\nhotspot, but subsequent wl_surface.attach request can move the\nrelative position. Attach requests must be confirmed with\nwl_surface.commit as usual. The icon surface is given the role of\na drag-and-drop icon. If the icon surface already has another role,\nit raises a protocol error.\n\nThe current and pending input regions of the icon wl_surface are\ncleared, and wl_surface.set_input_region is ignored until the\nwl_surface is no longer used as the icon surface. When the use\nas an icon ends, the current and pending input regions become\nundefined, and the wl_surface is unmapped."]
        pub fn start_drag(
            &self,
            source: Option<&super::wl_data_source::WlDataSource>,
            origin: &super::wl_surface::WlSurface,
            icon: Option<&super::wl_surface::WlSurface>,
            serial: u32,
        ) -> () {
            let msg = Request::StartDrag {
                source: source.map(|o| o.clone()),
                origin: origin.clone(),
                icon: icon.map(|o| o.clone()),
                serial: serial,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "copy data to the selection\n\nThis request asks the compositor to set the selection\nto the data from the source on behalf of the client.\n\nTo unset the selection, set the source to NULL."]
        pub fn set_selection(
            &self,
            source: Option<&super::wl_data_source::WlDataSource>,
            serial: u32,
        ) -> () {
            let msg = Request::SetSelection {
                source: source.map(|o| o.clone()),
                serial: serial,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "destroy data device\n\nThis request destroys the data device.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called.\nOnly available since version 2 of the interface."]
        pub fn release(&self) -> () {
            let msg = Request::Release;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_START_DRAG_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_SELECTION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_RELEASE_SINCE: u32 = 2u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DATA_OFFER_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_ENTER_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_LEAVE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_MOTION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DROP_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_SELECTION_SINCE: u32 = 1u32;
    static mut wl_data_device_requests_start_drag_types: [*const wl_interface; 4] = [
        unsafe { &super::wl_data_source::wl_data_source_interface as *const wl_interface },
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
    ];
    static mut wl_data_device_requests_set_selection_types: [*const wl_interface; 2] = [
        unsafe { &super::wl_data_source::wl_data_source_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_data_device_requests: [wl_message; 3] = [
        wl_message {
            name: b"start_drag\0" as *const u8 as *const c_char,
            signature: b"?oo?ou\0" as *const u8 as *const c_char,
            types: unsafe { &wl_data_device_requests_start_drag_types as *const _ },
        },
        wl_message {
            name: b"set_selection\0" as *const u8 as *const c_char,
            signature: b"?ou\0" as *const u8 as *const c_char,
            types: unsafe { &wl_data_device_requests_set_selection_types as *const _ },
        },
        wl_message {
            name: b"release\0" as *const u8 as *const c_char,
            signature: b"2\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    static mut wl_data_device_events_data_offer_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_data_offer::wl_data_offer_interface as *const wl_interface }];
    static mut wl_data_device_events_enter_types: [*const wl_interface; 5] = [
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_data_offer::wl_data_offer_interface as *const wl_interface },
    ];
    static mut wl_data_device_events_selection_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_data_offer::wl_data_offer_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_data_device_events: [wl_message; 6] = [
        wl_message {
            name: b"data_offer\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &wl_data_device_events_data_offer_types as *const _ },
        },
        wl_message {
            name: b"enter\0" as *const u8 as *const c_char,
            signature: b"uoff?o\0" as *const u8 as *const c_char,
            types: unsafe { &wl_data_device_events_enter_types as *const _ },
        },
        wl_message {
            name: b"leave\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"motion\0" as *const u8 as *const c_char,
            signature: b"uff\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"drop\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"selection\0" as *const u8 as *const c_char,
            signature: b"?o\0" as *const u8 as *const c_char,
            types: unsafe { &wl_data_device_events_selection_types as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_data_device_interface: wl_interface = wl_interface {
        name: b"wl_data_device\0" as *const u8 as *const c_char,
        version: 3,
        request_count: 3,
        requests: unsafe { &wl_data_device_requests as *const _ },
        event_count: 6,
        events: unsafe { &wl_data_device_events as *const _ },
    };
}
#[doc = "data transfer interface\n\nThe wl_data_device_manager is a singleton global object that\nprovides access to inter-client data transfer mechanisms such as\ncopy-and-paste and drag-and-drop.  These mechanisms are tied to\na wl_seat and this interface lets a client get a wl_data_device\ncorresponding to a wl_seat.\n\nDepending on the version bound, the objects created from the bound\nwl_data_device_manager object will have different requirements for\nfunctioning properly. See wl_data_source.set_actions,\nwl_data_offer.accept and wl_data_offer.finish for details."]
pub mod wl_data_device_manager {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    bitflags! { # [ doc = "drag and drop actions\n\nThis is a bitmask of the available/preferred actions in a\ndrag-and-drop operation.\n\nIn the compositor, the selected action is a result of matching the\nactions offered by the source and destination sides.  \"action\" events\nwith a \"none\" action will be sent to both source and destination if\nthere is no match. All further checks will effectively happen on\n(source actions ∩ destination actions).\n\nIn addition, compositors may also pick different actions in\nreaction to key modifiers being pressed. One common design that\nis used in major toolkits (and the behavior recommended for\ncompositors) is:\n\n- If no modifiers are pressed, the first match (in bit order)\nwill be used.\n- Pressing Shift selects \"move\", if enabled in the mask.\n- Pressing Control selects \"copy\", if enabled in the mask.\n\nBehavior beyond that is considered implementation-dependent.\nCompositors may for example bind other modifiers (like Alt/Meta)\nor drags initiated with other buttons than BTN_LEFT to specific\nactions (e.g. \"ask\")." ] pub struct DndAction : u32 { # [ doc = "no action" ] const None = 0 ; # [ doc = "copy action" ] const Copy = 1 ; # [ doc = "move action" ] const Move = 2 ; # [ doc = "ask action" ] const Ask = 4 ; } }
    impl DndAction {
        pub fn from_raw(n: u32) -> Option<DndAction> {
            Some(DndAction::from_bits_truncate(n))
        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "create a new data source\n\nCreate a new data source."]
        CreateDataSource {},
        #[doc = "create a new data device\n\nCreate a new data device for a given seat."]
        GetDataDevice { seat: super::wl_seat::WlSeat },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "create_data_source",
                since: 1,
                signature: &[super::ArgumentType::NewId],
                destructor: false,
            },
            super::MessageDesc {
                name: "get_data_device",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Object],
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
                Request::CreateDataSource { .. } => 0,
                Request::GetDataDevice { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::CreateDataSource { .. } => 1,
                Request::GetDataDevice { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(
                    Object::from_interface::<super::wl_data_source::WlDataSource>(
                        version,
                        meta.child(),
                    ),
                ),
                1 => Some(
                    Object::from_interface::<super::wl_data_device::WlDataDevice>(
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
                Request::CreateDataSource {} => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![Argument::NewId(0),],
                },
                Request::GetDataDevice { seat } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::NewId(0), Argument::Object(seat.as_ref().id()),],
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
                Request::CreateDataSource {} => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    f(0, &mut _args_array)
                }
                Request::GetDataDevice { seat } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].o = seat.as_ref().c_ptr() as *mut _;
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
    pub struct WlDataDeviceManager(Proxy<WlDataDeviceManager>);
    impl AsRef<Proxy<WlDataDeviceManager>> for WlDataDeviceManager {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlDataDeviceManager>> for WlDataDeviceManager {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlDataDeviceManager(value)
        }
    }
    impl From<WlDataDeviceManager> for Proxy<WlDataDeviceManager> {
        #[inline]
        fn from(value: WlDataDeviceManager) -> Self {
            value.0
        }
    }
    impl Interface for WlDataDeviceManager {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_data_device_manager";
        const VERSION: u32 = 3;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_data_device_manager_interface }
        }
    }
    impl WlDataDeviceManager {
        #[doc = "create a new data source\n\nCreate a new data source."]
        pub fn create_data_source(&self) -> Main<super::wl_data_source::WlDataSource> {
            let msg = Request::CreateDataSource {};
            self.0.send(msg, None).unwrap()
        }
        #[doc = "create a new data device\n\nCreate a new data device for a given seat."]
        pub fn get_data_device(
            &self,
            seat: &super::wl_seat::WlSeat,
        ) -> Main<super::wl_data_device::WlDataDevice> {
            let msg = Request::GetDataDevice { seat: seat.clone() };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CREATE_DATA_SOURCE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_DATA_DEVICE_SINCE: u32 = 1u32;
    static mut wl_data_device_manager_requests_create_data_source_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_data_source::wl_data_source_interface as *const wl_interface }];
    static mut wl_data_device_manager_requests_get_data_device_types: [*const wl_interface; 2] = [
        unsafe { &super::wl_data_device::wl_data_device_interface as *const wl_interface },
        unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_data_device_manager_requests: [wl_message; 2] = [
        wl_message {
            name: b"create_data_source\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &wl_data_device_manager_requests_create_data_source_types as *const _ },
        },
        wl_message {
            name: b"get_data_device\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe { &wl_data_device_manager_requests_get_data_device_types as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_data_device_manager_interface: wl_interface = wl_interface {
        name: b"wl_data_device_manager\0" as *const u8 as *const c_char,
        version: 3,
        request_count: 2,
        requests: unsafe { &wl_data_device_manager_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "create desktop-style surfaces\n\nThis interface is implemented by servers that provide\ndesktop-style user interfaces.\n\nIt allows clients to associate a wl_shell_surface with\na basic surface.\n\nNote! This protocol is deprecated and not intended for production use.\nFor desktop-style user interfaces, use xdg_shell."]
pub mod wl_shell {
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
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::Role),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "create a shell surface from a surface\n\nCreate a shell surface for an existing surface. This gives\nthe wl_surface the role of a shell surface. If the wl_surface\nalready has another role, it raises a protocol error.\n\nOnly one shell surface can be associated with a given surface."]
        GetShellSurface {
            surface: super::wl_surface::WlSurface,
        },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "get_shell_surface",
            since: 1,
            signature: &[super::ArgumentType::NewId, super::ArgumentType::Object],
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
                Request::GetShellSurface { .. } => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::GetShellSurface { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<
                    super::wl_shell_surface::WlShellSurface,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::GetShellSurface { surface } => Message {
                    sender_id: sender_id,
                    opcode: 0,
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
                Request::GetShellSurface { surface } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].o = surface.as_ref().c_ptr() as *mut _;
                    f(0, &mut _args_array)
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
    pub struct WlShell(Proxy<WlShell>);
    impl AsRef<Proxy<WlShell>> for WlShell {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlShell>> for WlShell {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlShell(value)
        }
    }
    impl From<WlShell> for Proxy<WlShell> {
        #[inline]
        fn from(value: WlShell) -> Self {
            value.0
        }
    }
    impl Interface for WlShell {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_shell";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_shell_interface }
        }
    }
    impl WlShell {
        #[doc = "create a shell surface from a surface\n\nCreate a shell surface for an existing surface. This gives\nthe wl_surface the role of a shell surface. If the wl_surface\nalready has another role, it raises a protocol error.\n\nOnly one shell surface can be associated with a given surface."]
        pub fn get_shell_surface(
            &self,
            surface: &super::wl_surface::WlSurface,
        ) -> Main<super::wl_shell_surface::WlShellSurface> {
            let msg = Request::GetShellSurface {
                surface: surface.clone(),
            };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_SHELL_SURFACE_SINCE: u32 = 1u32;
    static mut wl_shell_requests_get_shell_surface_types: [*const wl_interface; 2] = [
        unsafe { &super::wl_shell_surface::wl_shell_surface_interface as *const wl_interface },
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_shell_requests: [wl_message; 1] = [wl_message {
        name: b"get_shell_surface\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe { &wl_shell_requests_get_shell_surface_types as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_shell_interface: wl_interface = wl_interface {
        name: b"wl_shell\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 1,
        requests: unsafe { &wl_shell_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "desktop-style metadata interface\n\nAn interface that may be implemented by a wl_surface, for\nimplementations that provide a desktop-style user interface.\n\nIt provides requests to treat surfaces like toplevel, fullscreen\nor popup windows, move, resize or maximize them, associate\nmetadata like title and class, etc.\n\nOn the server side the object is automatically destroyed when\nthe related wl_surface is destroyed. On the client side,\nwl_shell_surface_destroy() must be called before destroying\nthe wl_surface object."]
pub mod wl_shell_surface {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    bitflags! { # [ doc = "edge values for resizing\n\nThese values are used to indicate which edge of a surface\nis being dragged in a resize operation. The server may\nuse this information to adapt its behavior, e.g. choose\nan appropriate cursor image." ] pub struct Resize : u32 { # [ doc = "no edge" ] const None = 0 ; # [ doc = "top edge" ] const Top = 1 ; # [ doc = "bottom edge" ] const Bottom = 2 ; # [ doc = "left edge" ] const Left = 4 ; # [ doc = "top and left edges" ] const TopLeft = 5 ; # [ doc = "bottom and left edges" ] const BottomLeft = 6 ; # [ doc = "right edge" ] const Right = 8 ; # [ doc = "top and right edges" ] const TopRight = 9 ; # [ doc = "bottom and right edges" ] const BottomRight = 10 ; } }
    impl Resize {
        pub fn from_raw(n: u32) -> Option<Resize> {
            Some(Resize::from_bits_truncate(n))
        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }
    bitflags! { # [ doc = "details of transient behaviour\n\nThese flags specify details of the expected behaviour\nof transient surfaces. Used in the set_transient request." ] pub struct Transient : u32 { # [ doc = "do not set keyboard focus" ] const Inactive = 1 ; } }
    impl Transient {
        pub fn from_raw(n: u32) -> Option<Transient> {
            Some(Transient::from_bits_truncate(n))
        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }
    #[doc = "different method to set the surface fullscreen\n\nHints to indicate to the compositor how to deal with a conflict\nbetween the dimensions of the surface and the dimensions of the\noutput. The compositor is free to ignore this parameter."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum FullscreenMethod {
        #[doc = "no preference, apply default policy"]
        Default = 0,
        #[doc = "scale, preserve the surface's aspect ratio and center on output"]
        Scale = 1,
        #[doc = "switch output mode to the smallest mode that can fit the surface, add black borders to compensate size mismatch"]
        Driver = 2,
        #[doc = "no upscaling, center on output and add black borders to compensate size mismatch"]
        Fill = 3,
    }
    impl FullscreenMethod {
        pub fn from_raw(n: u32) -> Option<FullscreenMethod> {
            match n {
                0 => Some(FullscreenMethod::Default),
                1 => Some(FullscreenMethod::Scale),
                2 => Some(FullscreenMethod::Driver),
                3 => Some(FullscreenMethod::Fill),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "respond to a ping event\n\nA client must respond to a ping event with a pong request or\nthe client may be deemed unresponsive."]
        Pong { serial: u32 },
        #[doc = "start an interactive move\n\nStart a pointer-driven move of the surface.\n\nThis request must be used in response to a button press event.\nThe server may ignore move requests depending on the state of\nthe surface (e.g. fullscreen or maximized)."]
        Move {
            seat: super::wl_seat::WlSeat,
            serial: u32,
        },
        #[doc = "start an interactive resize\n\nStart a pointer-driven resizing of the surface.\n\nThis request must be used in response to a button press event.\nThe server may ignore resize requests depending on the state of\nthe surface (e.g. fullscreen or maximized)."]
        Resize {
            seat: super::wl_seat::WlSeat,
            serial: u32,
            edges: Resize,
        },
        #[doc = "make the surface a toplevel surface\n\nMap the surface as a toplevel surface.\n\nA toplevel surface is not fullscreen, maximized or transient."]
        SetToplevel,
        #[doc = "make the surface a transient surface\n\nMap the surface relative to an existing surface.\n\nThe x and y arguments specify the location of the upper left\ncorner of the surface relative to the upper left corner of the\nparent surface, in surface-local coordinates.\n\nThe flags argument controls details of the transient behaviour."]
        SetTransient {
            parent: super::wl_surface::WlSurface,
            x: i32,
            y: i32,
            flags: Transient,
        },
        #[doc = "make the surface a fullscreen surface\n\nMap the surface as a fullscreen surface.\n\nIf an output parameter is given then the surface will be made\nfullscreen on that output. If the client does not specify the\noutput then the compositor will apply its policy - usually\nchoosing the output on which the surface has the biggest surface\narea.\n\nThe client may specify a method to resolve a size conflict\nbetween the output size and the surface size - this is provided\nthrough the method parameter.\n\nThe framerate parameter is used only when the method is set\nto \"driver\", to indicate the preferred framerate. A value of 0\nindicates that the client does not care about framerate.  The\nframerate is specified in mHz, that is framerate of 60000 is 60Hz.\n\nA method of \"scale\" or \"driver\" implies a scaling operation of\nthe surface, either via a direct scaling operation or a change of\nthe output mode. This will override any kind of output scaling, so\nthat mapping a surface with a buffer size equal to the mode can\nfill the screen independent of buffer_scale.\n\nA method of \"fill\" means we don't scale up the buffer, however\nany output scale is applied. This means that you may run into\nan edge case where the application maps a buffer with the same\nsize of the output mode but buffer_scale 1 (thus making a\nsurface larger than the output). In this case it is allowed to\ndownscale the results to fit the screen.\n\nThe compositor must reply to this request with a configure event\nwith the dimensions for the output on which the surface will\nbe made fullscreen."]
        SetFullscreen {
            method: FullscreenMethod,
            framerate: u32,
            output: Option<super::wl_output::WlOutput>,
        },
        #[doc = "make the surface a popup surface\n\nMap the surface as a popup.\n\nA popup surface is a transient surface with an added pointer\ngrab.\n\nAn existing implicit grab will be changed to owner-events mode,\nand the popup grab will continue after the implicit grab ends\n(i.e. releasing the mouse button does not cause the popup to\nbe unmapped).\n\nThe popup grab continues until the window is destroyed or a\nmouse button is pressed in any other client's window. A click\nin any of the client's surfaces is reported as normal, however,\nclicks in other clients' surfaces will be discarded and trigger\nthe callback.\n\nThe x and y arguments specify the location of the upper left\ncorner of the surface relative to the upper left corner of the\nparent surface, in surface-local coordinates."]
        SetPopup {
            seat: super::wl_seat::WlSeat,
            serial: u32,
            parent: super::wl_surface::WlSurface,
            x: i32,
            y: i32,
            flags: Transient,
        },
        #[doc = "make the surface a maximized surface\n\nMap the surface as a maximized surface.\n\nIf an output parameter is given then the surface will be\nmaximized on that output. If the client does not specify the\noutput then the compositor will apply its policy - usually\nchoosing the output on which the surface has the biggest surface\narea.\n\nThe compositor will reply with a configure event telling\nthe expected new surface size. The operation is completed\non the next buffer attach to this surface.\n\nA maximized surface typically fills the entire output it is\nbound to, except for desktop elements such as panels. This is\nthe main difference between a maximized shell surface and a\nfullscreen shell surface.\n\nThe details depend on the compositor implementation."]
        SetMaximized {
            output: Option<super::wl_output::WlOutput>,
        },
        #[doc = "set surface title\n\nSet a short title for the surface.\n\nThis string may be used to identify the surface in a task bar,\nwindow list, or other user interface elements provided by the\ncompositor.\n\nThe string must be encoded in UTF-8."]
        SetTitle { title: String },
        #[doc = "set surface class\n\nSet a class for the surface.\n\nThe surface class identifies the general class of applications\nto which the surface belongs. A common convention is to use the\nfile name (or the full path if it is a non-standard location) of\nthe application's .desktop file as the class."]
        SetClass { class_: String },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "pong",
                since: 1,
                signature: &[super::ArgumentType::Uint],
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
                name: "set_toplevel",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_transient",
                since: 1,
                signature: &[
                    super::ArgumentType::Object,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_fullscreen",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Object,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_popup",
                since: 1,
                signature: &[
                    super::ArgumentType::Object,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Object,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_maximized",
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
                name: "set_class",
                since: 1,
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
                Request::Pong { .. } => 0,
                Request::Move { .. } => 1,
                Request::Resize { .. } => 2,
                Request::SetToplevel => 3,
                Request::SetTransient { .. } => 4,
                Request::SetFullscreen { .. } => 5,
                Request::SetPopup { .. } => 6,
                Request::SetMaximized { .. } => 7,
                Request::SetTitle { .. } => 8,
                Request::SetClass { .. } => 9,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Pong { .. } => 1,
                Request::Move { .. } => 1,
                Request::Resize { .. } => 1,
                Request::SetToplevel => 1,
                Request::SetTransient { .. } => 1,
                Request::SetFullscreen { .. } => 1,
                Request::SetPopup { .. } => 1,
                Request::SetMaximized { .. } => 1,
                Request::SetTitle { .. } => 1,
                Request::SetClass { .. } => 1,
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
                Request::Pong { serial } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![Argument::Uint(serial),],
                },
                Request::Move { seat, serial } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::Object(seat.as_ref().id()), Argument::Uint(serial),],
                },
                Request::Resize {
                    seat,
                    serial,
                    edges,
                } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![
                        Argument::Object(seat.as_ref().id()),
                        Argument::Uint(serial),
                        Argument::Uint(edges.to_raw()),
                    ],
                },
                Request::SetToplevel => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: smallvec![],
                },
                Request::SetTransient {
                    parent,
                    x,
                    y,
                    flags,
                } => Message {
                    sender_id: sender_id,
                    opcode: 4,
                    args: smallvec![
                        Argument::Object(parent.as_ref().id()),
                        Argument::Int(x),
                        Argument::Int(y),
                        Argument::Uint(flags.to_raw()),
                    ],
                },
                Request::SetFullscreen {
                    method,
                    framerate,
                    output,
                } => Message {
                    sender_id: sender_id,
                    opcode: 5,
                    args: smallvec![
                        Argument::Uint(method.to_raw()),
                        Argument::Uint(framerate),
                        Argument::Object(output.map(|o| o.as_ref().id()).unwrap_or(0)),
                    ],
                },
                Request::SetPopup {
                    seat,
                    serial,
                    parent,
                    x,
                    y,
                    flags,
                } => Message {
                    sender_id: sender_id,
                    opcode: 6,
                    args: smallvec![
                        Argument::Object(seat.as_ref().id()),
                        Argument::Uint(serial),
                        Argument::Object(parent.as_ref().id()),
                        Argument::Int(x),
                        Argument::Int(y),
                        Argument::Uint(flags.to_raw()),
                    ],
                },
                Request::SetMaximized { output } => Message {
                    sender_id: sender_id,
                    opcode: 7,
                    args: smallvec![Argument::Object(
                        output.map(|o| o.as_ref().id()).unwrap_or(0)
                    ),],
                },
                Request::SetTitle { title } => Message {
                    sender_id: sender_id,
                    opcode: 8,
                    args: smallvec![Argument::Str(Box::new(unsafe {
                        ::std::ffi::CString::from_vec_unchecked(title.into())
                    })),],
                },
                Request::SetClass { class_ } => Message {
                    sender_id: sender_id,
                    opcode: 9,
                    args: smallvec![Argument::Str(Box::new(unsafe {
                        ::std::ffi::CString::from_vec_unchecked(class_.into())
                    })),],
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
                Request::Pong { serial } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    f(0, &mut _args_array)
                }
                Request::Move { seat, serial } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.as_ref().c_ptr() as *mut _;
                    _args_array[1].u = serial;
                    f(1, &mut _args_array)
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
                    f(2, &mut _args_array)
                }
                Request::SetToplevel => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(3, &mut _args_array)
                }
                Request::SetTransient {
                    parent,
                    x,
                    y,
                    flags,
                } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = parent.as_ref().c_ptr() as *mut _;
                    _args_array[1].i = x;
                    _args_array[2].i = y;
                    _args_array[3].u = flags.to_raw();
                    f(4, &mut _args_array)
                }
                Request::SetFullscreen {
                    method,
                    framerate,
                    output,
                } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = method.to_raw();
                    _args_array[1].u = framerate;
                    _args_array[2].o = output
                        .map(|o| o.as_ref().c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    f(5, &mut _args_array)
                }
                Request::SetPopup {
                    seat,
                    serial,
                    parent,
                    x,
                    y,
                    flags,
                } => {
                    let mut _args_array: [wl_argument; 6] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.as_ref().c_ptr() as *mut _;
                    _args_array[1].u = serial;
                    _args_array[2].o = parent.as_ref().c_ptr() as *mut _;
                    _args_array[3].i = x;
                    _args_array[4].i = y;
                    _args_array[5].u = flags.to_raw();
                    f(6, &mut _args_array)
                }
                Request::SetMaximized { output } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = output
                        .map(|o| o.as_ref().c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    f(7, &mut _args_array)
                }
                Request::SetTitle { title } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(title).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    f(8, &mut _args_array)
                }
                Request::SetClass { class_ } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(class_).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    f(9, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "ping client\n\nPing a client to check if it is receiving events and sending\nrequests. A client is expected to reply with a pong request."]
        Ping { serial: u32 },
        #[doc = "suggest resize\n\nThe configure event asks the client to resize its surface.\n\nThe size is a hint, in the sense that the client is free to\nignore it if it doesn't resize, pick a smaller size (to\nsatisfy aspect ratio or resize in steps of NxM pixels).\n\nThe edges parameter provides a hint about how the surface\nwas resized. The client may use this information to decide\nhow to adjust its content to the new size (e.g. a scrolling\narea might adjust its content position to leave the viewable\ncontent unmoved).\n\nThe client is free to dismiss all but the last configure\nevent it received.\n\nThe width and height arguments specify the size of the window\nin surface-local coordinates."]
        Configure {
            edges: Resize,
            width: i32,
            height: i32,
        },
        #[doc = "popup interaction is done\n\nThe popup_done event is sent out when a popup grab is broken,\nthat is, when the user clicks a surface that doesn't belong\nto the client owning the popup surface."]
        PopupDone,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "ping",
                since: 1,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "configure",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
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
                Event::Ping { .. } => 0,
                Event::Configure { .. } => 1,
                Event::PopupDone => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Ping { .. } => 1,
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
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Configure {
                        edges: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                Resize::from_raw(val).ok_or(())?
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
                2 => Ok(Event::PopupDone),
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
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Configure {
                        edges: Resize::from_raw(_args[0].u).ok_or(())?,
                        width: _args[1].i,
                        height: _args[2].i,
                    })
                }
                2 => Ok(Event::PopupDone),
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
    pub struct WlShellSurface(Proxy<WlShellSurface>);
    impl AsRef<Proxy<WlShellSurface>> for WlShellSurface {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlShellSurface>> for WlShellSurface {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlShellSurface(value)
        }
    }
    impl From<WlShellSurface> for Proxy<WlShellSurface> {
        #[inline]
        fn from(value: WlShellSurface) -> Self {
            value.0
        }
    }
    impl Interface for WlShellSurface {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_shell_surface";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_shell_surface_interface }
        }
    }
    impl WlShellSurface {
        #[doc = "respond to a ping event\n\nA client must respond to a ping event with a pong request or\nthe client may be deemed unresponsive."]
        pub fn pong(&self, serial: u32) -> () {
            let msg = Request::Pong { serial: serial };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "start an interactive move\n\nStart a pointer-driven move of the surface.\n\nThis request must be used in response to a button press event.\nThe server may ignore move requests depending on the state of\nthe surface (e.g. fullscreen or maximized)."]
        pub fn _move(&self, seat: &super::wl_seat::WlSeat, serial: u32) -> () {
            let msg = Request::Move {
                seat: seat.clone(),
                serial: serial,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "start an interactive resize\n\nStart a pointer-driven resizing of the surface.\n\nThis request must be used in response to a button press event.\nThe server may ignore resize requests depending on the state of\nthe surface (e.g. fullscreen or maximized)."]
        pub fn resize(&self, seat: &super::wl_seat::WlSeat, serial: u32, edges: Resize) -> () {
            let msg = Request::Resize {
                seat: seat.clone(),
                serial: serial,
                edges: edges,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "make the surface a toplevel surface\n\nMap the surface as a toplevel surface.\n\nA toplevel surface is not fullscreen, maximized or transient."]
        pub fn set_toplevel(&self) -> () {
            let msg = Request::SetToplevel;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "make the surface a transient surface\n\nMap the surface relative to an existing surface.\n\nThe x and y arguments specify the location of the upper left\ncorner of the surface relative to the upper left corner of the\nparent surface, in surface-local coordinates.\n\nThe flags argument controls details of the transient behaviour."]
        pub fn set_transient(
            &self,
            parent: &super::wl_surface::WlSurface,
            x: i32,
            y: i32,
            flags: Transient,
        ) -> () {
            let msg = Request::SetTransient {
                parent: parent.clone(),
                x: x,
                y: y,
                flags: flags,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "make the surface a fullscreen surface\n\nMap the surface as a fullscreen surface.\n\nIf an output parameter is given then the surface will be made\nfullscreen on that output. If the client does not specify the\noutput then the compositor will apply its policy - usually\nchoosing the output on which the surface has the biggest surface\narea.\n\nThe client may specify a method to resolve a size conflict\nbetween the output size and the surface size - this is provided\nthrough the method parameter.\n\nThe framerate parameter is used only when the method is set\nto \"driver\", to indicate the preferred framerate. A value of 0\nindicates that the client does not care about framerate.  The\nframerate is specified in mHz, that is framerate of 60000 is 60Hz.\n\nA method of \"scale\" or \"driver\" implies a scaling operation of\nthe surface, either via a direct scaling operation or a change of\nthe output mode. This will override any kind of output scaling, so\nthat mapping a surface with a buffer size equal to the mode can\nfill the screen independent of buffer_scale.\n\nA method of \"fill\" means we don't scale up the buffer, however\nany output scale is applied. This means that you may run into\nan edge case where the application maps a buffer with the same\nsize of the output mode but buffer_scale 1 (thus making a\nsurface larger than the output). In this case it is allowed to\ndownscale the results to fit the screen.\n\nThe compositor must reply to this request with a configure event\nwith the dimensions for the output on which the surface will\nbe made fullscreen."]
        pub fn set_fullscreen(
            &self,
            method: FullscreenMethod,
            framerate: u32,
            output: Option<&super::wl_output::WlOutput>,
        ) -> () {
            let msg = Request::SetFullscreen {
                method: method,
                framerate: framerate,
                output: output.map(|o| o.clone()),
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "make the surface a popup surface\n\nMap the surface as a popup.\n\nA popup surface is a transient surface with an added pointer\ngrab.\n\nAn existing implicit grab will be changed to owner-events mode,\nand the popup grab will continue after the implicit grab ends\n(i.e. releasing the mouse button does not cause the popup to\nbe unmapped).\n\nThe popup grab continues until the window is destroyed or a\nmouse button is pressed in any other client's window. A click\nin any of the client's surfaces is reported as normal, however,\nclicks in other clients' surfaces will be discarded and trigger\nthe callback.\n\nThe x and y arguments specify the location of the upper left\ncorner of the surface relative to the upper left corner of the\nparent surface, in surface-local coordinates."]
        pub fn set_popup(
            &self,
            seat: &super::wl_seat::WlSeat,
            serial: u32,
            parent: &super::wl_surface::WlSurface,
            x: i32,
            y: i32,
            flags: Transient,
        ) -> () {
            let msg = Request::SetPopup {
                seat: seat.clone(),
                serial: serial,
                parent: parent.clone(),
                x: x,
                y: y,
                flags: flags,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "make the surface a maximized surface\n\nMap the surface as a maximized surface.\n\nIf an output parameter is given then the surface will be\nmaximized on that output. If the client does not specify the\noutput then the compositor will apply its policy - usually\nchoosing the output on which the surface has the biggest surface\narea.\n\nThe compositor will reply with a configure event telling\nthe expected new surface size. The operation is completed\non the next buffer attach to this surface.\n\nA maximized surface typically fills the entire output it is\nbound to, except for desktop elements such as panels. This is\nthe main difference between a maximized shell surface and a\nfullscreen shell surface.\n\nThe details depend on the compositor implementation."]
        pub fn set_maximized(&self, output: Option<&super::wl_output::WlOutput>) -> () {
            let msg = Request::SetMaximized {
                output: output.map(|o| o.clone()),
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set surface title\n\nSet a short title for the surface.\n\nThis string may be used to identify the surface in a task bar,\nwindow list, or other user interface elements provided by the\ncompositor.\n\nThe string must be encoded in UTF-8."]
        pub fn set_title(&self, title: String) -> () {
            let msg = Request::SetTitle { title: title };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set surface class\n\nSet a class for the surface.\n\nThe surface class identifies the general class of applications\nto which the surface belongs. A common convention is to use the\nfile name (or the full path if it is a non-standard location) of\nthe application's .desktop file as the class."]
        pub fn set_class(&self, class_: String) -> () {
            let msg = Request::SetClass { class_: class_ };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_PONG_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_MOVE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_RESIZE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_TOPLEVEL_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_TRANSIENT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_FULLSCREEN_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_POPUP_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_MAXIMIZED_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_TITLE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_CLASS_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_PING_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CONFIGURE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_POPUP_DONE_SINCE: u32 = 1u32;
    static mut wl_shell_surface_requests_move_types: [*const wl_interface; 2] = [
        unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
    ];
    static mut wl_shell_surface_requests_resize_types: [*const wl_interface; 3] = [
        unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    static mut wl_shell_surface_requests_set_transient_types: [*const wl_interface; 4] = [
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    static mut wl_shell_surface_requests_set_fullscreen_types: [*const wl_interface; 3] = [
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_output::wl_output_interface as *const wl_interface },
    ];
    static mut wl_shell_surface_requests_set_popup_types: [*const wl_interface; 6] = [
        unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    static mut wl_shell_surface_requests_set_maximized_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_output::wl_output_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_shell_surface_requests: [wl_message; 10] = [
        wl_message {
            name: b"pong\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"move\0" as *const u8 as *const c_char,
            signature: b"ou\0" as *const u8 as *const c_char,
            types: unsafe { &wl_shell_surface_requests_move_types as *const _ },
        },
        wl_message {
            name: b"resize\0" as *const u8 as *const c_char,
            signature: b"ouu\0" as *const u8 as *const c_char,
            types: unsafe { &wl_shell_surface_requests_resize_types as *const _ },
        },
        wl_message {
            name: b"set_toplevel\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_transient\0" as *const u8 as *const c_char,
            signature: b"oiiu\0" as *const u8 as *const c_char,
            types: unsafe { &wl_shell_surface_requests_set_transient_types as *const _ },
        },
        wl_message {
            name: b"set_fullscreen\0" as *const u8 as *const c_char,
            signature: b"uu?o\0" as *const u8 as *const c_char,
            types: unsafe { &wl_shell_surface_requests_set_fullscreen_types as *const _ },
        },
        wl_message {
            name: b"set_popup\0" as *const u8 as *const c_char,
            signature: b"ouoiiu\0" as *const u8 as *const c_char,
            types: unsafe { &wl_shell_surface_requests_set_popup_types as *const _ },
        },
        wl_message {
            name: b"set_maximized\0" as *const u8 as *const c_char,
            signature: b"?o\0" as *const u8 as *const c_char,
            types: unsafe { &wl_shell_surface_requests_set_maximized_types as *const _ },
        },
        wl_message {
            name: b"set_title\0" as *const u8 as *const c_char,
            signature: b"s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_class\0" as *const u8 as *const c_char,
            signature: b"s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_shell_surface_events: [wl_message; 3] = [
        wl_message {
            name: b"ping\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"configure\0" as *const u8 as *const c_char,
            signature: b"uii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"popup_done\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_shell_surface_interface: wl_interface = wl_interface {
        name: b"wl_shell_surface\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 10,
        requests: unsafe { &wl_shell_surface_requests as *const _ },
        event_count: 3,
        events: unsafe { &wl_shell_surface_events as *const _ },
    };
}
#[doc = "an onscreen surface\n\nA surface is a rectangular area that is displayed on the screen.\nIt has a location, size and pixel contents.\n\nThe size of a surface (and relative positions on it) is described\nin surface-local coordinates, which may differ from the buffer\ncoordinates of the pixel content, in case a buffer_transform\nor a buffer_scale is used.\n\nA surface without a \"role\" is fairly useless: a compositor does\nnot know where, when or how to present it. The role is the\npurpose of a wl_surface. Examples of roles are a cursor for a\npointer (as set by wl_pointer.set_cursor), a drag icon\n(wl_data_device.start_drag), a sub-surface\n(wl_subcompositor.get_subsurface), and a window as defined by a\nshell protocol (e.g. wl_shell.get_shell_surface).\n\nA surface can have only one role at a time. Initially a\nwl_surface does not have a role. Once a wl_surface is given a\nrole, it is set permanently for the whole lifetime of the\nwl_surface object. Giving the current role again is allowed,\nunless explicitly forbidden by the relevant interface\nspecification.\n\nSurface roles are given by requests in other interfaces such as\nwl_pointer.set_cursor. The request should explicitly mention\nthat this request gives a role to a wl_surface. Often, this\nrequest also creates a new protocol object that represents the\nrole and adds additional functionality to wl_surface. When a\nclient wants to destroy a wl_surface, they must destroy this 'role\nobject' before the wl_surface.\n\nDestroying the role object does not remove the role from the\nwl_surface, but it may stop the wl_surface from \"playing the role\".\nFor instance, if a wl_subsurface object is destroyed, the wl_surface\nit was created for will be unmapped and forget its position and\nz-order. It is allowed to create a wl_subsurface for the same\nwl_surface again, but it is not allowed to use the wl_surface as\na cursor (cursor is a different role than sub-surface, and role\nswitching is not allowed)."]
pub mod wl_surface {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[doc = "wl_surface error values\n\nThese errors can be emitted in response to wl_surface requests."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Error {
        #[doc = "buffer scale value is invalid"]
        InvalidScale = 0,
        #[doc = "buffer transform value is invalid"]
        InvalidTransform = 1,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidScale),
                1 => Some(Error::InvalidTransform),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "delete surface\n\nDeletes the surface and invalidates its object ID.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "set the surface contents\n\nSet a buffer as the content of this surface.\n\nThe new size of the surface is calculated based on the buffer\nsize transformed by the inverse buffer_transform and the\ninverse buffer_scale. This means that the supplied buffer\nmust be an integer multiple of the buffer_scale.\n\nThe x and y arguments specify the location of the new pending\nbuffer's upper left corner, relative to the current buffer's upper\nleft corner, in surface-local coordinates. In other words, the\nx and y, combined with the new surface size define in which\ndirections the surface's size changes.\n\nSurface contents are double-buffered state, see wl_surface.commit.\n\nThe initial surface contents are void; there is no content.\nwl_surface.attach assigns the given wl_buffer as the pending\nwl_buffer. wl_surface.commit makes the pending wl_buffer the new\nsurface contents, and the size of the surface becomes the size\ncalculated from the wl_buffer, as described above. After commit,\nthere is no pending buffer until the next attach.\n\nCommitting a pending wl_buffer allows the compositor to read the\npixels in the wl_buffer. The compositor may access the pixels at\nany time after the wl_surface.commit request. When the compositor\nwill not access the pixels anymore, it will send the\nwl_buffer.release event. Only after receiving wl_buffer.release,\nthe client may reuse the wl_buffer. A wl_buffer that has been\nattached and then replaced by another attach instead of committed\nwill not receive a release event, and is not used by the\ncompositor.\n\nDestroying the wl_buffer after wl_buffer.release does not change\nthe surface contents. However, if the client destroys the\nwl_buffer before receiving the wl_buffer.release event, the surface\ncontents become undefined immediately.\n\nIf wl_surface.attach is sent with a NULL wl_buffer, the\nfollowing wl_surface.commit will remove the surface content."]
        Attach {
            buffer: Option<super::wl_buffer::WlBuffer>,
            x: i32,
            y: i32,
        },
        #[doc = "mark part of the surface damaged\n\nThis request is used to describe the regions where the pending\nbuffer is different from the current surface contents, and where\nthe surface therefore needs to be repainted. The compositor\nignores the parts of the damage that fall outside of the surface.\n\nDamage is double-buffered state, see wl_surface.commit.\n\nThe damage rectangle is specified in surface-local coordinates,\nwhere x and y specify the upper left corner of the damage rectangle.\n\nThe initial value for pending damage is empty: no damage.\nwl_surface.damage adds pending damage: the new pending damage\nis the union of old pending damage and the given rectangle.\n\nwl_surface.commit assigns pending damage as the current damage,\nand clears pending damage. The server will clear the current\ndamage as it repaints the surface.\n\nAlternatively, damage can be posted with wl_surface.damage_buffer\nwhich uses buffer coordinates instead of surface coordinates,\nand is probably the preferred and intuitive way of doing this."]
        Damage {
            x: i32,
            y: i32,
            width: i32,
            height: i32,
        },
        #[doc = "request a frame throttling hint\n\nRequest a notification when it is a good time to start drawing a new\nframe, by creating a frame callback. This is useful for throttling\nredrawing operations, and driving animations.\n\nWhen a client is animating on a wl_surface, it can use the 'frame'\nrequest to get notified when it is a good time to draw and commit the\nnext frame of animation. If the client commits an update earlier than\nthat, it is likely that some updates will not make it to the display,\nand the client is wasting resources by drawing too often.\n\nThe frame request will take effect on the next wl_surface.commit.\nThe notification will only be posted for one frame unless\nrequested again. For a wl_surface, the notifications are posted in\nthe order the frame requests were committed.\n\nThe server must send the notifications so that a client\nwill not send excessive updates, while still allowing\nthe highest possible update rate for clients that wait for the reply\nbefore drawing again. The server should give some time for the client\nto draw and commit after sending the frame callback events to let it\nhit the next output refresh.\n\nA server should avoid signaling the frame callbacks if the\nsurface is not visible in any way, e.g. the surface is off-screen,\nor completely obscured by other opaque surfaces.\n\nThe object returned by this request will be destroyed by the\ncompositor after the callback is fired and as such the client must not\nattempt to use it after that point.\n\nThe callback_data passed in the callback is the current time, in\nmilliseconds, with an undefined base."]
        Frame {},
        #[doc = "set opaque region\n\nThis request sets the region of the surface that contains\nopaque content.\n\nThe opaque region is an optimization hint for the compositor\nthat lets it optimize the redrawing of content behind opaque\nregions.  Setting an opaque region is not required for correct\nbehaviour, but marking transparent content as opaque will result\nin repaint artifacts.\n\nThe opaque region is specified in surface-local coordinates.\n\nThe compositor ignores the parts of the opaque region that fall\noutside of the surface.\n\nOpaque region is double-buffered state, see wl_surface.commit.\n\nwl_surface.set_opaque_region changes the pending opaque region.\nwl_surface.commit copies the pending region to the current region.\nOtherwise, the pending and current regions are never changed.\n\nThe initial value for an opaque region is empty. Setting the pending\nopaque region has copy semantics, and the wl_region object can be\ndestroyed immediately. A NULL wl_region causes the pending opaque\nregion to be set to empty."]
        SetOpaqueRegion {
            region: Option<super::wl_region::WlRegion>,
        },
        #[doc = "set input region\n\nThis request sets the region of the surface that can receive\npointer and touch events.\n\nInput events happening outside of this region will try the next\nsurface in the server surface stack. The compositor ignores the\nparts of the input region that fall outside of the surface.\n\nThe input region is specified in surface-local coordinates.\n\nInput region is double-buffered state, see wl_surface.commit.\n\nwl_surface.set_input_region changes the pending input region.\nwl_surface.commit copies the pending region to the current region.\nOtherwise the pending and current regions are never changed,\nexcept cursor and icon surfaces are special cases, see\nwl_pointer.set_cursor and wl_data_device.start_drag.\n\nThe initial value for an input region is infinite. That means the\nwhole surface will accept input. Setting the pending input region\nhas copy semantics, and the wl_region object can be destroyed\nimmediately. A NULL wl_region causes the input region to be set\nto infinite."]
        SetInputRegion {
            region: Option<super::wl_region::WlRegion>,
        },
        #[doc = "commit pending surface state\n\nSurface state (input, opaque, and damage regions, attached buffers,\netc.) is double-buffered. Protocol requests modify the pending state,\nas opposed to the current state in use by the compositor. A commit\nrequest atomically applies all pending state, replacing the current\nstate. After commit, the new pending state is as documented for each\nrelated request.\n\nOn commit, a pending wl_buffer is applied first, and all other state\nsecond. This means that all coordinates in double-buffered state are\nrelative to the new wl_buffer coming into use, except for\nwl_surface.attach itself. If there is no pending wl_buffer, the\ncoordinates are relative to the current surface contents.\n\nAll requests that need a commit to become effective are documented\nto affect double-buffered state.\n\nOther interfaces may add further double-buffered surface state."]
        Commit,
        #[doc = "sets the buffer transformation\n\nThis request sets an optional transformation on how the compositor\ninterprets the contents of the buffer attached to the surface. The\naccepted values for the transform parameter are the values for\nwl_output.transform.\n\nBuffer transform is double-buffered state, see wl_surface.commit.\n\nA newly created surface has its buffer transformation set to normal.\n\nwl_surface.set_buffer_transform changes the pending buffer\ntransformation. wl_surface.commit copies the pending buffer\ntransformation to the current one. Otherwise, the pending and current\nvalues are never changed.\n\nThe purpose of this request is to allow clients to render content\naccording to the output transform, thus permitting the compositor to\nuse certain optimizations even if the display is rotated. Using\nhardware overlays and scanning out a client buffer for fullscreen\nsurfaces are examples of such optimizations. Those optimizations are\nhighly dependent on the compositor implementation, so the use of this\nrequest should be considered on a case-by-case basis.\n\nNote that if the transform value includes 90 or 270 degree rotation,\nthe width of the buffer will become the surface height and the height\nof the buffer will become the surface width.\n\nIf transform is not one of the values from the\nwl_output.transform enum the invalid_transform protocol error\nis raised.\n\nOnly available since version 2 of the interface"]
        SetBufferTransform {
            transform: super::wl_output::Transform,
        },
        #[doc = "sets the buffer scaling factor\n\nThis request sets an optional scaling factor on how the compositor\ninterprets the contents of the buffer attached to the window.\n\nBuffer scale is double-buffered state, see wl_surface.commit.\n\nA newly created surface has its buffer scale set to 1.\n\nwl_surface.set_buffer_scale changes the pending buffer scale.\nwl_surface.commit copies the pending buffer scale to the current one.\nOtherwise, the pending and current values are never changed.\n\nThe purpose of this request is to allow clients to supply higher\nresolution buffer data for use on high resolution outputs. It is\nintended that you pick the same buffer scale as the scale of the\noutput that the surface is displayed on. This means the compositor\ncan avoid scaling when rendering the surface on that output.\n\nNote that if the scale is larger than 1, then you have to attach\na buffer that is larger (by a factor of scale in each dimension)\nthan the desired surface size.\n\nIf scale is not positive the invalid_scale protocol error is\nraised.\n\nOnly available since version 3 of the interface"]
        SetBufferScale { scale: i32 },
        #[doc = "mark part of the surface damaged using buffer coordinates\n\nThis request is used to describe the regions where the pending\nbuffer is different from the current surface contents, and where\nthe surface therefore needs to be repainted. The compositor\nignores the parts of the damage that fall outside of the surface.\n\nDamage is double-buffered state, see wl_surface.commit.\n\nThe damage rectangle is specified in buffer coordinates,\nwhere x and y specify the upper left corner of the damage rectangle.\n\nThe initial value for pending damage is empty: no damage.\nwl_surface.damage_buffer adds pending damage: the new pending\ndamage is the union of old pending damage and the given rectangle.\n\nwl_surface.commit assigns pending damage as the current damage,\nand clears pending damage. The server will clear the current\ndamage as it repaints the surface.\n\nThis request differs from wl_surface.damage in only one way - it\ntakes damage in buffer coordinates instead of surface-local\ncoordinates. While this generally is more intuitive than surface\ncoordinates, it is especially desirable when using wp_viewport\nor when a drawing library (like EGL) is unaware of buffer scale\nand buffer transform.\n\nNote: Because buffer transformation changes and damage requests may\nbe interleaved in the protocol stream, it is impossible to determine\nthe actual mapping between surface and buffer damage until\nwl_surface.commit time. Therefore, compositors wishing to take both\nkinds of damage into account will have to accumulate damage from the\ntwo requests separately and only transform from one to the other\nafter receiving the wl_surface.commit.\n\nOnly available since version 4 of the interface"]
        DamageBuffer {
            x: i32,
            y: i32,
            width: i32,
            height: i32,
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
                name: "attach",
                since: 1,
                signature: &[
                    super::ArgumentType::Object,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "damage",
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
                name: "frame",
                since: 1,
                signature: &[super::ArgumentType::NewId],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_opaque_region",
                since: 1,
                signature: &[super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_input_region",
                since: 1,
                signature: &[super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "commit",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_buffer_transform",
                since: 2,
                signature: &[super::ArgumentType::Int],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_buffer_scale",
                since: 3,
                signature: &[super::ArgumentType::Int],
                destructor: false,
            },
            super::MessageDesc {
                name: "damage_buffer",
                since: 4,
                signature: &[
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
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
                Request::Attach { .. } => 1,
                Request::Damage { .. } => 2,
                Request::Frame { .. } => 3,
                Request::SetOpaqueRegion { .. } => 4,
                Request::SetInputRegion { .. } => 5,
                Request::Commit => 6,
                Request::SetBufferTransform { .. } => 7,
                Request::SetBufferScale { .. } => 8,
                Request::DamageBuffer { .. } => 9,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::Attach { .. } => 1,
                Request::Damage { .. } => 1,
                Request::Frame { .. } => 1,
                Request::SetOpaqueRegion { .. } => 1,
                Request::SetInputRegion { .. } => 1,
                Request::Commit => 1,
                Request::SetBufferTransform { .. } => 2,
                Request::SetBufferScale { .. } => 3,
                Request::DamageBuffer { .. } => 4,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                3 => Some(Object::from_interface::<super::wl_callback::WlCallback>(
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
                Request::Attach { buffer, x, y } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![
                        Argument::Object(buffer.map(|o| o.as_ref().id()).unwrap_or(0)),
                        Argument::Int(x),
                        Argument::Int(y),
                    ],
                },
                Request::Damage {
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
                Request::Frame {} => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: smallvec![Argument::NewId(0),],
                },
                Request::SetOpaqueRegion { region } => Message {
                    sender_id: sender_id,
                    opcode: 4,
                    args: smallvec![Argument::Object(
                        region.map(|o| o.as_ref().id()).unwrap_or(0)
                    ),],
                },
                Request::SetInputRegion { region } => Message {
                    sender_id: sender_id,
                    opcode: 5,
                    args: smallvec![Argument::Object(
                        region.map(|o| o.as_ref().id()).unwrap_or(0)
                    ),],
                },
                Request::Commit => Message {
                    sender_id: sender_id,
                    opcode: 6,
                    args: smallvec![],
                },
                Request::SetBufferTransform { transform } => Message {
                    sender_id: sender_id,
                    opcode: 7,
                    args: smallvec![Argument::Int(transform.to_raw() as i32),],
                },
                Request::SetBufferScale { scale } => Message {
                    sender_id: sender_id,
                    opcode: 8,
                    args: smallvec![Argument::Int(scale),],
                },
                Request::DamageBuffer {
                    x,
                    y,
                    width,
                    height,
                } => Message {
                    sender_id: sender_id,
                    opcode: 9,
                    args: smallvec![
                        Argument::Int(x),
                        Argument::Int(y),
                        Argument::Int(width),
                        Argument::Int(height),
                    ],
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
                Request::Attach { buffer, x, y } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = buffer
                        .map(|o| o.as_ref().c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    _args_array[1].i = x;
                    _args_array[2].i = y;
                    f(1, &mut _args_array)
                }
                Request::Damage {
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
                Request::Frame {} => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    f(3, &mut _args_array)
                }
                Request::SetOpaqueRegion { region } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = region
                        .map(|o| o.as_ref().c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    f(4, &mut _args_array)
                }
                Request::SetInputRegion { region } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = region
                        .map(|o| o.as_ref().c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    f(5, &mut _args_array)
                }
                Request::Commit => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(6, &mut _args_array)
                }
                Request::SetBufferTransform { transform } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = transform.to_raw() as i32;
                    f(7, &mut _args_array)
                }
                Request::SetBufferScale { scale } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = scale;
                    f(8, &mut _args_array)
                }
                Request::DamageBuffer {
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
                    f(9, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "surface enters an output\n\nThis is emitted whenever a surface's creation, movement, or resizing\nresults in some part of it being within the scanout region of an\noutput.\n\nNote that a surface may be overlapping with zero or more outputs."]
        Enter { output: super::wl_output::WlOutput },
        #[doc = "surface leaves an output\n\nThis is emitted whenever a surface's creation, movement, or resizing\nresults in it no longer having any part of it within the scanout region\nof an output."]
        Leave { output: super::wl_output::WlOutput },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "enter",
                since: 1,
                signature: &[super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "leave",
                since: 1,
                signature: &[super::ArgumentType::Object],
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
                Event::Enter { .. } => 0,
                Event::Leave { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Enter { .. } => 1,
                Event::Leave { .. } => 1,
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
                    Ok(Event::Enter {
                        output: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?.into()
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Leave {
                        output: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?.into()
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
                    Ok(Event::Enter {
                        output: Proxy::<super::wl_output::WlOutput>::from_c_ptr(
                            _args[0].o as *mut _,
                        )
                        .into(),
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Leave {
                        output: Proxy::<super::wl_output::WlOutput>::from_c_ptr(
                            _args[0].o as *mut _,
                        )
                        .into(),
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
    pub struct WlSurface(Proxy<WlSurface>);
    impl AsRef<Proxy<WlSurface>> for WlSurface {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlSurface>> for WlSurface {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlSurface(value)
        }
    }
    impl From<WlSurface> for Proxy<WlSurface> {
        #[inline]
        fn from(value: WlSurface) -> Self {
            value.0
        }
    }
    impl Interface for WlSurface {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_surface";
        const VERSION: u32 = 4;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_surface_interface }
        }
    }
    impl WlSurface {
        #[doc = "delete surface\n\nDeletes the surface and invalidates its object ID.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the surface contents\n\nSet a buffer as the content of this surface.\n\nThe new size of the surface is calculated based on the buffer\nsize transformed by the inverse buffer_transform and the\ninverse buffer_scale. This means that the supplied buffer\nmust be an integer multiple of the buffer_scale.\n\nThe x and y arguments specify the location of the new pending\nbuffer's upper left corner, relative to the current buffer's upper\nleft corner, in surface-local coordinates. In other words, the\nx and y, combined with the new surface size define in which\ndirections the surface's size changes.\n\nSurface contents are double-buffered state, see wl_surface.commit.\n\nThe initial surface contents are void; there is no content.\nwl_surface.attach assigns the given wl_buffer as the pending\nwl_buffer. wl_surface.commit makes the pending wl_buffer the new\nsurface contents, and the size of the surface becomes the size\ncalculated from the wl_buffer, as described above. After commit,\nthere is no pending buffer until the next attach.\n\nCommitting a pending wl_buffer allows the compositor to read the\npixels in the wl_buffer. The compositor may access the pixels at\nany time after the wl_surface.commit request. When the compositor\nwill not access the pixels anymore, it will send the\nwl_buffer.release event. Only after receiving wl_buffer.release,\nthe client may reuse the wl_buffer. A wl_buffer that has been\nattached and then replaced by another attach instead of committed\nwill not receive a release event, and is not used by the\ncompositor.\n\nDestroying the wl_buffer after wl_buffer.release does not change\nthe surface contents. However, if the client destroys the\nwl_buffer before receiving the wl_buffer.release event, the surface\ncontents become undefined immediately.\n\nIf wl_surface.attach is sent with a NULL wl_buffer, the\nfollowing wl_surface.commit will remove the surface content."]
        pub fn attach(&self, buffer: Option<&super::wl_buffer::WlBuffer>, x: i32, y: i32) -> () {
            let msg = Request::Attach {
                buffer: buffer.map(|o| o.clone()),
                x: x,
                y: y,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "mark part of the surface damaged\n\nThis request is used to describe the regions where the pending\nbuffer is different from the current surface contents, and where\nthe surface therefore needs to be repainted. The compositor\nignores the parts of the damage that fall outside of the surface.\n\nDamage is double-buffered state, see wl_surface.commit.\n\nThe damage rectangle is specified in surface-local coordinates,\nwhere x and y specify the upper left corner of the damage rectangle.\n\nThe initial value for pending damage is empty: no damage.\nwl_surface.damage adds pending damage: the new pending damage\nis the union of old pending damage and the given rectangle.\n\nwl_surface.commit assigns pending damage as the current damage,\nand clears pending damage. The server will clear the current\ndamage as it repaints the surface.\n\nAlternatively, damage can be posted with wl_surface.damage_buffer\nwhich uses buffer coordinates instead of surface coordinates,\nand is probably the preferred and intuitive way of doing this."]
        pub fn damage(&self, x: i32, y: i32, width: i32, height: i32) -> () {
            let msg = Request::Damage {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "request a frame throttling hint\n\nRequest a notification when it is a good time to start drawing a new\nframe, by creating a frame callback. This is useful for throttling\nredrawing operations, and driving animations.\n\nWhen a client is animating on a wl_surface, it can use the 'frame'\nrequest to get notified when it is a good time to draw and commit the\nnext frame of animation. If the client commits an update earlier than\nthat, it is likely that some updates will not make it to the display,\nand the client is wasting resources by drawing too often.\n\nThe frame request will take effect on the next wl_surface.commit.\nThe notification will only be posted for one frame unless\nrequested again. For a wl_surface, the notifications are posted in\nthe order the frame requests were committed.\n\nThe server must send the notifications so that a client\nwill not send excessive updates, while still allowing\nthe highest possible update rate for clients that wait for the reply\nbefore drawing again. The server should give some time for the client\nto draw and commit after sending the frame callback events to let it\nhit the next output refresh.\n\nA server should avoid signaling the frame callbacks if the\nsurface is not visible in any way, e.g. the surface is off-screen,\nor completely obscured by other opaque surfaces.\n\nThe object returned by this request will be destroyed by the\ncompositor after the callback is fired and as such the client must not\nattempt to use it after that point.\n\nThe callback_data passed in the callback is the current time, in\nmilliseconds, with an undefined base."]
        pub fn frame(&self) -> Main<super::wl_callback::WlCallback> {
            let msg = Request::Frame {};
            self.0.send(msg, None).unwrap()
        }
        #[doc = "set opaque region\n\nThis request sets the region of the surface that contains\nopaque content.\n\nThe opaque region is an optimization hint for the compositor\nthat lets it optimize the redrawing of content behind opaque\nregions.  Setting an opaque region is not required for correct\nbehaviour, but marking transparent content as opaque will result\nin repaint artifacts.\n\nThe opaque region is specified in surface-local coordinates.\n\nThe compositor ignores the parts of the opaque region that fall\noutside of the surface.\n\nOpaque region is double-buffered state, see wl_surface.commit.\n\nwl_surface.set_opaque_region changes the pending opaque region.\nwl_surface.commit copies the pending region to the current region.\nOtherwise, the pending and current regions are never changed.\n\nThe initial value for an opaque region is empty. Setting the pending\nopaque region has copy semantics, and the wl_region object can be\ndestroyed immediately. A NULL wl_region causes the pending opaque\nregion to be set to empty."]
        pub fn set_opaque_region(&self, region: Option<&super::wl_region::WlRegion>) -> () {
            let msg = Request::SetOpaqueRegion {
                region: region.map(|o| o.clone()),
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set input region\n\nThis request sets the region of the surface that can receive\npointer and touch events.\n\nInput events happening outside of this region will try the next\nsurface in the server surface stack. The compositor ignores the\nparts of the input region that fall outside of the surface.\n\nThe input region is specified in surface-local coordinates.\n\nInput region is double-buffered state, see wl_surface.commit.\n\nwl_surface.set_input_region changes the pending input region.\nwl_surface.commit copies the pending region to the current region.\nOtherwise the pending and current regions are never changed,\nexcept cursor and icon surfaces are special cases, see\nwl_pointer.set_cursor and wl_data_device.start_drag.\n\nThe initial value for an input region is infinite. That means the\nwhole surface will accept input. Setting the pending input region\nhas copy semantics, and the wl_region object can be destroyed\nimmediately. A NULL wl_region causes the input region to be set\nto infinite."]
        pub fn set_input_region(&self, region: Option<&super::wl_region::WlRegion>) -> () {
            let msg = Request::SetInputRegion {
                region: region.map(|o| o.clone()),
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "commit pending surface state\n\nSurface state (input, opaque, and damage regions, attached buffers,\netc.) is double-buffered. Protocol requests modify the pending state,\nas opposed to the current state in use by the compositor. A commit\nrequest atomically applies all pending state, replacing the current\nstate. After commit, the new pending state is as documented for each\nrelated request.\n\nOn commit, a pending wl_buffer is applied first, and all other state\nsecond. This means that all coordinates in double-buffered state are\nrelative to the new wl_buffer coming into use, except for\nwl_surface.attach itself. If there is no pending wl_buffer, the\ncoordinates are relative to the current surface contents.\n\nAll requests that need a commit to become effective are documented\nto affect double-buffered state.\n\nOther interfaces may add further double-buffered surface state."]
        pub fn commit(&self) -> () {
            let msg = Request::Commit;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "sets the buffer transformation\n\nThis request sets an optional transformation on how the compositor\ninterprets the contents of the buffer attached to the surface. The\naccepted values for the transform parameter are the values for\nwl_output.transform.\n\nBuffer transform is double-buffered state, see wl_surface.commit.\n\nA newly created surface has its buffer transformation set to normal.\n\nwl_surface.set_buffer_transform changes the pending buffer\ntransformation. wl_surface.commit copies the pending buffer\ntransformation to the current one. Otherwise, the pending and current\nvalues are never changed.\n\nThe purpose of this request is to allow clients to render content\naccording to the output transform, thus permitting the compositor to\nuse certain optimizations even if the display is rotated. Using\nhardware overlays and scanning out a client buffer for fullscreen\nsurfaces are examples of such optimizations. Those optimizations are\nhighly dependent on the compositor implementation, so the use of this\nrequest should be considered on a case-by-case basis.\n\nNote that if the transform value includes 90 or 270 degree rotation,\nthe width of the buffer will become the surface height and the height\nof the buffer will become the surface width.\n\nIf transform is not one of the values from the\nwl_output.transform enum the invalid_transform protocol error\nis raised.\n\nOnly available since version 2 of the interface."]
        pub fn set_buffer_transform(&self, transform: super::wl_output::Transform) -> () {
            let msg = Request::SetBufferTransform {
                transform: transform,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "sets the buffer scaling factor\n\nThis request sets an optional scaling factor on how the compositor\ninterprets the contents of the buffer attached to the window.\n\nBuffer scale is double-buffered state, see wl_surface.commit.\n\nA newly created surface has its buffer scale set to 1.\n\nwl_surface.set_buffer_scale changes the pending buffer scale.\nwl_surface.commit copies the pending buffer scale to the current one.\nOtherwise, the pending and current values are never changed.\n\nThe purpose of this request is to allow clients to supply higher\nresolution buffer data for use on high resolution outputs. It is\nintended that you pick the same buffer scale as the scale of the\noutput that the surface is displayed on. This means the compositor\ncan avoid scaling when rendering the surface on that output.\n\nNote that if the scale is larger than 1, then you have to attach\na buffer that is larger (by a factor of scale in each dimension)\nthan the desired surface size.\n\nIf scale is not positive the invalid_scale protocol error is\nraised.\n\nOnly available since version 3 of the interface."]
        pub fn set_buffer_scale(&self, scale: i32) -> () {
            let msg = Request::SetBufferScale { scale: scale };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "mark part of the surface damaged using buffer coordinates\n\nThis request is used to describe the regions where the pending\nbuffer is different from the current surface contents, and where\nthe surface therefore needs to be repainted. The compositor\nignores the parts of the damage that fall outside of the surface.\n\nDamage is double-buffered state, see wl_surface.commit.\n\nThe damage rectangle is specified in buffer coordinates,\nwhere x and y specify the upper left corner of the damage rectangle.\n\nThe initial value for pending damage is empty: no damage.\nwl_surface.damage_buffer adds pending damage: the new pending\ndamage is the union of old pending damage and the given rectangle.\n\nwl_surface.commit assigns pending damage as the current damage,\nand clears pending damage. The server will clear the current\ndamage as it repaints the surface.\n\nThis request differs from wl_surface.damage in only one way - it\ntakes damage in buffer coordinates instead of surface-local\ncoordinates. While this generally is more intuitive than surface\ncoordinates, it is especially desirable when using wp_viewport\nor when a drawing library (like EGL) is unaware of buffer scale\nand buffer transform.\n\nNote: Because buffer transformation changes and damage requests may\nbe interleaved in the protocol stream, it is impossible to determine\nthe actual mapping between surface and buffer damage until\nwl_surface.commit time. Therefore, compositors wishing to take both\nkinds of damage into account will have to accumulate damage from the\ntwo requests separately and only transform from one to the other\nafter receiving the wl_surface.commit.\n\nOnly available since version 4 of the interface."]
        pub fn damage_buffer(&self, x: i32, y: i32, width: i32, height: i32) -> () {
            let msg = Request::DamageBuffer {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_ATTACH_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DAMAGE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_FRAME_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_OPAQUE_REGION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_INPUT_REGION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_COMMIT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_BUFFER_TRANSFORM_SINCE: u32 = 2u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_BUFFER_SCALE_SINCE: u32 = 3u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DAMAGE_BUFFER_SINCE: u32 = 4u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_ENTER_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_LEAVE_SINCE: u32 = 1u32;
    static mut wl_surface_requests_attach_types: [*const wl_interface; 3] = [
        unsafe { &super::wl_buffer::wl_buffer_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    static mut wl_surface_requests_frame_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_callback::wl_callback_interface as *const wl_interface }];
    static mut wl_surface_requests_set_opaque_region_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_region::wl_region_interface as *const wl_interface }];
    static mut wl_surface_requests_set_input_region_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_region::wl_region_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_surface_requests: [wl_message; 10] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"attach\0" as *const u8 as *const c_char,
            signature: b"?oii\0" as *const u8 as *const c_char,
            types: unsafe { &wl_surface_requests_attach_types as *const _ },
        },
        wl_message {
            name: b"damage\0" as *const u8 as *const c_char,
            signature: b"iiii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"frame\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &wl_surface_requests_frame_types as *const _ },
        },
        wl_message {
            name: b"set_opaque_region\0" as *const u8 as *const c_char,
            signature: b"?o\0" as *const u8 as *const c_char,
            types: unsafe { &wl_surface_requests_set_opaque_region_types as *const _ },
        },
        wl_message {
            name: b"set_input_region\0" as *const u8 as *const c_char,
            signature: b"?o\0" as *const u8 as *const c_char,
            types: unsafe { &wl_surface_requests_set_input_region_types as *const _ },
        },
        wl_message {
            name: b"commit\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_buffer_transform\0" as *const u8 as *const c_char,
            signature: b"2i\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_buffer_scale\0" as *const u8 as *const c_char,
            signature: b"3i\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"damage_buffer\0" as *const u8 as *const c_char,
            signature: b"4iiii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    static mut wl_surface_events_enter_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_output::wl_output_interface as *const wl_interface }];
    static mut wl_surface_events_leave_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_output::wl_output_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_surface_events: [wl_message; 2] = [
        wl_message {
            name: b"enter\0" as *const u8 as *const c_char,
            signature: b"o\0" as *const u8 as *const c_char,
            types: unsafe { &wl_surface_events_enter_types as *const _ },
        },
        wl_message {
            name: b"leave\0" as *const u8 as *const c_char,
            signature: b"o\0" as *const u8 as *const c_char,
            types: unsafe { &wl_surface_events_leave_types as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_surface_interface: wl_interface = wl_interface {
        name: b"wl_surface\0" as *const u8 as *const c_char,
        version: 4,
        request_count: 10,
        requests: unsafe { &wl_surface_requests as *const _ },
        event_count: 2,
        events: unsafe { &wl_surface_events as *const _ },
    };
}
#[doc = "group of input devices\n\nA seat is a group of keyboards, pointer and touch devices. This\nobject is published as a global during start up, or when such a\ndevice is hot plugged.  A seat typically has a pointer and\nmaintains a keyboard focus and a pointer focus."]
pub mod wl_seat {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    bitflags! { # [ doc = "seat capability bitmask\n\nThis is a bitmask of capabilities this seat has; if a member is\nset, then it is present on the seat." ] pub struct Capability : u32 { # [ doc = "the seat has pointer devices" ] const Pointer = 1 ; # [ doc = "the seat has one or more keyboards" ] const Keyboard = 2 ; # [ doc = "the seat has touch devices" ] const Touch = 4 ; } }
    impl Capability {
        pub fn from_raw(n: u32) -> Option<Capability> {
            Some(Capability::from_bits_truncate(n))
        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "return pointer object\n\nThe ID provided will be initialized to the wl_pointer interface\nfor this seat.\n\nThis request only takes effect if the seat has the pointer\ncapability, or has had the pointer capability in the past.\nIt is a protocol violation to issue this request on a seat that has\nnever had the pointer capability."]
        GetPointer {},
        #[doc = "return keyboard object\n\nThe ID provided will be initialized to the wl_keyboard interface\nfor this seat.\n\nThis request only takes effect if the seat has the keyboard\ncapability, or has had the keyboard capability in the past.\nIt is a protocol violation to issue this request on a seat that has\nnever had the keyboard capability."]
        GetKeyboard {},
        #[doc = "return touch object\n\nThe ID provided will be initialized to the wl_touch interface\nfor this seat.\n\nThis request only takes effect if the seat has the touch\ncapability, or has had the touch capability in the past.\nIt is a protocol violation to issue this request on a seat that has\nnever had the touch capability."]
        GetTouch {},
        #[doc = "release the seat object\n\nUsing this request a client can tell the server that it is not going to\nuse the seat object anymore.\n\nThis is a destructor, once sent this object cannot be used any longer.\nOnly available since version 5 of the interface"]
        Release,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "get_pointer",
                since: 1,
                signature: &[super::ArgumentType::NewId],
                destructor: false,
            },
            super::MessageDesc {
                name: "get_keyboard",
                since: 1,
                signature: &[super::ArgumentType::NewId],
                destructor: false,
            },
            super::MessageDesc {
                name: "get_touch",
                since: 1,
                signature: &[super::ArgumentType::NewId],
                destructor: false,
            },
            super::MessageDesc {
                name: "release",
                since: 5,
                signature: &[],
                destructor: true,
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Release => true,
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::GetPointer { .. } => 0,
                Request::GetKeyboard { .. } => 1,
                Request::GetTouch { .. } => 2,
                Request::Release => 3,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::GetPointer { .. } => 1,
                Request::GetKeyboard { .. } => 1,
                Request::GetTouch { .. } => 1,
                Request::Release => 5,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<super::wl_pointer::WlPointer>(
                    version,
                    meta.child(),
                )),
                1 => Some(Object::from_interface::<super::wl_keyboard::WlKeyboard>(
                    version,
                    meta.child(),
                )),
                2 => Some(Object::from_interface::<super::wl_touch::WlTouch>(
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
                Request::GetPointer {} => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![Argument::NewId(0),],
                },
                Request::GetKeyboard {} => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::NewId(0),],
                },
                Request::GetTouch {} => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![Argument::NewId(0),],
                },
                Request::Release => Message {
                    sender_id: sender_id,
                    opcode: 3,
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
                Request::GetPointer {} => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    f(0, &mut _args_array)
                }
                Request::GetKeyboard {} => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    f(1, &mut _args_array)
                }
                Request::GetTouch {} => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    f(2, &mut _args_array)
                }
                Request::Release => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(3, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "seat capabilities changed\n\nThis is emitted whenever a seat gains or loses the pointer,\nkeyboard or touch capabilities.  The argument is a capability\nenum containing the complete set of capabilities this seat has.\n\nWhen the pointer capability is added, a client may create a\nwl_pointer object using the wl_seat.get_pointer request. This object\nwill receive pointer events until the capability is removed in the\nfuture.\n\nWhen the pointer capability is removed, a client should destroy the\nwl_pointer objects associated with the seat where the capability was\nremoved, using the wl_pointer.release request. No further pointer\nevents will be received on these objects.\n\nIn some compositors, if a seat regains the pointer capability and a\nclient has a previously obtained wl_pointer object of version 4 or\nless, that object may start sending pointer events again. This\nbehavior is considered a misinterpretation of the intended behavior\nand must not be relied upon by the client. wl_pointer objects of\nversion 5 or later must not send events if created before the most\nrecent event notifying the client of an added pointer capability.\n\nThe above behavior also applies to wl_keyboard and wl_touch with the\nkeyboard and touch capabilities, respectively."]
        Capabilities { capabilities: Capability },
        #[doc = "unique identifier for this seat\n\nIn a multiseat configuration this can be used by the client to help\nidentify which physical devices the seat represents. Based on\nthe seat configuration used by the compositor.\n\nOnly available since version 2 of the interface"]
        Name { name: String },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "capabilities",
                since: 1,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "name",
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
                Event::Capabilities { .. } => 0,
                Event::Name { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Capabilities { .. } => 1,
                Event::Name { .. } => 2,
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
                    Ok(Event::Capabilities {
                        capabilities: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                Capability::from_raw(val).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => {
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
                    Ok(Event::Capabilities {
                        capabilities: Capability::from_raw(_args[0].u).ok_or(())?,
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Name {
                        name: ::std::ffi::CStr::from_ptr(_args[0].s)
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
    pub struct WlSeat(Proxy<WlSeat>);
    impl AsRef<Proxy<WlSeat>> for WlSeat {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlSeat>> for WlSeat {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlSeat(value)
        }
    }
    impl From<WlSeat> for Proxy<WlSeat> {
        #[inline]
        fn from(value: WlSeat) -> Self {
            value.0
        }
    }
    impl Interface for WlSeat {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_seat";
        const VERSION: u32 = 6;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_seat_interface }
        }
    }
    impl WlSeat {
        #[doc = "return pointer object\n\nThe ID provided will be initialized to the wl_pointer interface\nfor this seat.\n\nThis request only takes effect if the seat has the pointer\ncapability, or has had the pointer capability in the past.\nIt is a protocol violation to issue this request on a seat that has\nnever had the pointer capability."]
        pub fn get_pointer(&self) -> Main<super::wl_pointer::WlPointer> {
            let msg = Request::GetPointer {};
            self.0.send(msg, None).unwrap()
        }
        #[doc = "return keyboard object\n\nThe ID provided will be initialized to the wl_keyboard interface\nfor this seat.\n\nThis request only takes effect if the seat has the keyboard\ncapability, or has had the keyboard capability in the past.\nIt is a protocol violation to issue this request on a seat that has\nnever had the keyboard capability."]
        pub fn get_keyboard(&self) -> Main<super::wl_keyboard::WlKeyboard> {
            let msg = Request::GetKeyboard {};
            self.0.send(msg, None).unwrap()
        }
        #[doc = "return touch object\n\nThe ID provided will be initialized to the wl_touch interface\nfor this seat.\n\nThis request only takes effect if the seat has the touch\ncapability, or has had the touch capability in the past.\nIt is a protocol violation to issue this request on a seat that has\nnever had the touch capability."]
        pub fn get_touch(&self) -> Main<super::wl_touch::WlTouch> {
            let msg = Request::GetTouch {};
            self.0.send(msg, None).unwrap()
        }
        #[doc = "release the seat object\n\nUsing this request a client can tell the server that it is not going to\nuse the seat object anymore.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called.\nOnly available since version 5 of the interface."]
        pub fn release(&self) -> () {
            let msg = Request::Release;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_POINTER_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_KEYBOARD_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_TOUCH_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_RELEASE_SINCE: u32 = 5u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CAPABILITIES_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_NAME_SINCE: u32 = 2u32;
    static mut wl_seat_requests_get_pointer_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_pointer::wl_pointer_interface as *const wl_interface }];
    static mut wl_seat_requests_get_keyboard_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_keyboard::wl_keyboard_interface as *const wl_interface }];
    static mut wl_seat_requests_get_touch_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_touch::wl_touch_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_seat_requests: [wl_message; 4] = [
        wl_message {
            name: b"get_pointer\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &wl_seat_requests_get_pointer_types as *const _ },
        },
        wl_message {
            name: b"get_keyboard\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &wl_seat_requests_get_keyboard_types as *const _ },
        },
        wl_message {
            name: b"get_touch\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &wl_seat_requests_get_touch_types as *const _ },
        },
        wl_message {
            name: b"release\0" as *const u8 as *const c_char,
            signature: b"5\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_seat_events: [wl_message; 2] = [
        wl_message {
            name: b"capabilities\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"name\0" as *const u8 as *const c_char,
            signature: b"2s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_seat_interface: wl_interface = wl_interface {
        name: b"wl_seat\0" as *const u8 as *const c_char,
        version: 6,
        request_count: 4,
        requests: unsafe { &wl_seat_requests as *const _ },
        event_count: 2,
        events: unsafe { &wl_seat_events as *const _ },
    };
}
#[doc = "pointer input device\n\nThe wl_pointer interface represents one or more input devices,\nsuch as mice, which control the pointer location and pointer_focus\nof a seat.\n\nThe wl_pointer interface generates motion, enter and leave\nevents for the surfaces that the pointer is located over,\nand button and axis events for button presses, button releases\nand scrolling."]
pub mod wl_pointer {
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
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::Role),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[doc = "physical button state\n\nDescribes the physical state of a button that produced the button\nevent."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum ButtonState {
        #[doc = "the button is not pressed"]
        Released = 0,
        #[doc = "the button is pressed"]
        Pressed = 1,
    }
    impl ButtonState {
        pub fn from_raw(n: u32) -> Option<ButtonState> {
            match n {
                0 => Some(ButtonState::Released),
                1 => Some(ButtonState::Pressed),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[doc = "axis types\n\nDescribes the axis types of scroll events."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Axis {
        #[doc = "vertical axis"]
        VerticalScroll = 0,
        #[doc = "horizontal axis"]
        HorizontalScroll = 1,
    }
    impl Axis {
        pub fn from_raw(n: u32) -> Option<Axis> {
            match n {
                0 => Some(Axis::VerticalScroll),
                1 => Some(Axis::HorizontalScroll),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[doc = "axis source types\n\nDescribes the source types for axis events. This indicates to the\nclient how an axis event was physically generated; a client may\nadjust the user interface accordingly. For example, scroll events\nfrom a \"finger\" source may be in a smooth coordinate space with\nkinetic scrolling whereas a \"wheel\" source may be in discrete steps\nof a number of lines.\n\nThe \"continuous\" axis source is a device generating events in a\ncontinuous coordinate space, but using something other than a\nfinger. One example for this source is button-based scrolling where\nthe vertical motion of a device is converted to scroll events while\na button is held down.\n\nThe \"wheel tilt\" axis source indicates that the actual device is a\nwheel but the scroll event is not caused by a rotation but a\n(usually sideways) tilt of the wheel."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum AxisSource {
        #[doc = "a physical wheel rotation"]
        Wheel = 0,
        #[doc = "finger on a touch surface"]
        Finger = 1,
        #[doc = "continuous coordinate space"]
        Continuous = 2,
        #[doc = "a physical wheel tilt"]
        WheelTilt = 3,
    }
    impl AxisSource {
        pub fn from_raw(n: u32) -> Option<AxisSource> {
            match n {
                0 => Some(AxisSource::Wheel),
                1 => Some(AxisSource::Finger),
                2 => Some(AxisSource::Continuous),
                3 => Some(AxisSource::WheelTilt),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "set the pointer surface\n\nSet the pointer surface, i.e., the surface that contains the\npointer image (cursor). This request gives the surface the role\nof a cursor. If the surface already has another role, it raises\na protocol error.\n\nThe cursor actually changes only if the pointer\nfocus for this device is one of the requesting client's surfaces\nor the surface parameter is the current pointer surface. If\nthere was a previous surface set with this request it is\nreplaced. If surface is NULL, the pointer image is hidden.\n\nThe parameters hotspot_x and hotspot_y define the position of\nthe pointer surface relative to the pointer location. Its\ntop-left corner is always at (x, y) - (hotspot_x, hotspot_y),\nwhere (x, y) are the coordinates of the pointer location, in\nsurface-local coordinates.\n\nOn surface.attach requests to the pointer surface, hotspot_x\nand hotspot_y are decremented by the x and y parameters\npassed to the request. Attach must be confirmed by\nwl_surface.commit as usual.\n\nThe hotspot can also be updated by passing the currently set\npointer surface to this request with new values for hotspot_x\nand hotspot_y.\n\nThe current and pending input regions of the wl_surface are\ncleared, and wl_surface.set_input_region is ignored until the\nwl_surface is no longer used as the cursor. When the use as a\ncursor ends, the current and pending input regions become\nundefined, and the wl_surface is unmapped."]
        SetCursor {
            serial: u32,
            surface: Option<super::wl_surface::WlSurface>,
            hotspot_x: i32,
            hotspot_y: i32,
        },
        #[doc = "release the pointer object\n\nUsing this request a client can tell the server that it is not going to\nuse the pointer object anymore.\n\nThis request destroys the pointer proxy object, so clients must not call\nwl_pointer_destroy() after using this request.\n\nThis is a destructor, once sent this object cannot be used any longer.\nOnly available since version 3 of the interface"]
        Release,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "set_cursor",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Object,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "release",
                since: 3,
                signature: &[],
                destructor: true,
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Release => true,
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::SetCursor { .. } => 0,
                Request::Release => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::SetCursor { .. } => 1,
                Request::Release => 3,
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
                Request::SetCursor {
                    serial,
                    surface,
                    hotspot_x,
                    hotspot_y,
                } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![
                        Argument::Uint(serial),
                        Argument::Object(surface.map(|o| o.as_ref().id()).unwrap_or(0)),
                        Argument::Int(hotspot_x),
                        Argument::Int(hotspot_y),
                    ],
                },
                Request::Release => Message {
                    sender_id: sender_id,
                    opcode: 1,
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
                Request::SetCursor {
                    serial,
                    surface,
                    hotspot_x,
                    hotspot_y,
                } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    _args_array[1].o = surface
                        .map(|o| o.as_ref().c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    _args_array[2].i = hotspot_x;
                    _args_array[3].i = hotspot_y;
                    f(0, &mut _args_array)
                }
                Request::Release => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(1, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "enter event\n\nNotification that this seat's pointer is focused on a certain\nsurface.\n\nWhen a seat's focus enters a surface, the pointer image\nis undefined and a client should respond to this event by setting\nan appropriate pointer image with the set_cursor request."]
        Enter {
            serial: u32,
            surface: super::wl_surface::WlSurface,
            surface_x: f64,
            surface_y: f64,
        },
        #[doc = "leave event\n\nNotification that this seat's pointer is no longer focused on\na certain surface.\n\nThe leave notification is sent before the enter notification\nfor the new focus."]
        Leave {
            serial: u32,
            surface: super::wl_surface::WlSurface,
        },
        #[doc = "pointer motion event\n\nNotification of pointer location change. The arguments\nsurface_x and surface_y are the location relative to the\nfocused surface."]
        Motion {
            time: u32,
            surface_x: f64,
            surface_y: f64,
        },
        #[doc = "pointer button event\n\nMouse button click and release notifications.\n\nThe location of the click is given by the last motion or\nenter event.\nThe time argument is a timestamp with millisecond\ngranularity, with an undefined base.\n\nThe button is a button code as defined in the Linux kernel's\nlinux/input-event-codes.h header file, e.g. BTN_LEFT.\n\nAny 16-bit button code value is reserved for future additions to the\nkernel's event code list. All other button codes above 0xFFFF are\ncurrently undefined but may be used in future versions of this\nprotocol."]
        Button {
            serial: u32,
            time: u32,
            button: u32,
            state: ButtonState,
        },
        #[doc = "axis event\n\nScroll and other axis notifications.\n\nFor scroll events (vertical and horizontal scroll axes), the\nvalue parameter is the length of a vector along the specified\naxis in a coordinate space identical to those of motion events,\nrepresenting a relative movement along the specified axis.\n\nFor devices that support movements non-parallel to axes multiple\naxis events will be emitted.\n\nWhen applicable, for example for touch pads, the server can\nchoose to emit scroll events where the motion vector is\nequivalent to a motion event vector.\n\nWhen applicable, a client can transform its content relative to the\nscroll distance."]
        Axis { time: u32, axis: Axis, value: f64 },
        #[doc = "end of a pointer event sequence\n\nIndicates the end of a set of events that logically belong together.\nA client is expected to accumulate the data in all events within the\nframe before proceeding.\n\nAll wl_pointer events before a wl_pointer.frame event belong\nlogically together. For example, in a diagonal scroll motion the\ncompositor will send an optional wl_pointer.axis_source event, two\nwl_pointer.axis events (horizontal and vertical) and finally a\nwl_pointer.frame event. The client may use this information to\ncalculate a diagonal vector for scrolling.\n\nWhen multiple wl_pointer.axis events occur within the same frame,\nthe motion vector is the combined motion of all events.\nWhen a wl_pointer.axis and a wl_pointer.axis_stop event occur within\nthe same frame, this indicates that axis movement in one axis has\nstopped but continues in the other axis.\nWhen multiple wl_pointer.axis_stop events occur within the same\nframe, this indicates that these axes stopped in the same instance.\n\nA wl_pointer.frame event is sent for every logical event group,\neven if the group only contains a single wl_pointer event.\nSpecifically, a client may get a sequence: motion, frame, button,\nframe, axis, frame, axis_stop, frame.\n\nThe wl_pointer.enter and wl_pointer.leave events are logical events\ngenerated by the compositor and not the hardware. These events are\nalso grouped by a wl_pointer.frame. When a pointer moves from one\nsurface to another, a compositor should group the\nwl_pointer.leave event within the same wl_pointer.frame.\nHowever, a client must not rely on wl_pointer.leave and\nwl_pointer.enter being in the same wl_pointer.frame.\nCompositor-specific policies may require the wl_pointer.leave and\nwl_pointer.enter event being split across multiple wl_pointer.frame\ngroups.\n\nOnly available since version 5 of the interface"]
        Frame,
        #[doc = "axis source event\n\nSource information for scroll and other axes.\n\nThis event does not occur on its own. It is sent before a\nwl_pointer.frame event and carries the source information for\nall events within that frame.\n\nThe source specifies how this event was generated. If the source is\nwl_pointer.axis_source.finger, a wl_pointer.axis_stop event will be\nsent when the user lifts the finger off the device.\n\nIf the source is wl_pointer.axis_source.wheel,\nwl_pointer.axis_source.wheel_tilt or\nwl_pointer.axis_source.continuous, a wl_pointer.axis_stop event may\nor may not be sent. Whether a compositor sends an axis_stop event\nfor these sources is hardware-specific and implementation-dependent;\nclients must not rely on receiving an axis_stop event for these\nscroll sources and should treat scroll sequences from these scroll\nsources as unterminated by default.\n\nThis event is optional. If the source is unknown for a particular\naxis event sequence, no event is sent.\nOnly one wl_pointer.axis_source event is permitted per frame.\n\nThe order of wl_pointer.axis_discrete and wl_pointer.axis_source is\nnot guaranteed.\n\nOnly available since version 5 of the interface"]
        AxisSource { axis_source: AxisSource },
        #[doc = "axis stop event\n\nStop notification for scroll and other axes.\n\nFor some wl_pointer.axis_source types, a wl_pointer.axis_stop event\nis sent to notify a client that the axis sequence has terminated.\nThis enables the client to implement kinetic scrolling.\nSee the wl_pointer.axis_source documentation for information on when\nthis event may be generated.\n\nAny wl_pointer.axis events with the same axis_source after this\nevent should be considered as the start of a new axis motion.\n\nThe timestamp is to be interpreted identical to the timestamp in the\nwl_pointer.axis event. The timestamp value may be the same as a\npreceding wl_pointer.axis event.\n\nOnly available since version 5 of the interface"]
        AxisStop { time: u32, axis: Axis },
        #[doc = "axis click event\n\nDiscrete step information for scroll and other axes.\n\nThis event carries the axis value of the wl_pointer.axis event in\ndiscrete steps (e.g. mouse wheel clicks).\n\nThis event does not occur on its own, it is coupled with a\nwl_pointer.axis event that represents this axis value on a\ncontinuous scale. The protocol guarantees that each axis_discrete\nevent is always followed by exactly one axis event with the same\naxis number within the same wl_pointer.frame. Note that the protocol\nallows for other events to occur between the axis_discrete and\nits coupled axis event, including other axis_discrete or axis\nevents.\n\nThis event is optional; continuous scrolling devices\nlike two-finger scrolling on touchpads do not have discrete\nsteps and do not generate this event.\n\nThe discrete value carries the directional information. e.g. a value\nof -2 is two steps towards the negative direction of this axis.\n\nThe axis number is identical to the axis number in the associated\naxis event.\n\nThe order of wl_pointer.axis_discrete and wl_pointer.axis_source is\nnot guaranteed.\n\nOnly available since version 5 of the interface"]
        AxisDiscrete { axis: Axis, discrete: i32 },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "enter",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Object,
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Fixed,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "leave",
                since: 1,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "motion",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Fixed,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "button",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "axis",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Fixed,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "frame",
                since: 5,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "axis_source",
                since: 5,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "axis_stop",
                since: 5,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "axis_discrete",
                since: 5,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Int],
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
                Event::Enter { .. } => 0,
                Event::Leave { .. } => 1,
                Event::Motion { .. } => 2,
                Event::Button { .. } => 3,
                Event::Axis { .. } => 4,
                Event::Frame => 5,
                Event::AxisSource { .. } => 6,
                Event::AxisStop { .. } => 7,
                Event::AxisDiscrete { .. } => 8,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Enter { .. } => 1,
                Event::Leave { .. } => 1,
                Event::Motion { .. } => 1,
                Event::Button { .. } => 1,
                Event::Axis { .. } => 1,
                Event::Frame => 5,
                Event::AxisSource { .. } => 5,
                Event::AxisStop { .. } => 5,
                Event::AxisDiscrete { .. } => 5,
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
                    Ok(Event::Enter {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        surface: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?.into()
                            } else {
                                return Err(());
                            }
                        },
                        surface_x: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                        surface_y: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Leave {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        surface: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?.into()
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Motion {
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        surface_x: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                        surface_y: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                3 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Button {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        button: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        state: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                ButtonState::from_raw(val).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                4 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Axis {
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        axis: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                Axis::from_raw(val).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                        value: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                5 => Ok(Event::Frame),
                6 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::AxisSource {
                        axis_source: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                AxisSource::from_raw(val).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                7 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::AxisStop {
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        axis: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                Axis::from_raw(val).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                8 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::AxisDiscrete {
                        axis: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                Axis::from_raw(val).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                        discrete: {
                            if let Some(Argument::Int(val)) = args.next() {
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
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Event::Enter {
                        serial: _args[0].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(
                            _args[1].o as *mut _,
                        )
                        .into(),
                        surface_x: (_args[2].f as f64) / 256.,
                        surface_y: (_args[3].f as f64) / 256.,
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Leave {
                        serial: _args[0].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(
                            _args[1].o as *mut _,
                        )
                        .into(),
                    })
                }
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Motion {
                        time: _args[0].u,
                        surface_x: (_args[1].f as f64) / 256.,
                        surface_y: (_args[2].f as f64) / 256.,
                    })
                }
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Event::Button {
                        serial: _args[0].u,
                        time: _args[1].u,
                        button: _args[2].u,
                        state: ButtonState::from_raw(_args[3].u).ok_or(())?,
                    })
                }
                4 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Axis {
                        time: _args[0].u,
                        axis: Axis::from_raw(_args[1].u).ok_or(())?,
                        value: (_args[2].f as f64) / 256.,
                    })
                }
                5 => Ok(Event::Frame),
                6 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::AxisSource {
                        axis_source: AxisSource::from_raw(_args[0].u).ok_or(())?,
                    })
                }
                7 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::AxisStop {
                        time: _args[0].u,
                        axis: Axis::from_raw(_args[1].u).ok_or(())?,
                    })
                }
                8 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::AxisDiscrete {
                        axis: Axis::from_raw(_args[0].u).ok_or(())?,
                        discrete: _args[1].i,
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
    pub struct WlPointer(Proxy<WlPointer>);
    impl AsRef<Proxy<WlPointer>> for WlPointer {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlPointer>> for WlPointer {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlPointer(value)
        }
    }
    impl From<WlPointer> for Proxy<WlPointer> {
        #[inline]
        fn from(value: WlPointer) -> Self {
            value.0
        }
    }
    impl Interface for WlPointer {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_pointer";
        const VERSION: u32 = 6;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_pointer_interface }
        }
    }
    impl WlPointer {
        #[doc = "set the pointer surface\n\nSet the pointer surface, i.e., the surface that contains the\npointer image (cursor). This request gives the surface the role\nof a cursor. If the surface already has another role, it raises\na protocol error.\n\nThe cursor actually changes only if the pointer\nfocus for this device is one of the requesting client's surfaces\nor the surface parameter is the current pointer surface. If\nthere was a previous surface set with this request it is\nreplaced. If surface is NULL, the pointer image is hidden.\n\nThe parameters hotspot_x and hotspot_y define the position of\nthe pointer surface relative to the pointer location. Its\ntop-left corner is always at (x, y) - (hotspot_x, hotspot_y),\nwhere (x, y) are the coordinates of the pointer location, in\nsurface-local coordinates.\n\nOn surface.attach requests to the pointer surface, hotspot_x\nand hotspot_y are decremented by the x and y parameters\npassed to the request. Attach must be confirmed by\nwl_surface.commit as usual.\n\nThe hotspot can also be updated by passing the currently set\npointer surface to this request with new values for hotspot_x\nand hotspot_y.\n\nThe current and pending input regions of the wl_surface are\ncleared, and wl_surface.set_input_region is ignored until the\nwl_surface is no longer used as the cursor. When the use as a\ncursor ends, the current and pending input regions become\nundefined, and the wl_surface is unmapped."]
        pub fn set_cursor(
            &self,
            serial: u32,
            surface: Option<&super::wl_surface::WlSurface>,
            hotspot_x: i32,
            hotspot_y: i32,
        ) -> () {
            let msg = Request::SetCursor {
                serial: serial,
                surface: surface.map(|o| o.clone()),
                hotspot_x: hotspot_x,
                hotspot_y: hotspot_y,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "release the pointer object\n\nUsing this request a client can tell the server that it is not going to\nuse the pointer object anymore.\n\nThis request destroys the pointer proxy object, so clients must not call\nwl_pointer_destroy() after using this request.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called.\nOnly available since version 3 of the interface."]
        pub fn release(&self) -> () {
            let msg = Request::Release;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_CURSOR_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_RELEASE_SINCE: u32 = 3u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_ENTER_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_LEAVE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_MOTION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_BUTTON_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_AXIS_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FRAME_SINCE: u32 = 5u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_AXIS_SOURCE_SINCE: u32 = 5u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_AXIS_STOP_SINCE: u32 = 5u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_AXIS_DISCRETE_SINCE: u32 = 5u32;
    static mut wl_pointer_requests_set_cursor_types: [*const wl_interface; 4] = [
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_pointer_requests: [wl_message; 2] = [
        wl_message {
            name: b"set_cursor\0" as *const u8 as *const c_char,
            signature: b"u?oii\0" as *const u8 as *const c_char,
            types: unsafe { &wl_pointer_requests_set_cursor_types as *const _ },
        },
        wl_message {
            name: b"release\0" as *const u8 as *const c_char,
            signature: b"3\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    static mut wl_pointer_events_enter_types: [*const wl_interface; 4] = [
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    static mut wl_pointer_events_leave_types: [*const wl_interface; 2] =
        [NULLPTR as *const wl_interface, unsafe {
            &super::wl_surface::wl_surface_interface as *const wl_interface
        }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_pointer_events: [wl_message; 9] = [
        wl_message {
            name: b"enter\0" as *const u8 as *const c_char,
            signature: b"uoff\0" as *const u8 as *const c_char,
            types: unsafe { &wl_pointer_events_enter_types as *const _ },
        },
        wl_message {
            name: b"leave\0" as *const u8 as *const c_char,
            signature: b"uo\0" as *const u8 as *const c_char,
            types: unsafe { &wl_pointer_events_leave_types as *const _ },
        },
        wl_message {
            name: b"motion\0" as *const u8 as *const c_char,
            signature: b"uff\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"button\0" as *const u8 as *const c_char,
            signature: b"uuuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"axis\0" as *const u8 as *const c_char,
            signature: b"uuf\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"frame\0" as *const u8 as *const c_char,
            signature: b"5\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"axis_source\0" as *const u8 as *const c_char,
            signature: b"5u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"axis_stop\0" as *const u8 as *const c_char,
            signature: b"5uu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"axis_discrete\0" as *const u8 as *const c_char,
            signature: b"5ui\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_pointer_interface: wl_interface = wl_interface {
        name: b"wl_pointer\0" as *const u8 as *const c_char,
        version: 6,
        request_count: 2,
        requests: unsafe { &wl_pointer_requests as *const _ },
        event_count: 9,
        events: unsafe { &wl_pointer_events as *const _ },
    };
}
#[doc = "keyboard input device\n\nThe wl_keyboard interface represents one or more keyboards\nassociated with a seat."]
pub mod wl_keyboard {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[doc = "keyboard mapping format\n\nThis specifies the format of the keymap provided to the\nclient with the wl_keyboard.keymap event."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum KeymapFormat {
        #[doc = "no keymap; client must understand how to interpret the raw keycode"]
        NoKeymap = 0,
        #[doc = "libxkbcommon compatible; to determine the xkb keycode, clients must add 8 to the key event keycode"]
        XkbV1 = 1,
    }
    impl KeymapFormat {
        pub fn from_raw(n: u32) -> Option<KeymapFormat> {
            match n {
                0 => Some(KeymapFormat::NoKeymap),
                1 => Some(KeymapFormat::XkbV1),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[doc = "physical key state\n\nDescribes the physical state of a key that produced the key event."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum KeyState {
        #[doc = "key is not pressed"]
        Released = 0,
        #[doc = "key is pressed"]
        Pressed = 1,
    }
    impl KeyState {
        pub fn from_raw(n: u32) -> Option<KeyState> {
            match n {
                0 => Some(KeyState::Released),
                1 => Some(KeyState::Pressed),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "release the keyboard object\n\n\n\nThis is a destructor, once sent this object cannot be used any longer.\nOnly available since version 3 of the interface"]
        Release,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "release",
            since: 3,
            signature: &[],
            destructor: true,
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Release => true,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Release => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Release => 3,
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
                Request::Release => Message {
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
                Request::Release => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "keyboard mapping\n\nThis event provides a file descriptor to the client which can be\nmemory-mapped to provide a keyboard mapping description."]
        Keymap {
            format: KeymapFormat,
            fd: ::std::os::unix::io::RawFd,
            size: u32,
        },
        #[doc = "enter event\n\nNotification that this seat's keyboard focus is on a certain\nsurface."]
        Enter {
            serial: u32,
            surface: super::wl_surface::WlSurface,
            keys: Vec<u8>,
        },
        #[doc = "leave event\n\nNotification that this seat's keyboard focus is no longer on\na certain surface.\n\nThe leave notification is sent before the enter notification\nfor the new focus."]
        Leave {
            serial: u32,
            surface: super::wl_surface::WlSurface,
        },
        #[doc = "key event\n\nA key was pressed or released.\nThe time argument is a timestamp with millisecond\ngranularity, with an undefined base."]
        Key {
            serial: u32,
            time: u32,
            key: u32,
            state: KeyState,
        },
        #[doc = "modifier and group state\n\nNotifies clients that the modifier and/or group state has\nchanged, and it should update its local state."]
        Modifiers {
            serial: u32,
            mods_depressed: u32,
            mods_latched: u32,
            mods_locked: u32,
            group: u32,
        },
        #[doc = "repeat rate and delay\n\nInforms the client about the keyboard's repeat rate and delay.\n\nThis event is sent as soon as the wl_keyboard object has been created,\nand is guaranteed to be received by the client before any key press\nevent.\n\nNegative values for either rate or delay are illegal. A rate of zero\nwill disable any repeating (regardless of the value of delay).\n\nThis event can be sent later on as well with a new value if necessary,\nso clients should continue listening for the event past the creation\nof wl_keyboard.\n\nOnly available since version 4 of the interface"]
        RepeatInfo { rate: i32, delay: i32 },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "keymap",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Fd,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "enter",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Object,
                    super::ArgumentType::Array,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "leave",
                since: 1,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "key",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "modifiers",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "repeat_info",
                since: 4,
                signature: &[super::ArgumentType::Int, super::ArgumentType::Int],
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
                Event::Keymap { .. } => 0,
                Event::Enter { .. } => 1,
                Event::Leave { .. } => 2,
                Event::Key { .. } => 3,
                Event::Modifiers { .. } => 4,
                Event::RepeatInfo { .. } => 5,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Keymap { .. } => 1,
                Event::Enter { .. } => 1,
                Event::Leave { .. } => 1,
                Event::Key { .. } => 1,
                Event::Modifiers { .. } => 1,
                Event::RepeatInfo { .. } => 4,
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
                    Ok(Event::Keymap {
                        format: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                KeymapFormat::from_raw(val).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                        fd: {
                            if let Some(Argument::Fd(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        size: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Enter {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        surface: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?.into()
                            } else {
                                return Err(());
                            }
                        },
                        keys: {
                            if let Some(Argument::Array(val)) = args.next() {
                                *val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Leave {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        surface: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?.into()
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                3 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Key {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        key: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        state: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                KeyState::from_raw(val).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                4 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Modifiers {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        mods_depressed: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        mods_latched: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        mods_locked: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        group: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                5 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::RepeatInfo {
                        rate: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        delay: {
                            if let Some(Argument::Int(val)) = args.next() {
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
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Keymap {
                        format: KeymapFormat::from_raw(_args[0].u).ok_or(())?,
                        fd: _args[1].h,
                        size: _args[2].u,
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Enter {
                        serial: _args[0].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(
                            _args[1].o as *mut _,
                        )
                        .into(),
                        keys: {
                            let array = &*_args[2].a;
                            ::std::slice::from_raw_parts(array.data as *const u8, array.size)
                                .to_owned()
                        },
                    })
                }
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Leave {
                        serial: _args[0].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(
                            _args[1].o as *mut _,
                        )
                        .into(),
                    })
                }
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Event::Key {
                        serial: _args[0].u,
                        time: _args[1].u,
                        key: _args[2].u,
                        state: KeyState::from_raw(_args[3].u).ok_or(())?,
                    })
                }
                4 => {
                    let _args = ::std::slice::from_raw_parts(args, 5);
                    Ok(Event::Modifiers {
                        serial: _args[0].u,
                        mods_depressed: _args[1].u,
                        mods_latched: _args[2].u,
                        mods_locked: _args[3].u,
                        group: _args[4].u,
                    })
                }
                5 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::RepeatInfo {
                        rate: _args[0].i,
                        delay: _args[1].i,
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
    pub struct WlKeyboard(Proxy<WlKeyboard>);
    impl AsRef<Proxy<WlKeyboard>> for WlKeyboard {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlKeyboard>> for WlKeyboard {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlKeyboard(value)
        }
    }
    impl From<WlKeyboard> for Proxy<WlKeyboard> {
        #[inline]
        fn from(value: WlKeyboard) -> Self {
            value.0
        }
    }
    impl Interface for WlKeyboard {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_keyboard";
        const VERSION: u32 = 6;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_keyboard_interface }
        }
    }
    impl WlKeyboard {
        #[doc = "release the keyboard object\n\n\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called.\nOnly available since version 3 of the interface."]
        pub fn release(&self) -> () {
            let msg = Request::Release;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_RELEASE_SINCE: u32 = 3u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_KEYMAP_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_ENTER_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_LEAVE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_KEY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_MODIFIERS_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_REPEAT_INFO_SINCE: u32 = 4u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_keyboard_requests: [wl_message; 1] = [wl_message {
        name: b"release\0" as *const u8 as *const c_char,
        signature: b"3\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    static mut wl_keyboard_events_enter_types: [*const wl_interface; 3] = [
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
    ];
    static mut wl_keyboard_events_leave_types: [*const wl_interface; 2] =
        [NULLPTR as *const wl_interface, unsafe {
            &super::wl_surface::wl_surface_interface as *const wl_interface
        }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_keyboard_events: [wl_message; 6] = [
        wl_message {
            name: b"keymap\0" as *const u8 as *const c_char,
            signature: b"uhu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"enter\0" as *const u8 as *const c_char,
            signature: b"uoa\0" as *const u8 as *const c_char,
            types: unsafe { &wl_keyboard_events_enter_types as *const _ },
        },
        wl_message {
            name: b"leave\0" as *const u8 as *const c_char,
            signature: b"uo\0" as *const u8 as *const c_char,
            types: unsafe { &wl_keyboard_events_leave_types as *const _ },
        },
        wl_message {
            name: b"key\0" as *const u8 as *const c_char,
            signature: b"uuuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"modifiers\0" as *const u8 as *const c_char,
            signature: b"uuuuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"repeat_info\0" as *const u8 as *const c_char,
            signature: b"4ii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_keyboard_interface: wl_interface = wl_interface {
        name: b"wl_keyboard\0" as *const u8 as *const c_char,
        version: 6,
        request_count: 1,
        requests: unsafe { &wl_keyboard_requests as *const _ },
        event_count: 6,
        events: unsafe { &wl_keyboard_events as *const _ },
    };
}
#[doc = "touchscreen input device\n\nThe wl_touch interface represents a touchscreen\nassociated with a seat.\n\nTouch interactions can consist of one or more contacts.\nFor each contact, a series of events is generated, starting\nwith a down event, followed by zero or more motion events,\nand ending with an up event. Events relating to the same\ncontact point can be identified by the ID of the sequence."]
pub mod wl_touch {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "release the touch object\n\n\n\nThis is a destructor, once sent this object cannot be used any longer.\nOnly available since version 3 of the interface"]
        Release,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "release",
            since: 3,
            signature: &[],
            destructor: true,
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Release => true,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Release => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Release => 3,
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
                Request::Release => Message {
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
                Request::Release => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "touch down event and beginning of a touch sequence\n\nA new touch point has appeared on the surface. This touch point is\nassigned a unique ID. Future events from this touch point reference\nthis ID. The ID ceases to be valid after a touch up event and may be\nreused in the future."]
        Down {
            serial: u32,
            time: u32,
            surface: super::wl_surface::WlSurface,
            id: i32,
            x: f64,
            y: f64,
        },
        #[doc = "end of a touch event sequence\n\nThe touch point has disappeared. No further events will be sent for\nthis touch point and the touch point's ID is released and may be\nreused in a future touch down event."]
        Up { serial: u32, time: u32, id: i32 },
        #[doc = "update of touch point coordinates\n\nA touch point has changed coordinates."]
        Motion { time: u32, id: i32, x: f64, y: f64 },
        #[doc = "end of touch frame event\n\nIndicates the end of a set of events that logically belong together.\nA client is expected to accumulate the data in all events within the\nframe before proceeding.\n\nA wl_touch.frame terminates at least one event but otherwise no\nguarantee is provided about the set of events within a frame. A client\nmust assume that any state not updated in a frame is unchanged from the\npreviously known state."]
        Frame,
        #[doc = "touch session cancelled\n\nSent if the compositor decides the touch stream is a global\ngesture. No further events are sent to the clients from that\nparticular gesture. Touch cancellation applies to all touch points\ncurrently active on this client's surface. The client is\nresponsible for finalizing the touch points, future touch points on\nthis surface may reuse the touch point ID."]
        Cancel,
        #[doc = "update shape of touch point\n\nSent when a touchpoint has changed its shape.\n\nThis event does not occur on its own. It is sent before a\nwl_touch.frame event and carries the new shape information for\nany previously reported, or new touch points of that frame.\n\nOther events describing the touch point such as wl_touch.down,\nwl_touch.motion or wl_touch.orientation may be sent within the\nsame wl_touch.frame. A client should treat these events as a single\nlogical touch point update. The order of wl_touch.shape,\nwl_touch.orientation and wl_touch.motion is not guaranteed.\nA wl_touch.down event is guaranteed to occur before the first\nwl_touch.shape event for this touch ID but both events may occur within\nthe same wl_touch.frame.\n\nA touchpoint shape is approximated by an ellipse through the major and\nminor axis length. The major axis length describes the longer diameter\nof the ellipse, while the minor axis length describes the shorter\ndiameter. Major and minor are orthogonal and both are specified in\nsurface-local coordinates. The center of the ellipse is always at the\ntouchpoint location as reported by wl_touch.down or wl_touch.move.\n\nThis event is only sent by the compositor if the touch device supports\nshape reports. The client has to make reasonable assumptions about the\nshape if it did not receive this event.\n\nOnly available since version 6 of the interface"]
        Shape { id: i32, major: f64, minor: f64 },
        #[doc = "update orientation of touch point\n\nSent when a touchpoint has changed its orientation.\n\nThis event does not occur on its own. It is sent before a\nwl_touch.frame event and carries the new shape information for\nany previously reported, or new touch points of that frame.\n\nOther events describing the touch point such as wl_touch.down,\nwl_touch.motion or wl_touch.shape may be sent within the\nsame wl_touch.frame. A client should treat these events as a single\nlogical touch point update. The order of wl_touch.shape,\nwl_touch.orientation and wl_touch.motion is not guaranteed.\nA wl_touch.down event is guaranteed to occur before the first\nwl_touch.orientation event for this touch ID but both events may occur\nwithin the same wl_touch.frame.\n\nThe orientation describes the clockwise angle of a touchpoint's major\naxis to the positive surface y-axis and is normalized to the -180 to\n+180 degree range. The granularity of orientation depends on the touch\ndevice, some devices only support binary rotation values between 0 and\n90 degrees.\n\nThis event is only sent by the compositor if the touch device supports\norientation reports.\n\nOnly available since version 6 of the interface"]
        Orientation { id: i32, orientation: f64 },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "down",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Object,
                    super::ArgumentType::Int,
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Fixed,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "up",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Int,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "motion",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Int,
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Fixed,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "frame",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "cancel",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "shape",
                since: 6,
                signature: &[
                    super::ArgumentType::Int,
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Fixed,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "orientation",
                since: 6,
                signature: &[super::ArgumentType::Int, super::ArgumentType::Fixed],
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
                Event::Down { .. } => 0,
                Event::Up { .. } => 1,
                Event::Motion { .. } => 2,
                Event::Frame => 3,
                Event::Cancel => 4,
                Event::Shape { .. } => 5,
                Event::Orientation { .. } => 6,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Down { .. } => 1,
                Event::Up { .. } => 1,
                Event::Motion { .. } => 1,
                Event::Frame => 1,
                Event::Cancel => 1,
                Event::Shape { .. } => 6,
                Event::Orientation { .. } => 6,
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
                    Ok(Event::Down {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        surface: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?.into()
                            } else {
                                return Err(());
                            }
                        },
                        id: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        x: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                        y: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Up {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        id: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Motion {
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        id: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        x: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                        y: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                3 => Ok(Event::Frame),
                4 => Ok(Event::Cancel),
                5 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Shape {
                        id: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        major: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                        minor: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                6 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Orientation {
                        id: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        orientation: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
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
                    let _args = ::std::slice::from_raw_parts(args, 6);
                    Ok(Event::Down {
                        serial: _args[0].u,
                        time: _args[1].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(
                            _args[2].o as *mut _,
                        )
                        .into(),
                        id: _args[3].i,
                        x: (_args[4].f as f64) / 256.,
                        y: (_args[5].f as f64) / 256.,
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Up {
                        serial: _args[0].u,
                        time: _args[1].u,
                        id: _args[2].i,
                    })
                }
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Event::Motion {
                        time: _args[0].u,
                        id: _args[1].i,
                        x: (_args[2].f as f64) / 256.,
                        y: (_args[3].f as f64) / 256.,
                    })
                }
                3 => Ok(Event::Frame),
                4 => Ok(Event::Cancel),
                5 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Shape {
                        id: _args[0].i,
                        major: (_args[1].f as f64) / 256.,
                        minor: (_args[2].f as f64) / 256.,
                    })
                }
                6 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Orientation {
                        id: _args[0].i,
                        orientation: (_args[1].f as f64) / 256.,
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
    pub struct WlTouch(Proxy<WlTouch>);
    impl AsRef<Proxy<WlTouch>> for WlTouch {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlTouch>> for WlTouch {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlTouch(value)
        }
    }
    impl From<WlTouch> for Proxy<WlTouch> {
        #[inline]
        fn from(value: WlTouch) -> Self {
            value.0
        }
    }
    impl Interface for WlTouch {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_touch";
        const VERSION: u32 = 6;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_touch_interface }
        }
    }
    impl WlTouch {
        #[doc = "release the touch object\n\n\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called.\nOnly available since version 3 of the interface."]
        pub fn release(&self) -> () {
            let msg = Request::Release;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_RELEASE_SINCE: u32 = 3u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DOWN_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_UP_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_MOTION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FRAME_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CANCEL_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_SHAPE_SINCE: u32 = 6u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_ORIENTATION_SINCE: u32 = 6u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_touch_requests: [wl_message; 1] = [wl_message {
        name: b"release\0" as *const u8 as *const c_char,
        signature: b"3\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    static mut wl_touch_events_down_types: [*const wl_interface; 6] = [
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_touch_events: [wl_message; 7] = [
        wl_message {
            name: b"down\0" as *const u8 as *const c_char,
            signature: b"uuoiff\0" as *const u8 as *const c_char,
            types: unsafe { &wl_touch_events_down_types as *const _ },
        },
        wl_message {
            name: b"up\0" as *const u8 as *const c_char,
            signature: b"uui\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"motion\0" as *const u8 as *const c_char,
            signature: b"uiff\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"frame\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"cancel\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"shape\0" as *const u8 as *const c_char,
            signature: b"6iff\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"orientation\0" as *const u8 as *const c_char,
            signature: b"6if\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_touch_interface: wl_interface = wl_interface {
        name: b"wl_touch\0" as *const u8 as *const c_char,
        version: 6,
        request_count: 1,
        requests: unsafe { &wl_touch_requests as *const _ },
        event_count: 7,
        events: unsafe { &wl_touch_events as *const _ },
    };
}
#[doc = "compositor output region\n\nAn output describes part of the compositor geometry.  The\ncompositor works in the 'compositor coordinate system' and an\noutput corresponds to a rectangular area in that space that is\nactually visible.  This typically corresponds to a monitor that\ndisplays part of the compositor space.  This object is published\nas global during start up, or when a monitor is hotplugged."]
pub mod wl_output {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[doc = "subpixel geometry information\n\nThis enumeration describes how the physical\npixels on an output are laid out."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Subpixel {
        #[doc = "unknown geometry"]
        Unknown = 0,
        #[doc = "no geometry"]
        None = 1,
        #[doc = "horizontal RGB"]
        HorizontalRgb = 2,
        #[doc = "horizontal BGR"]
        HorizontalBgr = 3,
        #[doc = "vertical RGB"]
        VerticalRgb = 4,
        #[doc = "vertical BGR"]
        VerticalBgr = 5,
    }
    impl Subpixel {
        pub fn from_raw(n: u32) -> Option<Subpixel> {
            match n {
                0 => Some(Subpixel::Unknown),
                1 => Some(Subpixel::None),
                2 => Some(Subpixel::HorizontalRgb),
                3 => Some(Subpixel::HorizontalBgr),
                4 => Some(Subpixel::VerticalRgb),
                5 => Some(Subpixel::VerticalBgr),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[doc = "transform from framebuffer to output\n\nThis describes the transform that a compositor will apply to a\nsurface to compensate for the rotation or mirroring of an\noutput device.\n\nThe flipped values correspond to an initial flip around a\nvertical axis followed by rotation.\n\nThe purpose is mainly to allow clients to render accordingly and\ntell the compositor, so that for fullscreen surfaces, the\ncompositor will still be able to scan out directly from client\nsurfaces."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Transform {
        #[doc = "no transform"]
        Normal = 0,
        #[doc = "90 degrees counter-clockwise"]
        _90 = 1,
        #[doc = "180 degrees counter-clockwise"]
        _180 = 2,
        #[doc = "270 degrees counter-clockwise"]
        _270 = 3,
        #[doc = "180 degree flip around a vertical axis"]
        Flipped = 4,
        #[doc = "flip and rotate 90 degrees counter-clockwise"]
        Flipped90 = 5,
        #[doc = "flip and rotate 180 degrees counter-clockwise"]
        Flipped180 = 6,
        #[doc = "flip and rotate 270 degrees counter-clockwise"]
        Flipped270 = 7,
    }
    impl Transform {
        pub fn from_raw(n: u32) -> Option<Transform> {
            match n {
                0 => Some(Transform::Normal),
                1 => Some(Transform::_90),
                2 => Some(Transform::_180),
                3 => Some(Transform::_270),
                4 => Some(Transform::Flipped),
                5 => Some(Transform::Flipped90),
                6 => Some(Transform::Flipped180),
                7 => Some(Transform::Flipped270),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    bitflags! { # [ doc = "mode information\n\nThese flags describe properties of an output mode.\nThey are used in the flags bitfield of the mode event." ] pub struct Mode : u32 { # [ doc = "indicates this is the current mode" ] const Current = 1 ; # [ doc = "indicates this is the preferred mode" ] const Preferred = 2 ; } }
    impl Mode {
        pub fn from_raw(n: u32) -> Option<Mode> {
            Some(Mode::from_bits_truncate(n))
        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "release the output object\n\nUsing this request a client can tell the server that it is not going to\nuse the output object anymore.\n\nThis is a destructor, once sent this object cannot be used any longer.\nOnly available since version 3 of the interface"]
        Release,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "release",
            since: 3,
            signature: &[],
            destructor: true,
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Release => true,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Release => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Release => 3,
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
                Request::Release => Message {
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
                Request::Release => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "properties of the output\n\nThe geometry event describes geometric properties of the output.\nThe event is sent when binding to the output object and whenever\nany of the properties change.\n\nThe physical size can be set to zero if it doesn't make sense for this\noutput (e.g. for projectors or virtual outputs)."]
        Geometry {
            x: i32,
            y: i32,
            physical_width: i32,
            physical_height: i32,
            subpixel: Subpixel,
            make: String,
            model: String,
            transform: Transform,
        },
        #[doc = "advertise available modes for the output\n\nThe mode event describes an available mode for the output.\n\nThe event is sent when binding to the output object and there\nwill always be one mode, the current mode.  The event is sent\nagain if an output changes mode, for the mode that is now\ncurrent.  In other words, the current mode is always the last\nmode that was received with the current flag set.\n\nThe size of a mode is given in physical hardware units of\nthe output device. This is not necessarily the same as\nthe output size in the global compositor space. For instance,\nthe output may be scaled, as described in wl_output.scale,\nor transformed, as described in wl_output.transform."]
        Mode {
            flags: Mode,
            width: i32,
            height: i32,
            refresh: i32,
        },
        #[doc = "sent all information about output\n\nThis event is sent after all other properties have been\nsent after binding to the output object and after any\nother property changes done after that. This allows\nchanges to the output properties to be seen as\natomic, even if they happen via multiple events.\n\nOnly available since version 2 of the interface"]
        Done,
        #[doc = "output scaling properties\n\nThis event contains scaling geometry information\nthat is not in the geometry event. It may be sent after\nbinding the output object or if the output scale changes\nlater. If it is not sent, the client should assume a\nscale of 1.\n\nA scale larger than 1 means that the compositor will\nautomatically scale surface buffers by this amount\nwhen rendering. This is used for very high resolution\ndisplays where applications rendering at the native\nresolution would be too small to be legible.\n\nIt is intended that scaling aware clients track the\ncurrent output of a surface, and if it is on a scaled\noutput it should use wl_surface.set_buffer_scale with\nthe scale of the output. That way the compositor can\navoid scaling the surface, and the client can supply\na higher detail image.\n\nOnly available since version 2 of the interface"]
        Scale { factor: i32 },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "geometry",
                since: 1,
                signature: &[
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Str,
                    super::ArgumentType::Str,
                    super::ArgumentType::Int,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "mode",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "done",
                since: 2,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "scale",
                since: 2,
                signature: &[super::ArgumentType::Int],
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
                Event::Geometry { .. } => 0,
                Event::Mode { .. } => 1,
                Event::Done => 2,
                Event::Scale { .. } => 3,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Geometry { .. } => 1,
                Event::Mode { .. } => 1,
                Event::Done => 2,
                Event::Scale { .. } => 2,
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
                    Ok(Event::Geometry {
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
                        physical_width: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        physical_height: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        subpixel: {
                            if let Some(Argument::Int(val)) = args.next() {
                                Subpixel::from_raw(val as u32).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                        make: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                        model: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                        transform: {
                            if let Some(Argument::Int(val)) = args.next() {
                                Transform::from_raw(val as u32).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Mode {
                        flags: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                Mode::from_raw(val).ok_or(())?
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
                        refresh: {
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
                    Ok(Event::Scale {
                        factor: {
                            if let Some(Argument::Int(val)) = args.next() {
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
                    let _args = ::std::slice::from_raw_parts(args, 8);
                    Ok(Event::Geometry {
                        x: _args[0].i,
                        y: _args[1].i,
                        physical_width: _args[2].i,
                        physical_height: _args[3].i,
                        subpixel: Subpixel::from_raw(_args[4].i as u32).ok_or(())?,
                        make: ::std::ffi::CStr::from_ptr(_args[5].s)
                            .to_string_lossy()
                            .into_owned(),
                        model: ::std::ffi::CStr::from_ptr(_args[6].s)
                            .to_string_lossy()
                            .into_owned(),
                        transform: Transform::from_raw(_args[7].i as u32).ok_or(())?,
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Event::Mode {
                        flags: Mode::from_raw(_args[0].u).ok_or(())?,
                        width: _args[1].i,
                        height: _args[2].i,
                        refresh: _args[3].i,
                    })
                }
                2 => Ok(Event::Done),
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Scale { factor: _args[0].i })
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
    pub struct WlOutput(Proxy<WlOutput>);
    impl AsRef<Proxy<WlOutput>> for WlOutput {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlOutput>> for WlOutput {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlOutput(value)
        }
    }
    impl From<WlOutput> for Proxy<WlOutput> {
        #[inline]
        fn from(value: WlOutput) -> Self {
            value.0
        }
    }
    impl Interface for WlOutput {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_output";
        const VERSION: u32 = 3;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_output_interface }
        }
    }
    impl WlOutput {
        #[doc = "release the output object\n\nUsing this request a client can tell the server that it is not going to\nuse the output object anymore.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called.\nOnly available since version 3 of the interface."]
        pub fn release(&self) -> () {
            let msg = Request::Release;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_RELEASE_SINCE: u32 = 3u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_GEOMETRY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_MODE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DONE_SINCE: u32 = 2u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_SCALE_SINCE: u32 = 2u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_output_requests: [wl_message; 1] = [wl_message {
        name: b"release\0" as *const u8 as *const c_char,
        signature: b"3\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_output_events: [wl_message; 4] = [
        wl_message {
            name: b"geometry\0" as *const u8 as *const c_char,
            signature: b"iiiiissi\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"mode\0" as *const u8 as *const c_char,
            signature: b"uiii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"done\0" as *const u8 as *const c_char,
            signature: b"2\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"scale\0" as *const u8 as *const c_char,
            signature: b"2i\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_output_interface: wl_interface = wl_interface {
        name: b"wl_output\0" as *const u8 as *const c_char,
        version: 3,
        request_count: 1,
        requests: unsafe { &wl_output_requests as *const _ },
        event_count: 4,
        events: unsafe { &wl_output_events as *const _ },
    };
}
#[doc = "region interface\n\nA region object describes an area.\n\nRegion objects are used to describe the opaque and input\nregions of a surface."]
pub mod wl_region {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy region\n\nDestroy the region.  This will invalidate the object ID.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "add rectangle to region\n\nAdd the specified rectangle to the region."]
        Add {
            x: i32,
            y: i32,
            width: i32,
            height: i32,
        },
        #[doc = "subtract rectangle from region\n\nSubtract the specified rectangle from the region."]
        Subtract {
            x: i32,
            y: i32,
            width: i32,
            height: i32,
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
                name: "add",
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
                name: "subtract",
                since: 1,
                signature: &[
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
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
                Request::Add { .. } => 1,
                Request::Subtract { .. } => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::Add { .. } => 1,
                Request::Subtract { .. } => 1,
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
                Request::Add {
                    x,
                    y,
                    width,
                    height,
                } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![
                        Argument::Int(x),
                        Argument::Int(y),
                        Argument::Int(width),
                        Argument::Int(height),
                    ],
                },
                Request::Subtract {
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
                Request::Add {
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
                    f(1, &mut _args_array)
                }
                Request::Subtract {
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
    pub struct WlRegion(Proxy<WlRegion>);
    impl AsRef<Proxy<WlRegion>> for WlRegion {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlRegion>> for WlRegion {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlRegion(value)
        }
    }
    impl From<WlRegion> for Proxy<WlRegion> {
        #[inline]
        fn from(value: WlRegion) -> Self {
            value.0
        }
    }
    impl Interface for WlRegion {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_region";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_region_interface }
        }
    }
    impl WlRegion {
        #[doc = "destroy region\n\nDestroy the region.  This will invalidate the object ID.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "add rectangle to region\n\nAdd the specified rectangle to the region."]
        pub fn add(&self, x: i32, y: i32, width: i32, height: i32) -> () {
            let msg = Request::Add {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "subtract rectangle from region\n\nSubtract the specified rectangle from the region."]
        pub fn subtract(&self, x: i32, y: i32, width: i32, height: i32) -> () {
            let msg = Request::Subtract {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_ADD_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SUBTRACT_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_region_requests: [wl_message; 3] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"add\0" as *const u8 as *const c_char,
            signature: b"iiii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"subtract\0" as *const u8 as *const c_char,
            signature: b"iiii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_region_interface: wl_interface = wl_interface {
        name: b"wl_region\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 3,
        requests: unsafe { &wl_region_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "sub-surface compositing\n\nThe global interface exposing sub-surface compositing capabilities.\nA wl_surface, that has sub-surfaces associated, is called the\nparent surface. Sub-surfaces can be arbitrarily nested and create\na tree of sub-surfaces.\n\nThe root surface in a tree of sub-surfaces is the main\nsurface. The main surface cannot be a sub-surface, because\nsub-surfaces must always have a parent.\n\nA main surface with its sub-surfaces forms a (compound) window.\nFor window management purposes, this set of wl_surface objects is\nto be considered as a single window, and it should also behave as\nsuch.\n\nThe aim of sub-surfaces is to offload some of the compositing work\nwithin a window from clients to the compositor. A prime example is\na video player with decorations and video in separate wl_surface\nobjects. This should allow the compositor to pass YUV video buffer\nprocessing to dedicated overlay hardware when possible."]
pub mod wl_subcompositor {
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
        #[doc = "the to-be sub-surface is invalid"]
        BadSurface = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::BadSurface),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "unbind from the subcompositor interface\n\nInforms the server that the client will not be using this\nprotocol object anymore. This does not affect any other\nobjects, wl_subsurface objects included.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "give a surface the role sub-surface\n\nCreate a sub-surface interface for the given surface, and\nassociate it with the given parent surface. This turns a\nplain wl_surface into a sub-surface.\n\nThe to-be sub-surface must not already have another role, and it\nmust not have an existing wl_subsurface object. Otherwise a protocol\nerror is raised.\n\nAdding sub-surfaces to a parent is a double-buffered operation on the\nparent (see wl_surface.commit). The effect of adding a sub-surface\nbecomes visible on the next time the state of the parent surface is\napplied.\n\nThis request modifies the behaviour of wl_surface.commit request on\nthe sub-surface, see the documentation on wl_subsurface interface."]
        GetSubsurface {
            surface: super::wl_surface::WlSurface,
            parent: super::wl_surface::WlSurface,
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
                name: "get_subsurface",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                    super::ArgumentType::Object,
                    super::ArgumentType::Object,
                ],
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
                Request::GetSubsurface { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::GetSubsurface { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(
                    Object::from_interface::<super::wl_subsurface::WlSubsurface>(
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
                Request::GetSubsurface { surface, parent } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![
                        Argument::NewId(0),
                        Argument::Object(surface.as_ref().id()),
                        Argument::Object(parent.as_ref().id()),
                    ],
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
                Request::GetSubsurface { surface, parent } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].o = surface.as_ref().c_ptr() as *mut _;
                    _args_array[2].o = parent.as_ref().c_ptr() as *mut _;
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
    pub struct WlSubcompositor(Proxy<WlSubcompositor>);
    impl AsRef<Proxy<WlSubcompositor>> for WlSubcompositor {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlSubcompositor>> for WlSubcompositor {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlSubcompositor(value)
        }
    }
    impl From<WlSubcompositor> for Proxy<WlSubcompositor> {
        #[inline]
        fn from(value: WlSubcompositor) -> Self {
            value.0
        }
    }
    impl Interface for WlSubcompositor {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_subcompositor";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_subcompositor_interface }
        }
    }
    impl WlSubcompositor {
        #[doc = "unbind from the subcompositor interface\n\nInforms the server that the client will not be using this\nprotocol object anymore. This does not affect any other\nobjects, wl_subsurface objects included.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "give a surface the role sub-surface\n\nCreate a sub-surface interface for the given surface, and\nassociate it with the given parent surface. This turns a\nplain wl_surface into a sub-surface.\n\nThe to-be sub-surface must not already have another role, and it\nmust not have an existing wl_subsurface object. Otherwise a protocol\nerror is raised.\n\nAdding sub-surfaces to a parent is a double-buffered operation on the\nparent (see wl_surface.commit). The effect of adding a sub-surface\nbecomes visible on the next time the state of the parent surface is\napplied.\n\nThis request modifies the behaviour of wl_surface.commit request on\nthe sub-surface, see the documentation on wl_subsurface interface."]
        pub fn get_subsurface(
            &self,
            surface: &super::wl_surface::WlSurface,
            parent: &super::wl_surface::WlSurface,
        ) -> Main<super::wl_subsurface::WlSubsurface> {
            let msg = Request::GetSubsurface {
                surface: surface.clone(),
                parent: parent.clone(),
            };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_SUBSURFACE_SINCE: u32 = 1u32;
    static mut wl_subcompositor_requests_get_subsurface_types: [*const wl_interface; 3] = [
        unsafe { &super::wl_subsurface::wl_subsurface_interface as *const wl_interface },
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_subcompositor_requests: [wl_message; 2] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"get_subsurface\0" as *const u8 as *const c_char,
            signature: b"noo\0" as *const u8 as *const c_char,
            types: unsafe { &wl_subcompositor_requests_get_subsurface_types as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_subcompositor_interface: wl_interface = wl_interface {
        name: b"wl_subcompositor\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &wl_subcompositor_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "sub-surface interface to a wl_surface\n\nAn additional interface to a wl_surface object, which has been\nmade a sub-surface. A sub-surface has one parent surface. A\nsub-surface's size and position are not limited to that of the parent.\nParticularly, a sub-surface is not automatically clipped to its\nparent's area.\n\nA sub-surface becomes mapped, when a non-NULL wl_buffer is applied\nand the parent surface is mapped. The order of which one happens\nfirst is irrelevant. A sub-surface is hidden if the parent becomes\nhidden, or if a NULL wl_buffer is applied. These rules apply\nrecursively through the tree of surfaces.\n\nThe behaviour of a wl_surface.commit request on a sub-surface\ndepends on the sub-surface's mode. The possible modes are\nsynchronized and desynchronized, see methods\nwl_subsurface.set_sync and wl_subsurface.set_desync. Synchronized\nmode caches the wl_surface state to be applied when the parent's\nstate gets applied, and desynchronized mode applies the pending\nwl_surface state directly. A sub-surface is initially in the\nsynchronized mode.\n\nSub-surfaces have also other kind of state, which is managed by\nwl_subsurface requests, as opposed to wl_surface requests. This\nstate includes the sub-surface position relative to the parent\nsurface (wl_subsurface.set_position), and the stacking order of\nthe parent and its sub-surfaces (wl_subsurface.place_above and\n.place_below). This state is applied when the parent surface's\nwl_surface state is applied, regardless of the sub-surface's mode.\nAs the exception, set_sync and set_desync are effective immediately.\n\nThe main surface can be thought to be always in desynchronized mode,\nsince it does not have a parent in the sub-surfaces sense.\n\nEven if a sub-surface is in desynchronized mode, it will behave as\nin synchronized mode, if its parent surface behaves as in\nsynchronized mode. This rule is applied recursively throughout the\ntree of surfaces. This means, that one can set a sub-surface into\nsynchronized mode, and then assume that all its child and grand-child\nsub-surfaces are synchronized, too, without explicitly setting them.\n\nIf the wl_surface associated with the wl_subsurface is destroyed, the\nwl_subsurface object becomes inert. Note, that destroying either object\ntakes effect immediately. If you need to synchronize the removal\nof a sub-surface to the parent surface update, unmap the sub-surface\nfirst by attaching a NULL wl_buffer, update parent, and then destroy\nthe sub-surface.\n\nIf the parent wl_surface object is destroyed, the sub-surface is\nunmapped."]
pub mod wl_subsurface {
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
        #[doc = "wl_surface is not a sibling or the parent"]
        BadSurface = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::BadSurface),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "remove sub-surface interface\n\nThe sub-surface interface is removed from the wl_surface object\nthat was turned into a sub-surface with a\nwl_subcompositor.get_subsurface request. The wl_surface's association\nto the parent is deleted, and the wl_surface loses its role as\na sub-surface. The wl_surface is unmapped immediately.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "reposition the sub-surface\n\nThis schedules a sub-surface position change.\nThe sub-surface will be moved so that its origin (top left\ncorner pixel) will be at the location x, y of the parent surface\ncoordinate system. The coordinates are not restricted to the parent\nsurface area. Negative values are allowed.\n\nThe scheduled coordinates will take effect whenever the state of the\nparent surface is applied. When this happens depends on whether the\nparent surface is in synchronized mode or not. See\nwl_subsurface.set_sync and wl_subsurface.set_desync for details.\n\nIf more than one set_position request is invoked by the client before\nthe commit of the parent surface, the position of a new request always\nreplaces the scheduled position from any previous request.\n\nThe initial position is 0, 0."]
        SetPosition { x: i32, y: i32 },
        #[doc = "restack the sub-surface\n\nThis sub-surface is taken from the stack, and put back just\nabove the reference surface, changing the z-order of the sub-surfaces.\nThe reference surface must be one of the sibling surfaces, or the\nparent surface. Using any other surface, including this sub-surface,\nwill cause a protocol error.\n\nThe z-order is double-buffered. Requests are handled in order and\napplied immediately to a pending state. The final pending state is\ncopied to the active state the next time the state of the parent\nsurface is applied. When this happens depends on whether the parent\nsurface is in synchronized mode or not. See wl_subsurface.set_sync and\nwl_subsurface.set_desync for details.\n\nA new sub-surface is initially added as the top-most in the stack\nof its siblings and parent."]
        PlaceAbove {
            sibling: super::wl_surface::WlSurface,
        },
        #[doc = "restack the sub-surface\n\nThe sub-surface is placed just below the reference surface.\nSee wl_subsurface.place_above."]
        PlaceBelow {
            sibling: super::wl_surface::WlSurface,
        },
        #[doc = "set sub-surface to synchronized mode\n\nChange the commit behaviour of the sub-surface to synchronized\nmode, also described as the parent dependent mode.\n\nIn synchronized mode, wl_surface.commit on a sub-surface will\naccumulate the committed state in a cache, but the state will\nnot be applied and hence will not change the compositor output.\nThe cached state is applied to the sub-surface immediately after\nthe parent surface's state is applied. This ensures atomic\nupdates of the parent and all its synchronized sub-surfaces.\nApplying the cached state will invalidate the cache, so further\nparent surface commits do not (re-)apply old state.\n\nSee wl_subsurface for the recursive effect of this mode."]
        SetSync,
        #[doc = "set sub-surface to desynchronized mode\n\nChange the commit behaviour of the sub-surface to desynchronized\nmode, also described as independent or freely running mode.\n\nIn desynchronized mode, wl_surface.commit on a sub-surface will\napply the pending state directly, without caching, as happens\nnormally with a wl_surface. Calling wl_surface.commit on the\nparent surface has no effect on the sub-surface's wl_surface\nstate. This mode allows a sub-surface to be updated on its own.\n\nIf cached state exists when wl_surface.commit is called in\ndesynchronized mode, the pending state is added to the cached\nstate, and applied as a whole. This invalidates the cache.\n\nNote: even if a sub-surface is set to desynchronized, a parent\nsub-surface may override it to behave as synchronized. For details,\nsee wl_subsurface.\n\nIf a surface's parent surface behaves as desynchronized, then\nthe cached state is applied on set_desync."]
        SetDesync,
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
                name: "set_position",
                since: 1,
                signature: &[super::ArgumentType::Int, super::ArgumentType::Int],
                destructor: false,
            },
            super::MessageDesc {
                name: "place_above",
                since: 1,
                signature: &[super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "place_below",
                since: 1,
                signature: &[super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_sync",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_desync",
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
                Request::SetPosition { .. } => 1,
                Request::PlaceAbove { .. } => 2,
                Request::PlaceBelow { .. } => 3,
                Request::SetSync => 4,
                Request::SetDesync => 5,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::SetPosition { .. } => 1,
                Request::PlaceAbove { .. } => 1,
                Request::PlaceBelow { .. } => 1,
                Request::SetSync => 1,
                Request::SetDesync => 1,
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
                Request::SetPosition { x, y } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::Int(x), Argument::Int(y),],
                },
                Request::PlaceAbove { sibling } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![Argument::Object(sibling.as_ref().id()),],
                },
                Request::PlaceBelow { sibling } => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: smallvec![Argument::Object(sibling.as_ref().id()),],
                },
                Request::SetSync => Message {
                    sender_id: sender_id,
                    opcode: 4,
                    args: smallvec![],
                },
                Request::SetDesync => Message {
                    sender_id: sender_id,
                    opcode: 5,
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
                Request::SetPosition { x, y } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = x;
                    _args_array[1].i = y;
                    f(1, &mut _args_array)
                }
                Request::PlaceAbove { sibling } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = sibling.as_ref().c_ptr() as *mut _;
                    f(2, &mut _args_array)
                }
                Request::PlaceBelow { sibling } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = sibling.as_ref().c_ptr() as *mut _;
                    f(3, &mut _args_array)
                }
                Request::SetSync => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(4, &mut _args_array)
                }
                Request::SetDesync => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(5, &mut _args_array)
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
    pub struct WlSubsurface(Proxy<WlSubsurface>);
    impl AsRef<Proxy<WlSubsurface>> for WlSubsurface {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<WlSubsurface>> for WlSubsurface {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            WlSubsurface(value)
        }
    }
    impl From<WlSubsurface> for Proxy<WlSubsurface> {
        #[inline]
        fn from(value: WlSubsurface) -> Self {
            value.0
        }
    }
    impl Interface for WlSubsurface {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_subsurface";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &wl_subsurface_interface }
        }
    }
    impl WlSubsurface {
        #[doc = "remove sub-surface interface\n\nThe sub-surface interface is removed from the wl_surface object\nthat was turned into a sub-surface with a\nwl_subcompositor.get_subsurface request. The wl_surface's association\nto the parent is deleted, and the wl_surface loses its role as\na sub-surface. The wl_surface is unmapped immediately.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "reposition the sub-surface\n\nThis schedules a sub-surface position change.\nThe sub-surface will be moved so that its origin (top left\ncorner pixel) will be at the location x, y of the parent surface\ncoordinate system. The coordinates are not restricted to the parent\nsurface area. Negative values are allowed.\n\nThe scheduled coordinates will take effect whenever the state of the\nparent surface is applied. When this happens depends on whether the\nparent surface is in synchronized mode or not. See\nwl_subsurface.set_sync and wl_subsurface.set_desync for details.\n\nIf more than one set_position request is invoked by the client before\nthe commit of the parent surface, the position of a new request always\nreplaces the scheduled position from any previous request.\n\nThe initial position is 0, 0."]
        pub fn set_position(&self, x: i32, y: i32) -> () {
            let msg = Request::SetPosition { x: x, y: y };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "restack the sub-surface\n\nThis sub-surface is taken from the stack, and put back just\nabove the reference surface, changing the z-order of the sub-surfaces.\nThe reference surface must be one of the sibling surfaces, or the\nparent surface. Using any other surface, including this sub-surface,\nwill cause a protocol error.\n\nThe z-order is double-buffered. Requests are handled in order and\napplied immediately to a pending state. The final pending state is\ncopied to the active state the next time the state of the parent\nsurface is applied. When this happens depends on whether the parent\nsurface is in synchronized mode or not. See wl_subsurface.set_sync and\nwl_subsurface.set_desync for details.\n\nA new sub-surface is initially added as the top-most in the stack\nof its siblings and parent."]
        pub fn place_above(&self, sibling: &super::wl_surface::WlSurface) -> () {
            let msg = Request::PlaceAbove {
                sibling: sibling.clone(),
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "restack the sub-surface\n\nThe sub-surface is placed just below the reference surface.\nSee wl_subsurface.place_above."]
        pub fn place_below(&self, sibling: &super::wl_surface::WlSurface) -> () {
            let msg = Request::PlaceBelow {
                sibling: sibling.clone(),
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set sub-surface to synchronized mode\n\nChange the commit behaviour of the sub-surface to synchronized\nmode, also described as the parent dependent mode.\n\nIn synchronized mode, wl_surface.commit on a sub-surface will\naccumulate the committed state in a cache, but the state will\nnot be applied and hence will not change the compositor output.\nThe cached state is applied to the sub-surface immediately after\nthe parent surface's state is applied. This ensures atomic\nupdates of the parent and all its synchronized sub-surfaces.\nApplying the cached state will invalidate the cache, so further\nparent surface commits do not (re-)apply old state.\n\nSee wl_subsurface for the recursive effect of this mode."]
        pub fn set_sync(&self) -> () {
            let msg = Request::SetSync;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set sub-surface to desynchronized mode\n\nChange the commit behaviour of the sub-surface to desynchronized\nmode, also described as independent or freely running mode.\n\nIn desynchronized mode, wl_surface.commit on a sub-surface will\napply the pending state directly, without caching, as happens\nnormally with a wl_surface. Calling wl_surface.commit on the\nparent surface has no effect on the sub-surface's wl_surface\nstate. This mode allows a sub-surface to be updated on its own.\n\nIf cached state exists when wl_surface.commit is called in\ndesynchronized mode, the pending state is added to the cached\nstate, and applied as a whole. This invalidates the cache.\n\nNote: even if a sub-surface is set to desynchronized, a parent\nsub-surface may override it to behave as synchronized. For details,\nsee wl_subsurface.\n\nIf a surface's parent surface behaves as desynchronized, then\nthe cached state is applied on set_desync."]
        pub fn set_desync(&self) -> () {
            let msg = Request::SetDesync;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_POSITION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_PLACE_ABOVE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_PLACE_BELOW_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_SYNC_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_DESYNC_SINCE: u32 = 1u32;
    static mut wl_subsurface_requests_place_above_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface }];
    static mut wl_subsurface_requests_place_below_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut wl_subsurface_requests: [wl_message; 6] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_position\0" as *const u8 as *const c_char,
            signature: b"ii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"place_above\0" as *const u8 as *const c_char,
            signature: b"o\0" as *const u8 as *const c_char,
            types: unsafe { &wl_subsurface_requests_place_above_types as *const _ },
        },
        wl_message {
            name: b"place_below\0" as *const u8 as *const c_char,
            signature: b"o\0" as *const u8 as *const c_char,
            types: unsafe { &wl_subsurface_requests_place_below_types as *const _ },
        },
        wl_message {
            name: b"set_sync\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_desync\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut wl_subsurface_interface: wl_interface = wl_interface {
        name: b"wl_subsurface\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 6,
        requests: unsafe { &wl_subsurface_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
