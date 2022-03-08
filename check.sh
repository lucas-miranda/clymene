#!/bin/bash

cargo clippy --all-targets -- -D warnings && cargo fmt --all -- --check
