//! Typio Basic Engine — standalone keyboard engine plugin.
//!
//! Implements a zero-dependency fallback engine that commits printable Unicode
//! text directly, with optional two-key compose sequences for accented chars.
//!
//! Exported C ABI:
//!   - typio_engine_get_info
//!   - typio_keyboard_engine_create

use std::ffi::{c_char, c_void, CString};
use std::ptr;

use typio_abi::*;

/* -------------------------------------------------------------------------- */
/* ABI version (not yet in typio-abi crate)                                   */
/* -------------------------------------------------------------------------- */

/// Mirror of `TypioAbiVersion` from `typio/abi/types.h`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TypioAbiVersion {
    pub major: u16,
    pub minor: u16,
}

/// ABI version this engine was built against — must match the host.
pub const TYPIO_ENGINE_ABI_MAJOR: u16 = 0;
pub const TYPIO_ENGINE_ABI_MINOR: u16 = 1;

static TYPIO_ENGINE_ABI_VERSION_STATIC: TypioAbiVersion = TypioAbiVersion {
    major: TYPIO_ENGINE_ABI_MAJOR,
    minor: TYPIO_ENGINE_ABI_MINOR,
};

/* -------------------------------------------------------------------------- */
/* External libtypio functions                                                */
/* -------------------------------------------------------------------------- */

extern "C" {
    fn typio_input_context_commit(ctx: *mut TypioInputContext, text: *const c_char);
    fn typio_input_context_clear(ctx: *mut TypioInputContext);
    fn typio_input_context_set_composition(ctx: *mut TypioInputContext, comp: *const TypioComposition);
    fn typio_key_event_is_modifier_only(event: *const TypioKeyEvent) -> bool;
    fn typio_key_event_is_escape(event: *const TypioKeyEvent) -> bool;
    fn typio_instance_get_config(instance: *mut TypioInstance) -> *mut c_void;
    fn typio_config_get_bool(config: *mut c_void, key: *const c_char, default: bool) -> bool;
}

/* -------------------------------------------------------------------------- */
/* Compose state machine                                                      */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ComposeResult {
    None,
    Consume,
    Commit(u32),
    Cancel(u32),
}

struct BasicCompose {
    active: bool,
    first: u32,
    preedit: String,
}

impl BasicCompose {
    fn new() -> Self {
        Self {
            active: false,
            first: 0,
            preedit: String::new(),
        }
    }

    fn process_key(&mut self, codepoint: u32) -> ComposeResult {
        if !self.active {
            if can_start_compose(codepoint) {
                self.active = true;
                self.first = codepoint;
                self.preedit = encode_utf8_to_string(codepoint);
                return ComposeResult::Consume;
            }
            return ComposeResult::None;
        }

        self.active = false;
        if let Some(result) = find_rule(self.first, codepoint) {
            return ComposeResult::Commit(result);
        }
        ComposeResult::Cancel(self.first)
    }

    fn get_preedit(&self) -> Option<&str> {
        if self.active { Some(&self.preedit) } else { None }
    }

