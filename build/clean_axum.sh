#!/usr/bin/env bash

echo "Cleaning..."
cd axum-based-server/ && cargo clean && cd ..
echo "Compiled executables has been cleaned"