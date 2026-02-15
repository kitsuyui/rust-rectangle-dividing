# rust-rectangle-dividing

![Coverage](https://raw.githubusercontent.com/kitsuyui/octocov-central/main/badges/kitsuyui/rust-rectangle-dividing/coverage.svg)

A library of rectangle dividing written in Rust.
It can be compiled to WebAssembly. So you can use it in JavaScript.

## Motivation

I want to divide a rectangle into smaller rectangles by given conditions (weights, aspect ratio, etc.).

Mainly, I want to use this for generating a map for my react-playground
- https://github.com/kitsuyui/react-playground
- https://react-playground.docs.kitsuyui.com/storybook/?path=/docs/base-treemap-introduction--docs

## Usage in JavaScript

### Install

This package is published on NPM [@kitsuyui/rectangle-dividing](https://www.npmjs.com/package/@kitsuyui/rectangle-dividing)

```sh
$ npm install @kitsuyui/rectangle-dividing
# or
$ yarn add @kitsuyui/rectangle-dividing
# or
$ pnpm add @kitsuyui/rectangle-dividing
```

### Example

`node --experimental-wasm-modules` flag is required to use WebAssembly in Node.js

```js
import { dividing } from "@kitsuyui/rectangle-dividing";

const rect = { x: 0, y: 0, w: 900, h: 800 };
const weights: Float32Array = Float32Array.from([4, 4, 1, 1, 1, 1]);
const aspectRatio = 1.5;
const verticalFirst = true;
const boustrophedron = true;
const divided = dividing(rect, weights, aspectRatio, verticalFirst, boustrophedron);
for (const d of divided) {
  console.log(d);
}
```

### Result

```js
{ x: 0, y: 0, w: 600, h: 400 }
{ x: 0, y: 400, w: 600, h: 400 }
{ x: 600, y: 600, w: 300, h: 200 }
{ x: 600, y: 400, w: 300, h: 200 }
{ x: 600, y: 200, w: 300, h: 200 }
{ x: 600, y: 0, w: 300, h: 200 }
```

dividing's arguments are

- `rect`: The rectangle to be divided
- `weights`: The weights of each rectangle
- `isVertical`: The direction of the first division
- `aspectRatio`: The aspect ratio of each rectangle
- `boustrophedon`: The direction of the next division in the same level

# License

MIT
