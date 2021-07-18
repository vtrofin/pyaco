## CSS to code generator

### A type safe CSS to "\*" code generator, tailored for Tailwind.

Generates code from _any_ valid css file (this CLI has been tested against complex CSS files generated by Tailwind). Currently supports TypeScript, ReScript, Elm, PureScript, and Rust (via a `css!` macro that doesn't require code generation, see below).

### Installation notice

This new version can be installed using npm/yarn (only on macos for now) using this command `[yarn add|npm install] https://github.com/scoville/tailwind-generator\#complete-refactor`.

### Commands:

To get help:

`$ style-generator --help`

```
style-generator

USAGE:
    style-generator [OPTIONS] --input <input> --lang <lang>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>                        CSS file to parse and generate code from
    -l, --lang <lang>
            Language used in generated code (elm|purescript|rescript|typescript)"

    -o, --output <output>                      Directory for generated code [default: ./]
    -f, --output-filename <output-filename>
            Filename (without extension) used for the generated code [default: Output]
```

`style-generator` uses [env_logger](https://docs.rs/env_logger/0.8.4/env_logger/) under the hood, so you can prefix your command with `RUST_LOG=info` for a more verbose output, the binary is silent by default.

### Generators

#### TypeScript (typescript)

A simple generator for TypeScript, it exports an [opaque type](https://en.wikipedia.org/wiki/Opaque_data_type) `CssClass`, `join` function build, and a set of `CssClass` objects:

```ts
import { join, textBlue100, rounded, border, borderBlue300 } from "./Output.ts";

// ...

<div className={join([textBlue100, rounded, border, borderBlue300])}>
  Hello
</div>;
```

Pros:

- Easy to use
- Very flexible
- Compatible with most TypeScript versions
- Safe, you can't pass any string to the `join` function
- Autocompletation

Cons:

- Cost at runtime: `CssClass` are JavaScript objects that help ensuring type opacity
- Cost at runtime: the array has to be joined into a string
- Imports can be verbose (unless you use `import * as ...`)
- Not the "standard" class names, `h-full` becomes `hFull`, etc...

#### TypeScript type 1 (typescript-type-1)

This generator doesn't generate any runtime code apart from the `join` function.

```ts
import { join } from "./Output.ts";

// ...

<div className={join("text-blue-100", "rounded", "border", "border-blue-300")}>
  Hello
</div>;
```

Pros:

- Easy to use
- Very flexible
- Compatible with most TypeScript versions
- Safe, you can't pass any string to the `tailwind` function
- "Standard" class names
- Light import (you only need the `join` function)
- Autocomplete

Cons:

- Cost at runtime: the classes must be "joined" into a string

#### TypeScript type 2 (typescript-type-2)

This generator doesn't generate any runtime code apart from the `css` function.

```ts
import { css } from "./Output.ts";

// ...

<div className={css("text-blue-100 rounded border border-blue-300")}>
  Hello
</div>;
```

Pros:

- Super easy to use
- Safe, you can't pass any string to the `tailwind` function
- "Standard" class names
- Light import (you only need the `css` function)
- No runtime cost at all
- Partial support for autocompletion

Cons:

- Not as flexible as the 2 other generators
- Compatible with TypeScript > 4.1 only
- Type error can be hard to debug
- Doesn't accept multiple spaces

#### Rust (rust)

_In progress._
