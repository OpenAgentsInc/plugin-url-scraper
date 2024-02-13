## Build
`cargo build --target wasm32-unknown-unknown --release`

## Test build locally
`extism call target/wasm32-unknown-unknown/release/plugin_url_scraper.wasm fetch_url_content --input '{"url": "https://raw.githubusercontent.com/OpenAgentsInc/plugin-url-scraper/main/src/lib.rs"}' --allow-host="*"`

