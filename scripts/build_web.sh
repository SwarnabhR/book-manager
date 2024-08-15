#!/bin/bash

# Navigate to the web directory
cd web

# Clean up previous builds
wasm-pack clean

# Build the Yew application
wasm-pack build --target web

# Optionally, move the build output to the static directory
# cp -r pkg/* static/
