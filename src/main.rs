use std::fs::File;
use std::io::{self, Read, Write};
use std::os::fd::FromRawFd;

const ENGINE_PROTOCOL_FD: i32 = 3;
const FRAME_MAGIC: u32 = 0x5459_4550;
const PROTOCOL_MAJOR: u16 = 1;
const PROTOCOL_MINOR: u16 = 0;
const MAX_PAYLOAD_LEN: usize = 1 << 20;
const HEADER_LEN: usize = 28;

const MSG_ENGINE_HELLO: u32 = 1;
const MSG_HOST_HELLO: u32 = 2;
const MSG_REQUEST: u32 = 3;
const MSG_RESPONSE: u32 = 4;
const MSG_ERROR: u32 = 6;

const MOD_CTRL: u32 = 1 << 1;
const MOD_ALT: u32 = 1 << 2;
const MOD_SUPER: u32 = 1 << 3;

const KEY_BACKSPACE: u32 = 0xff08;
const KEY_RETURN: u32 = 0xff0d;
const KEY_KP_ENTER: u32 = 0xff8d;
const KEY_ESCAPE: u32 = 0xff1b;
const KEY_SPACE: u32 = 0x20;
const KEY_SHIFT_L: u32 = 0xffe1;
const KEY_SHIFT_R: u32 = 0xffe2;
const KEY_ALT_L: u32 = 0xffe9;
const KEY_ALT_R: u32 = 0xffea;

const HOST_SEL_ALL: u32 = 0x0f;

