use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 6] = [
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
];
#[doc = "factory for creating dmabuf-based wl_buffers\n\nFollowing the interfaces from:\nhttps://www.khronos.org/registry/egl/extensions/EXT/EGL_EXT_image_dma_buf_import.txt\nhttps://www.khronos.org/registry/EGL/extensions/EXT/EGL_EXT_image_dma_buf_import_modifiers.txt\nand the Linux DRM sub-system's AddFb2 ioctl.\n\nThis interface offers ways to create generic dmabuf-based\nwl_buffers. Immediately after a client binds to this interface,\nthe set of supported formats and format modifiers is sent with\n'format' and 'modifier' events.\n\nThe following are required from clients:\n\n- Clients must ensure that either all data in the dma-buf is\ncoherent for all subsequent read access or that coherency is\ncorrectly handled by the underlying kernel-side dma-buf\nimplementation.\n\n- Don't make any more attachments after sending the buffer to the\ncompositor. Making more attachments later increases the risk of\nthe compositor not being able to use (re-import) an existing\ndmabuf-based wl_buffer.\n\nThe underlying graphics stack must ensure the following:\n\n- The dmabuf file descriptors relayed to the server will stay valid\nfor the whole lifetime of the wl_buffer. This means the server may\nat any time use those fds to import the dmabuf into any kernel\nsub-system that might accept it.\n\nTo create a wl_buffer from one or more dmabufs, a client creates a\nzwp_linux_dmabuf_params_v1 object with a zwp_linux_dmabuf_v1.create_params\nrequest. All planes required by the intended format are added with\nthe 'add' request. Finally, a 'create' or 'create_immed' request is\nissued, which has the following outcome depending on the import success.\n\nThe 'create' request,\n- on success, triggers a 'created' event which provides the final\nwl_buffer to the client.\n- on failure, triggers a 'failed' event to convey that the server\ncannot use the dmabufs received from the client.\n\nFor the 'create_immed' request,\n- on success, the server immediately imports the added dmabufs to\ncreate a wl_buffer. No event is sent from the server in this case.\n- on failure, the server can choose to either:\n- terminate the client by raising a fatal error.\n- mark the wl_buffer as failed, and send a 'failed' event to the\nclient. If the client uses a failed wl_buffer as an argument to any\nrequest, the behaviour is compositor implementation-defined.\n\nWarning! The protocol described in this file is experimental and\nbackward incompatible changes may be made. Backward compatible changes\nmay be added together with the corresponding interface version bump.\nBackward incompatible changes are done by bumping the version number in\nthe protocol and interface names and resetting the interface version.\nOnce the protocol is to be declared stable, the 'z' prefix and the\nversion number in the protocol and interface names are removed and the\ninterface version number is reset."]
pub mod zwp_linux_dmabuf_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "unbind the factory\n\nObjects created through this interface, especially wl_buffers, will\nremain valid.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "create a temporary object for buffer parameters\n\nThis temporary object is used to collect multiple dmabuf handles into\na single batch to create a wl_buffer. It can only be used once and\nshould be destroyed after a 'created' or 'failed' event has been\nreceived."]
        CreateParams {},
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
                name: "create_params",
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
                Request::CreateParams { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::CreateParams { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1,
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
                Request::CreateParams {} => Message {
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
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
                Request::CreateParams {} => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    f(1, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "supported buffer format\n\nThis event advertises one buffer format that the server supports.\nAll the supported formats are advertised once when the client\nbinds to this interface. A roundtrip after binding guarantees\nthat the client has received all supported formats.\n\nFor the definition of the format codes, see the\nzwp_linux_buffer_params_v1::create request.\n\nWarning: the 'format' event is likely to be deprecated and replaced\nwith the 'modifier' event introduced in zwp_linux_dmabuf_v1\nversion 3, described below. Please refrain from using the information\nreceived from this event."]
        Format { format: u32 },
        #[doc = "supported buffer format modifier\n\nThis event advertises the formats that the server supports, along with\nthe modifiers supported for each format. All the supported modifiers\nfor all the supported formats are advertised once when the client\nbinds to this interface. A roundtrip after binding guarantees that\nthe client has received all supported format-modifier pairs.\n\nFor legacy support, DRM_FORMAT_MOD_INVALID (that is, modifier_hi ==\n0x00ffffff and modifier_lo == 0xffffffff) is allowed in this event.\nIt indicates that the server can support the format with an implicit\nmodifier. When a plane has DRM_FORMAT_MOD_INVALID as its modifier, it\nis as if no explicit modifier is specified. The effective modifier\nwill be derived from the dmabuf.\n\nFor the definition of the format and modifier codes, see the\nzwp_linux_buffer_params_v1::create and zwp_linux_buffer_params_v1::add\nrequests.\n\nOnly available since version 3 of the interface"]
        Modifier {
            format: u32,
            modifier_hi: u32,
            modifier_lo: u32,
        },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "format",
                since: 1,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "modifier",
                since: 3,
                signature: &[
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
                Event::Format { .. } => 0,
                Event::Modifier { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Format { .. } => 1,
                Event::Modifier { .. } => 3,
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
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Modifier {
                        format: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        modifier_hi: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        modifier_lo: {
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
                    Ok(Event::Format { format: _args[0].u })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Modifier {
                        format: _args[0].u,
                        modifier_hi: _args[1].u,
                        modifier_lo: _args[2].u,
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
    pub struct ZwpLinuxDmabufV1(Proxy<ZwpLinuxDmabufV1>);
    impl AsRef<Proxy<ZwpLinuxDmabufV1>> for ZwpLinuxDmabufV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpLinuxDmabufV1>> for ZwpLinuxDmabufV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpLinuxDmabufV1(value)
        }
    }
    impl From<ZwpLinuxDmabufV1> for Proxy<ZwpLinuxDmabufV1> {
        #[inline]
        fn from(value: ZwpLinuxDmabufV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwpLinuxDmabufV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_linux_dmabuf_v1";
        const VERSION: u32 = 3;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_linux_dmabuf_v1_interface }
        }
    }
    impl ZwpLinuxDmabufV1 {
        #[doc = "unbind the factory\n\nObjects created through this interface, especially wl_buffers, will\nremain valid.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "create a temporary object for buffer parameters\n\nThis temporary object is used to collect multiple dmabuf handles into\na single batch to create a wl_buffer. It can only be used once and\nshould be destroyed after a 'created' or 'failed' event has been\nreceived."]
        pub fn create_params(
            &self,
        ) -> Main<super::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1> {
            let msg = Request::CreateParams {};
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CREATE_PARAMS_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FORMAT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_MODIFIER_SINCE: u32 = 3u32;
    static mut zwp_linux_dmabuf_v1_requests_create_params_types: [*const wl_interface; 1] =
        [unsafe {
            &super::zwp_linux_buffer_params_v1::zwp_linux_buffer_params_v1_interface
                as *const wl_interface
        }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_linux_dmabuf_v1_requests: [wl_message; 2] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"create_params\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &zwp_linux_dmabuf_v1_requests_create_params_types as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_linux_dmabuf_v1_events: [wl_message; 2] = [
        wl_message {
            name: b"format\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"modifier\0" as *const u8 as *const c_char,
            signature: b"3uuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_linux_dmabuf_v1_interface: wl_interface = wl_interface {
        name: b"zwp_linux_dmabuf_v1\0" as *const u8 as *const c_char,
        version: 3,
        request_count: 2,
        requests: unsafe { &zwp_linux_dmabuf_v1_requests as *const _ },
        event_count: 2,
        events: unsafe { &zwp_linux_dmabuf_v1_events as *const _ },
    };
}
#[doc = "parameters for creating a dmabuf-based wl_buffer\n\nThis temporary object is a collection of dmabufs and other\nparameters that together form a single logical buffer. The temporary\nobject may eventually create one wl_buffer unless cancelled by\ndestroying it before requesting 'create'.\n\nSingle-planar formats only require one dmabuf, however\nmulti-planar formats may require more than one dmabuf. For all\nformats, an 'add' request must be called once per plane (even if the\nunderlying dmabuf fd is identical).\n\nYou must use consecutive plane indices ('plane_idx' argument for 'add')\nfrom zero to the number of planes used by the drm_fourcc format code.\nAll planes required by the format must be given exactly once, but can\nbe given in any order. Each plane index can be set only once."]
pub mod zwp_linux_buffer_params_v1 {
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
        #[doc = "the dmabuf_batch object has already been used to create a wl_buffer"]
        AlreadyUsed = 0,
        #[doc = "plane index out of bounds"]
        PlaneIdx = 1,
        #[doc = "the plane index was already set"]
        PlaneSet = 2,
        #[doc = "missing or too many planes to create a buffer"]
        Incomplete = 3,
        #[doc = "format not supported"]
        InvalidFormat = 4,
        #[doc = "invalid width or height"]
        InvalidDimensions = 5,
        #[doc = "offset + stride * height goes out of dmabuf bounds"]
        OutOfBounds = 6,
        #[doc = "invalid wl_buffer resulted from importing dmabufs via the create_immed request on given buffer_params"]
        InvalidWlBuffer = 7,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::AlreadyUsed),
                1 => Some(Error::PlaneIdx),
                2 => Some(Error::PlaneSet),
                3 => Some(Error::Incomplete),
                4 => Some(Error::InvalidFormat),
                5 => Some(Error::InvalidDimensions),
                6 => Some(Error::OutOfBounds),
                7 => Some(Error::InvalidWlBuffer),
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
    pub enum Flags {
        #[doc = "contents are y-inverted"]
        YInvert = 1,
        #[doc = "content is interlaced"]
        Interlaced = 2,
        #[doc = "bottom field first"]
        BottomFirst = 4,
    }
    impl Flags {
        pub fn from_raw(n: u32) -> Option<Flags> {
            match n {
                1 => Some(Flags::YInvert),
                2 => Some(Flags::Interlaced),
                4 => Some(Flags::BottomFirst),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "delete this object, used or not\n\nCleans up the temporary data sent to the server for dmabuf-based\nwl_buffer creation.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "add a dmabuf to the temporary set\n\nThis request adds one dmabuf to the set in this\nzwp_linux_buffer_params_v1.\n\nThe 64-bit unsigned value combined from modifier_hi and modifier_lo\nis the dmabuf layout modifier. DRM AddFB2 ioctl calls this the\nfb modifier, which is defined in drm_mode.h of Linux UAPI.\nThis is an opaque token. Drivers use this token to express tiling,\ncompression, etc. driver-specific modifications to the base format\ndefined by the DRM fourcc code.\n\nWarning: It should be an error if the format/modifier pair was not\nadvertised with the modifier event. This is not enforced yet because\nsome implementations always accept DRM_FORMAT_MOD_INVALID. Also\nversion 2 of this protocol does not have the modifier event.\n\nThis request raises the PLANE_IDX error if plane_idx is too large.\nThe error PLANE_SET is raised if attempting to set a plane that\nwas already set."]
        Add {
            fd: ::std::os::unix::io::RawFd,
            plane_idx: u32,
            offset: u32,
            stride: u32,
            modifier_hi: u32,
            modifier_lo: u32,
        },
        #[doc = "create a wl_buffer from the given dmabufs\n\nThis asks for creation of a wl_buffer from the added dmabuf\nbuffers. The wl_buffer is not created immediately but returned via\nthe 'created' event if the dmabuf sharing succeeds. The sharing\nmay fail at runtime for reasons a client cannot predict, in\nwhich case the 'failed' event is triggered.\n\nThe 'format' argument is a DRM_FORMAT code, as defined by the\nlibdrm's drm_fourcc.h. The Linux kernel's DRM sub-system is the\nauthoritative source on how the format codes should work.\n\nThe 'flags' is a bitfield of the flags defined in enum \"flags\".\n'y_invert' means the that the image needs to be y-flipped.\n\nFlag 'interlaced' means that the frame in the buffer is not\nprogressive as usual, but interlaced. An interlaced buffer as\nsupported here must always contain both top and bottom fields.\nThe top field always begins on the first pixel row. The temporal\nordering between the two fields is top field first, unless\n'bottom_first' is specified. It is undefined whether 'bottom_first'\nis ignored if 'interlaced' is not set.\n\nThis protocol does not convey any information about field rate,\nduration, or timing, other than the relative ordering between the\ntwo fields in one buffer. A compositor may have to estimate the\nintended field rate from the incoming buffer rate. It is undefined\nwhether the time of receiving wl_surface.commit with a new buffer\nattached, applying the wl_surface state, wl_surface.frame callback\ntrigger, presentation, or any other point in the compositor cycle\nis used to measure the frame or field times. There is no support\nfor detecting missed or late frames/fields/buffers either, and\nthere is no support whatsoever for cooperating with interlaced\ncompositor output.\n\nThe composited image quality resulting from the use of interlaced\nbuffers is explicitly undefined. A compositor may use elaborate\nhardware features or software to deinterlace and create progressive\noutput frames from a sequence of interlaced input buffers, or it\nmay produce substandard image quality. However, compositors that\ncannot guarantee reasonable image quality in all cases are recommended\nto just reject all interlaced buffers.\n\nAny argument errors, including non-positive width or height,\nmismatch between the number of planes and the format, bad\nformat, bad offset or stride, may be indicated by fatal protocol\nerrors: INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS,\nOUT_OF_BOUNDS.\n\nDmabuf import errors in the server that are not obvious client\nbugs are returned via the 'failed' event as non-fatal. This\nallows attempting dmabuf sharing and falling back in the client\nif it fails.\n\nThis request can be sent only once in the object's lifetime, after\nwhich the only legal request is destroy. This object should be\ndestroyed after issuing a 'create' request. Attempting to use this\nobject after issuing 'create' raises ALREADY_USED protocol error.\n\nIt is not mandatory to issue 'create'. If a client wants to\ncancel the buffer creation, it can just destroy this object."]
        Create {
            width: i32,
            height: i32,
            format: u32,
            flags: u32,
        },
        #[doc = "immediately create a wl_buffer from the given dmabufs\n\nThis asks for immediate creation of a wl_buffer by importing the\nadded dmabufs.\n\nIn case of import success, no event is sent from the server, and the\nwl_buffer is ready to be used by the client.\n\nUpon import failure, either of the following may happen, as seen fit\nby the implementation:\n- the client is terminated with one of the following fatal protocol\nerrors:\n- INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS, OUT_OF_BOUNDS,\nin case of argument errors such as mismatch between the number\nof planes and the format, bad format, non-positive width or\nheight, or bad offset or stride.\n- INVALID_WL_BUFFER, in case the cause for failure is unknown or\nplaform specific.\n- the server creates an invalid wl_buffer, marks it as failed and\nsends a 'failed' event to the client. The result of using this\ninvalid wl_buffer as an argument in any request by the client is\ndefined by the compositor implementation.\n\nThis takes the same arguments as a 'create' request, and obeys the\nsame restrictions.\n\nOnly available since version 2 of the interface"]
        CreateImmed {
            width: i32,
            height: i32,
            format: u32,
            flags: u32,
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
                    super::ArgumentType::Fd,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "create",
                since: 1,
                signature: &[
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "create_immed",
                since: 2,
                signature: &[
                    super::ArgumentType::NewId,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
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
                Request::Create { .. } => 2,
                Request::CreateImmed { .. } => 3,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::Add { .. } => 1,
                Request::Create { .. } => 1,
                Request::CreateImmed { .. } => 2,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                3 => Some(Object::from_interface::<super::wl_buffer::WlBuffer>(
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
                Request::Add {
                    fd,
                    plane_idx,
                    offset,
                    stride,
                    modifier_hi,
                    modifier_lo,
                } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![
                        Argument::Fd(fd),
                        Argument::Uint(plane_idx),
                        Argument::Uint(offset),
                        Argument::Uint(stride),
                        Argument::Uint(modifier_hi),
                        Argument::Uint(modifier_lo),
                    ],
                },
                Request::Create {
                    width,
                    height,
                    format,
                    flags,
                } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![
                        Argument::Int(width),
                        Argument::Int(height),
                        Argument::Uint(format),
                        Argument::Uint(flags),
                    ],
                },
                Request::CreateImmed {
                    width,
                    height,
                    format,
                    flags,
                } => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: smallvec![
                        Argument::NewId(0),
                        Argument::Int(width),
                        Argument::Int(height),
                        Argument::Uint(format),
                        Argument::Uint(flags),
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
                    fd,
                    plane_idx,
                    offset,
                    stride,
                    modifier_hi,
                    modifier_lo,
                } => {
                    let mut _args_array: [wl_argument; 6] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].h = fd;
                    _args_array[1].u = plane_idx;
                    _args_array[2].u = offset;
                    _args_array[3].u = stride;
                    _args_array[4].u = modifier_hi;
                    _args_array[5].u = modifier_lo;
                    f(1, &mut _args_array)
                }
                Request::Create {
                    width,
                    height,
                    format,
                    flags,
                } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = width;
                    _args_array[1].i = height;
                    _args_array[2].u = format;
                    _args_array[3].u = flags;
                    f(2, &mut _args_array)
                }
                Request::CreateImmed {
                    width,
                    height,
                    format,
                    flags,
                } => {
                    let mut _args_array: [wl_argument; 5] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].i = width;
                    _args_array[2].i = height;
                    _args_array[3].u = format;
                    _args_array[4].u = flags;
                    f(3, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "buffer creation succeeded\n\nThis event indicates that the attempted buffer creation was\nsuccessful. It provides the new wl_buffer referencing the dmabuf(s).\n\nUpon receiving this event, the client should destroy the\nzlinux_dmabuf_params object."]
        Created {
            buffer: Main<super::wl_buffer::WlBuffer>,
        },
        #[doc = "buffer creation failed\n\nThis event indicates that the attempted buffer creation has\nfailed. It usually means that one of the dmabuf constraints\nhas not been fulfilled.\n\nUpon receiving this event, the client should destroy the\nzlinux_buffer_params object."]
        Failed,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "created",
                since: 1,
                signature: &[super::ArgumentType::NewId],
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
                Event::Created { .. } => 0,
                Event::Failed => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Created { .. } => 1,
                Event::Failed => 1,
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
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Created {
                        buffer: {
                            if let Some(Argument::NewId(val)) = args.next() {
                                map.get_new(val).ok_or(())?
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
                    Ok(Event::Created {
                        buffer: Main::<super::wl_buffer::WlBuffer>::from_c_ptr(
                            _args[0].o as *mut _,
                        ),
                    })
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
    pub struct ZwpLinuxBufferParamsV1(Proxy<ZwpLinuxBufferParamsV1>);
    impl AsRef<Proxy<ZwpLinuxBufferParamsV1>> for ZwpLinuxBufferParamsV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpLinuxBufferParamsV1>> for ZwpLinuxBufferParamsV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpLinuxBufferParamsV1(value)
        }
    }
    impl From<ZwpLinuxBufferParamsV1> for Proxy<ZwpLinuxBufferParamsV1> {
        #[inline]
        fn from(value: ZwpLinuxBufferParamsV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwpLinuxBufferParamsV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_linux_buffer_params_v1";
        const VERSION: u32 = 3;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_linux_buffer_params_v1_interface }
        }
    }
    impl ZwpLinuxBufferParamsV1 {
        #[doc = "delete this object, used or not\n\nCleans up the temporary data sent to the server for dmabuf-based\nwl_buffer creation.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "add a dmabuf to the temporary set\n\nThis request adds one dmabuf to the set in this\nzwp_linux_buffer_params_v1.\n\nThe 64-bit unsigned value combined from modifier_hi and modifier_lo\nis the dmabuf layout modifier. DRM AddFB2 ioctl calls this the\nfb modifier, which is defined in drm_mode.h of Linux UAPI.\nThis is an opaque token. Drivers use this token to express tiling,\ncompression, etc. driver-specific modifications to the base format\ndefined by the DRM fourcc code.\n\nWarning: It should be an error if the format/modifier pair was not\nadvertised with the modifier event. This is not enforced yet because\nsome implementations always accept DRM_FORMAT_MOD_INVALID. Also\nversion 2 of this protocol does not have the modifier event.\n\nThis request raises the PLANE_IDX error if plane_idx is too large.\nThe error PLANE_SET is raised if attempting to set a plane that\nwas already set."]
        pub fn add(
            &self,
            fd: ::std::os::unix::io::RawFd,
            plane_idx: u32,
            offset: u32,
            stride: u32,
            modifier_hi: u32,
            modifier_lo: u32,
        ) -> () {
            let msg = Request::Add {
                fd: fd,
                plane_idx: plane_idx,
                offset: offset,
                stride: stride,
                modifier_hi: modifier_hi,
                modifier_lo: modifier_lo,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "create a wl_buffer from the given dmabufs\n\nThis asks for creation of a wl_buffer from the added dmabuf\nbuffers. The wl_buffer is not created immediately but returned via\nthe 'created' event if the dmabuf sharing succeeds. The sharing\nmay fail at runtime for reasons a client cannot predict, in\nwhich case the 'failed' event is triggered.\n\nThe 'format' argument is a DRM_FORMAT code, as defined by the\nlibdrm's drm_fourcc.h. The Linux kernel's DRM sub-system is the\nauthoritative source on how the format codes should work.\n\nThe 'flags' is a bitfield of the flags defined in enum \"flags\".\n'y_invert' means the that the image needs to be y-flipped.\n\nFlag 'interlaced' means that the frame in the buffer is not\nprogressive as usual, but interlaced. An interlaced buffer as\nsupported here must always contain both top and bottom fields.\nThe top field always begins on the first pixel row. The temporal\nordering between the two fields is top field first, unless\n'bottom_first' is specified. It is undefined whether 'bottom_first'\nis ignored if 'interlaced' is not set.\n\nThis protocol does not convey any information about field rate,\nduration, or timing, other than the relative ordering between the\ntwo fields in one buffer. A compositor may have to estimate the\nintended field rate from the incoming buffer rate. It is undefined\nwhether the time of receiving wl_surface.commit with a new buffer\nattached, applying the wl_surface state, wl_surface.frame callback\ntrigger, presentation, or any other point in the compositor cycle\nis used to measure the frame or field times. There is no support\nfor detecting missed or late frames/fields/buffers either, and\nthere is no support whatsoever for cooperating with interlaced\ncompositor output.\n\nThe composited image quality resulting from the use of interlaced\nbuffers is explicitly undefined. A compositor may use elaborate\nhardware features or software to deinterlace and create progressive\noutput frames from a sequence of interlaced input buffers, or it\nmay produce substandard image quality. However, compositors that\ncannot guarantee reasonable image quality in all cases are recommended\nto just reject all interlaced buffers.\n\nAny argument errors, including non-positive width or height,\nmismatch between the number of planes and the format, bad\nformat, bad offset or stride, may be indicated by fatal protocol\nerrors: INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS,\nOUT_OF_BOUNDS.\n\nDmabuf import errors in the server that are not obvious client\nbugs are returned via the 'failed' event as non-fatal. This\nallows attempting dmabuf sharing and falling back in the client\nif it fails.\n\nThis request can be sent only once in the object's lifetime, after\nwhich the only legal request is destroy. This object should be\ndestroyed after issuing a 'create' request. Attempting to use this\nobject after issuing 'create' raises ALREADY_USED protocol error.\n\nIt is not mandatory to issue 'create'. If a client wants to\ncancel the buffer creation, it can just destroy this object."]
        pub fn create(&self, width: i32, height: i32, format: u32, flags: u32) -> () {
            let msg = Request::Create {
                width: width,
                height: height,
                format: format,
                flags: flags,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "immediately create a wl_buffer from the given dmabufs\n\nThis asks for immediate creation of a wl_buffer by importing the\nadded dmabufs.\n\nIn case of import success, no event is sent from the server, and the\nwl_buffer is ready to be used by the client.\n\nUpon import failure, either of the following may happen, as seen fit\nby the implementation:\n- the client is terminated with one of the following fatal protocol\nerrors:\n- INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS, OUT_OF_BOUNDS,\nin case of argument errors such as mismatch between the number\nof planes and the format, bad format, non-positive width or\nheight, or bad offset or stride.\n- INVALID_WL_BUFFER, in case the cause for failure is unknown or\nplaform specific.\n- the server creates an invalid wl_buffer, marks it as failed and\nsends a 'failed' event to the client. The result of using this\ninvalid wl_buffer as an argument in any request by the client is\ndefined by the compositor implementation.\n\nThis takes the same arguments as a 'create' request, and obeys the\nsame restrictions.\n\nOnly available since version 2 of the interface."]
        pub fn create_immed(
            &self,
            width: i32,
            height: i32,
            format: u32,
            flags: u32,
        ) -> Main<super::wl_buffer::WlBuffer> {
            let msg = Request::CreateImmed {
                width: width,
                height: height,
                format: format,
                flags: flags,
            };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_ADD_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CREATE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CREATE_IMMED_SINCE: u32 = 2u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CREATED_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FAILED_SINCE: u32 = 1u32;
    static mut zwp_linux_buffer_params_v1_requests_create_immed_types: [*const wl_interface; 5] = [
        unsafe { &super::wl_buffer::wl_buffer_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_linux_buffer_params_v1_requests: [wl_message; 4] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"add\0" as *const u8 as *const c_char,
            signature: b"huuuuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"create\0" as *const u8 as *const c_char,
            signature: b"iiuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"create_immed\0" as *const u8 as *const c_char,
            signature: b"2niiuu\0" as *const u8 as *const c_char,
            types: unsafe { &zwp_linux_buffer_params_v1_requests_create_immed_types as *const _ },
        },
    ];
    static mut zwp_linux_buffer_params_v1_events_created_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_buffer::wl_buffer_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_linux_buffer_params_v1_events: [wl_message; 2] = [
        wl_message {
            name: b"created\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &zwp_linux_buffer_params_v1_events_created_types as *const _ },
        },
        wl_message {
            name: b"failed\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_linux_buffer_params_v1_interface: wl_interface = wl_interface {
        name: b"zwp_linux_buffer_params_v1\0" as *const u8 as *const c_char,
        version: 3,
        request_count: 4,
        requests: unsafe { &zwp_linux_buffer_params_v1_requests as *const _ },
        event_count: 2,
        events: unsafe { &zwp_linux_buffer_params_v1_events as *const _ },
    };
}
