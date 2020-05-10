use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 1] =
    [NULLPTR as *const sys::common::wl_interface];
#[doc = "displays a single surface per output\n\nDisplays a single surface per output.\n\nThis interface provides a mechanism for a single client to display\nsimple full-screen surfaces.  While there technically may be multiple\nclients bound to this interface, only one of those clients should be\nshown at a time.\n\nTo present a surface, the client uses either the present_surface or\npresent_surface_for_mode requests.  Presenting a surface takes effect\non the next wl_surface.commit.  See the individual requests for\ndetails about scaling and mode switches.\n\nThe client can have at most one surface per output at any time.\nRequesting a surface to be presented on an output that already has a\nsurface replaces the previously presented surface.  Presenting a null\nsurface removes its content and effectively disables the output.\nExactly what happens when an output is \"disabled\" is\ncompositor-specific.  The same surface may be presented on multiple\noutputs simultaneously.\n\nOnce a surface is presented on an output, it stays on that output\nuntil either the client removes it or the compositor destroys the\noutput.  This way, the client can update the output's contents by\nsimply attaching a new buffer.\n\nWarning! The protocol described in this file is experimental and\nbackward incompatible changes may be made. Backward compatible changes\nmay be added together with the corresponding interface version bump.\nBackward incompatible changes are done by bumping the version number in\nthe protocol and interface names and resetting the interface version.\nOnce the protocol is to be declared stable, the 'z' prefix and the\nversion number in the protocol and interface names are removed and the\ninterface version number is reset."]
pub mod zwp_fullscreen_shell_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[doc = "capabilities advertised by the compositor\n\nVarious capabilities that can be advertised by the compositor.  They\nare advertised one-at-a-time when the wl_fullscreen_shell interface is\nbound.  See the wl_fullscreen_shell.capability event for more details.\n\nARBITRARY_MODES:\nThis is a hint to the client that indicates that the compositor is\ncapable of setting practically any mode on its outputs.  If this\ncapability is provided, wl_fullscreen_shell.present_surface_for_mode\nwill almost never fail and clients should feel free to set whatever\nmode they like.  If the compositor does not advertise this, it may\nstill support some modes that are not advertised through wl_global.mode\nbut it is less likely.\n\nCURSOR_PLANE:\nThis is a hint to the client that indicates that the compositor can\nhandle a cursor surface from the client without actually compositing.\nThis may be because of a hardware cursor plane or some other mechanism.\nIf the compositor does not advertise this capability then setting\nwl_pointer.cursor may degrade performance or be ignored entirely.  If\nCURSOR_PLANE is not advertised, it is recommended that the client draw\nits own cursor and set wl_pointer.cursor(NULL)."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Capability {
        #[doc = "compositor is capable of almost any output mode"]
        ArbitraryModes = 1,
        #[doc = "compositor has a separate cursor plane"]
        CursorPlane = 2,
    }
    impl Capability {
        pub fn from_raw(n: u32) -> Option<Capability> {
            match n {
                1 => Some(Capability::ArbitraryModes),
                2 => Some(Capability::CursorPlane),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[doc = "different method to set the surface fullscreen\n\nHints to indicate to the compositor how to deal with a conflict\nbetween the dimensions of the surface and the dimensions of the\noutput. The compositor is free to ignore this parameter."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum PresentMethod {
        #[doc = "no preference, apply default policy"]
        Default = 0,
        #[doc = "center the surface on the output"]
        Center = 1,
        #[doc = "scale the surface, preserving aspect ratio, to the largest size that will fit on the output"]
        Zoom = 2,
        #[doc = "scale the surface, preserving aspect ratio, to fully fill the output cropping if needed"]
        ZoomCrop = 3,
        #[doc = "scale the surface to the size of the output ignoring aspect ratio"]
        Stretch = 4,
    }
    impl PresentMethod {
        pub fn from_raw(n: u32) -> Option<PresentMethod> {
            match n {
                0 => Some(PresentMethod::Default),
                1 => Some(PresentMethod::Center),
                2 => Some(PresentMethod::Zoom),
                3 => Some(PresentMethod::ZoomCrop),
                4 => Some(PresentMethod::Stretch),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[doc = "wl_fullscreen_shell error values\n\nThese errors can be emitted in response to wl_fullscreen_shell requests."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Error {
        #[doc = "present_method is not known"]
        InvalidMethod = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidMethod),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "release the wl_fullscreen_shell interface\n\nRelease the binding from the wl_fullscreen_shell interface.\n\nThis destroys the server-side object and frees this binding.  If\nthe client binds to wl_fullscreen_shell multiple times, it may wish\nto free some of those bindings.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Release,
        #[doc = "present surface for display\n\nPresent a surface on the given output.\n\nIf the output is null, the compositor will present the surface on\nwhatever display (or displays) it thinks best.  In particular, this\nmay replace any or all surfaces currently presented so it should\nnot be used in combination with placing surfaces on specific\noutputs.\n\nThe method parameter is a hint to the compositor for how the surface\nis to be presented.  In particular, it tells the compositor how to\nhandle a size mismatch between the presented surface and the\noutput.  The compositor is free to ignore this parameter.\n\nThe \"zoom\", \"zoom_crop\", and \"stretch\" methods imply a scaling\noperation on the surface.  This will override any kind of output\nscaling, so the buffer_scale property of the surface is effectively\nignored."]
        PresentSurface {
            surface: Option<super::wl_surface::WlSurface>,
            method: u32,
            output: Option<super::wl_output::WlOutput>,
        },
        #[doc = "present surface for display at a particular mode\n\nPresents a surface on the given output for a particular mode.\n\nIf the current size of the output differs from that of the surface,\nthe compositor will attempt to change the size of the output to\nmatch the surface.  The result of the mode-switch operation will be\nreturned via the provided wl_fullscreen_shell_mode_feedback object.\n\nIf the current output mode matches the one requested or if the\ncompositor successfully switches the mode to match the surface,\nthen the mode_successful event will be sent and the output will\ncontain the contents of the given surface.  If the compositor\ncannot match the output size to the surface size, the mode_failed\nwill be sent and the output will contain the contents of the\npreviously presented surface (if any).  If another surface is\npresented on the given output before either of these has a chance\nto happen, the present_cancelled event will be sent.\n\nDue to race conditions and other issues unknown to the client, no\nmode-switch operation is guaranteed to succeed.  However, if the\nmode is one advertised by wl_output.mode or if the compositor\nadvertises the ARBITRARY_MODES capability, then the client should\nexpect that the mode-switch operation will usually succeed.\n\nIf the size of the presented surface changes, the resulting output\nis undefined.  The compositor may attempt to change the output mode\nto compensate.  However, there is no guarantee that a suitable mode\nwill be found and the client has no way to be notified of success\nor failure.\n\nThe framerate parameter specifies the desired framerate for the\noutput in mHz.  The compositor is free to ignore this parameter.  A\nvalue of 0 indicates that the client has no preference.\n\nIf the value of wl_output.scale differs from wl_surface.buffer_scale,\nthen the compositor may choose a mode that matches either the buffer\nsize or the surface size.  In either case, the surface will fill the\noutput."]
        PresentSurfaceForMode {
            surface: super::wl_surface::WlSurface,
            output: super::wl_output::WlOutput,
            framerate: i32,
        },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "release",
                since: 1,
                signature: &[],
                destructor: true,
            },
            super::MessageDesc {
                name: "present_surface",
                since: 1,
                signature: &[
                    super::ArgumentType::Object,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Object,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "present_surface_for_mode",
                since: 1,
                signature: &[
                    super::ArgumentType::Object,
                    super::ArgumentType::Object,
                    super::ArgumentType::Int,
                    super::ArgumentType::NewId,
                ],
                destructor: false,
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
                Request::Release => 0,
                Request::PresentSurface { .. } => 1,
                Request::PresentSurfaceForMode { .. } => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Release => 1,
                Request::PresentSurface { .. } => 1,
                Request::PresentSurfaceForMode { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                2 => Some(Object::from_interface::<
                    super::zwp_fullscreen_shell_mode_feedback_v1::ZwpFullscreenShellModeFeedbackV1,
                >(version, meta.child())),
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
                Request::PresentSurface {
                    surface,
                    method,
                    output,
                } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![
                        Argument::Object(surface.map(|o| o.as_ref().id()).unwrap_or(0)),
                        Argument::Uint(method),
                        Argument::Object(output.map(|o| o.as_ref().id()).unwrap_or(0)),
                    ],
                },
                Request::PresentSurfaceForMode {
                    surface,
                    output,
                    framerate,
                } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![
                        Argument::Object(surface.as_ref().id()),
                        Argument::Object(output.as_ref().id()),
                        Argument::Int(framerate),
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
                Request::Release => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
                Request::PresentSurface {
                    surface,
                    method,
                    output,
                } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = surface
                        .map(|o| o.as_ref().c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    _args_array[1].u = method;
                    _args_array[2].o = output
                        .map(|o| o.as_ref().c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    f(1, &mut _args_array)
                }
                Request::PresentSurfaceForMode {
                    surface,
                    output,
                    framerate,
                } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = surface.as_ref().c_ptr() as *mut _;
                    _args_array[1].o = output.as_ref().c_ptr() as *mut _;
                    _args_array[2].i = framerate;
                    _args_array[3].o = ::std::ptr::null_mut() as *mut _;
                    f(2, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "advertises a capability of the compositor\n\nAdvertises a single capability of the compositor.\n\nWhen the wl_fullscreen_shell interface is bound, this event is emitted\nonce for each capability advertised.  Valid capabilities are given by\nthe wl_fullscreen_shell.capability enum.  If clients want to take\nadvantage of any of these capabilities, they should use a\nwl_display.sync request immediately after binding to ensure that they\nreceive all the capability events."]
        Capability { capability: u32 },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "capability",
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
                Event::Capability { .. } => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Capability { .. } => 1,
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
                    Ok(Event::Capability {
                        capability: {
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
                    Ok(Event::Capability {
                        capability: _args[0].u,
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
    pub struct ZwpFullscreenShellV1(Proxy<ZwpFullscreenShellV1>);
    impl AsRef<Proxy<ZwpFullscreenShellV1>> for ZwpFullscreenShellV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpFullscreenShellV1>> for ZwpFullscreenShellV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpFullscreenShellV1(value)
        }
    }
    impl From<ZwpFullscreenShellV1> for Proxy<ZwpFullscreenShellV1> {
        #[inline]
        fn from(value: ZwpFullscreenShellV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwpFullscreenShellV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_fullscreen_shell_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_fullscreen_shell_v1_interface }
        }
    }
    impl ZwpFullscreenShellV1 {
        #[doc = "release the wl_fullscreen_shell interface\n\nRelease the binding from the wl_fullscreen_shell interface.\n\nThis destroys the server-side object and frees this binding.  If\nthe client binds to wl_fullscreen_shell multiple times, it may wish\nto free some of those bindings.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn release(&self) -> () {
            let msg = Request::Release;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "present surface for display\n\nPresent a surface on the given output.\n\nIf the output is null, the compositor will present the surface on\nwhatever display (or displays) it thinks best.  In particular, this\nmay replace any or all surfaces currently presented so it should\nnot be used in combination with placing surfaces on specific\noutputs.\n\nThe method parameter is a hint to the compositor for how the surface\nis to be presented.  In particular, it tells the compositor how to\nhandle a size mismatch between the presented surface and the\noutput.  The compositor is free to ignore this parameter.\n\nThe \"zoom\", \"zoom_crop\", and \"stretch\" methods imply a scaling\noperation on the surface.  This will override any kind of output\nscaling, so the buffer_scale property of the surface is effectively\nignored."]
        pub fn present_surface(
            &self,
            surface: Option<&super::wl_surface::WlSurface>,
            method: u32,
            output: Option<&super::wl_output::WlOutput>,
        ) -> () {
            let msg = Request::PresentSurface {
                surface: surface.map(|o| o.clone()),
                method: method,
                output: output.map(|o| o.clone()),
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "present surface for display at a particular mode\n\nPresents a surface on the given output for a particular mode.\n\nIf the current size of the output differs from that of the surface,\nthe compositor will attempt to change the size of the output to\nmatch the surface.  The result of the mode-switch operation will be\nreturned via the provided wl_fullscreen_shell_mode_feedback object.\n\nIf the current output mode matches the one requested or if the\ncompositor successfully switches the mode to match the surface,\nthen the mode_successful event will be sent and the output will\ncontain the contents of the given surface.  If the compositor\ncannot match the output size to the surface size, the mode_failed\nwill be sent and the output will contain the contents of the\npreviously presented surface (if any).  If another surface is\npresented on the given output before either of these has a chance\nto happen, the present_cancelled event will be sent.\n\nDue to race conditions and other issues unknown to the client, no\nmode-switch operation is guaranteed to succeed.  However, if the\nmode is one advertised by wl_output.mode or if the compositor\nadvertises the ARBITRARY_MODES capability, then the client should\nexpect that the mode-switch operation will usually succeed.\n\nIf the size of the presented surface changes, the resulting output\nis undefined.  The compositor may attempt to change the output mode\nto compensate.  However, there is no guarantee that a suitable mode\nwill be found and the client has no way to be notified of success\nor failure.\n\nThe framerate parameter specifies the desired framerate for the\noutput in mHz.  The compositor is free to ignore this parameter.  A\nvalue of 0 indicates that the client has no preference.\n\nIf the value of wl_output.scale differs from wl_surface.buffer_scale,\nthen the compositor may choose a mode that matches either the buffer\nsize or the surface size.  In either case, the surface will fill the\noutput."]
        pub fn present_surface_for_mode(
            &self,
            surface: &super::wl_surface::WlSurface,
            output: &super::wl_output::WlOutput,
            framerate: i32,
        ) -> Main<super::zwp_fullscreen_shell_mode_feedback_v1::ZwpFullscreenShellModeFeedbackV1>
        {
            let msg = Request::PresentSurfaceForMode {
                surface: surface.clone(),
                output: output.clone(),
                framerate: framerate,
            };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_RELEASE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_PRESENT_SURFACE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_PRESENT_SURFACE_FOR_MODE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CAPABILITY_SINCE: u32 = 1u32;
    static mut zwp_fullscreen_shell_v1_requests_present_surface_types: [*const wl_interface; 3] = [
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_output::wl_output_interface as *const wl_interface },
    ];
    static mut zwp_fullscreen_shell_v1_requests_present_surface_for_mode_types:
        [*const wl_interface; 4] = [
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        unsafe { &super::wl_output::wl_output_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        unsafe {
            & super :: zwp_fullscreen_shell_mode_feedback_v1 :: zwp_fullscreen_shell_mode_feedback_v1_interface as * const wl_interface
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_fullscreen_shell_v1_requests: [wl_message; 3] = [
        wl_message {
            name: b"release\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"present_surface\0" as *const u8 as *const c_char,
            signature: b"?ou?o\0" as *const u8 as *const c_char,
            types: unsafe { &zwp_fullscreen_shell_v1_requests_present_surface_types as *const _ },
        },
        wl_message {
            name: b"present_surface_for_mode\0" as *const u8 as *const c_char,
            signature: b"ooin\0" as *const u8 as *const c_char,
            types: unsafe {
                &zwp_fullscreen_shell_v1_requests_present_surface_for_mode_types as *const _
            },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_fullscreen_shell_v1_events: [wl_message; 1] = [wl_message {
        name: b"capability\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_fullscreen_shell_v1_interface: wl_interface = wl_interface {
        name: b"zwp_fullscreen_shell_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 3,
        requests: unsafe { &zwp_fullscreen_shell_v1_requests as *const _ },
        event_count: 1,
        events: unsafe { &zwp_fullscreen_shell_v1_events as *const _ },
    };
}
pub mod zwp_fullscreen_shell_mode_feedback_v1 {
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
        #[doc = "mode switch succeeded\n\nThis event indicates that the attempted mode switch operation was\nsuccessful.  A surface of the size requested in the mode switch\nwill fill the output without scaling.\n\nUpon receiving this event, the client should destroy the\nwl_fullscreen_shell_mode_feedback object."]
        ModeSuccessful,
        #[doc = "mode switch failed\n\nThis event indicates that the attempted mode switch operation\nfailed.  This may be because the requested output mode is not\npossible or it may mean that the compositor does not want to allow it.\n\nUpon receiving this event, the client should destroy the\nwl_fullscreen_shell_mode_feedback object."]
        ModeFailed,
        #[doc = "mode switch cancelled\n\nThis event indicates that the attempted mode switch operation was\ncancelled.  Most likely this is because the client requested a\nsecond mode switch before the first one completed.\n\nUpon receiving this event, the client should destroy the\nwl_fullscreen_shell_mode_feedback object."]
        PresentCancelled,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "mode_successful",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "mode_failed",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "present_cancelled",
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
                Event::ModeSuccessful => 0,
                Event::ModeFailed => 1,
                Event::PresentCancelled => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::ModeSuccessful => 1,
                Event::ModeFailed => 1,
                Event::PresentCancelled => 1,
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
                0 => Ok(Event::ModeSuccessful),
                1 => Ok(Event::ModeFailed),
                2 => Ok(Event::PresentCancelled),
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
                0 => Ok(Event::ModeSuccessful),
                1 => Ok(Event::ModeFailed),
                2 => Ok(Event::PresentCancelled),
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
    pub struct ZwpFullscreenShellModeFeedbackV1(Proxy<ZwpFullscreenShellModeFeedbackV1>);
    impl AsRef<Proxy<ZwpFullscreenShellModeFeedbackV1>> for ZwpFullscreenShellModeFeedbackV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpFullscreenShellModeFeedbackV1>> for ZwpFullscreenShellModeFeedbackV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpFullscreenShellModeFeedbackV1(value)
        }
    }
    impl From<ZwpFullscreenShellModeFeedbackV1> for Proxy<ZwpFullscreenShellModeFeedbackV1> {
        #[inline]
        fn from(value: ZwpFullscreenShellModeFeedbackV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwpFullscreenShellModeFeedbackV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_fullscreen_shell_mode_feedback_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_fullscreen_shell_mode_feedback_v1_interface }
        }
    }
    impl ZwpFullscreenShellModeFeedbackV1 {}
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_MODE_SUCCESSFUL_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_MODE_FAILED_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_PRESENT_CANCELLED_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_fullscreen_shell_mode_feedback_v1_events: [wl_message; 3] = [
        wl_message {
            name: b"mode_successful\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"mode_failed\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"present_cancelled\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_fullscreen_shell_mode_feedback_v1_interface: wl_interface = wl_interface {
        name: b"zwp_fullscreen_shell_mode_feedback_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 0,
        requests: NULLPTR as *const wl_message,
        event_count: 3,
        events: unsafe { &zwp_fullscreen_shell_mode_feedback_v1_events as *const _ },
    };
}
