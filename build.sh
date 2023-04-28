#!/bin/bash
cd "$(dirname "$0")"
cargo run test .wav 2 24 48000 8