#!/usr/bin/env bash

echo "Cleaning..."
cd server/ && cargo clean && cd ..
echo "Compiled executables has been cleaned"