const COMPOSE_RULES: &[(u32, u32, u32)] = &[
    (b'\'' as u32, b'A' as u32, 0x00C1),
    (b'\'' as u32, b'C' as u32, 0x0106),
    (b'\'' as u32, b'E' as u32, 0x00C9),
    (b'\'' as u32, b'G' as u32, 0x01F4),
    (b'\'' as u32, b'I' as u32, 0x00CD),
    (b'\'' as u32, b'L' as u32, 0x0139),
    (b'\'' as u32, b'N' as u32, 0x0143),
    (b'\'' as u32, b'O' as u32, 0x00D3),
    (b'\'' as u32, b'R' as u32, 0x0154),
    (b'\'' as u32, b'S' as u32, 0x015A),
    (b'\'' as u32, b'U' as u32, 0x00DA),
    (b'\'' as u32, b'Y' as u32, 0x00DD),
    (b'\'' as u32, b'Z' as u32, 0x0179),
    (b'\'' as u32, b'a' as u32, 0x00E1),
    (b'\'' as u32, b'c' as u32, 0x0107),
    (b'\'' as u32, b'e' as u32, 0x00E9),
    (b'\'' as u32, b'g' as u32, 0x01F5),
    (b'\'' as u32, b'i' as u32, 0x00ED),
    (b'\'' as u32, b'k' as u32, 0x1E31),
    (b'\'' as u32, b'l' as u32, 0x013A),
    (b'\'' as u32, b'm' as u32, 0x1E3F),
    (b'\'' as u32, b'n' as u32, 0x0144),
    (b'\'' as u32, b'o' as u32, 0x00F3),
    (b'\'' as u32, b'p' as u32, 0x1E55),
    (b'\'' as u32, b'r' as u32, 0x0155),
    (b'\'' as u32, b's' as u32, 0x015B),
    (b'\'' as u32, b'u' as u32, 0x00FA),
    (b'\'' as u32, b'y' as u32, 0x00FD),
    (b'\'' as u32, b'z' as u32, 0x017A),
    (b'`' as u32, b'A' as u32, 0x00C0),
    (b'`' as u32, b'E' as u32, 0x00C8),
    (b'`' as u32, b'I' as u32, 0x00CC),
    (b'`' as u32, b'O' as u32, 0x00D2),
    (b'`' as u32, b'U' as u32, 0x00D9),
    (b'`' as u32, b'a' as u32, 0x00E0),
    (b'`' as u32, b'e' as u32, 0x00E8),
    (b'`' as u32, b'i' as u32, 0x00EC),
    (b'`' as u32, b'o' as u32, 0x00F2),
    (b'`' as u32, b'u' as u32, 0x00F9),
    (b'^' as u32, b'A' as u32, 0x00C2),
    (b'^' as u32, b'E' as u32, 0x00CA),
    (b'^' as u32, b'I' as u32, 0x00CE),
    (b'^' as u32, b'O' as u32, 0x00D4),
    (b'^' as u32, b'U' as u32, 0x00DB),
    (b'^' as u32, b'a' as u32, 0x00E2),
    (b'^' as u32, b'e' as u32, 0x00EA),
    (b'^' as u32, b'i' as u32, 0x00EE),
    (b'^' as u32, b'o' as u32, 0x00F4),
    (b'^' as u32, b'u' as u32, 0x00FB),
    (b'"' as u32, b'A' as u32, 0x00C4),
    (b'"' as u32, b'E' as u32, 0x00CB),
    (b'"' as u32, b'I' as u32, 0x00CF),
    (b'"' as u32, b'O' as u32, 0x00D6),
    (b'"' as u32, b'U' as u32, 0x00DC),
    (b'"' as u32, b'Y' as u32, 0x0178),
    (b'"' as u32, b'a' as u32, 0x00E4),
    (b'"' as u32, b'e' as u32, 0x00EB),
    (b'"' as u32, b'i' as u32, 0x00EF),
    (b'"' as u32, b'o' as u32, 0x00F6),
    (b'"' as u32, b'u' as u32, 0x00FC),
    (b'"' as u32, b'y' as u32, 0x00FF),
    (b'~' as u32, b'A' as u32, 0x00C3),
    (b'~' as u32, b'N' as u32, 0x00D1),
    (b'~' as u32, b'O' as u32, 0x00D5),
    (b'~' as u32, b'a' as u32, 0x00E3),
    (b'~' as u32, b'n' as u32, 0x00F1),
    (b'~' as u32, b'o' as u32, 0x00F5),
    (b',' as u32, b'C' as u32, 0x00C7),
    (b',' as u32, b'c' as u32, 0x00E7),
    (b'/' as u32, b'L' as u32, 0x0141),
    (b'/' as u32, b'O' as u32, 0x00D8),
    (b'/' as u32, b'l' as u32, 0x0142),
    (b'/' as u32, b'o' as u32, 0x00F8),
    (b'?' as u32, b'?' as u32, 0x00BF),
    (b'!' as u32, b'!' as u32, 0x00A1),
    (b'<' as u32, b'<' as u32, 0x00AB),
    (b'>' as u32, b'>' as u32, 0x00BB),
    (b'\'' as u32, b'\'' as u32, 0x0027),
    (b'`' as u32, b'`' as u32, 0x0060),
    (b'"' as u32, b'"' as u32, 0x0022),
    (b'^' as u32, b'^' as u32, 0x005E),
    (b'~' as u32, b'~' as u32, 0x007E),
    (b',' as u32, b',' as u32, 0x002C),
    (b'-' as u32, b'-' as u32, 0x2013),
    (b'-' as u32, b'=' as u32, 0x2014),
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

#[derive(Default)]
struct Worker {
    picker: ComposePicker,
    shift_chord_pending: bool,
}

#[derive(Default)]
struct ComposePicker {
    active: bool,
    buffer: String,
    candidates: Vec<String>,
    selected: i32,
}

impl ComposePicker {
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

    fn append_char(&mut self, codepoint: u32) {
        if self.buffer.chars().count() >= 2 {
            return;
        }
        self.buffer
            .push(char::from_u32(codepoint).unwrap_or('\u{fffd}'));
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

    fn search(&mut self) {
        self.candidates.clear();
        self.selected = -1;
        let chars: Vec<u32> = self.buffer.chars().map(|c| c as u32).collect();
        match chars.as_slice() {
            [base] => {
                for &(first, second, result) in COMPOSE_RULES {
                    if first == *base || second == *base {
                        self.candidates.push(codepoint_to_string(result));
                    }
                }
            }
            [first, second] => {
                for &(rule_first, rule_second, result) in COMPOSE_RULES {
                    if rule_first == *first && rule_second == *second {
                        self.candidates.push(codepoint_to_string(result));
                    }
                }
            }
            _ => {}
        }
        if !self.candidates.is_empty() {
            self.selected = 0;
        }
    }
}

#[derive(Clone, Copy)]
struct KeyEvent {
    is_press: bool,
    keysym: u32,
    modifiers: u32,
    unicode: u32,
}

fn main() {
    let fd = std::env::var("TYPIO_ENGINE_FD")
        .ok()
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(ENGINE_PROTOCOL_FD);
    let mut protocol = unsafe { File::from_raw_fd(fd) };
    let mut worker = Worker::default();

    if write_frame(
        &mut protocol,
        MSG_ENGINE_HELLO,
        0,
        engine_hello().as_bytes(),
    )
    .is_err()
    {
        return;
    }
    let Ok(host_hello) = read_frame(&mut protocol) else {
        return;
    };
    if host_hello.message_type != MSG_HOST_HELLO {
        let _ = write_frame(
            &mut protocol,
            MSG_ERROR,
            host_hello.request_id,
            b"expected host hello",
        );
        return;
    }

    loop {
        let Ok(frame) = read_frame(&mut protocol) else {
            break;
        };
        if frame.message_type != MSG_REQUEST {
            let _ = write_frame(
                &mut protocol,
                MSG_ERROR,
                frame.request_id,
                b"expected request",
            );
            continue;
        }
        let Ok(line) = String::from_utf8(frame.payload) else {
            let _ = write_frame(
                &mut protocol,
                MSG_ERROR,
                frame.request_id,
                b"invalid request utf-8",
            );
            continue;
        };
        if line == "shutdown" {
            break;
        }
        let mut response = Vec::new();
        worker.handle_request(&line, &mut response);
        let _ = write_frame(&mut protocol, MSG_RESPONSE, frame.request_id, &response);
    }
}

struct Frame {
    message_type: u32,
    request_id: u64,
    payload: Vec<u8>,
}

fn engine_hello() -> &'static str {
    "protocol\t1.0\nengine\tbasic\ntype\tkeyboard"
}

fn read_frame(reader: &mut impl Read) -> io::Result<Frame> {
    let mut header = [0u8; HEADER_LEN];
    reader.read_exact(&mut header)?;
    let magic = u32::from_be_bytes(header[0..4].try_into().unwrap());
    if magic != FRAME_MAGIC {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "bad frame magic",
        ));
    }
    let major = u16::from_be_bytes(header[4..6].try_into().unwrap());
    if major != PROTOCOL_MAJOR {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unsupported protocol major",
        ));
    }
    let message_type = u32::from_be_bytes(header[8..12].try_into().unwrap());
    let request_id = u64::from_be_bytes(header[16..24].try_into().unwrap());
    let payload_len = u32::from_be_bytes(header[24..28].try_into().unwrap()) as usize;
    if payload_len > MAX_PAYLOAD_LEN {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "oversized frame",
        ));
    }
    let mut payload = vec![0u8; payload_len];
    reader.read_exact(&mut payload)?;
    Ok(Frame {
        message_type,
        request_id,
        payload,
    })
}

