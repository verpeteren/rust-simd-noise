use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 1] =
    [NULLPTR as *const sys::common::wl_interface];
#[doc = "protocol for providing explicit synchronization\n\nThis global is a factory interface, allowing clients to request\nexplicit synchronization for buffers on a per-surface basis.\n\nSee zwp_linux_surface_synchronization_v1 for more information.\n\nThis interface is derived from Chromium's\nzcr_linux_explicit_synchronization_v1.\n\nWarning! The protocol described in this file is experimental and\nbackward incompatible changes may be made. Backward compatible changes\nmay be added together with the corresponding interface version bump.\nBackward incompatible changes are done by bumping the version number in\nthe protocol and interface names and resetting the interface version.\nOnce the protocol is to be declared stable, the 'z' prefix and the\nversion number in the protocol and interface names are removed and the\ninterface version number is reset."]
pub mod zwp_linux_explicit_synchronization_v1 {
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
        #[doc = "the surface already has a synchronization object associated"]
        SynchronizationExists = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::SynchronizationExists),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy explicit synchronization factory object\n\nDestroy this explicit synchronization factory object. Other objects,\nincluding zwp_linux_surface_synchronization_v1 objects created by this\nfactory, shall not be affected by this request.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "extend surface interface for explicit synchronization\n\nInstantiate an interface extension for the given wl_surface to provide\nexplicit synchronization.\n\nIf the given wl_surface already has an explicit synchronization object\nassociated, the synchronization_exists protocol error is raised.\n\nGraphics APIs, like EGL or Vulkan, that manage the buffer queue and\ncommits of a wl_surface themselves, are likely to be using this\nextension internally. If a client is using such an API for a\nwl_surface, it should not directly use this extension on that surface,\nto avoid raising a synchronization_exists protocol error."]
        GetSynchronization {
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
                name: "get_synchronization",
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
                Request::GetSynchronization { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::GetSynchronization { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zwp_linux_surface_synchronization_v1::ZwpLinuxSurfaceSynchronizationV1,
                >(version, meta.child())),
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
                Request::GetSynchronization { surface } => Message {
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
                Request::GetSynchronization { surface } => {
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
    pub struct ZwpLinuxExplicitSynchronizationV1(Proxy<ZwpLinuxExplicitSynchronizationV1>);
    impl AsRef<Proxy<ZwpLinuxExplicitSynchronizationV1>> for ZwpLinuxExplicitSynchronizationV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpLinuxExplicitSynchronizationV1>> for ZwpLinuxExplicitSynchronizationV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpLinuxExplicitSynchronizationV1(value)
        }
    }
    impl From<ZwpLinuxExplicitSynchronizationV1> for Proxy<ZwpLinuxExplicitSynchronizationV1> {
        #[inline]
        fn from(value: ZwpLinuxExplicitSynchronizationV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwpLinuxExplicitSynchronizationV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_linux_explicit_synchronization_v1";
        const VERSION: u32 = 2;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_linux_explicit_synchronization_v1_interface }
        }
    }
    impl ZwpLinuxExplicitSynchronizationV1 {
        #[doc = "destroy explicit synchronization factory object\n\nDestroy this explicit synchronization factory object. Other objects,\nincluding zwp_linux_surface_synchronization_v1 objects created by this\nfactory, shall not be affected by this request.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "extend surface interface for explicit synchronization\n\nInstantiate an interface extension for the given wl_surface to provide\nexplicit synchronization.\n\nIf the given wl_surface already has an explicit synchronization object\nassociated, the synchronization_exists protocol error is raised.\n\nGraphics APIs, like EGL or Vulkan, that manage the buffer queue and\ncommits of a wl_surface themselves, are likely to be using this\nextension internally. If a client is using such an API for a\nwl_surface, it should not directly use this extension on that surface,\nto avoid raising a synchronization_exists protocol error."]
        pub fn get_synchronization(
            &self,
            surface: &super::wl_surface::WlSurface,
        ) -> Main<super::zwp_linux_surface_synchronization_v1::ZwpLinuxSurfaceSynchronizationV1>
        {
            let msg = Request::GetSynchronization {
                surface: surface.clone(),
            };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_SYNCHRONIZATION_SINCE: u32 = 1u32;
    static mut zwp_linux_explicit_synchronization_v1_requests_get_synchronization_types:
        [*const wl_interface; 2] = [
        unsafe {
            & super :: zwp_linux_surface_synchronization_v1 :: zwp_linux_surface_synchronization_v1_interface as * const wl_interface
        },
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_linux_explicit_synchronization_v1_requests: [wl_message; 2] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"get_synchronization\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe {
                &zwp_linux_explicit_synchronization_v1_requests_get_synchronization_types
                    as *const _
            },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_linux_explicit_synchronization_v1_interface: wl_interface = wl_interface {
        name: b"zwp_linux_explicit_synchronization_v1\0" as *const u8 as *const c_char,
        version: 2,
        request_count: 2,
        requests: unsafe { &zwp_linux_explicit_synchronization_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "per-surface explicit synchronization support\n\nThis object implements per-surface explicit synchronization.\n\nSynchronization refers to co-ordination of pipelined operations performed\non buffers. Most GPU clients will schedule an asynchronous operation to\nrender to the buffer, then immediately send the buffer to the compositor\nto be attached to a surface.\n\nIn implicit synchronization, ensuring that the rendering operation is\ncomplete before the compositor displays the buffer is an implementation\ndetail handled by either the kernel or userspace graphics driver.\n\nBy contrast, in explicit synchronization, dma_fence objects mark when the\nasynchronous operations are complete. When submitting a buffer, the\nclient provides an acquire fence which will be waited on before the\ncompositor accesses the buffer. The Wayland server, through a\nzwp_linux_buffer_release_v1 object, will inform the client with an event\nwhich may be accompanied by a release fence, when the compositor will no\nlonger access the buffer contents due to the specific commit that\nrequested the release event.\n\nEach surface can be associated with only one object of this interface at\nany time.\n\nIn version 1 of this interface, explicit synchronization is only\nguaranteed to be supported for buffers created with any version of the\nwp_linux_dmabuf buffer factory. Version 2 additionally guarantees\nexplicit synchronization support for opaque EGL buffers, which is a type\nof platform specific buffers described in the EGL_WL_bind_wayland_display\nextension. Compositors are free to support explicit synchronization for\nadditional buffer types."]
pub mod zwp_linux_surface_synchronization_v1 {
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
        #[doc = "the fence specified by the client could not be imported"]
        InvalidFence = 0,
        #[doc = "multiple fences added for a single surface commit"]
        DuplicateFence = 1,
        #[doc = "multiple releases added for a single surface commit"]
        DuplicateRelease = 2,
        #[doc = "the associated wl_surface was destroyed"]
        NoSurface = 3,
        #[doc = "the buffer does not support explicit synchronization"]
        UnsupportedBuffer = 4,
        #[doc = "no buffer was attached"]
        NoBuffer = 5,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidFence),
                1 => Some(Error::DuplicateFence),
                2 => Some(Error::DuplicateRelease),
                3 => Some(Error::NoSurface),
                4 => Some(Error::UnsupportedBuffer),
                5 => Some(Error::NoBuffer),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy synchronization object\n\nDestroy this explicit synchronization object.\n\nAny fence set by this object with set_acquire_fence since the last\ncommit will be discarded by the server. Any fences set by this object\nbefore the last commit are not affected.\n\nzwp_linux_buffer_release_v1 objects created by this object are not\naffected by this request.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "set the acquire fence\n\nSet the acquire fence that must be signaled before the compositor\nmay sample from the buffer attached with wl_surface.attach. The fence\nis a dma_fence kernel object.\n\nThe acquire fence is double-buffered state, and will be applied on the\nnext wl_surface.commit request for the associated surface. Thus, it\napplies only to the buffer that is attached to the surface at commit\ntime.\n\nIf the provided fd is not a valid dma_fence fd, then an INVALID_FENCE\nerror is raised.\n\nIf a fence has already been attached during the same commit cycle, a\nDUPLICATE_FENCE error is raised.\n\nIf the associated wl_surface was destroyed, a NO_SURFACE error is\nraised.\n\nIf at surface commit time the attached buffer does not support explicit\nsynchronization, an UNSUPPORTED_BUFFER error is raised.\n\nIf at surface commit time there is no buffer attached, a NO_BUFFER\nerror is raised."]
        SetAcquireFence { fd: ::std::os::unix::io::RawFd },
        #[doc = "release fence for last-attached buffer\n\nCreate a listener for the release of the buffer attached by the\nclient with wl_surface.attach. See zwp_linux_buffer_release_v1\ndocumentation for more information.\n\nThe release object is double-buffered state, and will be associated\nwith the buffer that is attached to the surface at wl_surface.commit\ntime.\n\nIf a zwp_linux_buffer_release_v1 object has already been requested for\nthe surface in the same commit cycle, a DUPLICATE_RELEASE error is\nraised.\n\nIf the associated wl_surface was destroyed, a NO_SURFACE error\nis raised.\n\nIf at surface commit time there is no buffer attached, a NO_BUFFER\nerror is raised."]
        GetRelease {},
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
                name: "set_acquire_fence",
                since: 1,
                signature: &[super::ArgumentType::Fd],
                destructor: false,
            },
            super::MessageDesc {
                name: "get_release",
                since: 1,
                signature: &[super::ArgumentType::NewId],
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
                Request::SetAcquireFence { .. } => 1,
                Request::GetRelease { .. } => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::SetAcquireFence { .. } => 1,
                Request::GetRelease { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                2 => Some(Object::from_interface::<
                    super::zwp_linux_buffer_release_v1::ZwpLinuxBufferReleaseV1,
                >(version, meta.child())),
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
                Request::SetAcquireFence { fd } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::Fd(fd),],
                },
                Request::GetRelease {} => Message {
                    sender_id: sender_id,
                    opcode: 2,
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
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
                Request::SetAcquireFence { fd } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].h = fd;
                    f(1, &mut _args_array)
                }
                Request::GetRelease {} => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
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
    pub struct ZwpLinuxSurfaceSynchronizationV1(Proxy<ZwpLinuxSurfaceSynchronizationV1>);
    impl AsRef<Proxy<ZwpLinuxSurfaceSynchronizationV1>> for ZwpLinuxSurfaceSynchronizationV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpLinuxSurfaceSynchronizationV1>> for ZwpLinuxSurfaceSynchronizationV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpLinuxSurfaceSynchronizationV1(value)
        }
    }
    impl From<ZwpLinuxSurfaceSynchronizationV1> for Proxy<ZwpLinuxSurfaceSynchronizationV1> {
        #[inline]
        fn from(value: ZwpLinuxSurfaceSynchronizationV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwpLinuxSurfaceSynchronizationV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_linux_surface_synchronization_v1";
        const VERSION: u32 = 2;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_linux_surface_synchronization_v1_interface }
        }
    }
    impl ZwpLinuxSurfaceSynchronizationV1 {
        #[doc = "destroy synchronization object\n\nDestroy this explicit synchronization object.\n\nAny fence set by this object with set_acquire_fence since the last\ncommit will be discarded by the server. Any fences set by this object\nbefore the last commit are not affected.\n\nzwp_linux_buffer_release_v1 objects created by this object are not\naffected by this request.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the acquire fence\n\nSet the acquire fence that must be signaled before the compositor\nmay sample from the buffer attached with wl_surface.attach. The fence\nis a dma_fence kernel object.\n\nThe acquire fence is double-buffered state, and will be applied on the\nnext wl_surface.commit request for the associated surface. Thus, it\napplies only to the buffer that is attached to the surface at commit\ntime.\n\nIf the provided fd is not a valid dma_fence fd, then an INVALID_FENCE\nerror is raised.\n\nIf a fence has already been attached during the same commit cycle, a\nDUPLICATE_FENCE error is raised.\n\nIf the associated wl_surface was destroyed, a NO_SURFACE error is\nraised.\n\nIf at surface commit time the attached buffer does not support explicit\nsynchronization, an UNSUPPORTED_BUFFER error is raised.\n\nIf at surface commit time there is no buffer attached, a NO_BUFFER\nerror is raised."]
        pub fn set_acquire_fence(&self, fd: ::std::os::unix::io::RawFd) -> () {
            let msg = Request::SetAcquireFence { fd: fd };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "release fence for last-attached buffer\n\nCreate a listener for the release of the buffer attached by the\nclient with wl_surface.attach. See zwp_linux_buffer_release_v1\ndocumentation for more information.\n\nThe release object is double-buffered state, and will be associated\nwith the buffer that is attached to the surface at wl_surface.commit\ntime.\n\nIf a zwp_linux_buffer_release_v1 object has already been requested for\nthe surface in the same commit cycle, a DUPLICATE_RELEASE error is\nraised.\n\nIf the associated wl_surface was destroyed, a NO_SURFACE error\nis raised.\n\nIf at surface commit time there is no buffer attached, a NO_BUFFER\nerror is raised."]
        pub fn get_release(
            &self,
        ) -> Main<super::zwp_linux_buffer_release_v1::ZwpLinuxBufferReleaseV1> {
            let msg = Request::GetRelease {};
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_ACQUIRE_FENCE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_RELEASE_SINCE: u32 = 1u32;
    static mut zwp_linux_surface_synchronization_v1_requests_get_release_types:
        [*const wl_interface; 1] = [unsafe {
        &super::zwp_linux_buffer_release_v1::zwp_linux_buffer_release_v1_interface
            as *const wl_interface
    }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_linux_surface_synchronization_v1_requests: [wl_message; 3] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_acquire_fence\0" as *const u8 as *const c_char,
            signature: b"h\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"get_release\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe {
                &zwp_linux_surface_synchronization_v1_requests_get_release_types as *const _
            },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_linux_surface_synchronization_v1_interface: wl_interface = wl_interface {
        name: b"zwp_linux_surface_synchronization_v1\0" as *const u8 as *const c_char,
        version: 2,
        request_count: 3,
        requests: unsafe { &zwp_linux_surface_synchronization_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "buffer release explicit synchronization\n\nThis object is instantiated in response to a\nzwp_linux_surface_synchronization_v1.get_release request.\n\nIt provides an alternative to wl_buffer.release events, providing a\nunique release from a single wl_surface.commit request. The release event\nalso supports explicit synchronization, providing a fence FD for the\nclient to synchronize against.\n\nExactly one event, either a fenced_release or an immediate_release, will\nbe emitted for the wl_surface.commit request. The compositor can choose\nrelease by release which event it uses.\n\nThis event does not replace wl_buffer.release events; servers are still\nrequired to send those events.\n\nOnce a buffer release object has delivered a 'fenced_release' or an\n'immediate_release' event it is automatically destroyed."]
pub mod zwp_linux_buffer_release_v1 {
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
        #[doc = "release buffer with fence\n\nSent when the compositor has finalised its usage of the associated\nbuffer for the relevant commit, providing a dma_fence which will be\nsignaled when all operations by the compositor on that buffer for that\ncommit have finished.\n\nOnce the fence has signaled, and assuming the associated buffer is not\npending release from other wl_surface.commit requests, no additional\nexplicit or implicit synchronization is required to safely reuse or\ndestroy the buffer.\n\nThis event destroys the zwp_linux_buffer_release_v1 object.\n\nThis is a destructor, once received this object cannot be used any longer."]
        FencedRelease { fence: ::std::os::unix::io::RawFd },
        #[doc = "release buffer immediately\n\nSent when the compositor has finalised its usage of the associated\nbuffer for the relevant commit, and either performed no operations\nusing it, or has a guarantee that all its operations on that buffer for\nthat commit have finished.\n\nOnce this event is received, and assuming the associated buffer is not\npending release from other wl_surface.commit requests, no additional\nexplicit or implicit synchronization is required to safely reuse or\ndestroy the buffer.\n\nThis event destroys the zwp_linux_buffer_release_v1 object.\n\nThis is a destructor, once received this object cannot be used any longer."]
        ImmediateRelease,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "fenced_release",
                since: 1,
                signature: &[super::ArgumentType::Fd],
                destructor: true,
            },
            super::MessageDesc {
                name: "immediate_release",
                since: 1,
                signature: &[],
                destructor: true,
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Event::FencedRelease { .. } => true,
                Event::ImmediateRelease => true,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::FencedRelease { .. } => 0,
                Event::ImmediateRelease => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::FencedRelease { .. } => 1,
                Event::ImmediateRelease => 1,
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
                    Ok(Event::FencedRelease {
                        fence: {
                            if let Some(Argument::Fd(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => Ok(Event::ImmediateRelease),
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
                    Ok(Event::FencedRelease { fence: _args[0].h })
                }
                1 => Ok(Event::ImmediateRelease),
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
    pub struct ZwpLinuxBufferReleaseV1(Proxy<ZwpLinuxBufferReleaseV1>);
    impl AsRef<Proxy<ZwpLinuxBufferReleaseV1>> for ZwpLinuxBufferReleaseV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpLinuxBufferReleaseV1>> for ZwpLinuxBufferReleaseV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpLinuxBufferReleaseV1(value)
        }
    }
    impl From<ZwpLinuxBufferReleaseV1> for Proxy<ZwpLinuxBufferReleaseV1> {
        #[inline]
        fn from(value: ZwpLinuxBufferReleaseV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwpLinuxBufferReleaseV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_linux_buffer_release_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_linux_buffer_release_v1_interface }
        }
    }
    impl ZwpLinuxBufferReleaseV1 {}
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FENCED_RELEASE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_IMMEDIATE_RELEASE_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_linux_buffer_release_v1_events: [wl_message; 2] = [
        wl_message {
            name: b"fenced_release\0" as *const u8 as *const c_char,
            signature: b"h\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"immediate_release\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_linux_buffer_release_v1_interface: wl_interface = wl_interface {
        name: b"zwp_linux_buffer_release_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 0,
        requests: NULLPTR as *const wl_message,
        event_count: 2,
        events: unsafe { &zwp_linux_buffer_release_v1_events as *const _ },
    };
}
