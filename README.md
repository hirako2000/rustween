# Tween demos

This repository contains several mini apps written in Rust.

- demo usages of the [tween](https://crates.io/crates/tween) crate.
- compile to WebAssembly (Wasm) and may be executed in the browser, via the [wasm-bindgen](https://crates.io/crates/wasm-bindgen) crate.
- Effectively execute the crate's tweening logic in the browser and use the animationFrame API for handling the rendering loop.

**Why Wasm**

Rust can compile to many platforms, but also offers first class support for compilation into Wasm. Browser can execute Wasm applications making their distribution more seamless. Users don't have to install anything.

Wasm applications can also run wasm application locally, as binaries.


# Live demo:

[https://rustween.surge.sh/](https://rustween.surge.sh/)

# Prerequisites

- node/npm
- Rust and Cargo

are installed

# Getting started

1. Install npm dependencies

```bash
npm install
```


2. dev mode

Navigate to one of the subfolder of this repo, e.g `elastic_out`

```bash
% cd elastic_out
```

```bash
npm run serve # then open open your browser http://localhost:8080/elastic-out.html
```

code changes will automatically reflect in the browser

3. build all

From the root folder of this repo:

```bash
npm run build:all 
```

4. Deploy 

From the root folder of this repo:

```bash
npm run deploy
```
  
# TODO

- [x] App demo-ing the elastic out tween
- [x] Demos of another half a dozen common tweens
- [x] Couple of demos of combined tweens
- [ ] Refactor with proper structs to keep pointers to dom elements: Looking up dom elements is expensive
- [ ] Make use of traits and modules for code reusability
- [ ] More comments
- [x] Deploy
- [ ] Unit tests
- [ ] Look into [wasmer](https://github.com/wasmerio/wasmer) to compile into a mono binary that can execute cross platform

## Credit

- of course kudos to the author of the Rust tween library. it prompted the idea to build this in the first place.
- the many contributors to the dom-binding crate.
- @sasha240100 for old times inspirations

# About this project
I started this project to learn Rust, while expanding my familiarity with Wasm. So what's in there ain't to be used as reference of Rust best practices or quality code. And needless to say may contain numerous bugs, such as memory leaks, deadlocks and whatnot. 

### What have I learnt
- That Rust is great!
- That I had forgotten quite a bit about pointers after years of focus on higher level programming languages
- That I missed and love strong types with low level control: unsigned/signed integers, 8, 32, 64bits, floats, yay!
- A bit on type type casting, and that I don't miss implicit coercion at all.
- That pointers in Rust are in fact simple, that passing of arguments as value, cloning etc is a breeze
- That Rust projects is be started off with some thoughts on structs and traits to define first! It becomes quickly cumbersome to try to refactor while going along, at least as a beginner.
- That I would love to see this [issue/PR](https://github.com/rustwasm/wasm-pack/pull/1070) merged into wasm-pack so that I can bundle pkg apps as modules
- That even if it works, I should remember to stop writing so much procedural code and fully adopt OOP.

### Worklog

#### Day 1

- Setup new Cargo project ~ 5 mins
- Look for Rust tweening libraries, glance over doc a bit ~ 1h
- Pull in tween crate, scratch some code with it to create a first simple tween, output to console. ~1h
- Look into building a wasm app! 
- Look for libs to help with that, read doc ~1h

#### Day 2

- Integrating Wasm, code some simple bindings to create and add dom elements ~90mins
- Render a box with a grid background ~ 30 mins
- Move the box with hard coded values, via animationframe ~ 30 mins
- Get a loop working that moves the box out the delta and the tween created with some range ~1h
- clean up this rusty code bit, split large functions into smaller ones ~2h
- Change a few types ~1h
- html page, links etc ~1h
- Doc etc ~2h. 

#### Day 3

- Added easing animation: cubic in and out ~30mins
- Added some combined tweeners animation: change of position + rotation ~30mins
- Added another combined tweeners animation: change of x and y position ~30mins
- minor adjustments to the root html ~30
  
#### Day 4

Style improvements to the main page:
- vertical align text links to demos, remove text decoration on those links ~10mins
- adds radius border to thumbnail images and add some transition on hover from grayscale to color to them ~10mins

Tweener demos adjustments:
- use new tweener constructor and pass over a current time of 1s so that the tween starts with a delay ~1h

#### Day 5

More restyling:
- Dark theme! ~1h
- Clean up image thumbnails from artifacts ~10mins
- align elements for good, adds back underlines to links ~30mins

Test/Fixes:
- Fixed paths in subREADMEs and other minor fixes ~30 mins