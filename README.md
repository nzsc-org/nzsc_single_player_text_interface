# nzsc_single_player_text_interface
A textual wrapper for [nzsc_single_player](https://github.com/nzsc-org/nzsc_single_player).

Starting with v0.5.0, `nzsc_single_player::single_player_game::SinglePlayerNZSCGame` no longer returns strings that the dependent application can use.
Instead, it returns an `nzsc_single_player::io::Output`, leaving the details of how that `Output` object is displayed up to the dependent application.

Writing a command-line interface would now require much more code, because you now have to implement the stringifiers yourself.

This project is meant to be a drop-in solution for that.

It provides an easy way to convert `nzsc_single_player::io::Output`s to strings (designed to be printed to a command-line):

```rust
extern crate nzsc_single_player_text_interface;
use nzsc_single_player_text_interface::{ question, notification };

let question: nzsc::io::Question = /*...*/;
let notification: nzsc::io::Notification = /*...*/;

let s = question::to_string(&question);
let s = notification::to_string(&notification);
```
