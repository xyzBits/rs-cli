# Rust Command Line

## Convert csv to json

```bash
cargo run -- csv --input input.csv 
```

## Convert csv to yaml 

```bash 
 cargo run -- csv --input assets/juventus.csv --output output.yaml
```

## Generate password 
```bash
 cargo run -- genpass --length 34

```

## Encode Cargo.toml into base64 
```shell
 cargo run -- base64 encode --input Cargo.toml
```

## Decode cargo_toml_base64.txt 
```shell
 cargo run -- base64 decode --input fixtures/cargo_toml_base64.txt
```

## Generate blake3 key 
```shell
 cargo run -- text generate --format blake3 --output-path fixtures
```

## Generate ed25519 key 
```shell
 cargo run -- text generate --format ed25519 --output-path fixtures
```

## Sign message using ed25519 
```shell
 cargo run -- text sign --input fixtures/hello.txt --key fixtures/ed25519.signing_key --format ed25519
```

## Verify message using ed25519
```shell
 cargo run -- text verify --input fixtures/hello.txt --sig fixtures/ed25519.sig --key fixtures/ed25519.verifying_key --format ed25519
```