fn write_frame(
    writer: &mut impl Write,
    message_type: u32,
    request_id: u64,
    payload: &[u8],
) -> io::Result<()> {
    if payload.len() > MAX_PAYLOAD_LEN {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "oversized frame",
        ));
    }
    let mut header = [0u8; HEADER_LEN];
    header[0..4].copy_from_slice(&FRAME_MAGIC.to_be_bytes());
    header[4..6].copy_from_slice(&PROTOCOL_MAJOR.to_be_bytes());
    header[6..8].copy_from_slice(&PROTOCOL_MINOR.to_be_bytes());
    header[8..12].copy_from_slice(&message_type.to_be_bytes());
    header[12..16].copy_from_slice(&0u32.to_be_bytes());
    header[16..24].copy_from_slice(&request_id.to_be_bytes());
    header[24..28].copy_from_slice(&(payload.len() as u32).to_be_bytes());
    writer.write_all(&header)?;
    writer.write_all(payload)?;
    writer.flush()
}

impl Worker {
    fn handle_request(&mut self, line: &str, stdout: &mut impl Write) {
        let mut fields = line.split('\t');
        let emit_mode = match fields.next().unwrap_or("") {
            "init" | "deactivate" | "focus-out" | "reload-config" => {
                let _ = writeln!(stdout, "OK");
                false
            }
            "focus-in" | "set-active-mode" => {
                let _ = writeln!(stdout, "OK");
                true
            }
            "reset" => {
                self.reset(stdout);
                let _ = writeln!(stdout, "OK");
                true
            }
            "availability" => {
                let _ = writeln!(stdout, "AVAILABILITY\tREADY");
                false
            }
            "list-modes" => {
                write_mode("MODE", stdout);
                false
            }
            "get-active-mode" => {
                write_mode("ACTIVE_MODE", stdout);
                false
            }
            "commit-candidate" => {
                let _ctx = fields.next();
                let index = fields
                    .next()
                    .and_then(|v| v.parse::<usize>().ok())
                    .unwrap_or(0);
                if self.commit_candidate(index, stdout) {
                    let _ = writeln!(stdout, "OK");
                } else {
                    let _ = writeln!(stdout, "ERR\tinvalid candidate");
                }
                false
            }
            "process-key" => {
                let _ctx = fields.next();
                let state = fields.next().unwrap_or("");
                let _keycode = fields.next();
                let event = KeyEvent {
                    is_press: state == "press",
                    keysym: parse_u32(fields.next()),
                    modifiers: parse_u32(fields.next()),
                    unicode: parse_u32(fields.next()),
                };
                self.process_key(event, stdout);
                true
            }
            _ => {
                let _ = writeln!(stdout, "ERR\tunknown request");
                false
            }
        };
        if emit_mode {
            write_mode("ACTIVE_MODE", stdout);
        }
        write_end(stdout);
    }

