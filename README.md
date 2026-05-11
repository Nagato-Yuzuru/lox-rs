# lox-rs

Me working through _Crafting Interpreters_ in Rust. In Rust because I want the practice.

Don't expect anything to work. I push half-finished chapters.

## Setup

You need [mise](https://mise.jdx.dev/) and [just](https://just.systems/) installed. After that:

```sh
just bootstrap
```

That handles the Rust toolchain, cargo tools (nextest, insta, llvm-cov, expand, flamegraph), dependencies, and git hooks. Don't `cargo install` things globally — let mise pin them per-project so this repo doesn't fight with whatever else you're working on.

## Running things

```sh
cargo run                       # REPL
cargo run -- script.lox         # run a file
cargo nextest run               # tests
cargo insta review              # accept new snapshot output
```

## Tests

Drop a `.lox` file into `tests/cases/`. The integration runner picks it up via `insta::glob!` and snapshots whatever `lox::run` produces (or panics with, while things are still `todo!()`). Review with `cargo insta review`.

## Commits

Conventional Commits, enforced by a commit-msg hook. If you got here from cloning, `just bootstrap` already installed it; otherwise `prek install`.

## License

MIT.