    fn cancel(&mut self) -> Option<u32> {
        if self.active {
            let cp = self.first;
            self.active = false;
            Some(cp)
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.active = false;
        self.first = 0;
        self.preedit.clear();
    }

    fn is_active(&self) -> bool {
        self.active
    }
}

const COMPOSE_RULES: &[(u32, u32, u32)] = &[
    // Acute accent (')
    (b'\'' as u32, b'A' as u32, 0x00C1), (b'\'' as u32, b'C' as u32, 0x0106),
    (b'\'' as u32, b'E' as u32, 0x00C9), (b'\'' as u32, b'G' as u32, 0x01F4),
    (b'\'' as u32, b'I' as u32, 0x00CD), (b'\'' as u32, b'L' as u32, 0x0139),
    (b'\'' as u32, b'N' as u32, 0x0143), (b'\'' as u32, b'O' as u32, 0x00D3),
    (b'\'' as u32, b'R' as u32, 0x0154), (b'\'' as u32, b'S' as u32, 0x015A),
    (b'\'' as u32, b'U' as u32, 0x00DA), (b'\'' as u32, b'Y' as u32, 0x00DD),
    (b'\'' as u32, b'Z' as u32, 0x0179),
    (b'\'' as u32, b'a' as u32, 0x00E1), (b'\'' as u32, b'c' as u32, 0x0107),
    (b'\'' as u32, b'e' as u32, 0x00E9), (b'\'' as u32, b'g' as u32, 0x01F5),
    (b'\'' as u32, b'i' as u32, 0x00ED), (b'\'' as u32, b'k' as u32, 0x1E31),
    (b'\'' as u32, b'l' as u32, 0x013A), (b'\'' as u32, b'm' as u32, 0x1E3F),
    (b'\'' as u32, b'n' as u32, 0x0144), (b'\'' as u32, b'o' as u32, 0x00F3),
    (b'\'' as u32, b'p' as u32, 0x1E55), (b'\'' as u32, b'r' as u32, 0x0155),
    (b'\'' as u32, b's' as u32, 0x015B), (b'\'' as u32, b'u' as u32, 0x00FA),
    (b'\'' as u32, b'y' as u32, 0x00FD), (b'\'' as u32, b'z' as u32, 0x017A),
    // Grave accent (`)
    (b'`' as u32, b'A' as u32, 0x00C0), (b'`' as u32, b'E' as u32, 0x00C8),
    (b'`' as u32, b'I' as u32, 0x00CC), (b'`' as u32, b'O' as u32, 0x00D2),
    (b'`' as u32, b'U' as u32, 0x00D9),
    (b'`' as u32, b'a' as u32, 0x00E0), (b'`' as u32, b'e' as u32, 0x00E8),
    (b'`' as u32, b'i' as u32, 0x00EC), (b'`' as u32, b'o' as u32, 0x00F2),
    (b'`' as u32, b'u' as u32, 0x00F9),
    // Circumflex (^)
    (b'^' as u32, b'A' as u32, 0x00C2), (b'^' as u32, b'E' as u32, 0x00CA),
    (b'^' as u32, b'I' as u32, 0x00CE), (b'^' as u32, b'O' as u32, 0x00D4),
    (b'^' as u32, b'U' as u32, 0x00DB),
    (b'^' as u32, b'a' as u32, 0x00E2), (b'^' as u32, b'e' as u32, 0x00EA),
    (b'^' as u32, b'i' as u32, 0x00EE), (b'^' as u32, b'o' as u32, 0x00F4),
    (b'^' as u32, b'u' as u32, 0x00FB),
    // Diaeresis/umlaut (")
    (b'"' as u32, b'A' as u32, 0x00C4), (b'"' as u32, b'E' as u32, 0x00CB),
    (b'"' as u32, b'I' as u32, 0x00CF), (b'"' as u32, b'O' as u32, 0x00D6),
    (b'"' as u32, b'U' as u32, 0x00DC), (b'"' as u32, b'Y' as u32, 0x0178),
    (b'"' as u32, b'a' as u32, 0x00E4), (b'"' as u32, b'e' as u32, 0x00EB),
    (b'"' as u32, b'i' as u32, 0x00EF), (b'"' as u32, b'o' as u32, 0x00F6),
    (b'"' as u32, b'u' as u32, 0x00FC), (b'"' as u32, b'y' as u32, 0x00FF),
    // Tilde (~)
    (b'~' as u32, b'A' as u32, 0x00C3), (b'~' as u32, b'N' as u32, 0x00D1),
    (b'~' as u32, b'O' as u32, 0x00D5),
    (b'~' as u32, b'a' as u32, 0x00E3), (b'~' as u32, b'n' as u32, 0x00F1),
    (b'~' as u32, b'o' as u32, 0x00F5),
    // Cedilla (,)
    (b',' as u32, b'C' as u32, 0x00C7), (b',' as u32, b'c' as u32, 0x00E7),
    // Slash (/)
    (b'/' as u32, b'L' as u32, 0x0141), (b'/' as u32, b'O' as u32, 0x00D8),
    (b'/' as u32, b'l' as u32, 0x0142), (b'/' as u32, b'o' as u32, 0x00F8),
    // Special punctuation
    (b'?' as u32, b'?' as u32, 0x00BF), (b'!' as u32, b'!' as u32, 0x00A1),
    (b'<' as u32, b'<' as u32, 0x00AB), (b'>' as u32, b'>' as u32, 0x00BB),
    (b'\'' as u32, b'\'' as u32, 0x0027), (b'`' as u32, b'`' as u32, 0x0060),
    (b'"' as u32, b'"' as u32, 0x0022), (b'^' as u32, b'^' as u32, 0x005E),
    (b'~' as u32, b'~' as u32, 0x007E), (b',' as u32, b',' as u32, 0x002C),
    (b'-' as u32, b'-' as u32, 0x2013), (b'-' as u32, b'=' as u32, 0x2014),
    (b'.' as u32, b'.' as u32, 0x2026),
];

fn find_rule(first: u32, second: u32) -> Option<u32> {
    for &(f, s, r) in COMPOSE_RULES {
        if f == first && s == second {
            return Some(r);
        }
    }
    None
}

fn can_start_compose(codepoint: u32) -> bool {
    for &(f, _, _) in COMPOSE_RULES {
        if f == codepoint {
            return true;
        }
    }
    false
}

fn encode_utf8_to_string(codepoint: u32) -> String {
    let mut buf = [0u8; 4];
    let s = char::from_u32(codepoint).unwrap_or('\u{FFFD}');
    let len = s.encode_utf8(&mut buf).len();
    String::from_utf8_lossy(&buf[..len]).into_owned()
}

/* -------------------------------------------------------------------------- */
/* Engine per-instance data                                                   */
/* -------------------------------------------------------------------------- */

struct BasicEngineData {
    compose: BasicCompose,
    compose_enabled: bool,
}

/* -------------------------------------------------------------------------- */
/* Base ops callbacks                                                         */
/* -------------------------------------------------------------------------- */

extern "C" fn basic_init(engine: *mut TypioEngine, instance: *mut TypioInstance) -> TypioResult {
    if engine.is_null() {
        return TypioResult::TypioErrorInvalidArgument;
    }
    let data = Box::new(BasicEngineData {
        compose: BasicCompose::new(),
        compose_enabled: false,
    });
    unsafe {
        (*engine).user_data = Box::into_raw(data) as *mut c_void;
    }
    if !instance.is_null() {
        let config = unsafe { typio_instance_get_config(instance) };
        if !config.is_null() {
            let key = CString::new("engines.basic.compose").unwrap();
            let val = unsafe { typio_config_get_bool(config, key.as_ptr(), false) };
            unsafe {
                let data = &mut *((*engine).user_data as *mut BasicEngineData);
                data.compose_enabled = val;
            }
        }
    }
    TypioResult::TypioOk
}

extern "C" fn basic_destroy(engine: *mut TypioEngine) {
    if engine.is_null() {
        return;
    }
    unsafe {
        if !(*engine).user_data.is_null() {
            drop(Box::from_raw((*engine).user_data as *mut BasicEngineData));
            (*engine).user_data = ptr::null_mut();
        }
    }
}

extern "C" fn basic_deactivate(_engine: *mut TypioEngine) {}

extern "C" fn basic_focus_in(_engine: *mut TypioEngine, _ctx: *mut TypioInputContext) {}

extern "C" fn basic_focus_out(engine: *mut TypioEngine, ctx: *mut TypioInputContext) {
    basic_reset(engine, ctx);
}

extern "C" fn basic_reset(engine: *mut TypioEngine, ctx: *mut TypioInputContext) {
    if engine.is_null() {
        return;
    }
    unsafe {
        let data = &mut *((*engine).user_data as *mut BasicEngineData);
        if data.compose.is_active() && !ctx.is_null() {
            typio_input_context_clear(ctx);
        }
        data.compose.reset();
    }
}

extern "C" fn basic_reload_config(_engine: *mut TypioEngine) -> TypioResult {
    TypioResult::TypioOk
}

/* -------------------------------------------------------------------------- */
/* Keyboard ops callbacks                                                     */
/* -------------------------------------------------------------------------- */

extern "C" fn basic_process_key(
    engine: *mut TypioKeyboardEngine,
    ctx: *mut TypioInputContext,
    event: *const c_void,
) -> TypioKeyProcessResult {
    if engine.is_null() || event.is_null() {
        return TypioKeyProcessResult::TypioKeyNotHandled;
    }
    let event = unsafe { &*(event as *const TypioKeyEvent) };
    if event.type_ != TypioEventType::TypioEventKeyPress {
        return TypioKeyProcessResult::TypioKeyNotHandled;
    }

    if unsafe { typio_key_event_is_modifier_only(event) } || has_blocking_modifiers(event) {
        return TypioKeyProcessResult::TypioKeyNotHandled;
    }

    let base = unsafe { &mut (*engine).base };
    if base.user_data.is_null() {
        return TypioKeyProcessResult::TypioKeyNotHandled;
    }
    let data = unsafe { &mut *(base.user_data as *mut BasicEngineData) };

    // Escape cancels active composition.
    if data.compose.is_active() && unsafe { typio_key_event_is_escape(event) } {
        data.compose.cancel();
        unsafe { typio_input_context_clear(ctx) };
        return TypioKeyProcessResult::TypioKeyHandled;
    }

    let codepoint = event.unicode;

    // Non-printable keys.
    if codepoint < 0x20 || codepoint == 0x7F {
        if data.compose.is_active() {
            if let Some(cp) = data.compose.cancel() {
                let text = CString::new(encode_utf8_to_string(cp)).unwrap_or_default();
                unsafe { typio_input_context_clear(ctx) };
                unsafe { typio_input_context_commit(ctx, text.as_ptr()) };
                return TypioKeyProcessResult::TypioKeyCommitted;
            }
            unsafe { typio_input_context_clear(ctx) };
            return TypioKeyProcessResult::TypioKeyHandled;
        }
        return TypioKeyProcessResult::TypioKeyNotHandled;
    }

    // Fast path: compose disabled.
    if !data.compose_enabled {
        let text = CString::new(encode_utf8_to_string(codepoint)).unwrap_or_default();
        unsafe { typio_input_context_commit(ctx, text.as_ptr()) };
        return TypioKeyProcessResult::TypioKeyCommitted;
    }

    match data.compose.process_key(codepoint) {
        ComposeResult::None => {
            let text = CString::new(encode_utf8_to_string(codepoint)).unwrap_or_default();
            unsafe { typio_input_context_commit(ctx, text.as_ptr()) };
            TypioKeyProcessResult::TypioKeyCommitted
        }
        ComposeResult::Consume => {
            if let Some(preedit) = data.compose.get_preedit() {
                let text = CString::new(preedit).unwrap_or_default();
                let segment = TypioPreeditSegment {
                    text: text.as_ptr(),
                    format: TypioPreeditFormat::TypioPreeditUnderline as u32,
                };
                let comp = TypioComposition {
                    struct_size: std::mem::size_of::<TypioComposition>(),
                    segments: &segment,
                    segment_count: 1,
                    cursor_pos: -1,
                    candidates: ptr::null(),
                    candidate_count: 0,
                    page: 0,
                    page_size: 0,
                    total: 0,
                    selected: -1,
                    has_prev: false,
                    has_next: false,
                    content_signature: 0,
                    revision: 0,
                };
                unsafe { typio_input_context_set_composition(ctx, &comp) };
                std::mem::forget(text);
            }
            TypioKeyProcessResult::TypioKeyComposing
        }
        ComposeResult::Commit(result_cp) => {
            unsafe { typio_input_context_clear(ctx) };
            let text = CString::new(encode_utf8_to_string(result_cp)).unwrap_or_default();
            unsafe { typio_input_context_commit(ctx, text.as_ptr()) };
            TypioKeyProcessResult::TypioKeyCommitted
        }
        ComposeResult::Cancel(flushed_cp) => {
            unsafe { typio_input_context_clear(ctx) };
            let text = CString::new(encode_utf8_to_string(flushed_cp)).unwrap_or_default();
            unsafe { typio_input_context_commit(ctx, text.as_ptr()) };
            let text2 = CString::new(encode_utf8_to_string(codepoint)).unwrap_or_default();
            unsafe { typio_input_context_commit(ctx, text2.as_ptr()) };
            TypioKeyProcessResult::TypioKeyCommitted
        }
    }
}

extern "C" fn basic_get_status(_engine: *mut TypioKeyboardEngine, _ctx: *mut TypioInputContext) -> *const TypioEngineStatus {
    ptr::null()
}

extern "C" fn basic_set_status(_engine: *mut TypioKeyboardEngine, _ctx: *mut TypioInputContext, _mode_id: *const c_char) -> TypioResult {
    TypioResult::TypioOk
}

fn has_blocking_modifiers(event: &TypioKeyEvent) -> bool {
    (event.modifiers & ((TypioModifier::TypioModCtrl as u32)
        | (TypioModifier::TypioModAlt as u32)
        | (TypioModifier::TypioModSuper as u32))) != 0
}

/* -------------------------------------------------------------------------- */
/* Static vtables & info                                                      */
/* -------------------------------------------------------------------------- */

static BASIC_BASE_OPS: TypioEngineBaseOps = TypioEngineBaseOps {
    init: Some(basic_init),
    destroy: Some(basic_destroy),
    deactivate: Some(basic_deactivate),
    focus_in: Some(basic_focus_in),
    focus_out: Some(basic_focus_out),
    reset: Some(basic_reset),
    reload_config: Some(basic_reload_config),
    on_config_change: None,
};

static BASIC_KEYBOARD_OPS: TypioKeyboardEngineOps = TypioKeyboardEngineOps {
    process_key: Some(basic_process_key),
    get_status: Some(basic_get_status),
    set_status: Some(basic_set_status),
};

static BASIC_ENGINE_INFO: TypioEngineInfo = TypioEngineInfo {
    name: c"basic".as_ptr(),
    display_name: c"Basic".as_ptr(),
    description: c"Built-in basic keyboard engine that commits printable text directly.".as_ptr(),
    author: c"Typio".as_ptr(),
    icon: c"typio-engine-basic".as_ptr(),
    language: c"und".as_ptr(),
    type_: TypioEngineType::TypioEngineTypeKeyboard,
    required_capabilities: ptr::null(),
    optional_capabilities: ptr::null(),
};

/* -------------------------------------------------------------------------- */
/* Exported entry points                                                      */
/* -------------------------------------------------------------------------- */

#[no_mangle]
pub extern "C" fn typio_engine_abi_version() -> *const TypioAbiVersion {
    &TYPIO_ENGINE_ABI_VERSION_STATIC
}

#[no_mangle]
pub extern "C" fn typio_engine_get_info() -> *const TypioEngineInfo {
    &BASIC_ENGINE_INFO
}

#[no_mangle]
pub extern "C" fn typio_keyboard_engine_create() -> *mut TypioKeyboardEngine {
    let engine = unsafe {
        libc::calloc(1, std::mem::size_of::<TypioKeyboardEngine>()) as *mut TypioKeyboardEngine
    };
    if engine.is_null() {
        return ptr::null_mut();
    }
    unsafe {
        (*engine).base.info = &BASIC_ENGINE_INFO;
        (*engine).base.base_ops = &BASIC_BASE_OPS;
        (*engine).keyboard = &BASIC_KEYBOARD_OPS;
        (*engine).base.active = false;
        (*engine).base.initialized = false;
    }
    engine
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_none_for_plain_key() {
        let mut compose = BasicCompose::new();
        assert_eq!(compose.process_key(b'a' as u32), ComposeResult::None);
        assert!(!compose.is_active());
    }

    #[test]
    fn compose_consume_first_key() {
        let mut compose = BasicCompose::new();
        assert_eq!(compose.process_key(b'\'' as u32), ComposeResult::Consume);
        assert!(compose.is_active());
        assert_eq!(compose.get_preedit(), Some("'"));
    }

    #[test]
    fn compose_commit_sequence() {
        let mut compose = BasicCompose::new();
        compose.process_key(b'\'' as u32);
        let result = compose.process_key(b'a' as u32);
        assert!(matches!(result, ComposeResult::Commit(0x00E1)));
        assert!(!compose.is_active());
    }

    #[test]
    fn compose_cancel_no_match() {
        let mut compose = BasicCompose::new();
        compose.process_key(b'\'' as u32);
        let result = compose.process_key(b'x' as u32); // ' + x has no rule
        assert_eq!(result, ComposeResult::Cancel(b'\'' as u32));
        assert!(!compose.is_active());
    }

    #[test]
    fn compose_cancel_manual() {
        let mut compose = BasicCompose::new();
        compose.process_key(b'\'' as u32);
        assert!(compose.is_active());
        let cp = compose.cancel();
        assert_eq!(cp, Some(b'\'' as u32));
        assert!(!compose.is_active());
    }

    #[test]
    fn compose_reset() {
        let mut compose = BasicCompose::new();
        compose.process_key(b'\'' as u32);
        assert!(compose.is_active());
        compose.reset();
        assert!(!compose.is_active());
        assert_eq!(compose.get_preedit(), None);
    }

    #[test]
    fn compose_acute_a_uppercase() {
        let mut compose = BasicCompose::new();
        compose.process_key(b'\'' as u32);
        let result = compose.process_key(b'A' as u32);
        assert!(matches!(result, ComposeResult::Commit(0x00C1)));
    }

    #[test]
    fn compose_grave_e_lowercase() {
        let mut compose = BasicCompose::new();
        compose.process_key(b'`' as u32);
        let result = compose.process_key(b'e' as u32);
        assert!(matches!(result, ComposeResult::Commit(0x00E8)));
    }

    #[test]
    fn compose_tilde_n() {
        let mut compose = BasicCompose::new();
        compose.process_key(b'~' as u32);
        let result = compose.process_key(b'n' as u32);
        assert!(matches!(result, ComposeResult::Commit(0x00F1)));
    }

    #[test]
    fn compose_literal_quote() {
        let mut compose = BasicCompose::new();
        compose.process_key(b'\'' as u32);
        let result = compose.process_key(b'\'' as u32);
        assert!(matches!(result, ComposeResult::Commit(0x0027)));
    }

    #[test]
    fn compose_en_dash() {
        let mut compose = BasicCompose::new();
        compose.process_key(b'-' as u32);
        let result = compose.process_key(b'-' as u32);
        assert!(matches!(result, ComposeResult::Commit(0x2013)));
    }

    #[test]
    fn compose_ellipsis() {
        let mut compose = BasicCompose::new();
        compose.process_key(b'.' as u32);
        let result = compose.process_key(b'.' as u32);
        assert!(matches!(result, ComposeResult::Commit(0x2026)));
    }
}

/// Integration tests using the shared typio-engine-test harness.
///
/// Best practice: import ABI types from `typio-vet` instead of replicating
/// them inline. The casts below are required because the keyboard process_key
/// callback takes `*const c_void` per the ABI.
#[cfg(test)]
mod harness_tests {
    use super::*;
    use typio_vet::{mock_context, mock_instance, ContextEvent};

