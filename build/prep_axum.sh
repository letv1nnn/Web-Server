#!/usr/bin/env bash

echo "Building the program..."
cd axum-based-server/ && cargo build --release && cd ..
echo "Program has been built successfully."
