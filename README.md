# Slater

`slater` is a static site generator written in Rust.

The project is currently in the early scaffolding stage. The repository already has:

- a CLI wired with `clap`
- an initial module layout for content, rendering, development tooling, and shared infrastructure
- an `init` command that can scaffold a new site from built-in starter assets and themes
- config loading and validation from `slater.toml`
- a design document that fixes the intended architecture

What is not finished yet:

- file watching and live reload
- real `new` workflow

## Status

This repository is for building the generator itself, not for hosting a blog site directly.

At the moment:

- `init` creates a starter site using common starter content plus a selected built-in theme
- `build` reads markdown content, renders article and index pages from templates, and copies static assets into the configured output directory
- `build` uses project templates when present and falls back to the selected built-in theme for missing template files
- `serve` builds the site and serves the output directory over a local HTTP server
- `new` is still a placeholder
- the project layout is intentionally separated into `cmd`, `content`, `render`, and `dev`

## Project Layout

```text
slater/
├── assets/
│   ├── init/          # common starter files used by `slater init`
│   └── themes/        # built-in themes used by `init` and template fallback
├── docs/
│   └── architecture.md
├── src/
│   ├── cmd/           # CLI subcommands
│   ├── content/       # content models and parsing
│   ├── render/        # rendering and site build orchestration
│   ├── dev/           # development-only workflows
│   ├── config.rs
│   ├── error.rs
│   ├── fs.rs
│   ├── lib.rs
│   └── main.rs
```

More detail is documented in [docs/architecture.md](docs/architecture.md).

## Getting Started

### Build the binary

```bash
cargo build
```

### Show CLI help

```bash
cargo run -- --help
```

### Initialize a site

Create a new site in a target directory:

```bash
cargo run -- init ./my-blog
```

Initialize in the current directory:

```bash
cargo run -- init
```

Override the default site title:

```bash
cargo run -- init ./my-blog --title "My Blog"
```

Allow initialization into a non-empty directory:

```bash
cargo run -- init ./my-blog --force
```

Initialize with a specific built-in theme:

```bash
cargo run -- init ./my-blog --theme minimal
```

The starter currently includes common content plus theme assets:

- `slater.toml`
- `content/hello-world.md`
- `content/about.md`
- `content/writing-process.md`
- `templates/base.html`
- `templates/index.html`
- `templates/post.html`
- `static/style.css`

### Build a site

Generate the static site into the configured `output_dir`:

```bash
cargo run -- build --config ./my-blog/slater.toml
```

For the starter site, this creates:

- `_site/index.html`
- `_site/hello-world/index.html`
- `_site/style.css`

## Commands

```text
slater build
slater serve
slater new
slater init [target_dir] [--title <title>] [--theme <theme>] [--force]
```

Current behavior:

- `init` creates a starter site from common starter assets plus a selected built-in theme
- `build` generates static files from markdown content, project templates, and built-in theme fallback templates
- `serve` serves the built output directory locally and supports `--host` / `--port`
- `new` is a scaffold only

## Themes

Slater now has a built-in theme model.

Theme assets live under `assets/themes/<name>/` and currently include:

- `templates/base.html`
- `templates/index.html`
- `templates/post.html`
- `static/style.css`

The generated site config stores the selected theme:

```toml
theme = "default"
```

Template resolution works like this:

1. Use project templates from `template_dir` when present
2. Fall back to `assets/themes/<theme>/templates/` for missing files
3. Fail only when neither source provides the required template

Starter content shared across all themes lives under `assets/init/`.

## Architecture

The current structure intentionally avoids a generic `core/` module.

The main boundaries are:

- `cmd`: parse user intent and dispatch workflows
- `content`: understand source content
- `render`: produce output files
- `dev`: local preview and watch workflows
- `config`, `fs`, `error`: shared infrastructure

See [docs/architecture.md](docs/architecture.md) for the full rationale.

## Near-Term Plan

- add more built-in themes and theme tooling
- implement file watching and live reload
- add post scaffolding to `slater new`

## Development

Format and check the project with:

```bash
cargo fmt
cargo check
```

## License

This project is licensed under either of the following, at your option:

- [MIT](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)
