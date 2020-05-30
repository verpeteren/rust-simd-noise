use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 4] = [
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
];
#[doc = "manager to inform clients and begin capturing\n\nThis object is a manager which offers requests to start capturing from a\nsource."]
pub mod zwlr_screencopy_manager_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "capture an output\n\nCapture the next frame of an entire output."]
        CaptureOutput {
            overlay_cursor: i32,
            output: super::wl_output::WlOutput,
        },
        #[doc = "capture an output's region\n\nCapture the next frame of an output's region.\n\nThe region is given in output logical coordinates, see\nxdg_output.logical_size. The region will be clipped to the output's\nextents."]
        CaptureOutputRegion {
            overlay_cursor: i32,
            output: super::wl_output::WlOutput,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
        },
        #[doc = "destroy the manager\n\nAll objects created by the manager will still remain valid, until their\nappropriate destroy request has been called.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "capture_output",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                    super::ArgumentType::Int,
                    super::ArgumentType::Object,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "capture_output_region",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                    super::ArgumentType::Int,
                    super::ArgumentType::Object,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
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
                Request::CaptureOutput { .. } => 0,
                Request::CaptureOutputRegion { .. } => 1,
                Request::Destroy => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::CaptureOutput { .. } => 1,
                Request::CaptureOutputRegion { .. } => 1,
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
                    super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1,
                >(version, meta.child())),
                1 => Some(Object::from_interface::<
                    super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::CaptureOutput {
                    overlay_cursor,
                    output,
                } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![
                        Argument::NewId(0),
                        Argument::Int(overlay_cursor),
                        Argument::Object(output.as_ref().id()),
                    ],
                },
                Request::CaptureOutputRegion {
                    overlay_cursor,
                    output,
                    x,
                    y,
                    width,
                    height,
                } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![
                        Argument::NewId(0),
                        Argument::Int(overlay_cursor),
                        Argument::Object(output.as_ref().id()),
                        Argument::Int(x),
                        Argument::Int(y),
                        Argument::Int(width),
                        Argument::Int(height),
                    ],
                },
                Request::Destroy => Message {
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
                Request::CaptureOutput {
                    overlay_cursor,
                    output,
                } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].i = overlay_cursor;
                    _args_array[2].o = output.as_ref().c_ptr() as *mut _;
                    f(0, &mut _args_array)
                }
                Request::CaptureOutputRegion {
                    overlay_cursor,
                    output,
                    x,
                    y,
                    width,
                    height,
                } => {
                    let mut _args_array: [wl_argument; 7] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].i = overlay_cursor;
                    _args_array[2].o = output.as_ref().c_ptr() as *mut _;
                    _args_array[3].i = x;
                    _args_array[4].i = y;
                    _args_array[5].i = width;
                    _args_array[6].i = height;
                    f(1, &mut _args_array)
                }
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
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
    pub struct ZwlrScreencopyManagerV1(Proxy<ZwlrScreencopyManagerV1>);
    impl AsRef<Proxy<ZwlrScreencopyManagerV1>> for ZwlrScreencopyManagerV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwlrScreencopyManagerV1>> for ZwlrScreencopyManagerV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwlrScreencopyManagerV1(value)
        }
    }
    impl From<ZwlrScreencopyManagerV1> for Proxy<ZwlrScreencopyManagerV1> {
        #[inline]
        fn from(value: ZwlrScreencopyManagerV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwlrScreencopyManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_screencopy_manager_v1";
        const VERSION: u32 = 2;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwlr_screencopy_manager_v1_interface }
        }
    }
    impl ZwlrScreencopyManagerV1 {
        #[doc = "capture an output\n\nCapture the next frame of an entire output."]
        pub fn capture_output(
            &self,
            overlay_cursor: i32,
            output: &super::wl_output::WlOutput,
        ) -> Main<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1> {
            let msg = Request::CaptureOutput {
                overlay_cursor: overlay_cursor,
                output: output.clone(),
            };
            self.0.send(msg, None).unwrap()
        }
        #[doc = "capture an output's region\n\nCapture the next frame of an output's region.\n\nThe region is given in output logical coordinates, see\nxdg_output.logical_size. The region will be clipped to the output's\nextents."]
        pub fn capture_output_region(
            &self,
            overlay_cursor: i32,
            output: &super::wl_output::WlOutput,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
        ) -> Main<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1> {
            let msg = Request::CaptureOutputRegion {
                overlay_cursor: overlay_cursor,
                output: output.clone(),
                x: x,
                y: y,
                width: width,
                height: height,
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
    pub const REQ_CAPTURE_OUTPUT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CAPTURE_OUTPUT_REGION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    static mut zwlr_screencopy_manager_v1_requests_capture_output_types: [*const wl_interface; 3] = [
        unsafe {
            &super::zwlr_screencopy_frame_v1::zwlr_screencopy_frame_v1_interface
                as *const wl_interface
        },
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_output::wl_output_interface as *const wl_interface },
    ];
    static mut zwlr_screencopy_manager_v1_requests_capture_output_region_types:
        [*const wl_interface; 7] = [
        unsafe {
            &super::zwlr_screencopy_frame_v1::zwlr_screencopy_frame_v1_interface
                as *const wl_interface
        },
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_output::wl_output_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwlr_screencopy_manager_v1_requests: [wl_message; 3] = [
        wl_message {
            name: b"capture_output\0" as *const u8 as *const c_char,
            signature: b"nio\0" as *const u8 as *const c_char,
            types: unsafe { &zwlr_screencopy_manager_v1_requests_capture_output_types as *const _ },
        },
        wl_message {
            name: b"capture_output_region\0" as *const u8 as *const c_char,
            signature: b"nioiiii\0" as *const u8 as *const c_char,
            types: unsafe {
                &zwlr_screencopy_manager_v1_requests_capture_output_region_types as *const _
            },
        },
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwlr_screencopy_manager_v1_interface: wl_interface = wl_interface {
        name: b"zwlr_screencopy_manager_v1\0" as *const u8 as *const c_char,
        version: 2,
        request_count: 3,
        requests: unsafe { &zwlr_screencopy_manager_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "a frame ready for copy\n\nThis object represents a single frame.\n\nWhen created, a \"buffer\" event will be sent. The client will then be able\nto send a \"copy\" request. If the capture is successful, the compositor\nwill send a \"flags\" followed by a \"ready\" event.\n\nIf the capture failed, the \"failed\" event is sent. This can happen anytime\nbefore the \"ready\" event.\n\nOnce either a \"ready\" or a \"failed\" event is received, the client should\ndestroy the frame."]
pub mod zwlr_screencopy_frame_v1 {
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
        #[doc = "the object has already been used to copy a wl_buffer"]
        AlreadyUsed = 0,
        #[doc = "buffer attributes are invalid"]
        InvalidBuffer = 1,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::AlreadyUsed),
                1 => Some(Error::InvalidBuffer),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    bitflags! { pub struct Flags : u32 { # [ doc = "contents are y-inverted" ] const YInvert = 1 ; } }
    impl Flags {
        pub fn from_raw(n: u32) -> Option<Flags> {
            Some(Flags::from_bits_truncate(n))
        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "copy the frame\n\nCopy the frame to the supplied buffer. The buffer must have a the\ncorrect size, see zwlr_screencopy_frame_v1.buffer. The buffer needs to\nhave a supported format.\n\nIf the frame is successfully copied, a \"flags\" and a \"ready\" events are\nsent. Otherwise, a \"failed\" event is sent."]
        Copy { buffer: super::wl_buffer::WlBuffer },
        #[doc = "delete this object, used or not\n\nDestroys the frame. This request can be sent at any time by the client.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "copy the frame when it's damaged\n\nSame as copy, except it waits until there is damage to copy.\n\nOnly available since version 2 of the interface"]
        CopyWithDamage { buffer: super::wl_buffer::WlBuffer },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "copy",
                since: 1,
                signature: &[super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
                destructor: true,
            },
            super::MessageDesc {
                name: "copy_with_damage",
                since: 2,
                signature: &[super::ArgumentType::Object],
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
                Request::Copy { .. } => 0,
                Request::Destroy => 1,
                Request::CopyWithDamage { .. } => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Copy { .. } => 1,
                Request::Destroy => 1,
                Request::CopyWithDamage { .. } => 2,
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
                Request::Copy { buffer } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![Argument::Object(buffer.as_ref().id()),],
                },
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![],
                },
                Request::CopyWithDamage { buffer } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![Argument::Object(buffer.as_ref().id()),],
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
                Request::Copy { buffer } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = buffer.as_ref().c_ptr() as *mut _;
                    f(0, &mut _args_array)
                }
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(1, &mut _args_array)
                }
                Request::CopyWithDamage { buffer } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = buffer.as_ref().c_ptr() as *mut _;
                    f(2, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "buffer information\n\nProvides information about the frame's buffer. This event is sent once\nas soon as the frame is created.\n\nThe client should then create a buffer with the provided attributes, and\nsend a \"copy\" request."]
        Buffer {
            format: u32,
            width: u32,
            height: u32,
            stride: u32,
        },
        #[doc = "frame flags\n\nProvides flags about the frame. This event is sent once before the\n\"ready\" event."]
        Flags { flags: Flags },
        #[doc = "indicates frame is available for reading\n\nCalled as soon as the frame is copied, indicating it is available\nfor reading. This event includes the time at which presentation happened\nat.\n\nThe timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,\neach component being an unsigned 32-bit value. Whole seconds are in\ntv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,\nand the additional fractional part in tv_nsec as nanoseconds. Hence,\nfor valid timestamps tv_nsec must be in [0, 999999999]. The seconds part\nmay have an arbitrary offset at start.\n\nAfter receiving this event, the client should destroy the object."]
        Ready {
            tv_sec_hi: u32,
            tv_sec_lo: u32,
            tv_nsec: u32,
        },
        #[doc = "frame copy failed\n\nThis event indicates that the attempted frame copy has failed.\n\nAfter receiving this event, the client should destroy the object."]
        Failed,
        #[doc = "carries the coordinates of the damaged region\n\nThis event is sent right before the ready event when copy_with_damage is\nrequested. It may be generated multiple times for each copy_with_damage\nrequest.\n\nThe arguments describe a box around an area that has changed since the\nlast copy request that was derived from the current screencopy manager\ninstance.\n\nThe union of all regions received between the call to copy_with_damage\nand a ready event is the total damage since the prior ready event.\n\nOnly available since version 2 of the interface"]
        Damage {
            x: u32,
            y: u32,
            width: u32,
            height: u32,
        },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "buffer",
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
                name: "flags",
                since: 1,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "ready",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "failed",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "damage",
                since: 2,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
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
                Event::Buffer { .. } => 0,
                Event::Flags { .. } => 1,
                Event::Ready { .. } => 2,
                Event::Failed => 3,
                Event::Damage { .. } => 4,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Buffer { .. } => 1,
                Event::Flags { .. } => 1,
                Event::Ready { .. } => 1,
                Event::Failed => 1,
                Event::Damage { .. } => 2,
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
                    Ok(Event::Buffer {
                        format: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        width: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        height: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        stride: {
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
                    Ok(Event::Flags {
                        flags: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                Flags::from_raw(val).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Ready {
                        tv_sec_hi: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        tv_sec_lo: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        tv_nsec: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                3 => Ok(Event::Failed),
                4 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Damage {
                        x: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        y: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        width: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        height: {
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
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Event::Buffer {
                        format: _args[0].u,
                        width: _args[1].u,
                        height: _args[2].u,
                        stride: _args[3].u,
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Flags {
                        flags: Flags::from_raw(_args[0].u).ok_or(())?,
                    })
                }
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Ready {
                        tv_sec_hi: _args[0].u,
                        tv_sec_lo: _args[1].u,
                        tv_nsec: _args[2].u,
                    })
                }
                3 => Ok(Event::Failed),
                4 => {
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Event::Damage {
                        x: _args[0].u,
                        y: _args[1].u,
                        width: _args[2].u,
                        height: _args[3].u,
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
    pub struct ZwlrScreencopyFrameV1(Proxy<ZwlrScreencopyFrameV1>);
    impl AsRef<Proxy<ZwlrScreencopyFrameV1>> for ZwlrScreencopyFrameV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwlrScreencopyFrameV1>> for ZwlrScreencopyFrameV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwlrScreencopyFrameV1(value)
        }
    }
    impl From<ZwlrScreencopyFrameV1> for Proxy<ZwlrScreencopyFrameV1> {
        #[inline]
        fn from(value: ZwlrScreencopyFrameV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwlrScreencopyFrameV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_screencopy_frame_v1";
        const VERSION: u32 = 2;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwlr_screencopy_frame_v1_interface }
        }
    }
    impl ZwlrScreencopyFrameV1 {
        #[doc = "copy the frame\n\nCopy the frame to the supplied buffer. The buffer must have a the\ncorrect size, see zwlr_screencopy_frame_v1.buffer. The buffer needs to\nhave a supported format.\n\nIf the frame is successfully copied, a \"flags\" and a \"ready\" events are\nsent. Otherwise, a \"failed\" event is sent."]
        pub fn copy(&self, buffer: &super::wl_buffer::WlBuffer) -> () {
            let msg = Request::Copy {
                buffer: buffer.clone(),
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "delete this object, used or not\n\nDestroys the frame. This request can be sent at any time by the client.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "copy the frame when it's damaged\n\nSame as copy, except it waits until there is damage to copy.\n\nOnly available since version 2 of the interface."]
        pub fn copy_with_damage(&self, buffer: &super::wl_buffer::WlBuffer) -> () {
            let msg = Request::CopyWithDamage {
                buffer: buffer.clone(),
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_COPY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_COPY_WITH_DAMAGE_SINCE: u32 = 2u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_BUFFER_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FLAGS_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_READY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FAILED_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DAMAGE_SINCE: u32 = 2u32;
    static mut zwlr_screencopy_frame_v1_requests_copy_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_buffer::wl_buffer_interface as *const wl_interface }];
    static mut zwlr_screencopy_frame_v1_requests_copy_with_damage_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_buffer::wl_buffer_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwlr_screencopy_frame_v1_requests: [wl_message; 3] = [
        wl_message {
            name: b"copy\0" as *const u8 as *const c_char,
            signature: b"o\0" as *const u8 as *const c_char,
            types: unsafe { &zwlr_screencopy_frame_v1_requests_copy_types as *const _ },
        },
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"copy_with_damage\0" as *const u8 as *const c_char,
            signature: b"2o\0" as *const u8 as *const c_char,
            types: unsafe { &zwlr_screencopy_frame_v1_requests_copy_with_damage_types as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwlr_screencopy_frame_v1_events: [wl_message; 5] = [
        wl_message {
            name: b"buffer\0" as *const u8 as *const c_char,
            signature: b"uuuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"flags\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"ready\0" as *const u8 as *const c_char,
            signature: b"uuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"failed\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"damage\0" as *const u8 as *const c_char,
            signature: b"2uuuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwlr_screencopy_frame_v1_interface: wl_interface = wl_interface {
        name: b"zwlr_screencopy_frame_v1\0" as *const u8 as *const c_char,
        version: 2,
        request_count: 3,
        requests: unsafe { &zwlr_screencopy_frame_v1_requests as *const _ },
        event_count: 5,
        events: unsafe { &zwlr_screencopy_frame_v1_events as *const _ },
    };
}
