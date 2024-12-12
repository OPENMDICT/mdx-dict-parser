# MDX-DICT-PARSER

This is a package based on [napi-rs](https://napi.rs/) and [mdict-parser](https://crates.io/crates/mdict-parser), which provides the node-binding to read all the words and their definitions from mdict(.mdx) file.

## Usage

This package is ESM only.

First, install the package

```bash
npm i mdx-dict-parser
```

You can write such code in typescript.

```ts
import { parseMdict, DictRecord } from "mdx-dict-parser";

const filePath = "./the/path/to/your/dictionary.mdx";
const data: DictRecord[] = parseMdict(filePath);
const firstRecord = data[0];
console.log(firstRecord); // Print the first record.
```

## Roadmap

This project is planned to implement these following functions soon.

- mdict 1.0 and 3.0 support.
- Read details for mdict file.
- MDD parser support.

## LICENSE

MIT@[但为君故](https://dreams.plus).
