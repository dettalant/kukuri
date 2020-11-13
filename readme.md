Kukuri
======

A versatile dialog text compiler for game develop.

## Feature

* Modern dialog text language
* Multi, and Editable outputs(Json, GDScript)
* Localization outputs(for gettext `.po`)

## Install

1. Prepare rust compiler(e.g. install from https://rustup.rs/) 
2. `git clone <this repo>` and `cd <this repo directory>`
3. `cargo build --release`
4. Move builded binary from `target/release` to your `$PATH` dir. 
5. `kukuri --help` showed some usages.

## TODO

* Documentation
* Multiple/Separate output
* Make output directory implement
* More efficient export type for godot
* YarnSpinner/Ink script parse method
* Fluent/CSV export
* Custom export template support
