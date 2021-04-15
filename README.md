# Github Action's Experiments

This is an experimental repository to demonstrate Github Action Workflow for Rust and Docker.

## Trigger: On Pull Request

The purpose is to test the build for both binary and Docker image.

```yaml
name: On Pull Request

on:
  pull_request:
    branches: [ main ]
    
env:
  CARGO_TERM_COLOR: always

jobs:
  cargo-test:
    runs-on: ubuntu-20.04
    steps:
    - uses: actions/checkout@v2
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: 1.51.0
        components: clippy
        target: x86_64-unknown-linux-musl
        override: true
    - name: Clippy deny all warnings
      run: cargo clippy --target=x86_64-unknown-linux-musl --all-targets --all-features -- -D warnings
    - name: All tests
      run: cargo test --target=x86_64-unknown-linux-musl --verbose
  docker-test:
    runs-on: ubuntu-20.04
    env:
      RUSTFLAGS: "-C link-args=-s"
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: 1.51.0
          target: x86_64-unknown-linux-musl
          override: true
      - name: All tests
        run: |
          cargo build --release --target=x86_64-unknown-linux-musl
          docker build -t test/image:latest -f Dockerfile .
```

## Trigger: On Release

The purpose is to publish a Docker image whenever a release tag is published.

```yaml
name: On Release

on:
  release:
    types: [published]

env:
  RUSTFLAGS: "-C link-args=-s"
  IMAGE_NAME: sayhello

jobs:
  docker-publish:
    runs-on: ubuntu-20.04
    env:
      RUSTFLAGS: "-C link-args=-s"
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: 1.51.0
          target: x86_64-unknown-linux-musl
          override: true
      - name: Build & Push
        run: |
          # Build portable native binary
          cargo build --release --target=x86_64-unknown-linux-musl

          # Copy the binary to docker image
          docker build -f Dockerfile -t $IMAGE_NAME .

          # Login to Github docker registry
          echo "${{ secrets.GITHUB_TOKEN }}" | docker login ghcr.io -u ${{ github.actor }} --password-stdin

          # Compose proper Image ID
          IMAGE_ID=ghcr.io/${{ github.repository }}/$IMAGE_NAME

          # Change all uppercase to lowercase
          IMAGE_ID=$(echo $IMAGE_ID | tr '[A-Z]' '[a-z]')

          # Strip git ref prefix from version
          VERSION=$(echo "${{ github.ref }}" | sed -e 's,.*/\(.*\),\1,')

          # Strip "v" prefix from tag name
          [[ "${{ github.ref }}" == "refs/tags/"* ]] && VERSION=$(echo $VERSION | sed -e 's/^v//')

          echo IMAGE_ID=$IMAGE_ID
          echo VERSION=$VERSION

          docker tag $IMAGE_NAME $IMAGE_ID:$VERSION
          docker push $IMAGE_ID:$VERSION
```

## Published Docker Image Demo

```bash
❯ docker run --pull --rm -it ghcr.io/ujang360/gh-actions-experiments/sayhello:0.1.0 sayhello --to Kresna
Hello, Kresna!
```

## License

Any code published in this repository is published under MIT License, except if it's inherited from other LICENSE.
