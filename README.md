# egui_layout_job_macro

[![Crates.io Version](https://img.shields.io/crates/v/egui_layout_job_macro)](https://crates.io/crates/egui_lyaout_job_macro)
[![Github Version](https://img.shields.io/github/v/release/iiiii7d/egui_layout_job_macro)](https://github.com/iiiii7d/egui_layout_job_macro)
![Crates.io MSRV](https://img.shields.io/crates/msrv/egui_layout_job_macro)
[![docs.rs](https://img.shields.io/docsrs/egui_layout_job_macro)](https://docs.rs/egui_layout_job_macro)
![GitHub License](https://img.shields.io/github/license/iiiii7d/egui_layout_job_macro)

![GitHub code size](https://img.shields.io/github/languages/code-size/iiiii7d/egui_layout_job_macro)
![GitHub repo size](https://img.shields.io/github/repo-size/iiiii7d/egui_layout_job_macro)
![GitHub last commit (branch)](https://img.shields.io/github/last-commit/iiiii7d/egui_layout_job_macro/main)
![GitHub commits since latest release (branch)](https://img.shields.io/github/commits-since/iiiii7d/egui_layout_job_macro/latest/main?include_prereleases)
![GitHub Release Date](https://img.shields.io/github/release-date/iiiii7d/egui_layout_job_macro)
![Libraries.io dependency status for GitHub repo](https://img.shields.io/librariesio/github/iiiii7d/egui_layout_job_macro)

![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/egui_layout_job_macro)
![Crates.io Total Downloads](https://img.shields.io/crates/d/egui_layout_job_macro)

> As you can see, constructing a LayoutJob is currently a lot of work. It would be nice to have a helper macro for it! *~ egui docs*

Macros for [egui](https://github.com/emilk/egui) `LayoutJob` and `TextFormat`

## Installation
```bash
cargo add egui_layout_job_macro
```
Macros are tested on the egui pinned in `Cargo.toml`, but it should theoretically work on any past and future releases.

## Usage
```rust
use egui_layout_job_macro::layout_job;

layout_job!(@size[14](@white("Hello ") @mono(@black("World!"))));
```
See the [documentation](https://docs.rs/egui_layout_job_macro) and [`example.rs`](https://github.com/iiiii7d/egui_layout_job_macro/blob/master/examples/example.rs) for more examples.