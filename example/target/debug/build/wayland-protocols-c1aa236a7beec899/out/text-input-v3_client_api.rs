use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 4] = [
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
];
#[doc = "text input\n\nThe zwp_text_input_v3 interface represents text input and input methods\nassociated with a seat. It provides enter/leave events to follow the\ntext input focus for a seat.\n\nRequests are used to enable/disable the text-input object and set\nstate information like surrounding and selected text or the content type.\nThe information about the entered text is sent to the text-input object\nvia the preedit_string and commit_string events.\n\nText is valid UTF-8 encoded, indices and lengths are in bytes. Indices\nmust not point to middle bytes inside a code point: they must either\npoint to the first byte of a code point or to the end of the buffer.\nLengths must be measured between two valid indices.\n\nFocus moving throughout surfaces will result in the emission of\nzwp_text_input_v3.enter and zwp_text_input_v3.leave events. The focused\nsurface must commit zwp_text_input_v3.enable and\nzwp_text_input_v3.disable requests as the keyboard focus moves across\neditable and non-editable elements of the UI. Those two requests are not\nexpected to be paired with each other, the compositor must be able to\nhandle consecutive series of the same request.\n\nState is sent by the state requests (set_surrounding_text,\nset_content_type and set_cursor_rectangle) and a commit request. After an\nenter event or disable request all state information is invalidated and\nneeds to be resent by the client."]
pub mod zwp_text_input_v3 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[doc = "text change reason\n\nReason for the change of surrounding text or cursor posision."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum ChangeCause {
        #[doc = "input method caused the change"]
        InputMethod = 0,
        #[doc = "something else than the input method caused the change"]
        Other = 1,
    }
    impl ChangeCause {
        pub fn from_raw(n: u32) -> Option<ChangeCause> {
            match n {
                0 => Some(ChangeCause::InputMethod),
                1 => Some(ChangeCause::Other),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    bitflags! { # [ doc = "content hint\n\nContent hint is a bitmask to allow to modify the behavior of the text\ninput." ] pub struct ContentHint : u32 { # [ doc = "no special behavior" ] const None = 0 ; # [ doc = "suggest word completions" ] const Completion = 1 ; # [ doc = "suggest word corrections" ] const Spellcheck = 2 ; # [ doc = "switch to uppercase letters at the start of a sentence" ] const AutoCapitalization = 4 ; # [ doc = "prefer lowercase letters" ] const Lowercase = 8 ; # [ doc = "prefer uppercase letters" ] const Uppercase = 16 ; # [ doc = "prefer casing for titles and headings (can be language dependent)" ] const Titlecase = 32 ; # [ doc = "characters should be hidden" ] const HiddenText = 64 ; # [ doc = "typed text should not be stored" ] const SensitiveData = 128 ; # [ doc = "just Latin characters should be entered" ] const Latin = 256 ; # [ doc = "the text input is multiline" ] const Multiline = 512 ; } }
    impl ContentHint {
        pub fn from_raw(n: u32) -> Option<ContentHint> {
            Some(ContentHint::from_bits_truncate(n))
        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }
    #[doc = "content purpose\n\nThe content purpose allows to specify the primary purpose of a text\ninput.\n\nThis allows an input method to show special purpose input panels with\nextra characters or to disallow some characters."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum ContentPurpose {
        #[doc = "default input, allowing all characters"]
        Normal = 0,
        #[doc = "allow only alphabetic characters"]
        Alpha = 1,
        #[doc = "allow only digits"]
        Digits = 2,
        #[doc = "input a number (including decimal separator and sign)"]
        Number = 3,
        #[doc = "input a phone number"]
        Phone = 4,
        #[doc = "input an URL"]
        Url = 5,
        #[doc = "input an email address"]
        Email = 6,
        #[doc = "input a name of a person"]
        Name = 7,
        #[doc = "input a password (combine with sensitive_data hint)"]
        Password = 8,
        #[doc = "input is a numeric password (combine with sensitive_data hint)"]
        Pin = 9,
        #[doc = "input a date"]
        Date = 10,
        #[doc = "input a time"]
        Time = 11,
        #[doc = "input a date and time"]
        Datetime = 12,
        #[doc = "input for a terminal"]
        Terminal = 13,
    }
    impl ContentPurpose {
        pub fn from_raw(n: u32) -> Option<ContentPurpose> {
            match n {
                0 => Some(ContentPurpose::Normal),
                1 => Some(ContentPurpose::Alpha),
                2 => Some(ContentPurpose::Digits),
                3 => Some(ContentPurpose::Number),
                4 => Some(ContentPurpose::Phone),
                5 => Some(ContentPurpose::Url),
                6 => Some(ContentPurpose::Email),
                7 => Some(ContentPurpose::Name),
                8 => Some(ContentPurpose::Password),
                9 => Some(ContentPurpose::Pin),
                10 => Some(ContentPurpose::Date),
                11 => Some(ContentPurpose::Time),
                12 => Some(ContentPurpose::Datetime),
                13 => Some(ContentPurpose::Terminal),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[non_exhaustive]
    pub enum Request {
        #[doc = "Destroy the wp_text_input\n\nDestroy the wp_text_input object. Also disables all surfaces enabled\nthrough this wp_text_input object.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "Request text input to be enabled\n\nRequests text input on the surface previously obtained from the enter\nevent.\n\nThis request must be issued every time the active text input changes\nto a new one, including within the current surface. Use\nzwp_text_input_v3.disable when there is no longer any input focus on\nthe current surface.\n\nThis request resets all state associated with previous enable, disable,\nset_surrounding_text, set_text_change_cause, set_content_type, and\nset_cursor_rectangle requests, as well as the state associated with\npreedit_string, commit_string, and delete_surrounding_text events.\n\nThe set_surrounding_text, set_content_type and set_cursor_rectangle\nrequests must follow if the text input supports the necessary\nfunctionality.\n\nState set with this request is double-buffered. It will get applied on\nthe next zwp_text_input_v3.commit request, and stay valid until the\nnext committed enable or disable request.\n\nThe changes must be applied by the compositor after issuing a\nzwp_text_input_v3.commit request."]
        Enable,
        #[doc = "Disable text input on a surface\n\nExplicitly disable text input on the current surface (typically when\nthere is no focus on any text entry inside the surface).\n\nState set with this request is double-buffered. It will get applied on\nthe next zwp_text_input_v3.commit request."]
        Disable,
        #[doc = "sets the surrounding text\n\nSets the surrounding plain text around the input, excluding the preedit\ntext.\n\nThe client should notify the compositor of any changes in any of the\nvalues carried with this request, including changes caused by handling\nincoming text-input events as well as changes caused by other\nmechanisms like keyboard typing.\n\nIf the client is unaware of the text around the cursor, it should not\nissue this request, to signify lack of support to the compositor.\n\nText is UTF-8 encoded, and should include the cursor position, the\ncomplete selection and additional characters before and after them.\nThere is a maximum length of wayland messages, so text can not be\nlonger than 4000 bytes.\n\nCursor is the byte offset of the cursor within text buffer.\n\nAnchor is the byte offset of the selection anchor within text buffer.\nIf there is no selected text, anchor is the same as cursor.\n\nIf any preedit text is present, it is replaced with a cursor for the\npurpose of this event.\n\nValues set with this request are double-buffered. They will get applied\non the next zwp_text_input_v3.commit request, and stay valid until the\nnext committed enable or disable request.\n\nThe initial state for affected fields is empty, meaning that the text\ninput does not support sending surrounding text. If the empty values\nget applied, subsequent attempts to change them may have no effect."]
        SetSurroundingText {
            text: String,
            cursor: i32,
            anchor: i32,
        },
        #[doc = "indicates the cause of surrounding text change\n\nTells the compositor why the text surrounding the cursor changed.\n\nWhenever the client detects an external change in text, cursor, or\nanchor posision, it must issue this request to the compositor. This\nrequest is intended to give the input method a chance to update the\npreedit text in an appropriate way, e.g. by removing it when the user\nstarts typing with a keyboard.\n\ncause describes the source of the change.\n\nThe value set with this request is double-buffered. It must be applied\nand reset to initial at the next zwp_text_input_v3.commit request.\n\nThe initial value of cause is input_method."]
        SetTextChangeCause { cause: ChangeCause },
        #[doc = "set content purpose and hint\n\nSets the content purpose and content hint. While the purpose is the\nbasic purpose of an input field, the hint flags allow to modify some of\nthe behavior.\n\nValues set with this request are double-buffered. They will get applied\non the next zwp_text_input_v3.commit request.\nSubsequent attempts to update them may have no effect. The values\nremain valid until the next committed enable or disable request.\n\nThe initial value for hint is none, and the initial value for purpose\nis normal."]
        SetContentType {
            hint: ContentHint,
            purpose: ContentPurpose,
        },
        #[doc = "set cursor position\n\nMarks an area around the cursor as a x, y, width, height rectangle in\nsurface local coordinates.\n\nAllows the compositor to put a window with word suggestions near the\ncursor, without obstructing the text being input.\n\nIf the client is unaware of the position of edited text, it should not\nissue this request, to signify lack of support to the compositor.\n\nValues set with this request are double-buffered. They will get applied\non the next zwp_text_input_v3.commit request, and stay valid until the\nnext committed enable or disable request.\n\nThe initial values describing a cursor rectangle are empty. That means\nthe text input does not support describing the cursor area. If the\nempty values get applied, subsequent attempts to change them may have\nno effect."]
        SetCursorRectangle {
            x: i32,
            y: i32,
            width: i32,
            height: i32,
        },
        #[doc = "commit state\n\nAtomically applies state changes recently sent to the compositor.\n\nThe commit request establishes and updates the state of the client, and\nmust be issued after any changes to apply them.\n\nText input state (enabled status, content purpose, content hint,\nsurrounding text and change cause, cursor rectangle) is conceptually\ndouble-buffered within the context of a text input, i.e. between a\ncommitted enable request and the following committed enable or disable\nrequest.\n\nProtocol requests modify the pending state, as opposed to the current\nstate in use by the input method. A commit request atomically applies\nall pending state, replacing the current state. After commit, the new\npending state is as documented for each related request.\n\nRequests are applied in the order of arrival.\n\nNeither current nor pending state are modified unless noted otherwise.\n\nThe compositor must count the number of commit requests coming from\neach zwp_text_input_v3 object and use the count as the serial in done\nevents."]
        Commit,
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
                name: "enable",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "disable",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_surrounding_text",
                since: 1,
                signature: &[
                    super::ArgumentType::Str,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_text_change_cause",
                since: 1,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_content_type",
                since: 1,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "set_cursor_rectangle",
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
                name: "commit",
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
                Request::Enable => 1,
                Request::Disable => 2,
                Request::SetSurroundingText { .. } => 3,
                Request::SetTextChangeCause { .. } => 4,
                Request::SetContentType { .. } => 5,
                Request::SetCursorRectangle { .. } => 6,
                Request::Commit => 7,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::Enable => 1,
                Request::Disable => 1,
                Request::SetSurroundingText { .. } => 1,
                Request::SetTextChangeCause { .. } => 1,
                Request::SetContentType { .. } => 1,
                Request::SetCursorRectangle { .. } => 1,
                Request::Commit => 1,
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
                Request::Enable => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![],
                },
                Request::Disable => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![],
                },
                Request::SetSurroundingText {
                    text,
                    cursor,
                    anchor,
                } => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: smallvec![
                        Argument::Str(Box::new(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(text.into())
                        })),
                        Argument::Int(cursor),
                        Argument::Int(anchor),
                    ],
                },
                Request::SetTextChangeCause { cause } => Message {
                    sender_id: sender_id,
                    opcode: 4,
                    args: smallvec![Argument::Uint(cause.to_raw()),],
                },
                Request::SetContentType { hint, purpose } => Message {
                    sender_id: sender_id,
                    opcode: 5,
                    args: smallvec![
                        Argument::Uint(hint.to_raw()),
                        Argument::Uint(purpose.to_raw()),
                    ],
                },
                Request::SetCursorRectangle {
                    x,
                    y,
                    width,
                    height,
                } => Message {
                    sender_id: sender_id,
                    opcode: 6,
                    args: smallvec![
                        Argument::Int(x),
                        Argument::Int(y),
                        Argument::Int(width),
                        Argument::Int(height),
                    ],
                },
                Request::Commit => Message {
                    sender_id: sender_id,
                    opcode: 7,
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
                Request::Enable => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(1, &mut _args_array)
                }
                Request::Disable => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(2, &mut _args_array)
                }
                Request::SetSurroundingText {
                    text,
                    cursor,
                    anchor,
                } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(text).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    _args_array[1].i = cursor;
                    _args_array[2].i = anchor;
                    f(3, &mut _args_array)
                }
                Request::SetTextChangeCause { cause } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = cause.to_raw();
                    f(4, &mut _args_array)
                }
                Request::SetContentType { hint, purpose } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = hint.to_raw();
                    _args_array[1].u = purpose.to_raw();
                    f(5, &mut _args_array)
                }
                Request::SetCursorRectangle {
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
                    f(6, &mut _args_array)
                }
                Request::Commit => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(7, &mut _args_array)
                }
            }
        }
    }
    #[non_exhaustive]
    pub enum Event {
        #[doc = "enter event\n\nNotification that this seat's text-input focus is on a certain surface.\n\nWhen the seat has the keyboard capability the text-input focus follows\nthe keyboard focus. This event sets the current surface for the\ntext-input object."]
        Enter {
            surface: super::wl_surface::WlSurface,
        },
        #[doc = "leave event\n\nNotification that this seat's text-input focus is no longer on a\ncertain surface. The client should reset any preedit string previously\nset.\n\nThe leave notification clears the current surface. It is sent before\nthe enter notification for the new focus.\n\nWhen the seat has the keyboard capability the text-input focus follows\nthe keyboard focus."]
        Leave {
            surface: super::wl_surface::WlSurface,
        },
        #[doc = "pre-edit\n\nNotify when a new composing text (pre-edit) should be set at the\ncurrent cursor position. Any previously set composing text must be\nremoved. Any previously existing selected text must be removed.\n\nThe argument text contains the pre-edit string buffer.\n\nThe parameters cursor_begin and cursor_end are counted in bytes\nrelative to the beginning of the submitted text buffer. Cursor should\nbe hidden when both are equal to -1.\n\nThey could be represented by the client as a line if both values are\nthe same, or as a text highlight otherwise.\n\nValues set with this event are double-buffered. They must be applied\nand reset to initial on the next zwp_text_input_v3.done event.\n\nThe initial value of text is an empty string, and cursor_begin,\ncursor_end and cursor_hidden are all 0."]
        PreeditString {
            text: Option<String>,
            cursor_begin: i32,
            cursor_end: i32,
        },
        #[doc = "text commit\n\nNotify when text should be inserted into the editor widget. The text to\ncommit could be either just a single character after a key press or the\nresult of some composing (pre-edit).\n\nValues set with this event are double-buffered. They must be applied\nand reset to initial on the next zwp_text_input_v3.done event.\n\nThe initial value of text is an empty string."]
        CommitString { text: Option<String> },
        #[doc = "delete surrounding text\n\nNotify when the text around the current cursor position should be\ndeleted.\n\nBefore_length and after_length are the number of bytes before and after\nthe current cursor index (excluding the selection) to delete.\n\nIf a preedit text is present, in effect before_length is counted from\nthe beginning of it, and after_length from its end (see done event\nsequence).\n\nValues set with this event are double-buffered. They must be applied\nand reset to initial on the next zwp_text_input_v3.done event.\n\nThe initial values of both before_length and after_length are 0."]
        DeleteSurroundingText {
            before_length: u32,
            after_length: u32,
        },
        #[doc = "apply changes\n\nInstruct the application to apply changes to state requested by the\npreedit_string, commit_string and delete_surrounding_text events. The\nstate relating to these events is double-buffered, and each one\nmodifies the pending state. This event replaces the current state with\nthe pending state.\n\nThe application must proceed by evaluating the changes in the following\norder:\n\n1. Replace existing preedit string with the cursor.\n2. Delete requested surrounding text.\n3. Insert commit string with the cursor at its end.\n4. Calculate surrounding text to send.\n5. Insert new preedit text in cursor position.\n6. Place cursor inside preedit text.\n\nThe serial number reflects the last state of the zwp_text_input_v3\nobject known to the compositor. The value of the serial argument must\nbe equal to the number of commit requests already issued on that object.\nWhen the client receives a done event with a serial different than the\nnumber of past commit requests, it must proceed as normal, except it\nshould not change the current state of the zwp_text_input_v3 object."]
        Done { serial: u32 },
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
            super::MessageDesc {
                name: "preedit_string",
                since: 1,
                signature: &[
                    super::ArgumentType::Str,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "commit_string",
                since: 1,
                signature: &[super::ArgumentType::Str],
                destructor: false,
            },
            super::MessageDesc {
                name: "delete_surrounding_text",
                since: 1,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "done",
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
                Event::Enter { .. } => 0,
                Event::Leave { .. } => 1,
                Event::PreeditString { .. } => 2,
                Event::CommitString { .. } => 3,
                Event::DeleteSurroundingText { .. } => 4,
                Event::Done { .. } => 5,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Enter { .. } => 1,
                Event::Leave { .. } => 1,
                Event::PreeditString { .. } => 1,
                Event::CommitString { .. } => 1,
                Event::DeleteSurroundingText { .. } => 1,
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
                    Ok(Event::Enter {
                        surface: {
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
                    Ok(Event::PreeditString {
                        text: {
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
                        cursor_begin: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        cursor_end: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                3 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::CommitString {
                        text: {
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
                4 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::DeleteSurroundingText {
                        before_length: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        after_length: {
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
                    Ok(Event::Done {
                        serial: {
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
                    Ok(Event::Enter {
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(
                            _args[0].o as *mut _,
                        )
                        .into(),
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Leave {
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(
                            _args[0].o as *mut _,
                        )
                        .into(),
                    })
                }
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::PreeditString {
                        text: if _args[0].s.is_null() {
                            None
                        } else {
                            Some(
                                ::std::ffi::CStr::from_ptr(_args[0].s)
                                    .to_string_lossy()
                                    .into_owned(),
                            )
                        },
                        cursor_begin: _args[1].i,
                        cursor_end: _args[2].i,
                    })
                }
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::CommitString {
                        text: if _args[0].s.is_null() {
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
                4 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::DeleteSurroundingText {
                        before_length: _args[0].u,
                        after_length: _args[1].u,
                    })
                }
                5 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Done { serial: _args[0].u })
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
    pub struct ZwpTextInputV3(Proxy<ZwpTextInputV3>);
    impl AsRef<Proxy<ZwpTextInputV3>> for ZwpTextInputV3 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpTextInputV3>> for ZwpTextInputV3 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpTextInputV3(value)
        }
    }
    impl From<ZwpTextInputV3> for Proxy<ZwpTextInputV3> {
        #[inline]
        fn from(value: ZwpTextInputV3) -> Self {
            value.0
        }
    }
    impl Interface for ZwpTextInputV3 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_text_input_v3";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_text_input_v3_interface }
        }
    }
    impl ZwpTextInputV3 {
        #[doc = "Destroy the wp_text_input\n\nDestroy the wp_text_input object. Also disables all surfaces enabled\nthrough this wp_text_input object.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "Request text input to be enabled\n\nRequests text input on the surface previously obtained from the enter\nevent.\n\nThis request must be issued every time the active text input changes\nto a new one, including within the current surface. Use\nzwp_text_input_v3.disable when there is no longer any input focus on\nthe current surface.\n\nThis request resets all state associated with previous enable, disable,\nset_surrounding_text, set_text_change_cause, set_content_type, and\nset_cursor_rectangle requests, as well as the state associated with\npreedit_string, commit_string, and delete_surrounding_text events.\n\nThe set_surrounding_text, set_content_type and set_cursor_rectangle\nrequests must follow if the text input supports the necessary\nfunctionality.\n\nState set with this request is double-buffered. It will get applied on\nthe next zwp_text_input_v3.commit request, and stay valid until the\nnext committed enable or disable request.\n\nThe changes must be applied by the compositor after issuing a\nzwp_text_input_v3.commit request."]
        pub fn enable(&self) -> () {
            let msg = Request::Enable;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "Disable text input on a surface\n\nExplicitly disable text input on the current surface (typically when\nthere is no focus on any text entry inside the surface).\n\nState set with this request is double-buffered. It will get applied on\nthe next zwp_text_input_v3.commit request."]
        pub fn disable(&self) -> () {
            let msg = Request::Disable;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "sets the surrounding text\n\nSets the surrounding plain text around the input, excluding the preedit\ntext.\n\nThe client should notify the compositor of any changes in any of the\nvalues carried with this request, including changes caused by handling\nincoming text-input events as well as changes caused by other\nmechanisms like keyboard typing.\n\nIf the client is unaware of the text around the cursor, it should not\nissue this request, to signify lack of support to the compositor.\n\nText is UTF-8 encoded, and should include the cursor position, the\ncomplete selection and additional characters before and after them.\nThere is a maximum length of wayland messages, so text can not be\nlonger than 4000 bytes.\n\nCursor is the byte offset of the cursor within text buffer.\n\nAnchor is the byte offset of the selection anchor within text buffer.\nIf there is no selected text, anchor is the same as cursor.\n\nIf any preedit text is present, it is replaced with a cursor for the\npurpose of this event.\n\nValues set with this request are double-buffered. They will get applied\non the next zwp_text_input_v3.commit request, and stay valid until the\nnext committed enable or disable request.\n\nThe initial state for affected fields is empty, meaning that the text\ninput does not support sending surrounding text. If the empty values\nget applied, subsequent attempts to change them may have no effect."]
        pub fn set_surrounding_text(&self, text: String, cursor: i32, anchor: i32) -> () {
            let msg = Request::SetSurroundingText {
                text: text,
                cursor: cursor,
                anchor: anchor,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "indicates the cause of surrounding text change\n\nTells the compositor why the text surrounding the cursor changed.\n\nWhenever the client detects an external change in text, cursor, or\nanchor posision, it must issue this request to the compositor. This\nrequest is intended to give the input method a chance to update the\npreedit text in an appropriate way, e.g. by removing it when the user\nstarts typing with a keyboard.\n\ncause describes the source of the change.\n\nThe value set with this request is double-buffered. It must be applied\nand reset to initial at the next zwp_text_input_v3.commit request.\n\nThe initial value of cause is input_method."]
        pub fn set_text_change_cause(&self, cause: ChangeCause) -> () {
            let msg = Request::SetTextChangeCause { cause: cause };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set content purpose and hint\n\nSets the content purpose and content hint. While the purpose is the\nbasic purpose of an input field, the hint flags allow to modify some of\nthe behavior.\n\nValues set with this request are double-buffered. They will get applied\non the next zwp_text_input_v3.commit request.\nSubsequent attempts to update them may have no effect. The values\nremain valid until the next committed enable or disable request.\n\nThe initial value for hint is none, and the initial value for purpose\nis normal."]
        pub fn set_content_type(&self, hint: ContentHint, purpose: ContentPurpose) -> () {
            let msg = Request::SetContentType {
                hint: hint,
                purpose: purpose,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "set cursor position\n\nMarks an area around the cursor as a x, y, width, height rectangle in\nsurface local coordinates.\n\nAllows the compositor to put a window with word suggestions near the\ncursor, without obstructing the text being input.\n\nIf the client is unaware of the position of edited text, it should not\nissue this request, to signify lack of support to the compositor.\n\nValues set with this request are double-buffered. They will get applied\non the next zwp_text_input_v3.commit request, and stay valid until the\nnext committed enable or disable request.\n\nThe initial values describing a cursor rectangle are empty. That means\nthe text input does not support describing the cursor area. If the\nempty values get applied, subsequent attempts to change them may have\nno effect."]
        pub fn set_cursor_rectangle(&self, x: i32, y: i32, width: i32, height: i32) -> () {
            let msg = Request::SetCursorRectangle {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "commit state\n\nAtomically applies state changes recently sent to the compositor.\n\nThe commit request establishes and updates the state of the client, and\nmust be issued after any changes to apply them.\n\nText input state (enabled status, content purpose, content hint,\nsurrounding text and change cause, cursor rectangle) is conceptually\ndouble-buffered within the context of a text input, i.e. between a\ncommitted enable request and the following committed enable or disable\nrequest.\n\nProtocol requests modify the pending state, as opposed to the current\nstate in use by the input method. A commit request atomically applies\nall pending state, replacing the current state. After commit, the new\npending state is as documented for each related request.\n\nRequests are applied in the order of arrival.\n\nNeither current nor pending state are modified unless noted otherwise.\n\nThe compositor must count the number of commit requests coming from\neach zwp_text_input_v3 object and use the count as the serial in done\nevents."]
        pub fn commit(&self) -> () {
            let msg = Request::Commit;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_ENABLE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DISABLE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_SURROUNDING_TEXT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_TEXT_CHANGE_CAUSE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_CONTENT_TYPE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_CURSOR_RECTANGLE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_COMMIT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_ENTER_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_LEAVE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_PREEDIT_STRING_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_COMMIT_STRING_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DELETE_SURROUNDING_TEXT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DONE_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_text_input_v3_requests: [wl_message; 8] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"enable\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"disable\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_surrounding_text\0" as *const u8 as *const c_char,
            signature: b"sii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_text_change_cause\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_content_type\0" as *const u8 as *const c_char,
            signature: b"uu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_cursor_rectangle\0" as *const u8 as *const c_char,
            signature: b"iiii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"commit\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    static mut zwp_text_input_v3_events_enter_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface }];
    static mut zwp_text_input_v3_events_leave_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_text_input_v3_events: [wl_message; 6] = [
        wl_message {
            name: b"enter\0" as *const u8 as *const c_char,
            signature: b"o\0" as *const u8 as *const c_char,
            types: unsafe { &zwp_text_input_v3_events_enter_types as *const _ },
        },
        wl_message {
            name: b"leave\0" as *const u8 as *const c_char,
            signature: b"o\0" as *const u8 as *const c_char,
            types: unsafe { &zwp_text_input_v3_events_leave_types as *const _ },
        },
        wl_message {
            name: b"preedit_string\0" as *const u8 as *const c_char,
            signature: b"?sii\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"commit_string\0" as *const u8 as *const c_char,
            signature: b"?s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"delete_surrounding_text\0" as *const u8 as *const c_char,
            signature: b"uu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"done\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_text_input_v3_interface: wl_interface = wl_interface {
        name: b"zwp_text_input_v3\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 8,
        requests: unsafe { &zwp_text_input_v3_requests as *const _ },
        event_count: 6,
        events: unsafe { &zwp_text_input_v3_events as *const _ },
    };
}
#[doc = "text input manager\n\nA factory for text-input objects. This object is a global singleton."]
pub mod zwp_text_input_manager_v3 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[non_exhaustive]
    pub enum Request {
        #[doc = "Destroy the wp_text_input_manager\n\nDestroy the wp_text_input_manager object.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "create a new text input object\n\nCreates a new text-input object for a given seat."]
        GetTextInput { seat: super::wl_seat::WlSeat },
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
                name: "get_text_input",
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
                Request::GetTextInput { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Destroy => 1,
                Request::GetTextInput { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zwp_text_input_v3::ZwpTextInputV3,
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
                Request::GetTextInput { seat } => Message {
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
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
                Request::GetTextInput { seat } => {
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
    pub struct ZwpTextInputManagerV3(Proxy<ZwpTextInputManagerV3>);
    impl AsRef<Proxy<ZwpTextInputManagerV3>> for ZwpTextInputManagerV3 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpTextInputManagerV3>> for ZwpTextInputManagerV3 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpTextInputManagerV3(value)
        }
    }
    impl From<ZwpTextInputManagerV3> for Proxy<ZwpTextInputManagerV3> {
        #[inline]
        fn from(value: ZwpTextInputManagerV3) -> Self {
            value.0
        }
    }
    impl Interface for ZwpTextInputManagerV3 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_text_input_manager_v3";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_text_input_manager_v3_interface }
        }
    }
    impl ZwpTextInputManagerV3 {
        #[doc = "Destroy the wp_text_input_manager\n\nDestroy the wp_text_input_manager object.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "create a new text input object\n\nCreates a new text-input object for a given seat."]
        pub fn get_text_input(
            &self,
            seat: &super::wl_seat::WlSeat,
        ) -> Main<super::zwp_text_input_v3::ZwpTextInputV3> {
            let msg = Request::GetTextInput { seat: seat.clone() };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_TEXT_INPUT_SINCE: u32 = 1u32;
    static mut zwp_text_input_manager_v3_requests_get_text_input_types: [*const wl_interface; 2] = [
        unsafe { &super::zwp_text_input_v3::zwp_text_input_v3_interface as *const wl_interface },
        unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_text_input_manager_v3_requests: [wl_message; 2] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"get_text_input\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe { &zwp_text_input_manager_v3_requests_get_text_input_types as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_text_input_manager_v3_interface: wl_interface = wl_interface {
        name: b"zwp_text_input_manager_v3\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &zwp_text_input_manager_v3_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