    fn key_press(unicode: char) -> TypioKeyEvent {
        TypioKeyEvent {
            struct_size: std::mem::size_of::<TypioKeyEvent>(),
            type_: TypioEventType::TypioEventKeyPress,
            keycode: 0,
            keysym: unicode as u32,
            modifiers: 0,
            unicode: unicode as u32,
            time: 0,
            is_repeat: false,
        }
    }

    #[test]
    fn basic_commits_printable_key() {
        let (ctx, log) = mock_context();
        let inst = mock_instance(Default::default());
        let engine = typio_keyboard_engine_create();
        assert!(!engine.is_null());

        unsafe {
            let base = &mut (*engine).base;
            if let Some(init) = (*base.base_ops).init {
                assert_eq!(init(base, inst.cast()), TypioResult::TypioOk);
            }

            let ev = key_press('a');
            let result = (*(*engine).keyboard).process_key.unwrap()(
                engine,
                ctx.cast(),
                &ev as *const _ as *const c_void,
            );
            assert_eq!(result, TypioKeyProcessResult::TypioKeyCommitted);
        }

        assert_eq!(log.take(), vec![ContextEvent::Commit("a".into())]);

        unsafe {
            let base = &mut (*engine).base;
            if let Some(destroy) = (*base.base_ops).destroy {
                destroy(base);
            }
            libc::free(engine as *mut c_void);
        }
    }

