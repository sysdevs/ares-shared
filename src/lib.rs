//! The script ABI shared by the ares-gameserver server and the ares-compiler compiler:
//! opcodes and their pointer rules, triggers, pointer types, var type codes, stat and
//! NPC mode tables and the compiled script file layout. Every value here reaches compiled script or
//! pack bytes, so a change is an output change on both sides.

pub mod client_opcode;
pub mod opcode;
pub mod pointer;
pub mod script;
pub mod tables;
pub mod trigger;
pub mod var_type;

/// `&str` equality usable in a `const fn`.
const fn str_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}
