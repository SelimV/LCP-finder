#!/bin/bash

echo cargo +nightly fmt
cargo +nightly fmt

echo cargo +nightly clippy
cargo +nightly clippy