    #[test]
    fn basic_passthrough_modifier() {
        let (ctx, log) = mock_context();
        let inst = mock_instance(Default::default());
        let engine = typio_keyboard_engine_create();
        assert!(!engine.is_null());

        unsafe {
            let base = &mut (*engine).base;
            if let Some(init) = (*base.base_ops).init {
                assert_eq!(init(base, inst.cast()), TypioResult::TypioOk);
            }

            let ev = TypioKeyEvent {
                struct_size: std::mem::size_of::<TypioKeyEvent>(),
                type_: TypioEventType::TypioEventKeyPress,
                keycode: 0,
                keysym: 0xFFE1, // Shift_L
                modifiers: TypioModifier::TypioModShift as u32,
                unicode: 0,
                time: 0,
                is_repeat: false,
            };
            let result = (*(*engine).keyboard).process_key.unwrap()(
                engine,
                ctx.cast(),
                &ev as *const _ as *const c_void,
            );
            assert_eq!(result, TypioKeyProcessResult::TypioKeyNotHandled);
        }

        assert!(log.take().is_empty());

        unsafe {
            let base = &mut (*engine).base;
            if let Some(destroy) = (*base.base_ops).destroy {
                destroy(base);
            }
            libc::free(engine as *mut c_void);
        }
    }

