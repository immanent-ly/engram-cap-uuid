# engram-cap-uuid

Official engram capability `uuid`. Provides the guest with fresh random UUIDs.
Published through the [engram-capabilities](https://github.com/immanent-ly/engram-capabilities) registry.

## Interface

- Provider export: `engram:cap-uuid/provider`.
- Host imports: the allowlisted WASI interfaces declared in `wit/world.wit`.

## Build

```sh
cargo component build --release --target wasm32-unknown-unknown
```

Artifact: `target/wasm32-unknown-unknown/release/cap_uuid.wasm`.

## License

FSL-1.1-ALv2. See LICENSE.md, CONTRIBUTING.md, and CLA.md.