    fn process_key(&mut self, event: KeyEvent, stdout: &mut impl Write) {
        let is_shift = event.keysym == KEY_SHIFT_L || event.keysym == KEY_SHIFT_R;
        let is_alt = event.keysym == KEY_ALT_L || event.keysym == KEY_ALT_R;

        if !event.is_press {
            if is_shift {
                self.shift_chord_pending = false;
            }
            let _ = writeln!(stdout, "RESULT\tNOT_HANDLED");
            return;
        }

        if is_shift {
            self.shift_chord_pending = true;
            let _ = writeln!(stdout, "RESULT\tNOT_HANDLED");
            return;
        }

        if is_alt {
            if self.shift_chord_pending {
                self.shift_chord_pending = false;
                if self.picker.active {
                    self.reset(stdout);
                } else {
                    self.picker.activate();
                    self.write_composition(stdout);
                }
                let _ = writeln!(stdout, "RESULT\tHANDLED");
            } else {
                let _ = writeln!(stdout, "RESULT\tNOT_HANDLED");
            }
            return;
        }

        self.shift_chord_pending = false;
        if has_blocking_modifiers(event.modifiers) {
            let _ = writeln!(stdout, "RESULT\tNOT_HANDLED");
            return;
        }

        if self.picker.active {
            self.process_picker_key(event, stdout);
            return;
        }

        if event.unicode >= 0x20 && event.unicode != 0x7f {
            let text = codepoint_to_string(event.unicode);
            let _ = writeln!(stdout, "RESULT\tCOMMITTED");
            let _ = writeln!(stdout, "COMMIT\t{}", hex_encode(text.as_bytes()));
        } else {
            let _ = writeln!(stdout, "RESULT\tNOT_HANDLED");
        }
    }

