#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate specs;

use specs::prelude::*;

mod loader;
use loader::{Loader, TokenKind};

#[derive(Debug)]
struct IdentityComponent {
    name: String,
}
impl Component for IdentityComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct ValueComponent {
    name: String,
    kind: TokenKind
}
impl Component for ValueComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct ReferenceComponent {
    token_ref: String,
}
impl Component for ReferenceComponent {
    type Storage = VecStorage<Self>;
}

fn main() {
    println!("Hello, world!");
    let loader = Loader::new("/Users/lukesmetham/Projects/smetham.dev/token-parser/tokens/$metadata.json".to_string());
    loader.load().unwrap();

    let mut world = World::new();
    world.register::<IdentityComponent>();
    world.register::<ValueComponent>();
    world.register::<ReferenceComponent>();
}
