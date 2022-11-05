# Theme
A collection of tokens from multiple sets, together comprising a theme that can be serialized to CSS or JSON.

# Token Set
A single collection of tokens that all logically belong together. It's up to the user in Figma tokens how these are organized.

# Token 
A single token value with the following shape:

```rust
struct TokenDefinition {
	/// The value, even numeric values are given as a string by Figma Tokens
    value: String,
	/// What type of value is this? Color, BorderRadius, FontFamily, etc. (Key is `type` in raw data, but aliased to `kind` for Rust.)
    kind: String,
}
```

## TODO
- [ ] Devise a way to output the source sets and enabled sets separately. We will want to also keep track of the source sets we have already processed so that we only handle them once (multiple themes may use the same source sets.) Consider also some kind of index file that `@import` all of the individual sets per-theme with the option to include all, namespaced by a class name on `:root`