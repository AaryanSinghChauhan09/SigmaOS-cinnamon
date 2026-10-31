# SigmaOS Cinnamon

Rust data models for Cinnamon-style applets, panels, layout, theme, and
configuration. `CinnamonDesktopManager` stores these values in memory and
provides add, remove, enable, lookup, and listing operations.

## Integration limits

This crate does not create a graphical panel, load Cinnamon applets, render a
theme, persist configuration, or connect to a display server. A successful
change to an in-memory model is not evidence that a desktop setting was
applied. Keep user overrides, keyboard navigation, and session recovery in the
eventual desktop integration contract.

## Development

The crate depends on `SigmaOS-kernel` through a sibling path dependency. CI
resolves SigmaOS path dependencies and runs `cargo test --all-targets --locked`.
