# `Figtok/Tokens`

This crate is responsible for defining the core structs used to define design tokens, the structures we use to store, retrieve and serialize tokens into CSS.

## `TokenDefinition`
The TokenDefinition struct is constructed via serde. Our raw data is deserialized directly into `TokenDefinition<T>` depending on the underlying type of token. So far we have three:

- Standard
- Composition
- Shadow

**Standard Tokens** are simple objects whose value is a `String` stored under `token.value`

**Composition Tokens** are objects whose value is a nested `serde::Value`. These nested values can contian any number of CSS properties, that should all be applied at once (i.e. as a single token, but containing multiple style properties) - because of this we serialize them slightly differently, producing a CSS Class definition rather than a CSS Variable so that the implementing application can apply all of the styles at once. This also has the benefit of allowing inner values to reference other tokens. When serializing each inner value is treated similar to a standard token where we unfurl any references to other tokens.

**Shadow Tokens** are defined similarly to composition tokens, where the value is a nested object - however, with shadows there are pre-defined set of inner properties that come together to create a `box-shadow: [X] [Y] [BLUR] [SPREAD] [COLOR]` because of this we deserialize similarly to composition tokens, but serialize more similarly to standard tokens, using the inner properties of the shadow token to produce a single value.

## `Token`
The Token enum is responsible for providing a generic API over the top of our `TokenDefinition<T>`.

The token enum stores a `TokenDefinition<T>` and exposes some getters for retrieving the properties of the token. As we iterate through our token sets to serialize, we can use these getters to retrieve the name, type, etc. and also the value - with the additional magic that when fetching the `Token.value()` instead of directly accessing `TokenDefinition<T>.value` figtok will enrich the value to unfurl any references to other tokens.

## `TokenSet`
TokenSets are simply a storage mechanism for keeping tokens that we're defined together, together. These sets can either be all serialized as individual files, or alternatively, if using the Themes feature of Figma Token Studio, as Figtok iterates through the themes it will only include the token sets relevant to that theme.

## `TokenStore`
TokenStore is a trait that is implemented by FigTok itself that adds functionality for retrieving tokens, and enriching their values. The implementing sturct can store these tokens however it likes, as long as these methods are implemented to create compatability with the structs & systems outlined above.


## TODO
- [ ] Re-instate JSON serialization?