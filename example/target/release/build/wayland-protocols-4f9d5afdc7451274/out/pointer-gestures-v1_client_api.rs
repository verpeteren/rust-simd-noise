use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 5] = [
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
];
#[doc = "touchpad gestures\n\nA global interface to provide semantic touchpad gestures for a given\npointer.\n\nTwo gestures are currently supported: swipe and zoom/rotate.\nAll gestures follow a three-stage cycle: begin, update, end and\nare identified by a unique id.\n\nWarning! The protocol described in this file is experimental and\nbackward incompatible changes may be made. Backward compatible changes\nmay be added together with the corresponding interface version bump.\nBackward incompatible changes are done by bumping the version number in\nthe protocol and interface names and resetting the interface version.\nOnce the protocol is to be declared stable, the 'z' prefix and the\nversion number in the protocol and interface names are removed and the\ninterface version number is reset."]
pub mod zwp_pointer_gestures_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "get swipe gesture\n\nCreate a swipe gesture object. See the\nwl_pointer_gesture_swipe interface for details."]
        GetSwipeGesture {
            pointer: super::wl_pointer::WlPointer,
        },
        #[doc = "get pinch gesture\n\nCreate a pinch gesture object. See the\nwl_pointer_gesture_pinch interface for details."]
        GetPinchGesture {
            pointer: super::wl_pointer::WlPointer,
        },
        #[doc = "destroy the pointer gesture object\n\nDestroy the pointer gesture object. Swipe and pinch objects created via this\ngesture object remain valid.\n\nThis is a destructor, once sent this object cannot be used any longer.\nOnly available since version 2 of the interface"]
        Release,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "get_swipe_gesture",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "get_pinch_gesture",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Object],
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
                Request::GetSwipeGesture { .. } => 0,
                Request::GetPinchGesture { .. } => 1,
                Request::Release => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::GetSwipeGesture { .. } => 1,
                Request::GetPinchGesture { .. } => 1,
                Request::Release => 2,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<
                    super::zwp_pointer_gesture_swipe_v1::ZwpPointerGestureSwipeV1,
                >(version, meta.child())),
                1 => Some(Object::from_interface::<
                    super::zwp_pointer_gesture_pinch_v1::ZwpPointerGesturePinchV1,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::GetSwipeGesture { pointer } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![Argument::NewId(0), Argument::Object(pointer.as_ref().id()),],
                },
                Request::GetPinchGesture { pointer } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::NewId(0), Argument::Object(pointer.as_ref().id()),],
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
                Request::GetSwipeGesture { pointer } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].o = pointer.as_ref().c_ptr() as *mut _;
                    f(0, &mut _args_array)
                }
                Request::GetPinchGesture { pointer } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].o = pointer.as_ref().c_ptr() as *mut _;
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
    pub struct ZwpPointerGesturesV1(Proxy<ZwpPointerGesturesV1>);
    impl AsRef<Proxy<ZwpPointerGesturesV1>> for ZwpPointerGesturesV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpPointerGesturesV1>> for ZwpPointerGesturesV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpPointerGesturesV1(value)
        }
    }
    impl From<ZwpPointerGesturesV1> for Proxy<ZwpPointerGesturesV1> {
        #[inline]
        fn from(value: ZwpPointerGesturesV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwpPointerGesturesV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_pointer_gestures_v1";
        const VERSION: u32 = 2;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_pointer_gestures_v1_interface }
        }
    }
    impl ZwpPointerGesturesV1 {
        #[doc = "get swipe gesture\n\nCreate a swipe gesture object. See the\nwl_pointer_gesture_swipe interface for details."]
        pub fn get_swipe_gesture(
            &self,
            pointer: &super::wl_pointer::WlPointer,
        ) -> Main<super::zwp_pointer_gesture_swipe_v1::ZwpPointerGestureSwipeV1> {
            let msg = Request::GetSwipeGesture {
                pointer: pointer.clone(),
            };
            self.0.send(msg, None).unwrap()
        }
        #[doc = "get pinch gesture\n\nCreate a pinch gesture object. See the\nwl_pointer_gesture_pinch interface for details."]
        pub fn get_pinch_gesture(
            &self,
            pointer: &super::wl_pointer::WlPointer,
        ) -> Main<super::zwp_pointer_gesture_pinch_v1::ZwpPointerGesturePinchV1> {
            let msg = Request::GetPinchGesture {
                pointer: pointer.clone(),
            };
            self.0.send(msg, None).unwrap()
        }
        #[doc = "destroy the pointer gesture object\n\nDestroy the pointer gesture object. Swipe and pinch objects created via this\ngesture object remain valid.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called.\nOnly available since version 2 of the interface."]
        pub fn release(&self) -> () {
            let msg = Request::Release;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_SWIPE_GESTURE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_PINCH_GESTURE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_RELEASE_SINCE: u32 = 2u32;
    static mut zwp_pointer_gestures_v1_requests_get_swipe_gesture_types: [*const wl_interface; 2] = [
        unsafe {
            &super::zwp_pointer_gesture_swipe_v1::zwp_pointer_gesture_swipe_v1_interface
                as *const wl_interface
        },
        unsafe { &super::wl_pointer::wl_pointer_interface as *const wl_interface },
    ];
    static mut zwp_pointer_gestures_v1_requests_get_pinch_gesture_types: [*const wl_interface; 2] = [
        unsafe {
            &super::zwp_pointer_gesture_pinch_v1::zwp_pointer_gesture_pinch_v1_interface
                as *const wl_interface
        },
        unsafe { &super::wl_pointer::wl_pointer_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_pointer_gestures_v1_requests: [wl_message; 3] = [
        wl_message {
            name: b"get_swipe_gesture\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe { &zwp_pointer_gestures_v1_requests_get_swipe_gesture_types as *const _ },
        },
        wl_message {
            name: b"get_pinch_gesture\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe { &zwp_pointer_gestures_v1_requests_get_pinch_gesture_types as *const _ },
        },
        wl_message {
            name: b"release\0" as *const u8 as *const c_char,
            signature: b"2\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_pointer_gestures_v1_interface: wl_interface = wl_interface {
        name: b"zwp_pointer_gestures_v1\0" as *const u8 as *const c_char,
        version: 2,
        request_count: 3,
        requests: unsafe { &zwp_pointer_gestures_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "a swipe gesture object\n\nA swipe gesture object notifies a client about a multi-finger swipe\ngesture detected on an indirect input device such as a touchpad.\nThe gesture is usually initiated by multiple fingers moving in the\nsame direction but once initiated the direction may change.\nThe precise conditions of when such a gesture is detected are\nimplementation-dependent.\n\nA gesture consists of three stages: begin, update (optional) and end.\nThere cannot be multiple simultaneous pinch or swipe gestures on a\nsame pointer/seat, how compositors prevent these situations is\nimplementation-dependent.\n\nA gesture may be cancelled by the compositor or the hardware.\nClients should not consider performing permanent or irreversible\nactions until the end of a gesture has been received."]
pub mod zwp_pointer_gesture_swipe_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy the pointer swipe gesture object\n\n\n\nThis is a destructor, once sent this object cannot be used any longer."]
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
        #[doc = "multi-finger swipe begin\n\nThis event is sent when a multi-finger swipe gesture is detected\non the device."]
        Begin {
            serial: u32,
            time: u32,
            surface: super::wl_surface::WlSurface,
            fingers: u32,
        },
        #[doc = "multi-finger swipe motion\n\nThis event is sent when a multi-finger swipe gesture changes the\nposition of the logical center.\n\nThe dx and dy coordinates are relative coordinates of the logical\ncenter of the gesture compared to the previous event."]
        Update { time: u32, dx: f64, dy: f64 },
        #[doc = "multi-finger swipe end\n\nThis event is sent when a multi-finger swipe gesture ceases to\nbe valid. This may happen when one or more fingers are lifted or\nthe gesture is cancelled.\n\nWhen a gesture is cancelled, the client should undo state changes\ncaused by this gesture. What causes a gesture to be cancelled is\nimplementation-dependent."]
        End {
            serial: u32,
            time: u32,
            cancelled: i32,
        },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "begin",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Object,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "update",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Fixed,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "end",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Int,
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
                Event::Begin { .. } => 0,
                Event::Update { .. } => 1,
                Event::End { .. } => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Begin { .. } => 1,
                Event::Update { .. } => 1,
                Event::End { .. } => 1,
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
                    Ok(Event::Begin {
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
                        fingers: {
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
                    Ok(Event::Update {
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        dx: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                        dy: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::End {
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
                        cancelled: {
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
                    Ok(Event::Begin {
                        serial: _args[0].u,
                        time: _args[1].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(
                            _args[2].o as *mut _,
                        )
                        .into(),
                        fingers: _args[3].u,
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Update {
                        time: _args[0].u,
                        dx: (_args[1].f as f64) / 256.,
                        dy: (_args[2].f as f64) / 256.,
                    })
                }
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::End {
                        serial: _args[0].u,
                        time: _args[1].u,
                        cancelled: _args[2].i,
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
    pub struct ZwpPointerGestureSwipeV1(Proxy<ZwpPointerGestureSwipeV1>);
    impl AsRef<Proxy<ZwpPointerGestureSwipeV1>> for ZwpPointerGestureSwipeV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpPointerGestureSwipeV1>> for ZwpPointerGestureSwipeV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpPointerGestureSwipeV1(value)
        }
    }
    impl From<ZwpPointerGestureSwipeV1> for Proxy<ZwpPointerGestureSwipeV1> {
        #[inline]
        fn from(value: ZwpPointerGestureSwipeV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwpPointerGestureSwipeV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_pointer_gesture_swipe_v1";
        const VERSION: u32 = 2;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_pointer_gesture_swipe_v1_interface }
        }
    }
    impl ZwpPointerGestureSwipeV1 {
        #[doc = "destroy the pointer swipe gesture object\n\n\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_BEGIN_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_UPDATE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_END_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_pointer_gesture_swipe_v1_requests: [wl_message; 1] = [wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    static mut zwp_pointer_gesture_swipe_v1_events_begin_types: [*const wl_interface; 4] = [
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_pointer_gesture_swipe_v1_events: [wl_message; 3] = [
        wl_message {
            name: b"begin\0" as *const u8 as *const c_char,
            signature: b"uuou\0" as *const u8 as *const c_char,
            types: unsafe { &zwp_pointer_gesture_swipe_v1_events_begin_types as *const _ },
        },
        wl_message {
            name: b"update\0" as *const u8 as *const c_char,
            signature: b"uff\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"end\0" as *const u8 as *const c_char,
            signature: b"uui\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_pointer_gesture_swipe_v1_interface: wl_interface = wl_interface {
        name: b"zwp_pointer_gesture_swipe_v1\0" as *const u8 as *const c_char,
        version: 2,
        request_count: 1,
        requests: unsafe { &zwp_pointer_gesture_swipe_v1_requests as *const _ },
        event_count: 3,
        events: unsafe { &zwp_pointer_gesture_swipe_v1_events as *const _ },
    };
}
#[doc = "a pinch gesture object\n\nA pinch gesture object notifies a client about a multi-finger pinch\ngesture detected on an indirect input device such as a touchpad.\nThe gesture is usually initiated by multiple fingers moving towards\neach other or away from each other, or by two or more fingers rotating\naround a logical center of gravity. The precise conditions of when\nsuch a gesture is detected are implementation-dependent.\n\nA gesture consists of three stages: begin, update (optional) and end.\nThere cannot be multiple simultaneous pinch or swipe gestures on a\nsame pointer/seat, how compositors prevent these situations is\nimplementation-dependent.\n\nA gesture may be cancelled by the compositor or the hardware.\nClients should not consider performing permanent or irreversible\nactions until the end of a gesture has been received."]
pub mod zwp_pointer_gesture_pinch_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy the pinch gesture object\n\n\n\nThis is a destructor, once sent this object cannot be used any longer."]
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
        #[doc = "multi-finger pinch begin\n\nThis event is sent when a multi-finger pinch gesture is detected\non the device."]
        Begin {
            serial: u32,
            time: u32,
            surface: super::wl_surface::WlSurface,
            fingers: u32,
        },
        #[doc = "multi-finger pinch motion\n\nThis event is sent when a multi-finger pinch gesture changes the\nposition of the logical center, the rotation or the relative scale.\n\nThe dx and dy coordinates are relative coordinates in the\nsurface coordinate space of the logical center of the gesture.\n\nThe scale factor is an absolute scale compared to the\npointer_gesture_pinch.begin event, e.g. a scale of 2 means the fingers\nare now twice as far apart as on pointer_gesture_pinch.begin.\n\nThe rotation is the relative angle in degrees clockwise compared to the previous\npointer_gesture_pinch.begin or pointer_gesture_pinch.update event."]
        Update {
            time: u32,
            dx: f64,
            dy: f64,
            scale: f64,
            rotation: f64,
        },
        #[doc = "multi-finger pinch end\n\nThis event is sent when a multi-finger pinch gesture ceases to\nbe valid. This may happen when one or more fingers are lifted or\nthe gesture is cancelled.\n\nWhen a gesture is cancelled, the client should undo state changes\ncaused by this gesture. What causes a gesture to be cancelled is\nimplementation-dependent."]
        End {
            serial: u32,
            time: u32,
            cancelled: i32,
        },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "begin",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Object,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "update",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Fixed,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "end",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Int,
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
                Event::Begin { .. } => 0,
                Event::Update { .. } => 1,
                Event::End { .. } => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Begin { .. } => 1,
                Event::Update { .. } => 1,
                Event::End { .. } => 1,
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
                    Ok(Event::Begin {
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
                        fingers: {
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
                    Ok(Event::Update {
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        dx: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                        dy: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                        scale: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                        rotation: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::End {
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
                        cancelled: {
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
                    Ok(Event::Begin {
                        serial: _args[0].u,
                        time: _args[1].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(
                            _args[2].o as *mut _,
                        )
                        .into(),
                        fingers: _args[3].u,
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 5);
                    Ok(Event::Update {
                        time: _args[0].u,
                        dx: (_args[1].f as f64) / 256.,
                        dy: (_args[2].f as f64) / 256.,
                        scale: (_args[3].f as f64) / 256.,
                        rotation: (_args[4].f as f64) / 256.,
                    })
                }
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::End {
                        serial: _args[0].u,
                        time: _args[1].u,
                        cancelled: _args[2].i,
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
    pub struct ZwpPointerGesturePinchV1(Proxy<ZwpPointerGesturePinchV1>);
    impl AsRef<Proxy<ZwpPointerGesturePinchV1>> for ZwpPointerGesturePinchV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpPointerGesturePinchV1>> for ZwpPointerGesturePinchV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpPointerGesturePinchV1(value)
        }
    }
    impl From<ZwpPointerGesturePinchV1> for Proxy<ZwpPointerGesturePinchV1> {
        #[inline]
        fn from(value: ZwpPointerGesturePinchV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwpPointerGesturePinchV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_pointer_gesture_pinch_v1";
        const VERSION: u32 = 2;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_pointer_gesture_pinch_v1_interface }
        }
    }
    impl ZwpPointerGesturePinchV1 {
        #[doc = "destroy the pinch gesture object\n\n\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_BEGIN_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_UPDATE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_END_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_pointer_gesture_pinch_v1_requests: [wl_message; 1] = [wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    static mut zwp_pointer_gesture_pinch_v1_events_begin_types: [*const wl_interface; 4] = [
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_pointer_gesture_pinch_v1_events: [wl_message; 3] = [
        wl_message {
            name: b"begin\0" as *const u8 as *const c_char,
            signature: b"uuou\0" as *const u8 as *const c_char,
            types: unsafe { &zwp_pointer_gesture_pinch_v1_events_begin_types as *const _ },
        },
        wl_message {
            name: b"update\0" as *const u8 as *const c_char,
            signature: b"uffff\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"end\0" as *const u8 as *const c_char,
            signature: b"uui\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_pointer_gesture_pinch_v1_interface: wl_interface = wl_interface {
        name: b"zwp_pointer_gesture_pinch_v1\0" as *const u8 as *const c_char,
        version: 2,
        request_count: 1,
        requests: unsafe { &zwp_pointer_gesture_pinch_v1_requests as *const _ },
        event_count: 3,
        events: unsafe { &zwp_pointer_gesture_pinch_v1_events as *const _ },
    };
}
