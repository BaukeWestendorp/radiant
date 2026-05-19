# Radiant

_An experimental effect based lighting control system written in Rust and [GPUI](https://www.gpui.rs)._

### ⚠️ **DISCLAIMER** ⚠️

This project is in an **experimental** and **very early** stage of development. In it's current state, Radiant is almost unusable for the average user. It does not remotely have a defined feature set and is subject breaking changes without any notice.

With GPUI, the UI framework used for Radiant, still being linked to the [Zed Editor](https://zed.dev) it's poorly documented and breaking changes in the API occur often, resulting in possible changes in behaviour. Still, I've chosen GPUI as the UI framework for Radiant to learn about GPUI and to test it in a codebase other than Zed's. Also it's a really cool framework, do try it out!

## Try it out!

For the time being, the only way to try out Radiant is by building it from source.
You can use `cargo run --release -- examples/capital_inspired` in the project root to build and run.

With exception for some settings, most of the configuraton of a showfile can only be done by manually editing the showfile itself. This is because I want to make sure the functionality is correct before implementing the UI.

![Radiant Editor Add Fixtures](README/highlight.gif)
