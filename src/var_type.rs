//! Var type codes, in two dialects that agree on every shared name except
//! `npc_uid` and `npc_stat`:
//!
//! - bytecode ([`PRIMITIVE`] and [`SCRIPT`]): codes written into compiled scripts
//!   (pushed type constants, `DEFINE_ARRAY`, `DEBUGPROC` parameters, queue argument
//!   strings), one byte per code.
//! - config (the `i32` constants and [`get_type`]/[`get_type_char`]): codes written
//!   into pack files by `type=` lines and read back by the engine. It adds `autoint`
//!   and `varp`, whose 86 is `writeinv` in the bytecode dialect.
//!
//! Codes are plain bytes. They started out as chars (`'i'` int, `'s'` string), which
//! is why most are ASCII letters. Bytecode codes must be unique across [`PRIMITIVE`]
//! and [`SCRIPT`], and config codes within the config dialect: a duplicate fails the
//! build, naming the later type. To add a type, append a row with any free code.

/// The compiler's primitive types as `(representation, code)`.
pub const PRIMITIVE: &[(&str, u8)] = &[
    ("int", 105),
    ("boolean", 49),
    ("coord", 99),
    ("string", 115),
    ("char", 122),
    ("long", 207),
    ("mapzone", 48),
];

/// Invokes `$then! { NAME "representation" = code, ... }` with every script var
/// type, in declaration order (a type's position fixes its compiler `TypeId`, so
/// append new types at the end). `NAME` is the compiler's `TypeId` const.
#[macro_export]
macro_rules! script_var_types {
    ($then:ident) => {
        $then! {
            SEQ "seq" = 65,
            LOC_SHAPE "locshape" = 72,
            COMPONENT "component" = 73,
            IDKIT "idkit" = 75,
            MIDI "midi" = 77,
            NPC_MODE "npc_mode" = 78,
            NAMEDOBJ "namedobj" = 79,
            SYNTH "synth" = 80,
            AREA "area" = 82,
            STAT "stat" = 83,
            NPC_STAT "npc_stat" = 84,
            WRITEINV "writeinv" = 86,
            MAPAREA "wma" = 96,
            GRAPHIC "graphic" = 100,
            FONTMETRICS "fontmetrics" = 102,
            ENUM "enum" = 103,
            HUNT "hunt" = 104,
            JINGLE "jingle" = 106,
            LOC "loc" = 108,
            MODEL "model" = 109,
            NPC "npc" = 110,
            OBJ "obj" = 111,
            PLAYER_UID "player_uid" = 112,
            SPOTANIM "spotanim" = 116,
            NPC_UID "npc_uid" = 117,
            INV "inv" = 118,
            TEXTURE "texture" = 120,
            CATEGORY "category" = 121,
            MAPELEMENT "mapelement" = 181,
            HITMARK "hitmark" = 215,
            STRUCT "struct" = 74,
            DBROW "dbrow" = 208,
            INTERFACE "interface" = 97,
            TOPLEVELINTERFACE "toplevelinterface" = 70,
            OVERLAYINTERFACE "overlayinterface" = 76,
            MOVESPEED "movespeed" = 221,
            ENTITYOVERLAY "entityoverlay" = 45,
            DBTABLE "dbtable" = 216,
            STRINGVECTOR "stringvector" = 184,
            MESANIM "mesanim" = 193,
            VERIFY_OBJECT "verifyobj" = 174,
            HITSPLAT "hitsplat" = 81,
        }
    };
}

macro_rules! script_table {
    ($($name:ident $rep:literal = $code:literal,)*) => {
        /// Every script var type as `(representation, code)`, in declaration order.
        pub const SCRIPT: &[(&str, u8)] = &[$(($rep, $code),)*];
    };
}
script_var_types!(script_table);

/// `NAME "representation" [= code]`: a config constant and its [`CONFIG`] row.
/// Without `= code` the code is the bytecode one.
macro_rules! config_types {
    ($($name:ident $rep:literal $(= $code:literal)?,)*) => {
        $(pub const $name: i32 = config_types!(@code $rep $($code)?);)*
        const CONFIG: &[(&str, i32)] = &[$(($rep, $name),)*];
    };
    (@code $rep:literal) => { bytecode($rep) as i32 };
    (@code $rep:literal $code:literal) => { $code };
}

