_help:
    @just --list

run_ew_probs_sim *ARGS:
     RUST_LOG=debug RUST_BACKTRACE=1 cargo run --release --bin run_ew_probs_sim -- {{ARGS}}

run_gen_probs *ARGS:
     RUST_LOG=debug RUST_BACKTRACE=1 cargo run --release --bin run_gen_probs -- {{ARGS}}

run_overbroke_sim *ARGS:
     RUST_LOG=debug RUST_BACKTRACE=1 cargo run --release --bin run_overbroke_sim -- {{ARGS}}

run_rank_matrix *ARGS:
     RUST_LOG=debug RUST_BACKTRACE=1 cargo run --release --bin run_rank_matrix -- {{ARGS}}

# run the tests
test:
    cargo test -- --include-ignored
    cargo test --examples
    cargo doc --no-deps
    cargo bench --no-run --profile dev

# run clippy with pedantic checks
clippy:
    cargo clippy -- -D clippy::pedantic -A clippy::must-use-candidate -A clippy::struct-excessive-bools -A clippy::single-match-else -A clippy::inline-always -A clippy::cast-possible-truncation -A clippy::cast-precision-loss -A clippy::items-after-statements

# install Rust
install-rust:
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
