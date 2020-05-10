use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 1] =
    [NULLPTR as *const sys::common::wl_interface];
#[doc = "manager to create per-output gamma controls\n\nThis interface is a manager that allows creating per-output gamma\ncontrols."]
pub mod zwlr_gamma_control_manager_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "get a gamma control for an output\n\nCreate a gamma control that can be used to adjust gamma tables for the\nprovided output."]
        GetGammaControl { output: super::wl_output::WlOutput },
        #[doc = "destroy the manager\n\nAll objects created by the manager will still remain valid, until their\nappropriate destroy request has been called.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "get_gamma_control",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
                destructor: true,
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
                Request::GetGammaControl { .. } => 0,
                Request::Destroy => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::GetGammaControl { .. } => 1,
                Request::Destroy => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<
                    super::zwlr_gamma_control_v1::ZwlrGammaControlV1,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::GetGammaControl { output } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![Argument::NewId(0), Argument::Object(output.as_ref().id()),],
                },
                Request::Destroy => Message {
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
                Request::GetGammaControl { output } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].o = output.as_ref().c_ptr() as *mut _;
                    f(0, &mut _args_array)
                }
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
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
    pub struct ZwlrGammaControlManagerV1(Proxy<ZwlrGammaControlManagerV1>);
    impl AsRef<Proxy<ZwlrGammaControlManagerV1>> for ZwlrGammaControlManagerV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwlrGammaControlManagerV1>> for ZwlrGammaControlManagerV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwlrGammaControlManagerV1(value)
        }
    }
    impl From<ZwlrGammaControlManagerV1> for Proxy<ZwlrGammaControlManagerV1> {
        #[inline]
        fn from(value: ZwlrGammaControlManagerV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwlrGammaControlManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_gamma_control_manager_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwlr_gamma_control_manager_v1_interface }
        }
    }
    impl ZwlrGammaControlManagerV1 {
        #[doc = "get a gamma control for an output\n\nCreate a gamma control that can be used to adjust gamma tables for the\nprovided output."]
        pub fn get_gamma_control(
            &self,
            output: &super::wl_output::WlOutput,
        ) -> Main<super::zwlr_gamma_control_v1::ZwlrGammaControlV1> {
            let msg = Request::GetGammaControl {
                output: output.clone(),
            };
            self.0.send(msg, None).unwrap()
        }
        #[doc = "destroy the manager\n\nAll objects created by the manager will still remain valid, until their\nappropriate destroy request has been called.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_GAMMA_CONTROL_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    static mut zwlr_gamma_control_manager_v1_requests_get_gamma_control_types:
        [*const wl_interface; 2] = [
        unsafe {
            &super::zwlr_gamma_control_v1::zwlr_gamma_control_v1_interface as *const wl_interface
        },
        unsafe { &super::wl_output::wl_output_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwlr_gamma_control_manager_v1_requests: [wl_message; 2] = [
        wl_message {
            name: b"get_gamma_control\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe {
                &zwlr_gamma_control_manager_v1_requests_get_gamma_control_types as *const _
            },
        },
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwlr_gamma_control_manager_v1_interface: wl_interface = wl_interface {
        name: b"zwlr_gamma_control_manager_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &zwlr_gamma_control_manager_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "adjust gamma tables for an output\n\nThis interface allows a client to adjust gamma tables for a particular\noutput.\n\nThe client will receive the gamma size, and will then be able to set gamma\ntables. At any time the compositor can send a failed event indicating that\nthis object is no longer valid.\n\nThere can only be at most one gamma control object per output, which\nhas exclusive access to this particular output. When the gamma control\nobject is destroyed, the gamma table is restored to its original value."]
pub mod zwlr_gamma_control_v1 {
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
        #[doc = "invalid gamma tables"]
        InvalidGamma = 1,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                1 => Some(Error::InvalidGamma),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "set the gamma table\n\nSet the gamma table. The file descriptor can be memory-mapped to provide\nthe raw gamma table, which contains successive gamma ramps for the red,\ngreen and blue channels. Each gamma ramp is an array of 16-byte unsigned\nintegers which has the same length as the gamma size.\n\nThe file descriptor data must have the same length as three times the\ngamma size."]
        SetGamma { fd: ::std::os::unix::io::RawFd },
        #[doc = "destroy this control\n\nDestroys the gamma control object. If the object is still valid, this\nrestores the original gamma tables.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "set_gamma",
                since: 1,
                signature: &[super::ArgumentType::Fd],
                destructor: false,
            },
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
                destructor: true,
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
                Request::SetGamma { .. } => 0,
                Request::Destroy => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::SetGamma { .. } => 1,
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
                Request::SetGamma { fd } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![Argument::Fd(fd),],
                },
                Request::Destroy => Message {
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
                Request::SetGamma { fd } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].h = fd;
                    f(0, &mut _args_array)
                }
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(1, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "size of gamma ramps\n\nAdvertise the size of each gamma ramp.\n\nThis event is sent immediately when the gamma control object is created."]
        GammaSize { size: u32 },
        #[doc = "object no longer valid\n\nThis event indicates that the gamma control is no longer valid. This\ncan happen for a number of reasons, including:\n- The output doesn't support gamma tables\n- Setting the gamma tables failed\n- Another client already has exclusive gamma control for this output\n- The compositor has transferred gamma control to another client\n\nUpon receiving this event, the client should destroy this object."]
        Failed,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "gamma_size",
                since: 1,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "failed",
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
                Event::GammaSize { .. } => 0,
                Event::Failed => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::GammaSize { .. } => 1,
                Event::Failed => 1,
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
                    Ok(Event::GammaSize {
                        size: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => Ok(Event::Failed),
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
                    Ok(Event::GammaSize { size: _args[0].u })
                }
                1 => Ok(Event::Failed),
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
    pub struct ZwlrGammaControlV1(Proxy<ZwlrGammaControlV1>);
    impl AsRef<Proxy<ZwlrGammaControlV1>> for ZwlrGammaControlV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwlrGammaControlV1>> for ZwlrGammaControlV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwlrGammaControlV1(value)
        }
    }
    impl From<ZwlrGammaControlV1> for Proxy<ZwlrGammaControlV1> {
        #[inline]
        fn from(value: ZwlrGammaControlV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwlrGammaControlV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_gamma_control_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwlr_gamma_control_v1_interface }
        }
    }
    impl ZwlrGammaControlV1 {
        #[doc = "set the gamma table\n\nSet the gamma table. The file descriptor can be memory-mapped to provide\nthe raw gamma table, which contains successive gamma ramps for the red,\ngreen and blue channels. Each gamma ramp is an array of 16-byte unsigned\nintegers which has the same length as the gamma size.\n\nThe file descriptor data must have the same length as three times the\ngamma size."]
        pub fn set_gamma(&self, fd: ::std::os::unix::io::RawFd) -> () {
            let msg = Request::SetGamma { fd: fd };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "destroy this control\n\nDestroys the gamma control object. If the object is still valid, this\nrestores the original gamma tables.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_GAMMA_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_GAMMA_SIZE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FAILED_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwlr_gamma_control_v1_requests: [wl_message; 2] = [
        wl_message {
            name: b"set_gamma\0" as *const u8 as *const c_char,
            signature: b"h\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwlr_gamma_control_v1_events: [wl_message; 2] = [
        wl_message {
            name: b"gamma_size\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"failed\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwlr_gamma_control_v1_interface: wl_interface = wl_interface {
        name: b"zwlr_gamma_control_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &zwlr_gamma_control_v1_requests as *const _ },
        event_count: 2,
        events: unsafe { &zwlr_gamma_control_v1_events as *const _ },
    };
}
