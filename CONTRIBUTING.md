# Contributing

This is a personal practice repo, but if you want to send a PR, here's the deal.

## Dev setup

You need [mise](https://mise.jdx.dev/) and [just](https://just.systems/). Then:

```sh
just bootstrap
```

That installs the Rust toolchain, cargo tools, dependencies, and git hooks.

## Before opening a PR

Run all three locally:

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo nextest run
```

CI runs the same checks; failing locally means failing in CI.

For behavior changes, add a `tests/cases/*.lox` snapshot. Run `cargo nextest run`, then `cargo insta review` to accept the new output.

## Commits

[Conventional Commits](https://www.conventionalcommits.org/). The commit-msg hook enforces this — if `just bootstrap` ran, it's already wired up.

Common prefixes:

- `feat:` — new language feature
- `fix:` — bug fix
- `refactor:` — restructuring with no behavior change
- `test:` — tests only
- `docs:` — docs only
- `chore:` — tooling, deps, build

Keep commits focused. One logical change per commit. The book is structured chapter by chapter; commits roughly tracking that is fine.

## Style

- No `unsafe` (forbidden at the crate level)
- Clippy `pedantic` and `nursery` are warnings; clean them up or `#[allow]` with a comment explaining why
- Prefer `Result` over `panic!` for user-facing errors; `todo!()` is fine for unimplemented book chapters

## Code of Conduct

See [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).