config_types! {
    INT "int",
    AUTOINT "autoint" = 255,
    STRING "string",
    ENUM "enum",
    OBJ "obj",
    LOC "loc",
    COMPONENT "component",
    NAMEDOBJ "namedobj",
    STRUCT "struct",
    BOOLEAN "boolean",
    COORD "coord",
    CATEGORY "category",
    SPOTANIM "spotanim",
    NPC "npc",
    INV "inv",
    SYNTH "synth",
    SEQ "seq",
    STAT "stat",
    VARP "varp" = 86,
    PLAYER_UID "player_uid",
    NPC_UID "npc_uid" = 78,
    INTERFACE "interface",
    NPC_STAT "npc_stat" = 254,
    IDKIT "idkit",
    DBROW "dbrow",
    MIDI "midi",
    HITSPLAT "hitsplat",
}

/// Row `i` of [`PRIMITIVE`] followed by [`SCRIPT`].
const fn row(i: usize) -> (&'static str, u8) {
    if i < PRIMITIVE.len() { PRIMITIVE[i] } else { SCRIPT[i - PRIMITIVE.len()] }
}

const _: () = {
    // Bytecode codes: unique across PRIMITIVE and SCRIPT.
    let n = PRIMITIVE.len() + SCRIPT.len();
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n {
            if row(i).1 == row(j).1 {
                panic!("{}", row(j).0); // duplicate bytecode code
            }
            j += 1;
        }
        i += 1;
    }
    // Config codes: unique within CONFIG.
    let mut i = 0;
    while i < CONFIG.len() {
        let mut j = i + 1;
        while j < CONFIG.len() {
            if CONFIG[i].1 == CONFIG[j].1 {
                panic!("{}", CONFIG[j].0); // duplicate config code
            }
            j += 1;
        }
        i += 1;
    }
};

/// The bytecode code of a [`PRIMITIVE`] or [`SCRIPT`] type; a compile error when
/// used in a const and the name is unknown.
pub const fn bytecode(name: &str) -> u8 {
    let mut i = 0;
    while i < PRIMITIVE.len() {
        if crate::str_eq(PRIMITIVE[i].0, name) {
            return PRIMITIVE[i].1;
        }
        i += 1;
    }
    SCRIPT[script_index(name)].1
}

/// The position of a [`SCRIPT`] type; a compile error when used in a const and the name is unknown.
pub const fn script_index(name: &str) -> usize {
    let mut i = 0;
    while i < SCRIPT.len() {
        if crate::str_eq(SCRIPT[i].0, name) {
            return i;
        }
        i += 1;
    }
    panic!("unknown script var type")
}

/// `ScriptVarType.getType(code)` (config dialect): `"unknown"` for anything else.
pub fn get_type(code: i32) -> &'static str {
    CONFIG.iter().find(|(_, c)| *c == code).map_or("unknown", |(n, _)| n)
}

/// `ScriptVarType.getTypeChar(name)` (config dialect): `None` for an unknown name.
pub fn get_type_char(name: &str) -> Option<i32> {
    CONFIG.iter().find(|(n, _)| *n == name).map(|(_, c)| *c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialects_differ_only_where_documented() {
        let differ: Vec<&str> = CONFIG
            .iter()
            .filter(|(n, c)| SCRIPT.iter().any(|(s, code)| s == n && i32::from(*code) != *c))
            .map(|(n, _)| *n)
            .collect();
        assert_eq!(differ, ["npc_uid", "npc_stat"]);
        assert_eq!(get_type(VARP), "varp");
        assert_eq!(i32::from(bytecode("writeinv")), VARP);
    }

    /// The codes are the original chars (Latin-1), byte for byte.
    #[test]
    fn codes_are_the_original_chars() {
        let chars = |t: &[(&str, u8)]| t.iter().map(|r| char::from(r.1)).collect::<String>();
        assert_eq!(chars(PRIMITIVE), "i1cszÏ0");
        assert_eq!(chars(SCRIPT), "AHIKMNOPRSTV`dfghjlmnoptuvxyµ×JÐaFLÝ-Ø¸Á®Q");
    }
}
