//! Script pointer types (`PointerType.ts`): what a command requires, sets or
//! corrupts, and what a trigger starts with.

macro_rules! pointer_types {
    ($($variant:ident $key:literal $rep:literal,)*) => {
        /// Declaration order is `PointerType.ALL` order; the discriminant is `PointerType.INDEX`.
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum PointerType { $($variant,)* }

        impl PointerType {
            pub const ALL: &[PointerType] = &[$(PointerType::$variant,)*];

            pub fn representation(self) -> &'static str {
                match self { $(PointerType::$variant => $rep,)* }
            }

            /// `getName()`: the static field name.
            pub fn name(self) -> &'static str {
                match self { $(PointerType::$variant => $key,)* }
            }
        }
    };
}

pointer_types! {
    ActivePlayer "ACTIVE_PLAYER" "active_player",
    ActivePlayer2 "ACTIVE_PLAYER2" ".active_player",
    PActivePlayer "P_ACTIVE_PLAYER" "p_active_player",
    PActivePlayer2 "P_ACTIVE_PLAYER2" ".p_active_player",
    ActiveNpc "ACTIVE_NPC" "active_npc",
    ActiveNpc2 "ACTIVE_NPC2" ".active_npc",
    ActiveLoc "ACTIVE_LOC" "active_loc",
    ActiveLoc2 "ACTIVE_LOC2" ".active_loc",
    ActiveObj "ACTIVE_OBJ" "active_obj",
    ActiveObj2 "ACTIVE_OBJ2" ".active_obj",
    FindPlayer "FIND_PLAYER" "find_player",
    FindNpc "FIND_NPC" "find_npc",
    FindLoc "FIND_LOC" "find_loc",
    FindObj "FIND_OBJ" "find_obj",
    FindDb "FIND_DB" "find_db",
    LastCom "LAST_COM" "last_com",
    LastInt "LAST_INT" "last_int",
    LastItem "LAST_ITEM" "last_item",
    LastSlot "LAST_SLOT" "last_slot",
    LastTargetslot "LAST_TARGETSLOT" "last_targetslot",
    LastUseitem "LAST_USEITEM" "last_useitem",
    LastUseslot "LAST_USESLOT" "last_useslot",
    ActiveInvObj "ACTIVE_INVOBJ" "active_invobj",
    ActiveInvObj2 "ACTIVE_INVOBJ2" ".active_invobj",
}

impl PointerType {
    /// `PointerType.INDEX.get(p)`.
    pub fn index(self) -> usize {
        self as usize
    }

    /// `forName`: matches the static key lowercased (`active_npc2`), not the representation.
    pub fn for_name(name: &str) -> Option<Self> {
        let name = name.to_lowercase();
        Self::ALL.iter().copied().find(|p| p.name().to_lowercase() == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_name_uses_key_not_representation() {
        assert_eq!(PointerType::for_name("active_npc2"), Some(PointerType::ActiveNpc2));
        assert_eq!(PointerType::for_name("ACTIVE_NPC"), Some(PointerType::ActiveNpc));
        assert_eq!(PointerType::for_name(".active_npc"), None);
        assert_eq!(PointerType::ALL.len(), 24);
        assert_eq!(PointerType::LastUseslot.index(), 21);
    }
}
