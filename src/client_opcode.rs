//! Client script command opcodes (the CS2 dialect), numbered as Jagex's client
//! numbers them. Rows marked `// ares` have no Jagex number and live at 20000+.
//! Core instructions (`PUSH_CONSTANT_INT`, branches, locals, ...) are the shared
//! ones in [`crate::opcode`]. Every command here carries a one-byte operand
//! (`1` selects the secondary `.` form).

use crate::opcode::Pointers;
use crate::pointer::PointerType::*;

macro_rules! client_opcodes {
    ($($name:ident = $id:literal $text:literal,)*) => {
        /// The client opcodes by name.
        pub mod op {
            $(pub const $name: i32 = $id;)*
        }

        /// `(name, opcode)` in opcode order.
        pub const CLIENT_OPCODES: &[(&str, i32)] = &[$(($text, $id),)*];
    };
}

client_opcodes! {
    CC_CREATE = 100 "cc_create",
    CC_DELETE = 101 "cc_delete",
    CC_DELETEALL = 102 "cc_deleteall",
    CC_FIND = 200 "cc_find",
    IF_FIND = 201 "if_find",
    CC_SETPOSITION = 1000 "cc_setposition",
    CC_SETSIZE = 1001 "cc_setsize",
    CC_SETHIDE = 1003 "cc_sethide",
    CC_SETNOCLICKTHROUGH = 1005 "cc_setnoclickthrough",
    CC_SETSCROLLPOS = 1100 "cc_setscrollpos",
    CC_SETCOLOUR = 1101 "cc_setcolour",
    CC_SETFILL = 1102 "cc_setfill",
    CC_SETTRANS = 1103 "cc_settrans",
    CC_SETLINEWID = 1104 "cc_setlinewid",
    CC_SETGRAPHIC = 1105 "cc_setgraphic",
    CC_SETTILING = 1107 "cc_settiling",
    CC_SETMODEL = 1108 "cc_setmodel",
    CC_SETMODELANGLE = 1109 "cc_setmodelangle",
    CC_SETMODELANIM = 1110 "cc_setmodelanim",
    CC_SETMODELORTHOG = 1111 "cc_setmodelorthog",
    CC_SETTEXT = 1112 "cc_settext",
    CC_SETTEXTFONT = 1113 "cc_settextfont",
    CC_SETTEXTALIGN = 1114 "cc_settextalign",
    CC_SETTEXTSHADOW = 1115 "cc_settextshadow",
    CC_SETOUTLINE = 1116 "cc_setoutline",
    CC_SETGRAPHICSHADOW = 1117 "cc_setgraphicshadow",
    CC_SETVFLIP = 1118 "cc_setvflip",
    CC_SETHFLIP = 1119 "cc_sethflip",
    CC_SETSCROLLSIZE = 1120 "cc_setscrollsize",
    CC_RESUME_PAUSEBUTTON = 1121 "cc_resume_pausebutton",
    CC_SETLINEDIRECTION = 1126 "cc_setlinedirection",
    CC_SETOBJECT = 1200 "cc_setobject",
    CC_SETNPCHEAD = 1201 "cc_setnpchead",
    CC_SETPLAYERHEAD_SELF = 1202 "cc_setplayerhead_self",
    CC_SETOBJECT_NONUM = 1205 "cc_setobject_nonum",
    CC_SETOP = 1300 "cc_setop",
    CC_SETOPBASE = 1305 "cc_setopbase",
    CC_SETTARGETVERB = 1306 "cc_settargetverb",
    CC_CLEAROPS = 1307 "cc_clearops",
    CC_SETONCLICK = 1400 "cc_setonclick",
    CC_SETONHOLD = 1401 "cc_setonhold",
    CC_SETONRELEASE = 1402 "cc_setonrelease",
    CC_SETONMOUSEOVER = 1403 "cc_setonmouseover",
    CC_SETONMOUSELEAVE = 1404 "cc_setonmouseleave",
    CC_SETONDRAG = 1405 "cc_setondrag",
    CC_SETONVARTRANSMIT = 1407 "cc_setonvartransmit",
    CC_SETONTIMER = 1408 "cc_setontimer",
    CC_SETONOP = 1409 "cc_setonop",
    CC_SETONDRAGCOMPLETE = 1410 "cc_setondragcomplete",
    CC_SETONMOUSEREPEAT = 1412 "cc_setonmouserepeat",
    CC_SETONINVTRANSMIT = 1414 "cc_setoninvtransmit",
    CC_SETONSTATTRANSMIT = 1415 "cc_setonstattransmit",
    CC_SETONSCROLLWHEEL = 1417 "cc_setonscrollwheel",
    CC_SETONKEY = 1419 "cc_setonkey",
    CC_GETX = 1500 "cc_getx",
    CC_GETY = 1501 "cc_gety",
    CC_GETWIDTH = 1502 "cc_getwidth",
    CC_GETHEIGHT = 1503 "cc_getheight",
    CC_GETHIDE = 1504 "cc_gethide",
    CC_GETLAYER = 1505 "cc_getlayer",
    CC_GETSCROLLX = 1600 "cc_getscrollx",
    CC_GETSCROLLY = 1601 "cc_getscrolly",
    CC_GETTEXT = 1602 "cc_gettext",
    CC_GETSCROLLWIDTH = 1603 "cc_getscrollwidth",
    CC_GETSCROLLHEIGHT = 1604 "cc_getscrollheight",
    CC_GETMODELZOOM = 1605 "cc_getmodelzoom",
    CC_GETTRANS = 1609 "cc_gettrans",
    CC_GETCOLOUR = 1611 "cc_getcolour",
    CC_GETINVOBJECT = 1700 "cc_getinvobject",
    CC_GETINVCOUNT = 1701 "cc_getinvcount",
    CC_GETID = 1702 "cc_getid",
    CC_GETOP = 1801 "cc_getop",
    CC_GETOPBASE = 1802 "cc_getopbase",
    IF_SETPOSITION = 2000 "if_setposition",
    IF_SETSIZE = 2001 "if_setsize",
    IF_SETHIDE = 2003 "if_sethide",
    IF_SETNOCLICKTHROUGH = 2005 "if_setnoclickthrough",
    IF_SETSCROLLPOS = 2100 "if_setscrollpos",
    IF_SETCOLOUR = 2101 "if_setcolour",
    IF_SETFILL = 2102 "if_setfill",
    IF_SETTRANS = 2103 "if_settrans",
    IF_SETLINEWID = 2104 "if_setlinewid",
    IF_SETGRAPHIC = 2105 "if_setgraphic",
    IF_SETTILING = 2107 "if_settiling",
    IF_SETMODEL = 2108 "if_setmodel",
    IF_SETMODELANGLE = 2109 "if_setmodelangle",
    IF_SETMODELANIM = 2110 "if_setmodelanim",
    IF_SETMODELORTHOG = 2111 "if_setmodelorthog",
    IF_SETTEXT = 2112 "if_settext",
    IF_SETTEXTFONT = 2113 "if_settextfont",
    IF_SETTEXTALIGN = 2114 "if_settextalign",
    IF_SETTEXTSHADOW = 2115 "if_settextshadow",
    IF_SETOUTLINE = 2116 "if_setoutline",
    IF_SETGRAPHICSHADOW = 2117 "if_setgraphicshadow",
    IF_SETVFLIP = 2118 "if_setvflip",
    IF_SETHFLIP = 2119 "if_sethflip",
    IF_SETSCROLLSIZE = 2120 "if_setscrollsize",
    IF_RESUME_PAUSEBUTTON = 2121 "if_resume_pausebutton",
    IF_SETLINEDIRECTION = 2126 "if_setlinedirection",
    IF_SETOBJECT = 2200 "if_setobject",
    IF_SETNPCHEAD = 2201 "if_setnpchead",
    IF_SETPLAYERHEAD_SELF = 2202 "if_setplayerhead_self",
    IF_SETOBJECT_NONUM = 2205 "if_setobject_nonum",
    IF_SETOP = 2300 "if_setop",
    IF_SETOPBASE = 2305 "if_setopbase",
    IF_SETTARGETVERB = 2306 "if_settargetverb",
    IF_CLEAROPS = 2307 "if_clearops",
    IF_SETONCLICK = 2400 "if_setonclick",
    IF_SETONHOLD = 2401 "if_setonhold",
    IF_SETONRELEASE = 2402 "if_setonrelease",
    IF_SETONMOUSEOVER = 2403 "if_setonmouseover",
    IF_SETONMOUSELEAVE = 2404 "if_setonmouseleave",
    IF_SETONDRAG = 2405 "if_setondrag",
    IF_SETONVARTRANSMIT = 2407 "if_setonvartransmit",
    IF_SETONTIMER = 2408 "if_setontimer",
    IF_SETONOP = 2409 "if_setonop",
    IF_SETONDRAGCOMPLETE = 2410 "if_setondragcomplete",
    IF_SETONMOUSEREPEAT = 2412 "if_setonmouserepeat",
    IF_SETONINVTRANSMIT = 2414 "if_setoninvtransmit",
    IF_SETONSTATTRANSMIT = 2415 "if_setonstattransmit",
    IF_SETONSCROLLWHEEL = 2417 "if_setonscrollwheel",
    IF_SETONKEY = 2419 "if_setonkey",
    IF_GETX = 2500 "if_getx",
    IF_GETY = 2501 "if_gety",
    IF_GETWIDTH = 2502 "if_getwidth",
    IF_GETHEIGHT = 2503 "if_getheight",
    IF_GETHIDE = 2504 "if_gethide",
    IF_GETLAYER = 2505 "if_getlayer",
    IF_GETSCROLLX = 2600 "if_getscrollx",
    IF_GETSCROLLY = 2601 "if_getscrolly",
    IF_GETTEXT = 2602 "if_gettext",
    IF_GETSCROLLWIDTH = 2603 "if_getscrollwidth",
    IF_GETSCROLLHEIGHT = 2604 "if_getscrollheight",
    IF_GETMODELZOOM = 2605 "if_getmodelzoom",
    IF_GETTRANS = 2609 "if_gettrans",
    IF_GETCOLOUR = 2611 "if_getcolour",
    IF_GETINVOBJECT = 2700 "if_getinvobject",
    IF_GETINVCOUNT = 2701 "if_getinvcount",
    IF_GETID = 2702 "if_getid",
    IF_GETOP = 2801 "if_getop",
    IF_GETOPBASE = 2802 "if_getopbase",
    MES = 3100 "mes",
    IF_CLOSE = 3103 "if_close",
    RESUME_COUNTDIALOG = 3104 "resume_countdialog",
    SOUND_SYNTH = 3200 "sound_synth",
    CLIENTCLOCK = 3300 "clientclock",
    INV_GETOBJ = 3301 "inv_getobj",
    INV_GETNUM = 3302 "inv_getnum",
    INV_TOTAL = 3303 "inv_total",
    INV_SIZE = 3304 "inv_size",
    STAT = 3305 "stat",
    STAT_BASE = 3306 "stat_base",
    STAT_XP = 3307 "stat_xp",
    COORD = 3308 "coord",
    COORDX = 3309 "coordx",
    COORDZ = 3310 "coordz",
    COORDY = 3311 "coordy",
    RUNENERGY_VISIBLE = 3321 "runenergy_visible",
    RUNWEIGHT_VISIBLE = 3322 "runweight_visible",
    ENUM_STRING = 3400 "enum_string",
    ENUM = 3408 "enum",
    ENUM_GETOUTPUTCOUNT = 3411 "enum_getoutputcount",
    ADD = 4000 "add",
    SUB = 4001 "sub",
    MULTIPLY = 4002 "multiply",
    DIVIDE = 4003 "divide",
    RANDOM = 4004 "random",
    RANDOMINC = 4005 "randominc",
    INTERPOLATE = 4006 "interpolate",
    ADDPERCENT = 4007 "addpercent",
    SETBIT = 4008 "setbit",
    CLEARBIT = 4009 "clearbit",
    TESTBIT = 4010 "testbit",
    MODULO = 4011 "modulo",
    POW = 4012 "pow",
    INVPOW = 4013 "invpow",
    AND = 4014 "and",
    OR = 4015 "or",
    MIN = 4016 "min",
    MAX = 4017 "max",
    SCALE = 4018 "scale",
    APPEND_NUM = 4100 "append_num",
    APPEND = 4101 "append",
    APPEND_SIGNNUM = 4102 "append_signnum",
    LOWERCASE = 4103 "lowercase",
    TOSTRING = 4106 "tostring",
    COMPARE = 4107 "compare",
    PARAWIDTH = 4109 "parawidth",
    STRING_LENGTH = 4117 "string_length",
    SUBSTRING = 4118 "substring",
    STRING_INDEXOF_CHAR = 4120 "string_indexof_char",
    STRING_INDEXOF_STRING = 4121 "string_indexof_string",
    OC_NAME = 4200 "oc_name",
    OC_OP = 4201 "oc_op",
    OC_IOP = 4202 "oc_iop",
    OC_COST = 4203 "oc_cost",
    OC_STACKABLE = 4204 "oc_stackable",
    OC_CERT = 4205 "oc_cert",
    OC_UNCERT = 4206 "oc_uncert",
    OC_MEMBERS = 4207 "oc_members",
    OC_PARAM = 6512 "oc_param",
    NC_PARAM = 6513 "nc_param",
    LC_PARAM = 6514 "lc_param",
    STRUCT_PARAM = 6516 "struct_param",
    OC_CATEGORY = 20000 "oc_category", // ares
    NC_NAME = 20001 "nc_name", // ares
    LC_NAME = 20002 "lc_name", // ares
}

