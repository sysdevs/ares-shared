//! Server trigger types: `(name, id)` in the compiler's declaration order
//! (`ServerTriggerType.ALL`), then the engine-only `ai_walktrigger`. Ids are not
//! unique: `if_button1..5`/`if_buttond` alias `inv_button1..5`/`inv_buttond`.

macro_rules! triggers {
    ($($name:ident = $id:literal $ident:literal,)*) => {
        $(pub const $name: i32 = $id;)*

        /// `(name, id)` in declaration order.
        pub const TRIGGERS: &[(&str, i32)] = &[$(($ident, $id),)*];
    };
}

triggers! {
    PROC = 0 "proc",
    LABEL = 1 "label",
    DEBUGPROC = 2 "debugproc",
    APNPC1 = 3 "apnpc1",
    APNPC2 = 4 "apnpc2",
    APNPC3 = 5 "apnpc3",
    APNPC4 = 6 "apnpc4",
    APNPC5 = 7 "apnpc5",
    APNPCU = 8 "apnpcu",
    APNPCT = 9 "apnpct",
    OPNPC1 = 10 "opnpc1",
    OPNPC2 = 11 "opnpc2",
    OPNPC3 = 12 "opnpc3",
    OPNPC4 = 13 "opnpc4",
    OPNPC5 = 14 "opnpc5",
    OPNPCU = 15 "opnpcu",
    OPNPCT = 16 "opnpct",
    AI_APNPC1 = 17 "ai_apnpc1",
    AI_APNPC2 = 18 "ai_apnpc2",
    AI_APNPC3 = 19 "ai_apnpc3",
    AI_APNPC4 = 20 "ai_apnpc4",
    AI_APNPC5 = 21 "ai_apnpc5",
    AI_OPNPC1 = 24 "ai_opnpc1",
    AI_OPNPC2 = 25 "ai_opnpc2",
    AI_OPNPC3 = 26 "ai_opnpc3",
    AI_OPNPC4 = 27 "ai_opnpc4",
    AI_OPNPC5 = 28 "ai_opnpc5",
    APOBJ1 = 31 "apobj1",
    APOBJ2 = 32 "apobj2",
    APOBJ3 = 33 "apobj3",
    APOBJ4 = 34 "apobj4",
    APOBJ5 = 35 "apobj5",
    APOBJU = 36 "apobju",
    APOBJT = 37 "apobjt",
    OPOBJ1 = 38 "opobj1",
    OPOBJ2 = 39 "opobj2",
    OPOBJ3 = 40 "opobj3",
    OPOBJ4 = 41 "opobj4",
    OPOBJ5 = 42 "opobj5",
    OPOBJU = 43 "opobju",
    OPOBJT = 44 "opobjt",
    AI_APOBJ1 = 45 "ai_apobj1",
    AI_APOBJ2 = 46 "ai_apobj2",
    AI_APOBJ3 = 47 "ai_apobj3",
    AI_APOBJ4 = 48 "ai_apobj4",
    AI_APOBJ5 = 49 "ai_apobj5",
    AI_OPOBJ1 = 52 "ai_opobj1",
    AI_OPOBJ2 = 53 "ai_opobj2",
    AI_OPOBJ3 = 54 "ai_opobj3",
    AI_OPOBJ4 = 55 "ai_opobj4",
    AI_OPOBJ5 = 56 "ai_opobj5",
    APLOC1 = 59 "aploc1",
    APLOC2 = 60 "aploc2",
    APLOC3 = 61 "aploc3",
    APLOC4 = 62 "aploc4",
    APLOC5 = 63 "aploc5",
    APLOCU = 64 "aplocu",
    APLOCT = 65 "aploct",
    OPLOC1 = 66 "oploc1",
    OPLOC2 = 67 "oploc2",
    OPLOC3 = 68 "oploc3",
    OPLOC4 = 69 "oploc4",
    OPLOC5 = 70 "oploc5",
    OPLOCU = 71 "oplocu",
    OPLOCT = 72 "oploct",
    AI_APLOC1 = 73 "ai_aploc1",
    AI_APLOC2 = 74 "ai_aploc2",
    AI_APLOC3 = 75 "ai_aploc3",
    AI_APLOC4 = 76 "ai_aploc4",
    AI_APLOC5 = 77 "ai_aploc5",
    AI_OPLOC1 = 80 "ai_oploc1",
    AI_OPLOC2 = 81 "ai_oploc2",
    AI_OPLOC3 = 82 "ai_oploc3",
    AI_OPLOC4 = 83 "ai_oploc4",
    AI_OPLOC5 = 84 "ai_oploc5",
    APPLAYER1 = 87 "applayer1",
    APPLAYER2 = 88 "applayer2",
    APPLAYER3 = 89 "applayer3",
    APPLAYER4 = 90 "applayer4",
    APPLAYER5 = 91 "applayer5",
    APPLAYERU = 92 "applayeru",
    APPLAYERT = 93 "applayert",
    OPPLAYER1 = 94 "opplayer1",
    OPPLAYER2 = 95 "opplayer2",
    OPPLAYER3 = 96 "opplayer3",
    OPPLAYER4 = 97 "opplayer4",
    OPPLAYER5 = 98 "opplayer5",
    OPPLAYERU = 99 "opplayeru",
    OPPLAYERT = 100 "opplayert",
    AI_APPLAYER1 = 101 "ai_applayer1",
    AI_APPLAYER2 = 102 "ai_applayer2",
    AI_APPLAYER3 = 103 "ai_applayer3",
    AI_APPLAYER4 = 104 "ai_applayer4",
    AI_APPLAYER5 = 105 "ai_applayer5",
    AI_OPPLAYER1 = 108 "ai_opplayer1",
    AI_OPPLAYER2 = 109 "ai_opplayer2",
    AI_OPPLAYER3 = 110 "ai_opplayer3",
    AI_OPPLAYER4 = 111 "ai_opplayer4",
    AI_OPPLAYER5 = 112 "ai_opplayer5",
    QUEUE = 116 "queue",
    AI_QUEUE1 = 117 "ai_queue1",
    AI_QUEUE2 = 118 "ai_queue2",
    AI_QUEUE3 = 119 "ai_queue3",
    AI_QUEUE4 = 120 "ai_queue4",
    AI_QUEUE5 = 121 "ai_queue5",
    AI_QUEUE6 = 122 "ai_queue6",
    AI_QUEUE7 = 123 "ai_queue7",
    AI_QUEUE8 = 124 "ai_queue8",
    AI_QUEUE9 = 125 "ai_queue9",
    AI_QUEUE10 = 126 "ai_queue10",
    AI_QUEUE11 = 127 "ai_queue11",
    AI_QUEUE12 = 128 "ai_queue12",
    AI_QUEUE13 = 129 "ai_queue13",
    AI_QUEUE14 = 130 "ai_queue14",
    AI_QUEUE15 = 131 "ai_queue15",
    AI_QUEUE16 = 132 "ai_queue16",
    AI_QUEUE17 = 133 "ai_queue17",
    AI_QUEUE18 = 134 "ai_queue18",
    AI_QUEUE19 = 135 "ai_queue19",
    AI_QUEUE20 = 136 "ai_queue20",
    SOFTTIMER = 137 "softtimer",
    TIMER = 138 "timer",
    AI_TIMER = 139 "ai_timer",
    OPHELD1 = 140 "opheld1",
    OPHELD2 = 141 "opheld2",
    OPHELD3 = 142 "opheld3",
    OPHELD4 = 143 "opheld4",
    OPHELD5 = 144 "opheld5",
    OPHELDU = 145 "opheldu",
    OPHELDT = 146 "opheldt",
    IF_BUTTON = 147 "if_button",
    IF_CLOSE = 148 "if_close",
    INV_BUTTON1 = 149 "inv_button1",
    IF_BUTTON1 = 149 "if_button1",
    INV_BUTTON2 = 150 "inv_button2",
    IF_BUTTON2 = 150 "if_button2",
    INV_BUTTON3 = 151 "inv_button3",
    IF_BUTTON3 = 151 "if_button3",
    INV_BUTTON4 = 152 "inv_button4",
    IF_BUTTON4 = 152 "if_button4",
    INV_BUTTON5 = 153 "inv_button5",
    IF_BUTTON5 = 153 "if_button5",
    INV_BUTTOND = 154 "inv_buttond",
    IF_BUTTOND = 154 "if_buttond",
    WALKTRIGGER = 155 "walktrigger",
    LOGIN = 157 "login",
    LOGOUT = 158 "logout",
    TUTORIAL = 159 "tutorial",
    ADVANCESTAT = 160 "advancestat",
    MAPZONE = 161 "mapzone",
    MAPZONEEXIT = 162 "mapzoneexit",
    ZONE = 163 "zone",
    ZONEEXIT = 164 "zoneexit",
    CHANGESTAT = 165 "changestat",
    AI_SPAWN = 166 "ai_spawn",
    AI_DESPAWN = 167 "ai_despawn",
    AI_WALKTRIGGER = 156 "ai_walktrigger",
}

