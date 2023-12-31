npx tailwindcss@3.3.0 -i input.css -o target/site/styles/styles.css &&
wasm-pack build --dev --target=web --out-dir=target/site/pkg --features=hydrate --no-default-features &&
cargo run --no-default-features --features=ssr --verbose