    #[test]
    fn basic_compose_sequence_emits_composition() {
        // Compose enabled
        let mut config = std::collections::HashMap::new();
        config.insert(
            "engines.basic.compose".to_string(),
            typio_vet::ConfigValue::Bool(true),
        );

        let (ctx, log) = mock_context();
        let inst = mock_instance(config);
        let engine = typio_keyboard_engine_create();
        assert!(!engine.is_null());

        unsafe {
            let base = &mut (*engine).base;
            if let Some(init) = (*base.base_ops).init {
                assert_eq!(init(base, inst.cast()), TypioResult::TypioOk);
            }

            // Press '\'' (compose starter)
            let ev1 = key_press('\'');
            let r1 = (*(*engine).keyboard).process_key.unwrap()(engine, ctx.cast(), &ev1 as *const _ as *const c_void);
            assert_eq!(r1, TypioKeyProcessResult::TypioKeyComposing);

            let events1 = log.clone_events();
            assert_eq!(events1.len(), 1);
            assert!(matches!(events1[0], ContextEvent::SetComposition { ref preedit, .. } if preedit == "'"));

            // Press 'a' -> should commit 'á'
            let ev2 = key_press('a');
            let r2 = (*(*engine).keyboard).process_key.unwrap()(engine, ctx.cast(), &ev2 as *const _ as *const c_void);
            assert_eq!(r2, TypioKeyProcessResult::TypioKeyCommitted);
        }

        let events2 = log.take();
        assert!(events2.iter().any(|e| matches!(e, ContextEvent::Commit(ref s) if s == "á")));

        unsafe {
            let base = &mut (*engine).base;
            if let Some(destroy) = (*base.base_ops).destroy {
                destroy(base);
            }
            libc::free(engine as *mut c_void);
        }
    }
}
