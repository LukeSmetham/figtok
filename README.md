# FigTok
Consumes Figma Tokens Studio (Formerly Figma Tokens) output files, and serializes the tokens to CSS variables to be used in your projects and design systems.

You can, for example, hook Figma Tokens Sync feature up to a GitHub repo, and then have `figtok` run in a GH action. Then on each update you get the updated CSS files outputted to a particular location. This would enable you to update styles in Figma, push changes from the UI, and have the updates automatically propagate to your live site/app/whatever.

### Gotchas
- If you use any math operators in your tokens, you need to make sure you have a whitespace character either side of the operator when you define the token (i.e. `{base-size} * {font-scale}` not `{base-size}*{font-scale}` or `{base-size}* {font-scale}` etc.)

## Installation
```bash
yarn add -D figtok
## or
yarn global add figtok
```

You can also install this package via Cargo
```
cargo install figtok
```

## Usage
```bash
# Below are the default values for the args, 
# you can pass no args or use the dir and out args to suit your environment
figtok --dir ./tokens --out ./build
```

## Contribution
This package is deployed both to Cargo and NPM so that users can install the CLI easily in JS projects alongside their design tokens if they prefer. To deploy, you can use the commands in the `Makefile` - currently this uses rust-to-npm to automatically print out the boilerplate files for JS environments.

Once you're ready to deploy:
- Make sure you have [Cargo Bump](https://crates.io/crates/cargo-bump) installed 
- Run `cargo bump [patch | minor | major]`
- Run `make build` to compile for prod, and update the necessary JS files to use the latest version.
- Run `make deploy` to push out to Cargo and NPM simultaneously (You may need to enter 2FA for NPM.)

## TODO
- [X] Support for JSON export (Export JSON that can be used in CSS-in-JS theme setups)
	- With JSON export, we'll want to replace the value in place rather than replace it with a `var(--whatever)` - or a reference to a part of the theme. This means that all values become static (`theme.background === '#fff'` instead of `theme.background === "var(--neutral-0)"`) 
- [X] ~~Improve usage of Clap/CLI~~ Proper separation of concerns between CLI `main.rs` and Lib `lib.rs`
- [X] Support for Figma tokens Math syntax
- [X] Support for if Figma Tokens is set up to export one huge file instead of splitting into files and directories.
- [X] Support for "Composition Tokens" (Similar to Typography tokens)
- [X] Support for typography tokens
- [X] Support for shadow tokens
- [X] Support for users that have no Themes, just token sets. (The below should help with this, if we can output the sets separately, we can change the theme output so that by default, all sets get their own css file, and the theme output just uses `@import` to include the relevant sets in one file.)
- [X] Devise a way to output the source sets and enabled sets separately. We will want to also keep track of the source sets we have already processed so that we only handle them once (multiple themes may use the same source sets.) Consider also some kind of index file that `@import` all of the individual sets per-theme with the option to include all, namespaced by a class name on `:root`
- [ ] Add support for HSL colors


## How to Deploy

```bash
rust-to-npm build
rust-to-npm deploy
```