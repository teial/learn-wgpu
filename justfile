export RUSTFLAGS := ""

# Show list of all available tasks
default:
    @just --list

# build wasm package
wasm:
    cd game && wasm-pack build --target web

# Copy wasm assets to server
copy-assets:
    cp assets/index.html static/
    cp -r game/pkg/* static/

# Build and serve project
serve: wasm copy-assets
    cargo run -p server

# Run game on desktop
run:
    cargo run -p game
