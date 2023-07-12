# mdbook-fl

This is a _very_ simple preprocessor for the markdown which converts a
few expressions into stuff that's FarLands related.

Conversions:

| Expression          | Result                     |
| ------------------- | -------------------------- |
| `{{#rank dev}}`     | **Dev** (with rank colour) |
| `{{#rank dev bob}}` | bob     (with rank colour) |

## Building

To build this preprocessor, you'll need to have [rust](https://www.rust-lang.org/tools/install) installed.

Once you've got that, you can just run

```sh
cargo build
```

to build the binary or

```sh
cargo test
```

to run the test ~~suite~~ (very minimal).
