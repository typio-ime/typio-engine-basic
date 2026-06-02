//! Typio Basic Engine — standalone keyboard engine plugin.
//!
//! Single "compose" mode with Active engagement. All printable keys commit
//! directly unless Shift+Alt opens the compose picker: type a base key (e.g.
//! 'a') and pick from candidate list of related Latin characters (á à â ä ã …).
//!
//! Exported C ABI:
//!   - typio_engine_get_info
//!   - typio_keyboard_engine_create

use std::ffi::{c_char, c_void, CString};
use std::ptr;

use typio_abi::*;

/* -------------------------------------------------------------------------- */
/* ABI version                                                                */
/* -------------------------------------------------------------------------- */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TypioAbiVersion {
    pub major: u16,
    pub minor: u16,
}

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
    fn typio_key_event_is_escape(event: *const TypioKeyEvent) -> bool;
}

/* -------------------------------------------------------------------------- */
/* Compose rule tables                                                        */
/* -------------------------------------------------------------------------- */

const COMPOSE_RULES: &[(u32, u32, u32)] = &[
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
    (b'`' as u32, b'A' as u32, 0x00C0), (b'`' as u32, b'E' as u32, 0x00C8),
    (b'`' as u32, b'I' as u32, 0x00CC), (b'`' as u32, b'O' as u32, 0x00D2),
    (b'`' as u32, b'U' as u32, 0x00D9),
    (b'`' as u32, b'a' as u32, 0x00E0), (b'`' as u32, b'e' as u32, 0x00E8),
    (b'`' as u32, b'i' as u32, 0x00EC), (b'`' as u32, b'o' as u32, 0x00F2),
    (b'`' as u32, b'u' as u32, 0x00F9),
    (b'^' as u32, b'A' as u32, 0x00C2), (b'^' as u32, b'E' as u32, 0x00CA),
    (b'^' as u32, b'I' as u32, 0x00CE), (b'^' as u32, b'O' as u32, 0x00D4),
    (b'^' as u32, b'U' as u32, 0x00DB),
    (b'^' as u32, b'a' as u32, 0x00E2), (b'^' as u32, b'e' as u32, 0x00EA),
    (b'^' as u32, b'i' as u32, 0x00EE), (b'^' as u32, b'o' as u32, 0x00F4),
    (b'^' as u32, b'u' as u32, 0x00FB),
    (b'"' as u32, b'A' as u32, 0x00C4), (b'"' as u32, b'E' as u32, 0x00CB),
    (b'"' as u32, b'I' as u32, 0x00CF), (b'"' as u32, b'O' as u32, 0x00D6),
    (b'"' as u32, b'U' as u32, 0x00DC), (b'"' as u32, b'Y' as u32, 0x0178),
    (b'"' as u32, b'a' as u32, 0x00E4), (b'"' as u32, b'e' as u32, 0x00EB),
    (b'"' as u32, b'i' as u32, 0x00EF), (b'"' as u32, b'o' as u32, 0x00F6),
    (b'"' as u32, b'u' as u32, 0x00FC), (b'"' as u32, b'y' as u32, 0x00FF),
    (b'~' as u32, b'A' as u32, 0x00C3), (b'~' as u32, b'N' as u32, 0x00D1),
    (b'~' as u32, b'O' as u32, 0x00D5),
    (b'~' as u32, b'a' as u32, 0x00E3), (b'~' as u32, b'n' as u32, 0x00F1),
    (b'~' as u32, b'o' as u32, 0x00F5),
    (b',' as u32, b'C' as u32, 0x00C7), (b',' as u32, b'c' as u32, 0x00E7),
    (b'/' as u32, b'L' as u32, 0x0141), (b'/' as u32, b'O' as u32, 0x00D8),
    (b'/' as u32, b'l' as u32, 0x0142), (b'/' as u32, b'o' as u32, 0x00F8),
    (b'?' as u32, b'?' as u32, 0x00BF), (b'!' as u32, b'!' as u32, 0x00A1),
    (b'<' as u32, b'<' as u32, 0x00AB), (b'>' as u32, b'>' as u32, 0x00BB),
    (b'\'' as u32, b'\'' as u32, 0x0027), (b'`' as u32, b'`' as u32, 0x0060),
    (b'"' as u32, b'"' as u32, 0x0022), (b'^' as u32, b'^' as u32, 0x005E),
    (b'~' as u32, b'~' as u32, 0x007E), (b',' as u32, b',' as u32, 0x002C),
    (b'-' as u32, b'-' as u32, 0x2013), (b'-' as u32, b'=' as u32, 0x2014),
    (b'.' as u32, b'.' as u32, 0x2026),
    (b'~' as u32, b'=' as u32, 0x2248),
    (b'!' as u32, b'=' as u32, 0x2260),
    (b'<' as u32, b'=' as u32, 0x2264),
    (b'>' as u32, b'=' as u32, 0x2265),
    (b'-' as u32, b'+' as u32, 0x2213),
    (b'^' as u32, b'1' as u32, 0x00B9),
    (b'^' as u32, b'2' as u32, 0x00B2),
    (b'^' as u32, b'3' as u32, 0x00B3),
    (b'+' as u32, b'-' as u32, 0x00B1),
    (b'=' as u32, b'=' as u32, 0x2261),
    (b'*' as u32, b'*' as u32, 0x00D7),
    (b'o' as u32, b'o' as u32, 0x00B0),
    (b'O' as u32, b'O' as u32, 0x00B0),
    (b's' as u32, b's' as u32, 0x00DF),
    (b'S' as u32, b'S' as u32, 0x1E9E),
    (b'a' as u32, b'e' as u32, 0x00E6),
    (b'o' as u32, b'e' as u32, 0x0153),
    (b'A' as u32, b'E' as u32, 0x00C6),
    (b'O' as u32, b'E' as u32, 0x0152),
];

