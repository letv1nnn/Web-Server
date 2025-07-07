#!/usr/bin/env bash

echo "Building the program..."
git clone git@github.com:letv1nnn/Web-Server.git && cd Wev-Server/
cd server && cargo build --release && cd ..
echo "Program has been built successfully."