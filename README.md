# flutter_rust_bridge testing

This branch aims to test Kaspa integration with Flutter environments via the [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) project.

## Bridge crate and flutter project

Here are the steps done in this branch.

### Scaffolding

Use flutter_rust_bridge directly from within the Kaspa project directory:

    flutter_rust_bridge_codegen create flutter_project --rust-crate-dir ../bridgeflutter/

### Rust workspace membership

The new crate bridgeflutter's Cargo.toml needs to be merged with Kaspa's Cargo.toml and the crate bridgeflutter needs to be added to Kaspa's Cargo.toml members list.

More doesn't need to be done as we don't need other crates to use the bridgeflutter.

### Edit crate to connect to Kaspa node as functional test

Link Kaspa crates and write code inside bridgeflutter/src/api/.

In this branch we use bridgeflutter/src/api/simple.rs which needs to be published in bridgeflutter/src/api/mod.rs.