/// The name of a client opcode.
pub fn name(opcode: i32) -> Option<&'static str> {
    CLIENT_OPCODES.iter().find(|&&(_, v)| v == opcode).map(|&(n, _)| n)
}

/// The client commands with pointer rules, sorted by opcode: `cc_create` and
/// `cc_find` set the active component, `cc_delete` requires and then corrupts
/// it, `cc_deleteall` (which names its layer) has no rule, every other `cc_*`
/// requires it.
pub const CLIENT_POINTERS: &[(i32, Pointers)] = &[
    (100, Pointers { require: None, require2: None, set: Some(&[ActiveComponent]), set2: Some(&[ActiveComponent2]), corrupt: None, corrupt2: None, conditional: false }),
    (101, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: Some(&[ActiveComponent]), corrupt2: Some(&[ActiveComponent2]), conditional: false }),
    (200, Pointers { require: None, require2: None, set: Some(&[ActiveComponent]), set2: Some(&[ActiveComponent2]), corrupt: None, corrupt2: None, conditional: true }),
    (1000, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1001, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1003, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1005, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1100, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1101, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1102, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1103, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1104, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1105, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1107, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1108, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1109, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1110, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1111, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1112, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1113, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1114, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1115, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1116, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1117, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1118, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1119, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1120, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1121, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1126, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1200, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1201, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1202, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1205, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1300, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1305, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1306, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1307, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1400, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1401, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1402, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1403, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1404, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1405, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1407, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1408, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1409, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1410, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1412, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1414, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1415, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1417, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1419, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1500, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1501, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1502, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1503, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1504, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1505, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1600, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1601, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1602, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1603, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1604, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1605, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1609, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1611, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1700, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1701, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1702, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1801, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
    (1802, Pointers { require: Some(&[ActiveComponent]), require2: Some(&[ActiveComponent2]), set: None, set2: None, corrupt: None, corrupt2: None, conditional: false }),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pointer::PointerType::*;

    #[test]
    fn client_opcodes_are_unique_and_above_core() {
        let mut ids: Vec<i32> = CLIENT_OPCODES.iter().map(|o| o.1).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), CLIENT_OPCODES.len());
        assert!(CLIENT_OPCODES.iter().all(|&(_, id)| id >= 100));
        assert!(CLIENT_OPCODES.iter().all(|&(_, id)| !crate::opcode::large_operand(id)));
    }

    #[test]
    fn client_pointers_sorted_and_named() {
        assert!(CLIENT_POINTERS.windows(2).all(|w| w[0].0 < w[1].0));
        assert!(CLIENT_POINTERS.iter().all(|p| name(p.0).is_some()));
    }

    #[test]
    fn cc_ops_require_component() {
        for &(n, id) in CLIENT_OPCODES.iter().filter(|(n, _)| n.starts_with("cc_")) {
            let p = CLIENT_POINTERS.iter().find(|p| p.0 == id).map(|p| p.1).unwrap_or(Pointers { require: None, require2: None, set: None, set2: None, corrupt: None, corrupt2: None, conditional: false });
            if n == "cc_deleteall" {
                // takes its layer as an argument; the build pattern calls it before any cc_create
                assert!(p.require.is_none() && p.set.is_none() && p.corrupt.is_none(), "{n}");
            } else if n == "cc_delete" {
                assert_eq!(p.require, Some(&[ActiveComponent][..]), "{n}");
                assert_eq!(p.corrupt, Some(&[ActiveComponent][..]), "{n}");
                assert_eq!(p.corrupt2, Some(&[ActiveComponent2][..]), "{n}");
            } else if n == "cc_create" || n == "cc_find" {
                assert_eq!(p.set, Some(&[ActiveComponent][..]), "{n}");
                assert_eq!(p.set2, Some(&[ActiveComponent2][..]), "{n}");
                assert_eq!(p.conditional, n == "cc_find", "{n}");
            } else {
                assert_eq!(p.require, Some(&[ActiveComponent][..]), "{n}");
                assert_eq!(p.require2, Some(&[ActiveComponent2][..]), "{n}");
            }
        }
    }
}
