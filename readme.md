Kukuri
======

A versatile dialog text compiler for game develop.

## Feature

* Modern dialog text language
* Multiple, and Editable outputs(Json, GDScript)
* Localization outputs(for gettext `.po`)

## Install

1. Prepare rust compiler(e.g. install from https://rustup.rs/) 
2. `git clone <this repo>` and `cd <this repo directory>`
3. `cargo build --release`
4. Move builded binary from `target/release` to your `$PATH` dir. 
5. `kukuri --help` showed some usages.

## Example

1. move this repo dir(e.g. `cd <this repo directory>`)
2. run `kukuri -c ./examples/kukuri_example_ja/config.toml ./examples/kukuri_example_ja/kukuri_dialog1_ja.md`
3. compiled dialog texts exported to `./examples/kukuri_example_ja/export` and `./examples/kukuri_example_ja/locale`

## TODO

* Documentation
* Multiple/Separate output
* More efficient export type for godot
* YarnSpinner/Ink script parse method
* Fluent/CSV export
* Custom export template support
* Rust-nized error handling
