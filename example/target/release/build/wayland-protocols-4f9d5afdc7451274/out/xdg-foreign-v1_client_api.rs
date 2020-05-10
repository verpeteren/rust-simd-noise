use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 1] =
    [NULLPTR as *const sys::common::wl_interface];
#[doc = "interface for exporting surfaces\n\nA global interface used for exporting surfaces that can later be imported\nusing xdg_importer."]
pub mod zxdg_exporter_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy the xdg_exporter object\n\nNotify the compositor that the xdg_exporter object will no longer be\nused.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "export a surface\n\nThe export request exports the passed surface so that it can later be\nimported via xdg_importer. When called, a new xdg_exported object will\nbe created and xdg_exported.handle will be sent immediately. See the\ncorresponding interface and event for details.\n\nA surface may be exported multiple times, and each exported handle may\nbe used to create a xdg_imported multiple times. Only xdg_surface\nsurfaces may be exported."]
        Export {
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
                name: "export",
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
                Request::Export { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::Export { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zxdg_exported_v1::ZxdgExportedV1,
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
                Request::Export { surface } => Message {
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
                Request::Export { surface } => {
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
    pub struct ZxdgExporterV1(Proxy<ZxdgExporterV1>);
    impl AsRef<Proxy<ZxdgExporterV1>> for ZxdgExporterV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZxdgExporterV1>> for ZxdgExporterV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZxdgExporterV1(value)
        }
    }
    impl From<ZxdgExporterV1> for Proxy<ZxdgExporterV1> {
        #[inline]
        fn from(value: ZxdgExporterV1) -> Self {
            value.0
        }
    }
    impl Interface for ZxdgExporterV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_exporter_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zxdg_exporter_v1_interface }
        }
    }
    impl ZxdgExporterV1 {
        #[doc = "destroy the xdg_exporter object\n\nNotify the compositor that the xdg_exporter object will no longer be\nused.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "export a surface\n\nThe export request exports the passed surface so that it can later be\nimported via xdg_importer. When called, a new xdg_exported object will\nbe created and xdg_exported.handle will be sent immediately. See the\ncorresponding interface and event for details.\n\nA surface may be exported multiple times, and each exported handle may\nbe used to create a xdg_imported multiple times. Only xdg_surface\nsurfaces may be exported."]
        pub fn export(
            &self,
            surface: &super::wl_surface::WlSurface,
        ) -> Main<super::zxdg_exported_v1::ZxdgExportedV1> {
            let msg = Request::Export {
                surface: surface.clone(),
            };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_EXPORT_SINCE: u32 = 1u32;
    static mut zxdg_exporter_v1_requests_export_types: [*const wl_interface; 2] = [
        unsafe { &super::zxdg_exported_v1::zxdg_exported_v1_interface as *const wl_interface },
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zxdg_exporter_v1_requests: [wl_message; 2] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"export\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe { &zxdg_exporter_v1_requests_export_types as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zxdg_exporter_v1_interface: wl_interface = wl_interface {
        name: b"zxdg_exporter_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &zxdg_exporter_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "interface for importing surfaces\n\nA global interface used for importing surfaces exported by xdg_exporter.\nWith this interface, a client can create a reference to a surface of\nanother client."]
pub mod zxdg_importer_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy the xdg_importer object\n\nNotify the compositor that the xdg_importer object will no longer be\nused.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "import a surface\n\nThe import request imports a surface from any client given a handle\nretrieved by exporting said surface using xdg_exporter.export. When\ncalled, a new xdg_imported object will be created. This new object\nrepresents the imported surface, and the importing client can\nmanipulate its relationship using it. See xdg_imported for details."]
        Import { handle: String },
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
                name: "import",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Str],
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
                Request::Import { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::Import { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zxdg_imported_v1::ZxdgImportedV1,
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
                Request::Import { handle } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![
                        Argument::NewId(0),
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(handle.into())
                        })),
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
                Request::Import { handle } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    let _arg_1 = ::std::ffi::CString::new(handle).unwrap();
                    _args_array[1].s = _arg_1.as_ptr();
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
    pub struct ZxdgImporterV1(Proxy<ZxdgImporterV1>);
    impl AsRef<Proxy<ZxdgImporterV1>> for ZxdgImporterV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZxdgImporterV1>> for ZxdgImporterV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZxdgImporterV1(value)
        }
    }
    impl From<ZxdgImporterV1> for Proxy<ZxdgImporterV1> {
        #[inline]
        fn from(value: ZxdgImporterV1) -> Self {
            value.0
        }
    }
    impl Interface for ZxdgImporterV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_importer_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zxdg_importer_v1_interface }
        }
    }
    impl ZxdgImporterV1 {
        #[doc = "destroy the xdg_importer object\n\nNotify the compositor that the xdg_importer object will no longer be\nused.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "import a surface\n\nThe import request imports a surface from any client given a handle\nretrieved by exporting said surface using xdg_exporter.export. When\ncalled, a new xdg_imported object will be created. This new object\nrepresents the imported surface, and the importing client can\nmanipulate its relationship using it. See xdg_imported for details."]
        pub fn import(&self, handle: String) -> Main<super::zxdg_imported_v1::ZxdgImportedV1> {
            let msg = Request::Import { handle: handle };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_IMPORT_SINCE: u32 = 1u32;
    static mut zxdg_importer_v1_requests_import_types: [*const wl_interface; 2] = [
        unsafe { &super::zxdg_imported_v1::zxdg_imported_v1_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zxdg_importer_v1_requests: [wl_message; 2] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"import\0" as *const u8 as *const c_char,
            signature: b"ns\0" as *const u8 as *const c_char,
            types: unsafe { &zxdg_importer_v1_requests_import_types as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zxdg_importer_v1_interface: wl_interface = wl_interface {
        name: b"zxdg_importer_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &zxdg_importer_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "an exported surface handle\n\nA xdg_exported object represents an exported reference to a surface. The\nexported surface may be referenced as long as the xdg_exported object not\ndestroyed. Destroying the xdg_exported invalidates any relationship the\nimporter may have established using xdg_imported."]
pub mod zxdg_exported_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "unexport the exported surface\n\nRevoke the previously exported surface. This invalidates any\nrelationship the importer may have set up using the xdg_imported created\ngiven the handle sent via xdg_exported.handle.\n\nThis is a destructor, once sent this object cannot be used any longer."]
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
        #[doc = "the exported surface handle\n\nThe handle event contains the unique handle of this exported surface\nreference. It may be shared with any client, which then can use it to\nimport the surface by calling xdg_importer.import. A handle may be\nused to import the surface multiple times."]
        Handle { handle: String },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "handle",
            since: 1,
            signature: &[super::ArgumentType::Str],
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
                Event::Handle { .. } => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Handle { .. } => 1,
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
                    Ok(Event::Handle {
                        handle: {
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
                    Ok(Event::Handle {
                        handle: ::std::ffi::CStr::from_ptr(_args[0].s)
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
    pub struct ZxdgExportedV1(Proxy<ZxdgExportedV1>);
    impl AsRef<Proxy<ZxdgExportedV1>> for ZxdgExportedV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZxdgExportedV1>> for ZxdgExportedV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZxdgExportedV1(value)
        }
    }
    impl From<ZxdgExportedV1> for Proxy<ZxdgExportedV1> {
        #[inline]
        fn from(value: ZxdgExportedV1) -> Self {
            value.0
        }
    }
    impl Interface for ZxdgExportedV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_exported_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zxdg_exported_v1_interface }
        }
    }
    impl ZxdgExportedV1 {
        #[doc = "unexport the exported surface\n\nRevoke the previously exported surface. This invalidates any\nrelationship the importer may have set up using the xdg_imported created\ngiven the handle sent via xdg_exported.handle.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_HANDLE_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zxdg_exported_v1_requests: [wl_message; 1] = [wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zxdg_exported_v1_events: [wl_message; 1] = [wl_message {
        name: b"handle\0" as *const u8 as *const c_char,
        signature: b"s\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zxdg_exported_v1_interface: wl_interface = wl_interface {
        name: b"zxdg_exported_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 1,
        requests: unsafe { &zxdg_exported_v1_requests as *const _ },
        event_count: 1,
        events: unsafe { &zxdg_exported_v1_events as *const _ },
    };
}
#[doc = "an imported surface handle\n\nA xdg_imported object represents an imported reference to surface exported\nby some client. A client can use this interface to manipulate\nrelationships between its own surfaces and the imported surface."]
pub mod zxdg_imported_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "destroy the xdg_imported object\n\nNotify the compositor that it will no longer use the xdg_imported\nobject. Any relationship that may have been set up will at this point\nbe invalidated.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "set as the parent of some surface\n\nSet the imported surface as the parent of some surface of the client.\nThe passed surface must be a toplevel xdg_surface. Calling this function\nsets up a surface to surface relation with the same stacking and positioning\nsemantics as xdg_surface.set_parent."]
        SetParentOf {
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
                name: "set_parent_of",
                since: 1,
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
                Request::Destroy => 0,
                Request::SetParentOf { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::SetParentOf { .. } => 1,
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
                Request::SetParentOf { surface } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::Object(surface.as_ref().id()),],
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
                Request::SetParentOf { surface } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = surface.as_ref().c_ptr() as *mut _;
                    f(1, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "the imported surface handle has been destroyed\n\nThe imported surface handle has been destroyed and any relationship set\nup has been invalidated. This may happen for various reasons, for\nexample if the exported surface or the exported surface handle has been\ndestroyed, if the handle used for importing was invalid."]
        Destroyed,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "destroyed",
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
                Event::Destroyed => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Destroyed => 1,
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
                0 => Ok(Event::Destroyed),
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
                0 => Ok(Event::Destroyed),
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
    pub struct ZxdgImportedV1(Proxy<ZxdgImportedV1>);
    impl AsRef<Proxy<ZxdgImportedV1>> for ZxdgImportedV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZxdgImportedV1>> for ZxdgImportedV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZxdgImportedV1(value)
        }
    }
    impl From<ZxdgImportedV1> for Proxy<ZxdgImportedV1> {
        #[inline]
        fn from(value: ZxdgImportedV1) -> Self {
            value.0
        }
    }
    impl Interface for ZxdgImportedV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_imported_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zxdg_imported_v1_interface }
        }
    }
    impl ZxdgImportedV1 {
        #[doc = "destroy the xdg_imported object\n\nNotify the compositor that it will no longer use the xdg_imported\nobject. Any relationship that may have been set up will at this point\nbe invalidated.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set as the parent of some surface\n\nSet the imported surface as the parent of some surface of the client.\nThe passed surface must be a toplevel xdg_surface. Calling this function\nsets up a surface to surface relation with the same stacking and positioning\nsemantics as xdg_surface.set_parent."]
        pub fn set_parent_of(&self, surface: &super::wl_surface::WlSurface) -> () {
            let msg = Request::SetParentOf {
                surface: surface.clone(),
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_PARENT_OF_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DESTROYED_SINCE: u32 = 1u32;
    static mut zxdg_imported_v1_requests_set_parent_of_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zxdg_imported_v1_requests: [wl_message; 2] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_parent_of\0" as *const u8 as *const c_char,
            signature: b"o\0" as *const u8 as *const c_char,
            types: unsafe { &zxdg_imported_v1_requests_set_parent_of_types as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zxdg_imported_v1_events: [wl_message; 1] = [wl_message {
        name: b"destroyed\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zxdg_imported_v1_interface: wl_interface = wl_interface {
        name: b"zxdg_imported_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &zxdg_imported_v1_requests as *const _ },
        event_count: 1,
        events: unsafe { &zxdg_imported_v1_events as *const _ },
    };
}
