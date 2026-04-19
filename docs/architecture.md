# Slater Architecture

## Purpose

This document fixes the initial project structure for `slater`, a static site generator written in Rust.

The goals of this structure are:

- keep CLI concerns separate from site generation logic
- make the core engine reusable as a library
- avoid a vague catch-all module like `core`
- leave clear extension points for templates, development server, and file watching

## Recommended Layout

```text
slater/
├── Cargo.toml
├── docs/
│   └── architecture.md
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── fs.rs
│   ├── error.rs
│   ├── cmd/
│   │   ├── mod.rs
│   │   ├── build.rs
│   │   ├── serve.rs
│   │   ├── new.rs
│   │   └── init.rs
│   ├── content/
│   │   ├── mod.rs
│   │   ├── post.rs
│   │   ├── front_matter.rs
│   │   └── parser.rs
│   ├── render/
│   │   ├── mod.rs
│   │   ├── template.rs
│   │   └── builder.rs
│   └── dev/
│       ├── mod.rs
│       ├── server.rs
│       └── watch.rs
└── templates/
```

## Module Responsibilities

### `src/main.rs`

Executable entry point.

Responsibilities:

- parse CLI arguments
- initialize logging and runtime setup
- dispatch to subcommands

Non-goals:

- no markdown parsing
- no template rendering
- no direct site build orchestration beyond calling command handlers

### `src/lib.rs`

Library entry point.

Responsibilities:

- expose reusable public APIs
- re-export stable types needed by external callers
- provide a clean boundary between binary and library usage

Typical examples:

- `build(config)`
- `load_config(path)`
- `serve(config)`

### `src/cmd/`

CLI command handlers. Each file maps to one user-facing command.

Responsibilities:

- translate CLI input into typed options
- call the correct library-level workflow
- present user-facing output and errors

Files:

- `build.rs`: full site build
- `serve.rs`: development server plus rebuild loop
- `new.rs`: generate a new post scaffold
- `init.rs`: initialize a new site project

Rule:

This layer coordinates workflows. It does not implement the engine itself.

### `src/config.rs`

Site configuration model and loading logic.

Responsibilities:

- define `SiteConfig`
- load config from disk
- apply defaults
- validate required fields
- separate user config from derived runtime config when needed

Expected examples:

- site title
- base URL
- content and output directories
- theme settings
- markdown options
- development server settings

### `src/content/`

Content ingestion and domain modeling for source files.

This module answers one question: what does the source content mean?

#### `src/content/post.rs`

Content entities and strongly typed domain models.

Responsibilities:

- define `Post`
- define `PostMeta`
- define tags, slugs, dates, summaries, and related metadata types

Rule:

Prefer keeping parsing logic out of this file. This module should mostly hold data structures and domain invariants.

#### `src/content/front_matter.rs`

Front matter model and validation.

Responsibilities:

- represent front matter input
- deserialize YAML or TOML front matter
- validate required or optional metadata
- normalize values such as tags or slug overrides

Why separate it:

Front matter rules change independently from markdown body parsing. Keeping them isolated prevents `parser.rs` from becoming a dumping ground.

#### `src/content/parser.rs`

Source parsing pipeline.

Responsibilities:

- read markdown source
- split front matter from body
- parse markdown into an internal representation
- derive summary, table of contents, reading time, and slug when needed
- turn source files into `Post` values

Rule:

This layer should convert raw files into structured content, not render HTML pages for final output.

### `src/render/`

Site rendering and build orchestration.

This module answers one question: how does structured content become output files?

#### `src/render/template.rs`

Template engine integration.

Responsibilities:

- initialize the template engine
- load built-in templates and optionally project templates
- register filters, helpers, and globals
- render pages from typed input models

Rule:

Given content plus context, return rendered output. Avoid mixing file scanning or watcher logic into this module.

#### `src/render/builder.rs`

Top-level site build orchestration.

Responsibilities:

- load configuration and runtime context
- gather content inputs
- call parsers
- sort and group posts
- render all output pages
- write files to the output directory
- copy static assets
- later generate RSS, sitemap, feeds, archive pages, or taxonomy pages

Rule:

This file coordinates the build. It should not become a monolith that manually implements every sub-step inline.

