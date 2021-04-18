# template-app-rust
Template Rust application

## Build

### Builder
```
docker build --target builder -t templateapprust .
```

### Full
```
docker build -t templateapprust .
```

## Test

### Unit
TBD

### Integration
TBD

## Run
```
docker run -it --rm templateapprust
```

In container
```
./target/debug/templateapp
```
