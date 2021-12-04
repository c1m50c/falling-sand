# **falling-sand**
Web Assembly application simulating the Falling Sand Cellular Automata.


---


## **Building & Running**

### Requirements
- <a href="https://www.rust-lang.org/tools/install">Rust Toolchain</a>
- <a href="https://rustwasm.github.io/wasm-pack/installer/">WASM Pack</a>
- <a href="https://docs.npmjs.com/getting-started">NPM</a>

### Building
```bash
$ cd falling-sand
$ wasm-pack build # Build to Web Assembly with wasm-pack.
...
```

### Running
```bash
$ cd falling-sand/www
$ npm install # Install dependencies for the site.
$ npm run start # Will host site on `localhost:8080`.
...
```