# WebAssembly Playground

Because I wanted to see if I could get WASM working in the browser, written in Rust, loaded by Typescript.
Next up is drawing to a canvas, then WebGL, then writing compiled WASM server modules running in NodeJS.

## Prerequisites

- Node JS and a package manager (Yarn is recommended, though NPM should also work).
- Rust Compiler and the WASM (`wasm32-unknown-unknown`) build target toolchain.

## Building

```bash
yarn install
yarn build
```

Open `build/dist/index.html` in WASM-compatible browser.