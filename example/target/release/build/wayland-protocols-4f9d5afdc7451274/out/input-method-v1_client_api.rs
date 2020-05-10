use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 5] = [
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
];
#[doc = "input method context\n\nCorresponds to a text input on the input method side. An input method context\nis created on text input activation on the input method side. It allows\nreceiving information about the text input from the application via events.\nInput method contexts do not keep state after deactivation and should be\ndestroyed after deactivation is handled.\n\nText is generally UTF-8 encoded, indices and lengths are in bytes.\n\nSerials are used to synchronize the state between the text input and\nan input method. New serials are sent by the text input in the\ncommit_state request and are used by the input method to indicate\nthe known text input state in events like preedit_string, commit_string,\nand keysym. The text input can then ignore events from the input method\nwhich are based on an outdated state (for example after a reset).\n\nWarning! The protocol described in this file is experimental and\nbackward incompatible changes may be made. Backward compatible changes\nmay be added together with the corresponding interface version bump.\nBackward incompatible changes are done by bumping the version number in\nthe protocol and interface names and resetting the interface version.\nOnce the protocol is to be declared stable, the 'z' prefix and the\nversion number in the protocol and interface names are removed and the\ninterface version number is reset."]
pub mod zwp_input_method_context_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "This is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "commit string\n\nSend the commit string text for insertion to the application.\n\nThe text to commit could be either just a single character after a key\npress or the result of some composing (pre-edit). It could be also an\nempty text when some text should be removed (see\ndelete_surrounding_text) or when the input cursor should be moved (see\ncursor_position).\n\nAny previously set composing text will be removed."]
        CommitString { serial: u32, text: String },
        #[doc = "pre-edit string\n\nSend the pre-edit string text to the application text input.\n\nThe commit text can be used to replace the pre-edit text on reset (for\nexample on unfocus).\n\nPreviously sent preedit_style and preedit_cursor requests are also\nprocessed by the text_input."]
        PreeditString {
            serial: u32,
            text: String,
            commit: String,
        },
        #[doc = "pre-edit styling\n\nSet the styling information on composing text. The style is applied for\nlength in bytes from index relative to the beginning of\nthe composing text (as byte offset). Multiple styles can\nbe applied to a composing text.\n\nThis request should be sent before sending a preedit_string request."]
        PreeditStyling { index: u32, length: u32, style: u32 },
        #[doc = "pre-edit cursor\n\nSet the cursor position inside the composing text (as byte offset)\nrelative to the start of the composing text.\n\nWhen index is negative no cursor should be displayed.\n\nThis request should be sent before sending a preedit_string request."]
        PreeditCursor { index: i32 },
        #[doc = "delete text\n\nRemove the surrounding text.\n\nThis request will be handled on the text_input side directly following\na commit_string request."]
        DeleteSurroundingText { index: i32, length: u32 },
        #[doc = "set cursor to a new position\n\nSet the cursor and anchor to a new position. Index is the new cursor\nposition in bytes (when >= 0 this is relative to the end of the inserted text,\notherwise it is relative to the beginning of the inserted text). Anchor is\nthe new anchor position in bytes (when >= 0 this is relative to the end of the\ninserted text, otherwise it is relative to the beginning of the inserted\ntext). When there should be no selected text, anchor should be the same\nas index.\n\nThis request will be handled on the text_input side directly following\na commit_string request."]
        CursorPosition { index: i32, anchor: i32 },
        #[doc = ""]
        ModifiersMap { map: Vec<u8> },
        #[doc = "keysym\n\nNotify when a key event was sent. Key events should not be used for\nnormal text input operations, which should be done with commit_string,\ndelete_surrounding_text, etc. The key event follows the wl_keyboard key\nevent convention. Sym is an XKB keysym, state is a wl_keyboard key_state."]
        Keysym {
            serial: u32,
            time: u32,
            sym: u32,
            state: u32,
            modifiers: u32,
        },
        #[doc = "grab hardware keyboard\n\nAllow an input method to receive hardware keyboard input and process\nkey events to generate text events (with pre-edit) over the wire. This\nallows input methods which compose multiple key events for inputting\ntext like it is done for CJK languages."]
        GrabKeyboard {},
        #[doc = "forward key event\n\nForward a wl_keyboard::key event to the client that was not processed\nby the input method itself. Should be used when filtering key events\nwith grab_keyboard.  The arguments should be the ones from the\nwl_keyboard::key event.\n\nFor generating custom key events use the keysym request instead."]
        Key {
            serial: u32,
            time: u32,
            key: u32,
            state: u32,
        },
        #[doc = "forward modifiers event\n\nForward a wl_keyboard::modifiers event to the client that was not\nprocessed by the input method itself.  Should be used when filtering\nkey events with grab_keyboard. The arguments should be the ones\nfrom the wl_keyboard::modifiers event."]
        Modifiers {
            serial: u32,
            mods_depressed: u32,
            mods_latched: u32,
            mods_locked: u32,
            group: u32,
        },
        #[doc = ""]
        Language { serial: u32, language: String },
        #[doc = ""]
        TextDirection { serial: u32, direction: u32 },
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
                name: "commit_string",
                since: 1,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Str],
                destructor: false,
            },
            super::MessageDesc {
                name: "preedit_string",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Str,
                    super::ArgumentType::Str,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "preedit_styling",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "preedit_cursor",
                since: 1,
                signature: &[super::ArgumentType::Int],
                destructor: false,
            },
            super::MessageDesc {
                name: "delete_surrounding_text",
                since: 1,
                signature: &[super::ArgumentType::Int, super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "cursor_position",
                since: 1,
                signature: &[super::ArgumentType::Int, super::ArgumentType::Int],
                destructor: false,
            },
            super::MessageDesc {
                name: "modifiers_map",
                since: 1,
                signature: &[super::ArgumentType::Array],
                destructor: false,
            },
            super::MessageDesc {
                name: "keysym",
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
                name: "grab_keyboard",
                since: 1,
                signature: &[super::ArgumentType::NewId],
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
                name: "language",
                since: 1,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Str],
                destructor: false,
            },
            super::MessageDesc {
                name: "text_direction",
                since: 1,
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
                Request::Destroy => 0,
                Request::CommitString { .. } => 1,
                Request::PreeditString { .. } => 2,
                Request::PreeditStyling { .. } => 3,
                Request::PreeditCursor { .. } => 4,
                Request::DeleteSurroundingText { .. } => 5,
                Request::CursorPosition { .. } => 6,
                Request::ModifiersMap { .. } => 7,
                Request::Keysym { .. } => 8,
                Request::GrabKeyboard { .. } => 9,
                Request::Key { .. } => 10,
                Request::Modifiers { .. } => 11,
                Request::Language { .. } => 12,
                Request::TextDirection { .. } => 13,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::CommitString { .. } => 1,
                Request::PreeditString { .. } => 1,
                Request::PreeditStyling { .. } => 1,
                Request::PreeditCursor { .. } => 1,
                Request::DeleteSurroundingText { .. } => 1,
                Request::CursorPosition { .. } => 1,
                Request::ModifiersMap { .. } => 1,
                Request::Keysym { .. } => 1,
                Request::GrabKeyboard { .. } => 1,
                Request::Key { .. } => 1,
                Request::Modifiers { .. } => 1,
                Request::Language { .. } => 1,
                Request::TextDirection { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                9 => Some(Object::from_interface::<super::wl_keyboard::WlKeyboard>(
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
                Request::CommitString { serial, text } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![
                        Argument::Uint(serial),
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(text.into())
                        })),
                    ],
                },
                Request::PreeditString {
                    serial,
                    text,
                    commit,
                } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![
                        Argument::Uint(serial),
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(text.into())
                        })),
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(commit.into())
                        })),
                    ],
                },
                Request::PreeditStyling {
                    index,
                    length,
                    style,
                } => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: smallvec![
                        Argument::Uint(index),
                        Argument::Uint(length),
                        Argument::Uint(style),
                    ],
                },
                Request::PreeditCursor { index } => Message {
                    sender_id: sender_id,
                    opcode: 4,
                    args: smallvec![Argument::Int(index),],
                },
                Request::DeleteSurroundingText { index, length } => Message {
                    sender_id: sender_id,
                    opcode: 5,
                    args: smallvec![Argument::Int(index), Argument::Uint(length),],
                },
                Request::CursorPosition { index, anchor } => Message {
                    sender_id: sender_id,
                    opcode: 6,
                    args: smallvec![Argument::Int(index), Argument::Int(anchor),],
                },
                Request::ModifiersMap { map } => Message {
                    sender_id: sender_id,
                    opcode: 7,
                    args: smallvec![Argument::Array(Box::new(map)),],
                },
                Request::Keysym {
                    serial,
                    time,
                    sym,
                    state,
                    modifiers,
                } => Message {
                    sender_id: sender_id,
                    opcode: 8,
                    args: smallvec![
                        Argument::Uint(serial),
                        Argument::Uint(time),
                        Argument::Uint(sym),
                        Argument::Uint(state),
                        Argument::Uint(modifiers),
                    ],
                },
                Request::GrabKeyboard {} => Message {
                    sender_id: sender_id,
                    opcode: 9,
                    args: smallvec![Argument::NewId(0),],
                },
                Request::Key {
                    serial,
                    time,
                    key,
                    state,
                } => Message {
                    sender_id: sender_id,
                    opcode: 10,
                    args: smallvec![
                        Argument::Uint(serial),
                        Argument::Uint(time),
                        Argument::Uint(key),
                        Argument::Uint(state),
                    ],
                },
                Request::Modifiers {
                    serial,
                    mods_depressed,
                    mods_latched,
                    mods_locked,
                    group,
                } => Message {
                    sender_id: sender_id,
                    opcode: 11,
                    args: smallvec![
                        Argument::Uint(serial),
                        Argument::Uint(mods_depressed),
                        Argument::Uint(mods_latched),
                        Argument::Uint(mods_locked),
                        Argument::Uint(group),
                    ],
                },
                Request::Language { serial, language } => Message {
                    sender_id: sender_id,
                    opcode: 12,
                    args: smallvec![
                        Argument::Uint(serial),
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(language.into())
                        })),
                    ],
                },
                Request::TextDirection { serial, direction } => Message {
                    sender_id: sender_id,
                    opcode: 13,
                    args: smallvec![Argument::Uint(serial), Argument::Uint(direction),],
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
                Request::CommitString { serial, text } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    let _arg_1 = ::std::ffi::CString::new(text).unwrap();
                    _args_array[1].s = _arg_1.as_ptr();
                    f(1, &mut _args_array)
                }
                Request::PreeditString {
                    serial,
                    text,
                    commit,
                } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    let _arg_1 = ::std::ffi::CString::new(text).unwrap();
                    _args_array[1].s = _arg_1.as_ptr();
                    let _arg_2 = ::std::ffi::CString::new(commit).unwrap();
                    _args_array[2].s = _arg_2.as_ptr();
                    f(2, &mut _args_array)
                }
                Request::PreeditStyling {
                    index,
                    length,
                    style,
                } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = index;
                    _args_array[1].u = length;
                    _args_array[2].u = style;
                    f(3, &mut _args_array)
                }
                Request::PreeditCursor { index } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = index;
                    f(4, &mut _args_array)
                }
                Request::DeleteSurroundingText { index, length } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = index;
                    _args_array[1].u = length;
                    f(5, &mut _args_array)
                }
                Request::CursorPosition { index, anchor } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = index;
                    _args_array[1].i = anchor;
                    f(6, &mut _args_array)
                }
                Request::ModifiersMap { map } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = wl_array {
                        size: map.len(),
                        alloc: map.capacity(),
                        data: map.as_ptr() as *mut _,
                    };
                    _args_array[0].a = &_arg_0;
                    f(7, &mut _args_array)
                }
                Request::Keysym {
                    serial,
                    time,
                    sym,
                    state,
                    modifiers,
                } => {
                    let mut _args_array: [wl_argument; 5] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    _args_array[1].u = time;
                    _args_array[2].u = sym;
                    _args_array[3].u = state;
                    _args_array[4].u = modifiers;
                    f(8, &mut _args_array)
                }
                Request::GrabKeyboard {} => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    f(9, &mut _args_array)
                }
                Request::Key {
                    serial,
                    time,
                    key,
                    state,
                } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    _args_array[1].u = time;
                    _args_array[2].u = key;
                    _args_array[3].u = state;
                    f(10, &mut _args_array)
                }
                Request::Modifiers {
                    serial,
                    mods_depressed,
                    mods_latched,
                    mods_locked,
                    group,
                } => {
                    let mut _args_array: [wl_argument; 5] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    _args_array[1].u = mods_depressed;
                    _args_array[2].u = mods_latched;
                    _args_array[3].u = mods_locked;
                    _args_array[4].u = group;
                    f(11, &mut _args_array)
                }
                Request::Language { serial, language } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    let _arg_1 = ::std::ffi::CString::new(language).unwrap();
                    _args_array[1].s = _arg_1.as_ptr();
                    f(12, &mut _args_array)
                }
                Request::TextDirection { serial, direction } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    _args_array[1].u = direction;
                    f(13, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "surrounding text event\n\nThe plain surrounding text around the input position. Cursor is the\nposition in bytes within the surrounding text relative to the beginning\nof the text. Anchor is the position in bytes of the selection anchor\nwithin the surrounding text relative to the beginning of the text. If\nthere is no selected text then anchor is the same as cursor."]
        SurroundingText {
            text: String,
            cursor: u32,
            anchor: u32,
        },
        #[doc = ""]
        Reset,
        #[doc = ""]
        ContentType { hint: u32, purpose: u32 },
        #[doc = ""]
        InvokeAction { button: u32, index: u32 },
        #[doc = ""]
        CommitState { serial: u32 },
        #[doc = ""]
        PreferredLanguage { language: String },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "surrounding_text",
                since: 1,
                signature: &[
                    super::ArgumentType::Str,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "reset",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "content_type",
                since: 1,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "invoke_action",
                since: 1,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "commit_state",
                since: 1,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "preferred_language",
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
                Event::SurroundingText { .. } => 0,
                Event::Reset => 1,
                Event::ContentType { .. } => 2,
                Event::InvokeAction { .. } => 3,
                Event::CommitState { .. } => 4,
                Event::PreferredLanguage { .. } => 5,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::SurroundingText { .. } => 1,
                Event::Reset => 1,
                Event::ContentType { .. } => 1,
                Event::InvokeAction { .. } => 1,
                Event::CommitState { .. } => 1,
                Event::PreferredLanguage { .. } => 1,
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
                    Ok(Event::SurroundingText {
                        text: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                        cursor: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        anchor: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => Ok(Event::Reset),
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::ContentType {
                        hint: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        purpose: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                3 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::InvokeAction {
                        button: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        index: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                4 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::CommitState {
                        serial: {
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
                    Ok(Event::PreferredLanguage {
                        language: {
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
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::SurroundingText {
                        text: ::std::ffi::CStr::from_ptr(_args[0].s)
                            .to_string_lossy()
                            .into_owned(),
                        cursor: _args[1].u,
                        anchor: _args[2].u,
                    })
                }
                1 => Ok(Event::Reset),
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::ContentType {
                        hint: _args[0].u,
                        purpose: _args[1].u,
                    })
                }
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::InvokeAction {
                        button: _args[0].u,
                        index: _args[1].u,
                    })
                }
                4 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::CommitState { serial: _args[0].u })
                }
                5 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::PreferredLanguage {
                        language: ::std::ffi::CStr::from_ptr(_args[0].s)
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
    pub struct ZwpInputMethodContextV1(Proxy<ZwpInputMethodContextV1>);
    impl AsRef<Proxy<ZwpInputMethodContextV1>> for ZwpInputMethodContextV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpInputMethodContextV1>> for ZwpInputMethodContextV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpInputMethodContextV1(value)
        }
    }
    impl From<ZwpInputMethodContextV1> for Proxy<ZwpInputMethodContextV1> {
        #[inline]
        fn from(value: ZwpInputMethodContextV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwpInputMethodContextV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_input_method_context_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_input_method_context_v1_interface }
        }
    }
    impl ZwpInputMethodContextV1 {
        #[doc = "This is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "commit string\n\nSend the commit string text for insertion to the application.\n\nThe text to commit could be either just a single character after a key\npress or the result of some composing (pre-edit). It could be also an\nempty text when some text should be removed (see\ndelete_surrounding_text) or when the input cursor should be moved (see\ncursor_position).\n\nAny previously set composing text will be removed."]
        pub fn commit_string(&self, serial: u32, text: String) -> () {
            let msg = Request::CommitString {
                serial: serial,
                text: text,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "pre-edit string\n\nSend the pre-edit string text to the application text input.\n\nThe commit text can be used to replace the pre-edit text on reset (for\nexample on unfocus).\n\nPreviously sent preedit_style and preedit_cursor requests are also\nprocessed by the text_input."]
        pub fn preedit_string(&self, serial: u32, text: String, commit: String) -> () {
            let msg = Request::PreeditString {
                serial: serial,
                text: text,
                commit: commit,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "pre-edit styling\n\nSet the styling information on composing text. The style is applied for\nlength in bytes from index relative to the beginning of\nthe composing text (as byte offset). Multiple styles can\nbe applied to a composing text.\n\nThis request should be sent before sending a preedit_string request."]
        pub fn preedit_styling(&self, index: u32, length: u32, style: u32) -> () {
            let msg = Request::PreeditStyling {
                index: index,
                length: length,
                style: style,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "pre-edit cursor\n\nSet the cursor position inside the composing text (as byte offset)\nrelative to the start of the composing text.\n\nWhen index is negative no cursor should be displayed.\n\nThis request should be sent before sending a preedit_string request."]
        pub fn preedit_cursor(&self, index: i32) -> () {
            let msg = Request::PreeditCursor { index: index };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "delete text\n\nRemove the surrounding text.\n\nThis request will be handled on the text_input side directly following\na commit_string request."]
        pub fn delete_surrounding_text(&self, index: i32, length: u32) -> () {
            let msg = Request::DeleteSurroundingText {
                index: index,
                length: length,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set cursor to a new position\n\nSet the cursor and anchor to a new position. Index is the new cursor\nposition in bytes (when >= 0 this is relative to the end of the inserted text,\notherwise it is relative to the beginning of the inserted text). Anchor is\nthe new anchor position in bytes (when >= 0 this is relative to the end of the\ninserted text, otherwise it is relative to the beginning of the inserted\ntext). When there should be no selected text, anchor should be the same\nas index.\n\nThis request will be handled on the text_input side directly following\na commit_string request."]
        pub fn cursor_position(&self, index: i32, anchor: i32) -> () {
            let msg = Request::CursorPosition {
                index: index,
                anchor: anchor,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = ""]
        pub fn modifiers_map(&self, map: Vec<u8>) -> () {
            let msg = Request::ModifiersMap { map: map };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "keysym\n\nNotify when a key event was sent. Key events should not be used for\nnormal text input operations, which should be done with commit_string,\ndelete_surrounding_text, etc. The key event follows the wl_keyboard key\nevent convention. Sym is an XKB keysym, state is a wl_keyboard key_state."]
        pub fn keysym(&self, serial: u32, time: u32, sym: u32, state: u32, modifiers: u32) -> () {
            let msg = Request::Keysym {
                serial: serial,
                time: time,
                sym: sym,
                state: state,
                modifiers: modifiers,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "grab hardware keyboard\n\nAllow an input method to receive hardware keyboard input and process\nkey events to generate text events (with pre-edit) over the wire. This\nallows input methods which compose multiple key events for inputting\ntext like it is done for CJK languages."]
        pub fn grab_keyboard(&self) -> Main<super::wl_keyboard::WlKeyboard> {
            let msg = Request::GrabKeyboard {};
            self.0.send(msg, None).unwrap()
        }
        #[doc = "forward key event\n\nForward a wl_keyboard::key event to the client that was not processed\nby the input method itself. Should be used when filtering key events\nwith grab_keyboard.  The arguments should be the ones from the\nwl_keyboard::key event.\n\nFor generating custom key events use the keysym request instead."]
        pub fn key(&self, serial: u32, time: u32, key: u32, state: u32) -> () {
            let msg = Request::Key {
                serial: serial,
                time: time,
                key: key,
                state: state,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "forward modifiers event\n\nForward a wl_keyboard::modifiers event to the client that was not\nprocessed by the input method itself.  Should be used when filtering\nkey events with grab_keyboard. The arguments should be the ones\nfrom the wl_keyboard::modifiers event."]
        pub fn modifiers(
            &self,
            serial: u32,
            mods_depressed: u32,
            mods_latched: u32,
            mods_locked: u32,
            group: u32,
        ) -> () {
            let msg = Request::Modifiers {
                serial: serial,
                mods_depressed: mods_depressed,
                mods_latched: mods_latched,
                mods_locked: mods_locked,
                group: group,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = ""]
        pub fn language(&self, serial: u32, language: String) -> () {
            let msg = Request::Language {
                serial: serial,
                language: language,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = ""]
        pub fn text_direction(&self, serial: u32, direction: u32) -> () {
            let msg = Request::TextDirection {
                serial: serial,
                direction: direction,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_COMMIT_STRING_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_PREEDIT_STRING_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_PREEDIT_STYLING_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_PREEDIT_CURSOR_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DELETE_SURROUNDING_TEXT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CURSOR_POSITION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_MODIFIERS_MAP_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_KEYSYM_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GRAB_KEYBOARD_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_KEY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_MODIFIERS_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_LANGUAGE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_TEXT_DIRECTION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_SURROUNDING_TEXT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_RESET_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CONTENT_TYPE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_INVOKE_ACTION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_COMMIT_STATE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_PREFERRED_LANGUAGE_SINCE: u32 = 1u32;
    static mut zwp_input_method_context_v1_requests_grab_keyboard_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_keyboard::wl_keyboard_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_input_method_context_v1_requests: [wl_message; 14] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"commit_string\0" as *const u8 as *const c_char,
            signature: b"us\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"preedit_string\0" as *const u8 as *const c_char,
            signature: b"uss\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"preedit_styling\0" as *const u8 as *const c_char,
            signature: b"uuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"preedit_cursor\0" as *const u8 as *const c_char,
            signature: b"i\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"delete_surrounding_text\0" as *const u8 as *const c_char,
            signature: b"iu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"cursor_position\0" as *const u8 as *const c_char,
            signature: b"ii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"modifiers_map\0" as *const u8 as *const c_char,
            signature: b"a\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"keysym\0" as *const u8 as *const c_char,
            signature: b"uuuuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"grab_keyboard\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &zwp_input_method_context_v1_requests_grab_keyboard_types as *const _ },
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
            name: b"language\0" as *const u8 as *const c_char,
            signature: b"us\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"text_direction\0" as *const u8 as *const c_char,
            signature: b"uu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_input_method_context_v1_events: [wl_message; 6] = [
        wl_message {
            name: b"surrounding_text\0" as *const u8 as *const c_char,
            signature: b"suu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"reset\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"content_type\0" as *const u8 as *const c_char,
            signature: b"uu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"invoke_action\0" as *const u8 as *const c_char,
            signature: b"uu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"commit_state\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"preferred_language\0" as *const u8 as *const c_char,
            signature: b"s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_input_method_context_v1_interface: wl_interface = wl_interface {
        name: b"zwp_input_method_context_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 14,
        requests: unsafe { &zwp_input_method_context_v1_requests as *const _ },
        event_count: 6,
        events: unsafe { &zwp_input_method_context_v1_events as *const _ },
    };
}
#[doc = "input method\n\nAn input method object is responsible for composing text in response to\ninput from hardware or virtual keyboards. There is one input method\nobject per seat. On activate there is a new input method context object\ncreated which allows the input method to communicate with the text input."]
pub mod zwp_input_method_v1 {
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
        #[doc = "activate event\n\nA text input was activated. Creates an input method context object\nwhich allows communication with the text input."]
        Activate {
            id: Main<super::zwp_input_method_context_v1::ZwpInputMethodContextV1>,
        },
        #[doc = "deactivate event\n\nThe text input corresponding to the context argument was deactivated.\nThe input method context should be destroyed after deactivation is\nhandled."]
        Deactivate {
            context: super::zwp_input_method_context_v1::ZwpInputMethodContextV1,
        },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "activate",
                since: 1,
                signature: &[super::ArgumentType::NewId],
                destructor: false,
            },
            super::MessageDesc {
                name: "deactivate",
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
                Event::Activate { .. } => 0,
                Event::Deactivate { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Activate { .. } => 1,
                Event::Deactivate { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<
                    super::zwp_input_method_context_v1::ZwpInputMethodContextV1,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Activate {
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
                    Ok(Event::Deactivate {
                        context: {
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
                    Ok ( Event :: Activate { id : Main :: < super :: zwp_input_method_context_v1 :: ZwpInputMethodContextV1 > :: from_c_ptr ( _args [ 0 ] . o as * mut _ ) , } )
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(
                        Event::Deactivate {
                            context: Proxy::<
                                super::zwp_input_method_context_v1::ZwpInputMethodContextV1,
                            >::from_c_ptr(_args[0].o as *mut _)
                            .into(),
                        },
                    )
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
    pub struct ZwpInputMethodV1(Proxy<ZwpInputMethodV1>);
    impl AsRef<Proxy<ZwpInputMethodV1>> for ZwpInputMethodV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpInputMethodV1>> for ZwpInputMethodV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpInputMethodV1(value)
        }
    }
    impl From<ZwpInputMethodV1> for Proxy<ZwpInputMethodV1> {
        #[inline]
        fn from(value: ZwpInputMethodV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwpInputMethodV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_input_method_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_input_method_v1_interface }
        }
    }
    impl ZwpInputMethodV1 {}
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_ACTIVATE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DEACTIVATE_SINCE: u32 = 1u32;
    static mut zwp_input_method_v1_events_activate_types: [*const wl_interface; 1] = [unsafe {
        &super::zwp_input_method_context_v1::zwp_input_method_context_v1_interface
            as *const wl_interface
    }];
    static mut zwp_input_method_v1_events_deactivate_types: [*const wl_interface; 1] = [unsafe {
        &super::zwp_input_method_context_v1::zwp_input_method_context_v1_interface
            as *const wl_interface
    }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_input_method_v1_events: [wl_message; 2] = [
        wl_message {
            name: b"activate\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &zwp_input_method_v1_events_activate_types as *const _ },
        },
        wl_message {
            name: b"deactivate\0" as *const u8 as *const c_char,
            signature: b"o\0" as *const u8 as *const c_char,
            types: unsafe { &zwp_input_method_v1_events_deactivate_types as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_input_method_v1_interface: wl_interface = wl_interface {
        name: b"zwp_input_method_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 0,
        requests: NULLPTR as *const wl_message,
        event_count: 2,
        events: unsafe { &zwp_input_method_v1_events as *const _ },
    };
}
#[doc = "interface for implementing keyboards\n\nOnly one client can bind this interface at a time."]
pub mod zwp_input_panel_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = ""]
        GetInputPanelSurface {
            surface: super::wl_surface::WlSurface,
        },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "get_input_panel_surface",
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
                Request::GetInputPanelSurface { .. } => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::GetInputPanelSurface { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<
                    super::zwp_input_panel_surface_v1::ZwpInputPanelSurfaceV1,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::GetInputPanelSurface { surface } => Message {
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
                Request::GetInputPanelSurface { surface } => {
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
    pub struct ZwpInputPanelV1(Proxy<ZwpInputPanelV1>);
    impl AsRef<Proxy<ZwpInputPanelV1>> for ZwpInputPanelV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpInputPanelV1>> for ZwpInputPanelV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpInputPanelV1(value)
        }
    }
    impl From<ZwpInputPanelV1> for Proxy<ZwpInputPanelV1> {
        #[inline]
        fn from(value: ZwpInputPanelV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwpInputPanelV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_input_panel_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_input_panel_v1_interface }
        }
    }
    impl ZwpInputPanelV1 {
        #[doc = ""]
        pub fn get_input_panel_surface(
            &self,
            surface: &super::wl_surface::WlSurface,
        ) -> Main<super::zwp_input_panel_surface_v1::ZwpInputPanelSurfaceV1> {
            let msg = Request::GetInputPanelSurface {
                surface: surface.clone(),
            };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_INPUT_PANEL_SURFACE_SINCE: u32 = 1u32;
    static mut zwp_input_panel_v1_requests_get_input_panel_surface_types: [*const wl_interface; 2] = [
        unsafe {
            &super::zwp_input_panel_surface_v1::zwp_input_panel_surface_v1_interface
                as *const wl_interface
        },
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_input_panel_v1_requests: [wl_message; 1] = [wl_message {
        name: b"get_input_panel_surface\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_input_panel_v1_requests_get_input_panel_surface_types as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_input_panel_v1_interface: wl_interface = wl_interface {
        name: b"zwp_input_panel_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 1,
        requests: unsafe { &zwp_input_panel_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
pub mod zwp_input_panel_surface_v1 {
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
    pub enum Position {
        CenterBottom = 0,
    }
    impl Position {
        pub fn from_raw(n: u32) -> Option<Position> {
            match n {
                0 => Some(Position::CenterBottom),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "set the surface type as a keyboard\n\nSet the input_panel_surface type to keyboard.\n\nA keyboard surface is only shown when a text input is active."]
        SetToplevel {
            output: super::wl_output::WlOutput,
            position: u32,
        },
        #[doc = "set the surface type as an overlay panel\n\nSet the input_panel_surface to be an overlay panel.\n\nThis is shown near the input cursor above the application window when\na text input is active."]
        SetOverlayPanel,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "set_toplevel",
                since: 1,
                signature: &[super::ArgumentType::Object, super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_overlay_panel",
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
                Request::SetToplevel { .. } => 0,
                Request::SetOverlayPanel => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::SetToplevel { .. } => 1,
                Request::SetOverlayPanel => 1,
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
                Request::SetToplevel { output, position } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![
                        Argument::Object(output.as_ref().id()),
                        Argument::Uint(position),
                    ],
                },
                Request::SetOverlayPanel => Message {
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
                Request::SetToplevel { output, position } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = output.as_ref().c_ptr() as *mut _;
                    _args_array[1].u = position;
                    f(0, &mut _args_array)
                }
                Request::SetOverlayPanel => {
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
    pub struct ZwpInputPanelSurfaceV1(Proxy<ZwpInputPanelSurfaceV1>);
    impl AsRef<Proxy<ZwpInputPanelSurfaceV1>> for ZwpInputPanelSurfaceV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpInputPanelSurfaceV1>> for ZwpInputPanelSurfaceV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpInputPanelSurfaceV1(value)
        }
    }
    impl From<ZwpInputPanelSurfaceV1> for Proxy<ZwpInputPanelSurfaceV1> {
        #[inline]
        fn from(value: ZwpInputPanelSurfaceV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwpInputPanelSurfaceV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_input_panel_surface_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_input_panel_surface_v1_interface }
        }
    }
    impl ZwpInputPanelSurfaceV1 {
        #[doc = "set the surface type as a keyboard\n\nSet the input_panel_surface type to keyboard.\n\nA keyboard surface is only shown when a text input is active."]
        pub fn set_toplevel(&self, output: &super::wl_output::WlOutput, position: u32) -> () {
            let msg = Request::SetToplevel {
                output: output.clone(),
                position: position,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set the surface type as an overlay panel\n\nSet the input_panel_surface to be an overlay panel.\n\nThis is shown near the input cursor above the application window when\na text input is active."]
        pub fn set_overlay_panel(&self) -> () {
            let msg = Request::SetOverlayPanel;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_TOPLEVEL_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_OVERLAY_PANEL_SINCE: u32 = 1u32;
    static mut zwp_input_panel_surface_v1_requests_set_toplevel_types: [*const wl_interface; 2] = [
        unsafe { &super::wl_output::wl_output_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_input_panel_surface_v1_requests: [wl_message; 2] = [
        wl_message {
            name: b"set_toplevel\0" as *const u8 as *const c_char,
            signature: b"ou\0" as *const u8 as *const c_char,
            types: unsafe { &zwp_input_panel_surface_v1_requests_set_toplevel_types as *const _ },
        },
        wl_message {
            name: b"set_overlay_panel\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_input_panel_surface_v1_interface: wl_interface = wl_interface {
        name: b"zwp_input_panel_surface_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &zwp_input_panel_surface_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
