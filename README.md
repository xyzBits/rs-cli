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