/// The name of a trigger id (the first declared, so `inv_button1` for 149).
pub fn name(id: i32) -> Option<&'static str> {
    TRIGGERS.iter().find(|&&(_, v)| v == id).map(|&(n, _)| n)
}

/// The id of a trigger name; a compile error when used in a const and the name is unknown.
pub const fn id(name: &str) -> i32 {
    let mut i = 0;
    while i < TRIGGERS.len() {
        if crate::str_eq(TRIGGERS[i].0, name) {
            return TRIGGERS[i].1;
        }
        i += 1;
    }
    panic!("unknown trigger")
}

/// What a trigger script is bound to, as part of its lookup key.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Subject {
    Global,
    Category(i32),
    Type(i32),
}

/// A script's lookup key: the trigger id (below 256) in the low byte, then
/// `1 << 8 | category << 10` or `2 << 8 | type << 10` (the shift wraps).
/// Name-mode triggers (procs, labels, ...) are keyed `-1` instead.
pub const fn lookup_key(trigger: i32, subject: Subject) -> i32 {
    match subject {
        Subject::Global => trigger,
        Subject::Category(c) => trigger | 1 << 8 | c.wrapping_shl(10),
        Subject::Type(t) => trigger | 2 << 8 | t.wrapping_shl(10),
    }
}

/// The trigger id of a lookup key.
pub const fn trigger_of(key: i32) -> i32 {
    key & 0xff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aliases_resolve_to_the_first_name() {
        assert_eq!(id("if_button1"), INV_BUTTON1);
        assert_eq!(name(149), Some("inv_button1"));
        assert_eq!(name(AI_WALKTRIGGER), Some("ai_walktrigger"));
    }

    #[test]
    fn lookup_keys() {
        assert!(TRIGGERS.iter().all(|&(_, id)| (0..256).contains(&id)));
        assert_eq!(lookup_key(OPNPC1, Subject::Type(5)), OPNPC1 | 0x200 | 5 << 10);
        assert_eq!(lookup_key(OPNPC1, Subject::Category(-1)), OPNPC1 | 0x100 | -1 << 10);
        assert_eq!(trigger_of(lookup_key(OPNPC1, Subject::Type(1 << 22))), OPNPC1);
    }
}