fn encode_utf8_to_string(codepoint: u32) -> String {
    let mut buf = [0u8; 4];
    let s = char::from_u32(codepoint).unwrap_or('\u{FFFD}');
    let len = s.encode_utf8(&mut buf).len();
    String::from_utf8_lossy(&buf[..len]).into_owned()
}

/* -------------------------------------------------------------------------- */
/* Compose picker state machine                                               */
/* -------------------------------------------------------------------------- */

struct ComposeCandidate {
    result_char: String,
}

struct ComposePicker {
    active: bool,
    buffer: String,
    candidates: Vec<ComposeCandidate>,
    selected: i32,
}

#[allow(dead_code)]
impl ComposePicker {
    fn new() -> Self {
        Self {
            active: false,
            buffer: String::new(),
            candidates: Vec::new(),
            selected: -1,
        }
    }

    fn activate(&mut self) {
        self.active = true;
        self.buffer.clear();
        self.candidates.clear();
        self.selected = -1;
    }

    fn deactivate(&mut self) {
        self.active = false;
        self.buffer.clear();
        self.candidates.clear();
        self.selected = -1;
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn append_char(&mut self, codepoint: u32) {
        if self.buffer.chars().count() >= 2 {
            return;
        }
        let ch = char::from_u32(codepoint).unwrap_or('\u{FFFD}');
        self.buffer.push(ch);
        self.search();
    }

    fn backspace(&mut self) -> bool {
        if self.buffer.is_empty() {
            return false;
        }
        self.buffer.pop();
        self.search();
        true
    }

    fn get_selected_result(&self) -> Option<&ComposeCandidate> {
        if self.selected >= 0 && (self.selected as usize) < self.candidates.len() {
            Some(&self.candidates[self.selected as usize])
        } else {
            None
        }
    }

    fn search(&mut self) {
        self.candidates.clear();
        self.selected = -1;
        let chars: Vec<u32> = self.buffer.chars().map(|c| c as u32).collect();
        match chars.len() {
            1 => self.search_by_base(chars[0]),
            2 => self.search_exact(chars[0], chars[1]),
            _ => {}
        }
        if !self.candidates.is_empty() {
            self.selected = 0;
        }
    }

    fn search_by_base(&mut self, cp: u32) {
        for &(f, s, r) in COMPOSE_RULES {
            if f == cp || s == cp {
                self.push_candidate(f, s, r);
            }
        }
    }

    fn search_exact(&mut self, first: u32, second: u32) {
        for &(f, s, r) in COMPOSE_RULES {
            if f == first && s == second {
                self.push_candidate(f, s, r);
            }
        }
    }

    fn push_candidate(&mut self, _f: u32, _s: u32, r: u32) {
        self.candidates.push(ComposeCandidate {
            result_char: encode_utf8_to_string(r),
        });
    }
}

/* -------------------------------------------------------------------------- */
/* Engine per-instance data                                                   */
/* -------------------------------------------------------------------------- */

struct BasicEngineData {
    picker: ComposePicker,
    picker_text_cache: Vec<CString>,
    shift_chord_pending: bool,
}

/* -------------------------------------------------------------------------- */
/* Base ops callbacks                                                         */
/* -------------------------------------------------------------------------- */

extern "C" fn basic_init(engine: *mut TypioEngine, _instance: *mut TypioInstance) -> TypioResult {
    if engine.is_null() {
        return TypioResult::TypioErrorInvalidArgument;
    }
    let data = Box::new(BasicEngineData {
        picker: ComposePicker::new(),
        picker_text_cache: Vec::new(),
        shift_chord_pending: false,
    });
    unsafe {
        (*engine).user_data = Box::into_raw(data) as *mut c_void;
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
        if data.picker.is_active() && !ctx.is_null() {
            typio_input_context_clear(ctx);
        }
        data.picker.deactivate();
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
    let base = unsafe { &mut (*engine).base };
    if base.user_data.is_null() {
        return TypioKeyProcessResult::TypioKeyNotHandled;
    }
    let data = unsafe { &mut *(base.user_data as *mut BasicEngineData) };

    let is_shift = event.keysym == TYPIO_KEY_Shift_L || event.keysym == TYPIO_KEY_Shift_R;
    let is_alt = event.keysym == TYPIO_KEY_Alt_L || event.keysym == TYPIO_KEY_Alt_R;

    if event.type_ == TypioEventType::TypioEventKeyRelease {
        if is_shift {
            data.shift_chord_pending = false;
        }
        return TypioKeyProcessResult::TypioKeyNotHandled;
    }

    if event.type_ != TypioEventType::TypioEventKeyPress {
        return TypioKeyProcessResult::TypioKeyNotHandled;
    }

    if is_shift {
        data.shift_chord_pending = true;
        return TypioKeyProcessResult::TypioKeyNotHandled;
    }

    if is_alt {
        if data.shift_chord_pending {
            data.shift_chord_pending = false;
            if data.picker.is_active() {
                data.picker.deactivate();
                data.picker_text_cache.clear();
                unsafe { typio_input_context_clear(ctx) };
            } else {
                data.picker.activate();
                picker_update_composition(data, ctx);
            }
            return TypioKeyProcessResult::TypioKeyHandled;
        }
        return TypioKeyProcessResult::TypioKeyNotHandled;
    }

    data.shift_chord_pending = false;

    if has_blocking_modifiers(event) {
        return TypioKeyProcessResult::TypioKeyNotHandled;
    }

    if data.picker.is_active() {
        return picker_process_key(data, ctx, event);
    }

    let codepoint = event.unicode;
    if codepoint >= 0x20 && codepoint != 0x7F {
        let text = CString::new(encode_utf8_to_string(codepoint)).unwrap_or_default();
        unsafe { typio_input_context_commit(ctx, text.as_ptr()) };
        return TypioKeyProcessResult::TypioKeyCommitted;
    }

    TypioKeyProcessResult::TypioKeyNotHandled
}

extern "C" fn basic_list_modes(_engine: *mut TypioKeyboardEngine, count: *mut usize) -> *const TypioKeyboardEngineMode {
    if count.is_null() {
        return ptr::null();
    }
    unsafe { *count = ENGINE_MODES.len() };
    ENGINE_MODES.as_ptr()
}

extern "C" fn basic_get_active_mode(_engine: *mut TypioKeyboardEngine, _ctx: *mut TypioInputContext) -> *const TypioKeyboardEngineMode {
    &ENGINE_MODES[0]
}

extern "C" fn basic_set_active_mode(_engine: *mut TypioKeyboardEngine, _ctx: *mut TypioInputContext, _mode_id: *const c_char) -> TypioResult {
    TypioResult::TypioOk
}

fn has_blocking_modifiers(event: &TypioKeyEvent) -> bool {
    (event.modifiers & ((TypioModifier::TypioModCtrl as u32)
        | (TypioModifier::TypioModAlt as u32)
        | (TypioModifier::TypioModSuper as u32))) != 0
}

static ENGINE_MODES: [TypioKeyboardEngineMode; 1] = [
    TypioKeyboardEngineMode {
        id: c"compose".as_ptr(),
        label: c"Compose".as_ptr(),
        display_label: c"Abc".as_ptr(),
        icon_name: ptr::null(),
        profile_id: ptr::null(),
        profile_label: ptr::null(),
        description: ptr::null(),
        salience: TypioStatusSalience::TypioStatusSalienceQuiet,
    },
];

/* -------------------------------------------------------------------------- */
/* Compose picker key handling                                                */
/* -------------------------------------------------------------------------- */

extern "C" fn basic_commit_candidate(
    engine: *mut TypioKeyboardEngine,
    ctx: *mut TypioInputContext,
    candidate_index: i32,
) -> TypioResult {
    if engine.is_null() || ctx.is_null() {
        return TypioResult::TypioErrorInvalidArgument;
    }
    let base = unsafe { &mut (*engine).base };
    if base.user_data.is_null() {
        return TypioResult::TypioErrorInvalidArgument;
    }
    let data = unsafe { &mut *(base.user_data as *mut BasicEngineData) };

    let idx = candidate_index as usize;
    if idx >= data.picker.candidates.len() {
        return TypioResult::TypioErrorInvalidArgument;
    }

    let text = CString::new(data.picker.candidates[idx].result_char.clone()).unwrap_or_default();
    data.picker.deactivate();
    data.picker_text_cache.clear();
    unsafe { typio_input_context_clear(ctx) };
    unsafe { typio_input_context_commit(ctx, text.as_ptr()) };
    TypioResult::TypioOk
}

fn picker_process_key(
    data: &mut BasicEngineData,
    ctx: *mut TypioInputContext,
    event: &TypioKeyEvent,
) -> TypioKeyProcessResult {
    let keysym = event.keysym;
    let codepoint = event.unicode;

    if unsafe { typio_key_event_is_escape(event) } {
        data.picker.deactivate();
        data.picker_text_cache.clear();
        unsafe { typio_input_context_clear(ctx) };
        return TypioKeyProcessResult::TypioKeyHandled;
    }

    if keysym == TYPIO_KEY_BackSpace {
        if data.picker.backspace() {
            picker_update_composition(data, ctx);
            return TypioKeyProcessResult::TypioKeyComposing;
        }
        data.picker.deactivate();
        data.picker_text_cache.clear();
        unsafe { typio_input_context_clear(ctx) };
        return TypioKeyProcessResult::TypioKeyHandled;
    }

    /* Space and Enter are host-managed selection keys; do not swallow them
     * as preedit input so the host can commit the selected candidate (Space)
     * or the raw preedit text (Enter). */
    if keysym == TYPIO_KEY_space || keysym == TYPIO_KEY_Return || keysym == TYPIO_KEY_KP_Enter {
        return TypioKeyProcessResult::TypioKeyNotHandled;
    }

    if codepoint >= 0x20 && codepoint != 0x7F {
        data.picker.append_char(codepoint);
        picker_update_composition(data, ctx);
        return TypioKeyProcessResult::TypioKeyComposing;
    }

    TypioKeyProcessResult::TypioKeyNotHandled
}

fn picker_update_composition(
    data: &mut BasicEngineData,
    ctx: *mut TypioInputContext,
) {
    data.picker_text_cache.clear();

    let preedit_cs = CString::new(data.picker.buffer.clone()).unwrap_or_default();
    let preedit_ptr = preedit_cs.as_ptr();
    data.picker_text_cache.push(preedit_cs);

    let cand_count = data.picker.candidates.len();
    for cand in &data.picker.candidates {
        let text_cs = CString::new(cand.result_char.clone()).unwrap_or_default();
        data.picker_text_cache.push(text_cs);
    }

    let mut cand_structs: Vec<TypioCandidate> = Vec::with_capacity(cand_count);
    for i in 0..cand_count {
        let text_idx = 1 + i;
        cand_structs.push(TypioCandidate {
            text: data.picker_text_cache[text_idx].as_ptr(),
            comment: ptr::null(),
            label: ptr::null(),
        });
    }

    let segment = TypioPreeditSegment {
        text: preedit_ptr,
        format: TypioPreeditFormat::TypioPreeditUnderline as u32,
    };

    let comp = TypioComposition {
        struct_size: std::mem::size_of::<TypioComposition>(),
        segments: &segment,
        segment_count: 1,
        cursor_pos: data.picker.buffer.len() as i32,
        candidates: if cand_structs.is_empty() { ptr::null() } else { cand_structs.as_ptr() },
        candidate_count: cand_count,
        page: 0,
        page_size: cand_count as i32,
        total: cand_count as i32,
        selected: data.picker.selected,
        has_prev: false,
        has_next: false,
        content_signature: 0,
        revision: 0,
        host_managed_selection: (TypioHostManagedSelection::TypioHostSelNavigate as u32)
            | (TypioHostManagedSelection::TypioHostSelCommit as u32)
            | (TypioHostManagedSelection::TypioHostSelCommitRaw as u32),
    };

    unsafe { typio_input_context_set_composition(ctx, &comp) };
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
    list_modes: Some(basic_list_modes),
    get_active_mode: Some(basic_get_active_mode),
    set_active_mode: Some(basic_set_active_mode),
    commit_candidate: Some(basic_commit_candidate),
};

static BASIC_ENGINE_INFO: TypioEngineInfo = TypioEngineInfo {
    name: c"basic".as_ptr(),
    display_name: c"Basic".as_ptr(),
    description: c"Basic keyboard engine with Shift+Alt compose picker for Latin characters.".as_ptr(),
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
    fn picker_search_base_a() {
        let mut picker = ComposePicker::new();
        picker.buffer.push('a');
        picker.search();
        assert!(!picker.candidates.is_empty());
        assert!(picker.candidates.iter().any(|c| c.result_char == "á"));
        assert!(picker.candidates.iter().any(|c| c.result_char == "à"));
        assert!(picker.candidates.iter().any(|c| c.result_char == "â"));
        assert!(picker.candidates.iter().any(|c| c.result_char == "ä"));
        assert!(picker.candidates.iter().any(|c| c.result_char == "ã"));
        assert!(picker.candidates.iter().any(|c| c.result_char == "æ"));
    }

    #[test]
    fn picker_search_base_e() {
        let mut picker = ComposePicker::new();
        picker.buffer.push('e');
        picker.search();
        assert!(picker.candidates.iter().any(|c| c.result_char == "é"));
        assert!(picker.candidates.iter().any(|c| c.result_char == "è"));
        assert!(picker.candidates.iter().any(|c| c.result_char == "ê"));
        assert!(picker.candidates.iter().any(|c| c.result_char == "ë"));
        assert!(picker.candidates.iter().any(|c| c.result_char == "œ"));
    }

    #[test]
    fn picker_search_base_n() {
        let mut picker = ComposePicker::new();
        picker.buffer.push('n');
        picker.search();
        assert!(picker.candidates.iter().any(|c| c.result_char == "ñ"));
    }

    #[test]
    fn picker_search_base_s() {
        let mut picker = ComposePicker::new();
        picker.buffer.push('s');
        picker.search();
        assert!(picker.candidates.iter().any(|c| c.result_char == "ß"));
    }

    #[test]
    fn picker_search_base_accent_starter() {
        let mut picker = ComposePicker::new();
        picker.buffer.push('\'');
        picker.search();
        assert!(!picker.candidates.is_empty());
        assert!(picker.candidates.iter().any(|c| c.result_char == "á"));
        assert!(picker.candidates.iter().any(|c| c.result_char == "é"));
        assert!(picker.candidates.iter().any(|c| c.result_char == "'"));
    }

    #[test]
    fn picker_search_exact_match() {
        let mut picker = ComposePicker::new();
        picker.buffer.push('\'');
        picker.buffer.push('a');
        picker.search();
        assert_eq!(picker.candidates.len(), 1);
        assert_eq!(picker.candidates[0].result_char, "á");
    }

    #[test]
    fn picker_search_no_match() {
        let mut picker = ComposePicker::new();
        picker.buffer.push('z');
        picker.buffer.push('q');
        picker.search();
        assert!(picker.candidates.is_empty());
        assert_eq!(picker.selected, -1);
    }

    #[test]
    fn picker_search_extra_rules() {
        let mut picker = ComposePicker::new();
        picker.buffer.push('+');
        picker.search();
        assert!(picker.candidates.iter().any(|c| c.result_char == "±"));

        picker.buffer.clear();
        picker.buffer.push('=');
        picker.search();
        assert!(picker.candidates.iter().any(|c| c.result_char == "≡"));

        picker.buffer.clear();
        picker.buffer.push('o');
        picker.search();
        assert!(picker.candidates.iter().any(|c| c.result_char == "°"));
        assert!(picker.candidates.iter().any(|c| c.result_char == "œ"));
    }

    #[test]
    fn picker_backspace() {
        let mut picker = ComposePicker::new();
        picker.buffer.push('\'');
        picker.buffer.push('a');
        picker.search();
        assert_eq!(picker.candidates.len(), 1);

        assert!(picker.backspace());
        assert_eq!(picker.buffer, "'");
        assert!(picker.candidates.len() > 1);

        assert!(picker.backspace());
        assert_eq!(picker.buffer, "");
        assert!(picker.candidates.is_empty());

        assert!(!picker.backspace());
    }

    #[test]
    fn picker_max_buffer() {
        let mut picker = ComposePicker::new();
        picker.append_char(b'a' as u32);
        picker.append_char(b'b' as u32);
        assert_eq!(picker.buffer, "ab");
        picker.append_char(b'c' as u32);
        assert_eq!(picker.buffer, "ab");
    }

    #[test]
    fn picker_activate_deactivate() {
        let mut picker = ComposePicker::new();
        assert!(!picker.is_active());

        picker.activate();
        assert!(picker.is_active());
        assert!(picker.buffer.is_empty());

        picker.append_char(b'a' as u32);
        assert!(!picker.buffer.is_empty());

        picker.deactivate();
        assert!(!picker.is_active());
        assert!(picker.buffer.is_empty());
    }
}

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
            base_keysym: unicode as u32,
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
                keysym: 0xFFE1,
                modifiers: TypioModifier::TypioModShift as u32,
                unicode: 0,
                time: 0,
                is_repeat: false,
                base_keysym: 0xFFE1,
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
}
