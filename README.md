# Skell

Skell is a very very very barebone shell written in Rust. It cannot do much unfortunately, but it can be used to open Emacs so it shouldn't matter.

### Support

Basic shell commands like `cd` work. You can launch pretty much anything coved in `PATH`. `grep` and `ag` don't work as of now so it's a big deal breaker.
This was a fun experiment with Rust and by no means is intended to replace your present shell.

### Build Instruction

If you want to take it out for a spin, here is what you do

```bash
https://github.com/Kriyszig/skell.git
cd skell
rustc main.rs
./main
```
