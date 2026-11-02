# ares-shared

The script ABI shared by the [ares-gameserver](https://github.com/sysdevs/ares-gameserver) game server
and the [ares-compiler](https://github.com/sysdevs/ares-compiler) compiler: opcodes and their pointer
rules, triggers, pointer types, var type codes, and the compiled script file
format. The compiler writes scripts with these values and the server reads
them back, so both crates take every value from here.

It has no dependencies and no I/O. Most of it is `const` data.

## Usage

```toml
[dependencies]
ares-shared = { path = "../ares-shared" }
```

Decode a compiled script (one `script.dat` group):

```rust
use ares_shared::script::ScriptFile;
use ares_shared::{opcode, trigger};

fn dump(id: usize, bytes: Vec<u8>) -> Result<(), String> {
    let script = ScriptFile::decode(id, bytes)?;
    println!("{} ({})", script.name(), script.file_name());
    println!("trigger {:?}", trigger::name(trigger::trigger_of(script.info.lookup_key)));
    for (pc, &op) in script.opcodes.iter().enumerate() {
        println!("{pc:5} {}", opcode::name(op).unwrap_or("?"));
    }
    Ok(())
}
```

## Modules

| Module | Contents |
|---|---|
| `opcode` | Opcode constants (`op::*`), `OPCODES` name table, operand size (`large_operand`) and per-command pointer rules (`POINTERS`) |
| `pointer` | `PointerType`: what a command requires, sets or corrupts, and what a trigger starts with |
| `trigger` | Trigger ids and names, `Subject`, and script lookup keys (`lookup_key`, `trigger_of`) |
| `var_type` | Var type codes in two dialects (see below) |
| `script` | `ScriptFile::decode` for the compiled script layout, plus pc-to-line lookup |

### Var type dialects

Type codes come in two dialects that agree on every shared name except
`npc_uid` and `npc_stat`:

- **bytecode** (`PRIMITIVE`, `SCRIPT`): codes the compiler writes into scripts
  (type constants, `DEFINE_ARRAY`, `DEBUGPROC` parameters, queue arguments).
- **config** (the `i32` constants, `get_type`, `get_type_char`): codes written
  into pack files by `type=` lines and read by the server. It adds `autoint`
  and `varp`.

Duplicate codes fail the build. To add a type, append a row with a free code.
Order in `SCRIPT` fixes the compiler's `TypeId`, so never insert in the middle.

## Changing anything

Every value here ends up in compiled script or pack bytes. A change is an
output change for both dependents. Run both test suites, and ares-compiler's
`content_matches` gate:

```
cargo test                                  # here
cd ../ares-compiler && cargo test --release
cd ../ares-gameserver && cargo test --release
```

If the change is intentional, re-bless the snapshots in each crate (see their
`docs/testing.md`) and review `git diff tests/`.

## Development

```
cargo build
cargo test
cargo clippy --all-targets
```
