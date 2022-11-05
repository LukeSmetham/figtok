# Token Parser
Consumes Figma Tokens JSON files, and outputs CSS.

> This project is in a super-WIP state currently. It's only really guaranteed to work with Figma Tokens Pro, and if you use the "multiple files" option that splits your tokens into separate json files in directories instead of one huge json file - although I plan to support this. It also relies solely on themes right now and has one command, where it will export a CSS file for each theme in your Figma Tokens setup - I plan to add more functionality in the future to allow for more configurable output, or the option to dump all tokens in one css file (including namespaced values via class name to retain theme support in this setup - see TODOs below.)

## Definitions [WIP]
### Theme
A collection of tokens from multiple sets, together comprising a theme that can be serialized to CSS or JSON.

### Token Set
A single collection of tokens that all logically belong together. It's up to the user in Figma tokens how these are organized.

### Token 
A single token value with the following shape:

```rust
struct TokenDefinition {
	/// The value, even numeric values are given as a string by Figma Tokens
    value: String,
	/// What type of value is this? Color, BorderRadius, FontFamily, etc. (Key is `type` in raw data, but aliased to `kind` for Rust.)
    kind: String,
	id: String, // <--- We add these last two based on the object property names to help with lookups.
	name: String, //
}
```

## TODO
- [ ] Support for if Figma Tokens is set up to export one huge file instead of splitting into files and directories.
- [ ] Support for users that have no Themes, just token sets. (The below should help with this, if we can output the sets separately, we can change the theme output so that by default, all sets get their own css file, and the theme output just uses `@import` to include the relevant sets in one file.)
- [ ] Devise a way to output the source sets and enabled sets separately. We will want to also keep track of the source sets we have already processed so that we only handle them once (multiple themes may use the same source sets.) Consider also some kind of index file that `@import` all of the individual sets per-theme with the option to include all, namespaced by a class name on `:root`