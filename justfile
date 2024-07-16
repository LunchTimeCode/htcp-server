set dotenv-load := true

import 'hurl.just'

alias v := verify
alias r := run



bt := '0'
log := "warn"

@_list:
    just --list --unsorted

run *args:
    cargo run {{args}}

[no-exit-message]
@run-q *args:
    cargo run {{args}} 2>&1

install:
    cargo install --path .

# Perform all verifications (compile, test, lint, etc.)
@verify: test lint
    #!/usr/bin/env bash
    just shutdown 
    just shutdown_test_server
    just run_test_server_release 2>&1
    just run-release 2>&1
    just api-test
    just shutdown
    just shutdown_test_server
    echo ------------ verify done! ------------


# Run the tests
test:
    cargo test

# Run the static code analysis
lint:
    cargo fmt --all -- --check
    cargo clippy

fmt:
    cargo fmt

