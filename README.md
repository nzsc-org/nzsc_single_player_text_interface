# nzsc_single_player_text_interface
A textual wrapper for [nzsc_single_player](https://github.com/nzsc-org/nzsc_single_player).

Starting with v0.5.0, [nzsc_single_player](https://github.com/nzsc-org/nzsc_single_player) no longer provides strings that the dependent application can use.
Instead, it provides raw `nzsc::io` data, leaving the details of how the data is displayed up to the dependent application.

This project is meant to convert `nzsc::io` data to strings for printing to a command-line.
