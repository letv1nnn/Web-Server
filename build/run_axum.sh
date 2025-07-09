#!/usr/bin/env bash

echo "Running the server..."
cd axum-based-server/

if [[ "$(uname -s)" =~ MINGW*|MSYS*|CYGWIN* ]]; then
    ./target/release/axum-based-server.exe
else
    ./target/release/axum-based-server
fi

cd ..