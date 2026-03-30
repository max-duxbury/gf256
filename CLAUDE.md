# AES / GF(256) — Claude Code Instructions

## Project Context
Rust library implementing AES encryption from scratch, grounded in Galois Theory.
Stack: Rust (no external dependencies beyond std).

## What I'm Learning
- Rust as a language (ownership, types, idioms)
- How Galois Field arithmetic maps to AES internals
- Memory management as complexity increases

## Rules Specific to This Project
Do not write any AES or GF(2^8) implementation code for me. This includes
gf_mul, SubBytes, MixColumns, KeyExpansion, and all other AES components.
Help me understand the theory and point out where my reasoning is wrong,
but the code must be mine.

When I hit Rust language problems (borrow checker, lifetimes, types), walk me
through diagnosis — don't just give me the fix.

When I hit Galois Theory mapping problems, explain the concept one layer at a
time and wait for me to connect it to the implementation myself.

Writing tests against my implementation is fair game — generate freely.

## Roadmap
1. GF(2^8) arithmetic — `gf_add`, `xtime`, `gf_mul` ✓ — multiplicative inverse next
2. AES building blocks — SubBytes, ShiftRows, MixColumns, AddRoundKey
3. Key schedule
4. Single block encrypt/decrypt
5. Block cipher modes (ECB, CBC)
6. Constant-time audit
7. GCM mode
8. Cryptanalysis (differential/linear on reduced-round AES)
9. Ratatui TUI + CLI wrapper

## Key References (point me here first)
- The Rust Book: https://doc.rust-lang.org/book/
- FIPS 197 (AES spec): https://csrc.nist.gov/publications/detail/fips/197/final
- AES explained with GF arithmetic: https://en.wikipedia.org/wiki/Rijndael_MixColumns
