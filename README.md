# node-svgcleaner

Ported fast-based rust [svgcleaner] library.

[svgcleaner]: https://github.com/RazrFalcon/svgcleaner


### Usage

```js
const svgcleaner = require('node-svgcleaner');

let raw = svgcleaner.readFile("bg-pattern.svg");

let result = svgcleaner.clean(raw, {});
```
