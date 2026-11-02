//! The compiled script file (one `script.dat` group): written by the compiler,
//! decoded by the server.
//!
//! Layout (big-endian): `name\0`, `source path\0`, `i32 lookup key`,
//! `u8 count` parameter type codes, `u16 count` `(i32 pc, i32 line)` pairs, the
//! instructions (`u16 opcode`, then a `\0`-terminated string for
//! `PUSH_CONSTANT_STRING`, else an operand sized by [`large_operand`]), then the
//! trailer: `i32 instruction count`, `u16` int locals, string locals, int args,
//! string args, `u8` switch count and per switch `u16 count` `(i32 key, i32
//! offset)` pairs, and last a `u16` trailer length (not counting its own 12
//! fixed bytes or itself). Strings are Latin-1.

use crate::opcode::{large_operand, op};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ScriptInfo {
    pub script_name: String,
    pub source_file_path: String,
    pub lookup_key: i32,
    pub parameter_types: Vec<i32>,
    pub pcs: Vec<i32>,
    pub lines: Vec<i32>,
}

#[derive(Clone, Debug)]
pub struct ScriptFile {
    pub id: usize,
    pub info: ScriptInfo,
    pub int_local_count: i32,
    pub string_local_count: i32,
    pub int_arg_count: i32,
    pub string_arg_count: i32,
    /// `key -> jump offset`; a repeated key keeps the last offset.
    pub switch_tables: Vec<HashMap<i32, i32>>,
    pub opcodes: Vec<i32>,
    /// The operand of every non-string instruction (0 for string pushes).
    pub int_operands: Vec<i32>,
    /// The operand of every `PUSH_CONSTANT_STRING` (`None` elsewhere).
    pub string_operands: Vec<Option<String>>,
}

/// A big-endian cursor; a read past the end is an error.
struct Reader {
    data: Vec<u8>,
    pos: usize,
}

type R<T> = Result<T, String>;

impl Reader {
    fn take<const N: usize>(&mut self) -> R<[u8; N]> {
        let b = self.data.get(self.pos..self.pos + N).ok_or("Invalid script file (truncated)")?;
        self.pos += N;
        Ok(b.try_into().unwrap())
    }

    fn g1(&mut self) -> R<i32> {
        Ok(self.take::<1>()?[0] as i32)
    }

    fn g2(&mut self) -> R<i32> {
        Ok(u16::from_be_bytes(self.take()?) as i32)
    }

    fn g4s(&mut self) -> R<i32> {
        Ok(i32::from_be_bytes(self.take()?))
    }

    /// Up to a `\0`; the last byte of the data also ends the string (and is dropped).
    fn gjstr(&mut self) -> R<String> {
        let mut s = String::new();
        loop {
            let b = self.g1()? as u8;
            if b == 0 || self.pos >= self.data.len() {
                return Ok(s);
            }
            s.push(b as char);
        }
    }
}

impl ScriptFile {
    /// Fails on a truncated file, or when the instructions do not end exactly
    /// at the trailer with its instruction count.
    pub fn decode(id: usize, data: Vec<u8>) -> Result<Self, String> {
        let length = data.len();
        if length < 16 {
            return Err("Invalid script file (minimum length)".into());
        }
        let mut s = Reader { data, pos: length - 2 };
        let trailer_pos = length as isize - s.g2()? as isize - 12 - 2;
        if trailer_pos < 0 || trailer_pos >= length as isize {
            return Err("Invalid script file (bad trailer pos)".into());
        }
        let trailer_pos = trailer_pos as usize;
        s.pos = trailer_pos;
        let instructions = s.g4s()?;
        let (int_local_count, string_local_count, int_arg_count, string_arg_count) = (s.g2()?, s.g2()?, s.g2()?, s.g2()?);
        let switches = s.g1()?;
        let mut switch_tables = Vec::with_capacity(switches as usize);
        for _ in 0..switches {
            let count = s.g2()?;
            let mut table = HashMap::new();
            for _ in 0..count {
                let key = s.g4s()?;
                table.insert(key, s.g4s()?);
            }
            switch_tables.push(table);
        }

        s.pos = 0;
        let script_name = s.gjstr()?;
        let source_file_path = s.gjstr()?;
        let lookup_key = s.g4s()?;
        let parameter_types = (0..s.g1()?).map(|_| s.g1()).collect::<R<_>>()?;
        let (mut pcs, mut lines) = (Vec::new(), Vec::new());
        for _ in 0..s.g2()? {
            pcs.push(s.g4s()?);
            lines.push(s.g4s()?);
        }

        let (mut opcodes, mut int_operands, mut string_operands) = (Vec::new(), Vec::new(), Vec::new());
        while trailer_pos > s.pos {
            let opcode = s.g2()?;
            if opcode == op::PUSH_CONSTANT_STRING {
                string_operands.push(Some(s.gjstr()?));
                int_operands.push(0);
            } else {
                int_operands.push(if large_operand(opcode) { s.g4s()? } else { s.g1()? });
                string_operands.push(None);
            }
            opcodes.push(opcode);
        }
        if s.pos != trailer_pos || opcodes.len() != instructions as usize {
            return Err("Invalid script file (instructions do not match the trailer)".into());
        }

        Ok(ScriptFile {
            id,
            info: ScriptInfo { script_name, source_file_path, lookup_key, parameter_types, pcs, lines },
            int_local_count,
            string_local_count,
            int_arg_count,
            string_arg_count,
            switch_tables,
            opcodes,
            int_operands,
            string_operands,
        })
    }

    pub fn name(&self) -> &str {
        &self.info.script_name
    }

    /// The base name of the source path, following the platform like `Path` does.
    pub fn file_name(&self) -> String {
        std::path::Path::new(&self.info.source_file_path)
            .file_name()
            .map_or_else(|| self.info.source_file_path.clone(), |n| n.to_string_lossy().into_owned())
    }

    /// The line of the last table entry at or before `pc`; `None` when the first entry is already past `pc` or
    /// the table is empty.
    pub fn line_number(&self, pc: i32) -> Option<i32> {
        match self.info.pcs.iter().position(|&p| p > pc) {
            Some(0) => None,
            Some(i) => Some(self.info.lines[i - 1]),
            None => self.info.lines.last().copied(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `[proc,a]` from `a.rs2`: `push_constant_int 7`, `return`.
    fn sample() -> Vec<u8> {
        let mut b = b"[proc,a]\0a.rs2\0".to_vec();
        b.extend((-1i32).to_be_bytes());
        b.extend([0, 0, 0]);
        b.extend((op::PUSH_CONSTANT_INT as u16).to_be_bytes());
        b.extend(7i32.to_be_bytes());
        b.extend((op::RETURN as u16).to_be_bytes());
        b.push(0);
        b.extend(2i32.to_be_bytes());
        b.extend([0; 9]);
        b.extend(1u16.to_be_bytes());
        b
    }

    #[test]
    fn decodes_and_rejects_damage() {
        let f = ScriptFile::decode(0, sample()).unwrap();
        assert_eq!((f.name(), &f.opcodes[..], &f.int_operands[..]), ("[proc,a]", &[op::PUSH_CONSTANT_INT, op::RETURN][..], &[7, 0][..]));
        let mut cut = sample();
        cut.drain(20..24);
        assert!(ScriptFile::decode(0, cut).is_err());
        let mut count = sample();
        let at = count.len() - 12;
        count[at] = 3;
        assert!(ScriptFile::decode(0, count).is_err());
    }
}
