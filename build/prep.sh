#!/usr/bin/env bash

echo "Building the program..."
cd Wev-Server/server && cargo build --release && cd ..
echo "Program has been built successfully."
