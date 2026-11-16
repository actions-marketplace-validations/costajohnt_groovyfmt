# Development

```sh
scripts/ci-local.sh [corpus-dir]         # everything CI checks on Linux; run it before merging
cargo test                               # unit and end-to-end CLI tests
cargo clippy --all-targets -- -D warnings
groovyfmt --stats <dir>                  # corpus report, including an idempotence check
groovyfmt --errors <dir>                 # first parse error per failing file, for grammar work
groovyfmt --tree <file>                  # dump the parse tree
```

## Changing the grammar

The grammar in `grammar/` is a copy, so don't edit it here. To change it:

1. Fix `grammar.js` or `src/scanner.c` in a [tree-sitter-groovy](https://github.com/costajohnt/tree-sitter-groovy) checkout.
2. Run `tree-sitter generate` and its tests there, and commit.
3. Run `scripts/update-grammar.sh <checkout> [rev]` here. The checkout must be clean, and `rev` defaults to its HEAD.
4. Compare `--stats` and `--lint` on the corpus before and after.

## Releases

Push a `v*` tag on `main` matching the `Cargo.toml` version and with a CHANGELOG heading. The release workflow publishes only after those checks, fmt, clippy, the tests on stable and Rust 1.90, and a build and smoke test of all five release binaries pass. [Releasing](releasing.md) has the details.
