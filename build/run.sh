#!/usr/bin/env bash

echo "Running the server..."
cd server

if [[ "$OS" == "Windows_NT" ]]; then
    ./target/release/server.exe
else
    ./target/release/server
fi

cd ..