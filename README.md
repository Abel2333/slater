# Slater

`slater` is a static site generator written in Rust.

The project is currently in the early scaffolding stage. The repository already has:

- a CLI wired with `clap`
- an initial module layout for content, rendering, development tooling, and shared infrastructure
- an `init` command that can scaffold a new site from bundled starter files
- a design document that fixes the intended architecture

What is not finished yet:

- config parsing from `slater.toml`
- markdown rendering
- template engine integration
- development server and file watching
- real `build` and `new` workflows

## Status

This repository is for building the generator itself, not for hosting a blog site directly.

At the moment:

- `init` is the only command with meaningful behavior
- `build`, `serve`, and `new` are still placeholders
- the project layout is intentionally separated into `cmd`, `content`, `render`, and `dev`

## Project Layout

```text
slater/
├── assets/
│   └── init/          # starter site files used by `slater init`
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
└── templates/         # reserved for built-in templates/default theme assets
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

The starter currently includes:

- `slater.toml`
- `content/hello-world.md`
- `templates/base.html`
- `templates/index.html`
- `templates/post.html`
- `static/style.css`

## Commands

```text
slater build
slater serve
slater new
slater init [target_dir] [--title <title>] [--force]
```

Current behavior:

- `init` creates a starter site directory from `assets/init/`
- `build` is a scaffold only
- `serve` is a scaffold only
- `new` is a scaffold only

## Starter Files

The files under `assets/init/` are the source of truth for the generated starter site.

This keeps `init` simple:

- read starter assets from the repository
- create the target directory structure
- write the starter files into the target site
- replace the default title in `slater.toml` when `--title` is provided

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

- load and validate `slater.toml`
- parse front matter and markdown content
- render posts and an index page
- copy static assets into the output directory
- implement a real development server
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
