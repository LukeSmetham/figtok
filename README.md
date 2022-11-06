# FigTok
Consumes Figma Tokens JSON files, and outputs CSS.

You can, for example, hook Figma Tokens Sync feature up to a GitHub repo, and then have `figtok` run in a GH action. Then on each update you get the updated CSS files outputted to a particular location. This would enable you to update styles in Figma, push changes from the UI, and have the updates automatically propagate to your live site/app/whatever.

> This project is in a super-WIP state currently and probably won't work for you. It's only really guaranteed to work with Figma Tokens Pro, and if you use the "multiple files" option that splits your tokens into separate json files in directories instead of one huge json file. (see TODOs below.)


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

## TODO
- [ ] Add remaining token types (TokenKind::*)
- [ ] Check/Add support for HSL colors
- [ ] Figma Tokens "Composition Tokens" Support
- [ ] Support for if Figma Tokens is set up to export one huge file instead of splitting into files and directories.
- [X] Support for users that have no Themes, just token sets. (The below should help with this, if we can output the sets separately, we can change the theme output so that by default, all sets get their own css file, and the theme output just uses `@import` to include the relevant sets in one file.)
- [X] Devise a way to output the source sets and enabled sets separately. We will want to also keep track of the source sets we have already processed so that we only handle them once (multiple themes may use the same source sets.) Consider also some kind of index file that `@import` all of the individual sets per-theme with the option to include all, namespaced by a class name on `:root`
- [ ] Support for JSON export.


## How to Deploy

```bash
rust-to-npm build
rust-to-npm deploy
```