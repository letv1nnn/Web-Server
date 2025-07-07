#!/usr/bin/env bash

echo "Building the program..."
cd /server && cargo build --release && cd ..
echo "Program has been built successfully."