    fn process_picker_key(&mut self, event: KeyEvent, stdout: &mut impl Write) {
        if event.keysym == KEY_ESCAPE {
            self.reset(stdout);
            let _ = writeln!(stdout, "RESULT\tHANDLED");
            return;
        }

        if event.keysym == KEY_BACKSPACE {
            if self.picker.backspace() {
                self.write_composition(stdout);
                let _ = writeln!(stdout, "RESULT\tCOMPOSING");
            } else {
                self.reset(stdout);
                let _ = writeln!(stdout, "RESULT\tHANDLED");
            }
            return;
        }

        if event.keysym == KEY_SPACE || event.keysym == KEY_RETURN || event.keysym == KEY_KP_ENTER {
            let _ = writeln!(stdout, "RESULT\tNOT_HANDLED");
            return;
        }

        if !self.picker.candidates.is_empty() && (0x30..=0x39).contains(&event.keysym) {
            let _ = writeln!(stdout, "RESULT\tNOT_HANDLED");
            return;
        }

        if event.unicode >= 0x20 && event.unicode != 0x7f {
            self.picker.append_char(event.unicode);
            self.write_composition(stdout);
            let _ = writeln!(stdout, "RESULT\tCOMPOSING");
        } else {
            let _ = writeln!(stdout, "RESULT\tNOT_HANDLED");
        }
    }

    fn commit_candidate(&mut self, index: usize, stdout: &mut impl Write) -> bool {
        let Some(text) = self.picker.candidates.get(index).cloned() else {
            return false;
        };
        self.picker.deactivate();
        let _ = writeln!(stdout, "COMMIT\t{}", hex_encode(text.as_bytes()));
        true
    }

    fn reset(&mut self, stdout: &mut impl Write) {
        if self.picker.active {
            let _ = writeln!(stdout, "CLEAR");
        }
        self.picker.deactivate();
    }

    fn write_composition(&self, stdout: &mut impl Write) {
        let segments = hex_encode(self.picker.buffer.as_bytes());
        let candidates = self
            .picker
            .candidates
            .iter()
            .map(|s| hex_encode(s.as_bytes()))
            .collect::<Vec<_>>()
            .join(",");
        let cursor = self.picker.buffer.chars().count();
        let count = self.picker.candidates.len();
        let _ = writeln!(
            stdout,
            "COMPOSITION\t{cursor}\t0\t{count}\t{count}\t{}\t0\t0\t{HOST_SEL_ALL}\t{segments}\t{candidates}",
            self.picker.selected
        );
    }
}

fn write_mode(prefix: &str, stdout: &mut impl Write) {
    let _ = writeln!(
        stdout,
        "{prefix}\t{}\t{}\t{}\t\t\t\t\t1\t0",
        hex_encode(b"compose"),
        hex_encode(b"Compose"),
        hex_encode(b"Abc")
    );
}

fn write_end(stdout: &mut impl Write) {
    let _ = writeln!(stdout, "END");
    let _ = stdout.flush();
}

fn has_blocking_modifiers(modifiers: u32) -> bool {
    (modifiers & (MOD_CTRL | MOD_ALT | MOD_SUPER)) != 0
}

fn parse_u32(value: Option<&str>) -> u32 {
    value.and_then(|v| v.parse::<u32>().ok()).unwrap_or(0)
}

fn codepoint_to_string(codepoint: u32) -> String {
    char::from_u32(codepoint).unwrap_or('\u{fffd}').to_string()
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picker_search_base_a() {
        let mut picker = ComposePicker::default();
        picker.append_char('a' as u32);
        assert!(picker.candidates.iter().any(|c| c == "á"));
        assert!(picker.candidates.iter().any(|c| c == "à"));
        assert!(picker.candidates.iter().any(|c| c == "â"));
        assert!(picker.candidates.iter().any(|c| c == "ä"));
        assert!(picker.candidates.iter().any(|c| c == "ã"));
        assert!(picker.candidates.iter().any(|c| c == "æ"));
    }

    #[test]
    fn picker_exact_sequence() {
        let mut picker = ComposePicker::default();
        picker.append_char('\'' as u32);
        picker.append_char('e' as u32);
        assert_eq!(picker.candidates, vec!["é"]);
    }
}