### `src/dev/`

Development-only workflows.

This module contains features that matter during authoring, but are not part of the content domain model.

#### `src/dev/server.rs`

Local preview server.

Responsibilities:

- serve generated output locally
- expose endpoints for browser refresh or preview helpers
- optionally support websocket-based live reload

#### `src/dev/watch.rs`

File watching and rebuild triggers.

Responsibilities:

- watch content, templates, static files, and config changes
- trigger rebuilds
- notify the preview server or browser when output changes

Rule:

Development tooling belongs here, not in `render` or `content`.

### `src/fs.rs`

Shared filesystem utilities.

Responsibilities:

- scan content directories
- create or clean output directories
- copy static assets
- normalize paths
- support incremental build checks later

Why separate it:

Filesystem code becomes repetitive quickly. Centralizing it avoids leaking low-level path and IO logic into every module.

### `src/error.rs`

Shared error model.

Responsibilities:

- define the project-wide error type
- map IO, parse, config, template, and server failures into a consistent shape
- expose a common `Result<T>` alias if desired

Rule:

Use this module to keep error handling coherent across the codebase instead of scattering unstructured errors everywhere.

## Templates Directory

`templates/` is reserved for built-in templates that ship with the generator, or for a default theme embedded into the binary.

Guideline:

- use this directory for bundled defaults
- do not confuse it with a user project's runtime theme directory unless that is an explicit design decision

If runtime user templates are supported later, they should be treated as project input, while `templates/` in this repository remains an implementation asset.

## Dependency Direction

Keep dependencies moving inward and downward in responsibility.

Preferred direction:

- `main` depends on `cmd`
- `cmd` depends on library modules
- `render` depends on `content`, `config`, `fs`, and `error`
- `dev` depends on `render`, `fs`, `config`, and `error`
- `content` depends on `config` and `error` only when necessary

Avoid:

- `content` depending on `dev`
- `template` depending on CLI parsing
- `main` directly owning business logic

## Typical Build Flow

The normal `build` path should look like this:

1. `main.rs` parses arguments.
2. `cmd/build.rs` translates them into build options.
3. `config.rs` loads and validates configuration.
4. `fs.rs` discovers source inputs.
5. `content/parser.rs` converts files into `Post` values.
6. `render/builder.rs` coordinates rendering.
7. `render/template.rs` renders final pages.
8. `fs.rs` writes outputs and copies assets.

## Typical Serve Flow

The normal `serve` path should look like this:

1. `main.rs` parses arguments.
2. `cmd/serve.rs` starts the development workflow.
3. `render/builder.rs` performs an initial build.
4. `dev/server.rs` serves the output directory.
5. `dev/watch.rs` watches for changes and triggers rebuilds.

## Why Not `core/`

`core/` sounds reasonable early on, but it usually becomes a catch-all bucket.

Problems with a `core` directory:

- responsibilities become unclear
- unrelated modules accumulate in one place
- new contributors cannot tell domain code from support code
- development-only features often leak into supposed engine code

Using names like `content`, `render`, and `dev` forces clearer ownership.

## When To Split Further

Start with the structure above, then split only when pressure appears.

Good reasons to split:

- `parser.rs` grows to include multiple independent parsing stages
- `builder.rs` starts handling feeds, taxonomies, pagination, and asset pipelines
- you add more content types such as pages, notes, docs, or projects
- the template system needs filters, globals, loaders, and theme resolution as separate modules

Possible future splits:

- `content/page.rs`
- `render/feed.rs`
- `render/taxonomy.rs`
- `render/pagination.rs`
- `dev/reload.rs`

## Architectural Rules

These rules should remain stable unless the design changes intentionally.

- keep CLI code thin
- keep content parsing separate from rendering
- keep development tooling separate from the build engine
- prefer explicit module names over generic buckets
- keep shared IO and error handling centralized
- add modules because responsibilities changed, not just because file count increased

## Summary

This structure is intended to support three modes of use:

- command-line application
- reusable Rust library
- maintainable long-term codebase

The key design decision is to separate:

- command handling
- content understanding
- output rendering
- development tooling

That boundary will matter much more than the exact file count.
