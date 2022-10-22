#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate specs;

use specs::prelude::*;

mod loader;
use loader::{Loader, TokenKind};

// Components 

#[derive(Debug)]
struct TokenComponent {}
impl Component for TokenComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct TokenSetComponent {
    id: String
}
impl Component for TokenSetComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct IdentityComponent {
    id: String
}
impl Component for IdentityComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct ValueComponent {
    _current: String,
    value: String
}
impl Component for ValueComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct ReferenceComponent {
    token: String
}
impl Component for ReferenceComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct ColorComponent {}
impl Component for ColorComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct BorderRadiusComponent {}
impl Component for BorderRadiusComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct FontFamilyComponent {}
impl Component for FontFamilyComponent {
    type Storage = VecStorage<Self>;
}

fn get_type(kind: TokenKind) {
    match kind {
        TokenKind::Color => {
            
        }
        TokenKind::BorderRadius => {
            
        }
        TokenKind::FontFamily => {

        }
    }
}

// Systems

/// Reference System
/// 
/// Responsible for linking together a token that references another token that is defined in the system.
struct ReferenceSystem;
impl<'a> System<'a> for ReferenceSystem {
    type SystemData = (WriteStorage<'a, ValueComponent>, ReadStorage<'a, ReferenceComponent>, ReadStorage<'a, IdentityComponent>, Entities<'a>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut value_store, reference_store, identity_store, entities) = data;

        println!("VALUE SYSTEM TICK");
        // Loop over all components that have a reference.
        for (reference, entity) in (&reference_store, &entities).join() {
            // println!("get reference: {:?}", reference);

            // Find the referenced token.
            (&identity_store, &entities).join()
                .filter(|(identity, _)| identity.id == reference.token.to_string())
                .for_each(|(_, e)| {
                    // This should only iterate once, technically if therre were two tokens with the same name, this would run twice and only the latest value would persist.
                    let ref_value = value_store.get(e);
                    let mut current_value = value_store.get_mut(entity);
                    // TODO: 
                    // current_value._current = ref_value.value.clone();
                    current_value.unwrap()._current = ref_value.unwrap().value.clone();
                });
        }
    }
}

struct DebugSystem;
impl<'a> System<'a> for DebugSystem {
    type SystemData = (ReadStorage<'a, IdentityComponent>, ReadStorage<'a, ValueComponent>, ReadStorage<'a, TokenComponent>, ReadStorage<'a, TokenSetComponent>);

    fn run(&mut self, data: Self::SystemData) {
        let (identity, value, token, token_set) = data;

        println!("DEBUG SYSTEM TICK");
        // Loop over all components that have a reference.
        for (id, value, _, token_set) in (&identity, &value, &token, &token_set).join() {
            println!("TOKEN • id: {:?} • value: {:?} • FROM SET: {:?}", id.id, value._current, token_set.id);
            // Follow the reference (get token by name), and Set "value" to the referenced value.
        }
    }
}

// Main
pub struct Builder {
    ecs: World,
    loader: Loader
}

fn main() {
    let mut builder = Builder {
        ecs: World::new(),
        loader: Loader::new("./tokens/$metadata.json".to_string())
    };

    // Register any components

    // Core Components
    builder.ecs.register::<IdentityComponent>();
    builder.ecs.register::<ValueComponent>();
    builder.ecs.register::<ReferenceComponent>();
    
    // Entity Type Components
    builder.ecs.register::<TokenComponent>();
    builder.ecs.register::<TokenSetComponent>();

    // Property Commponents
    builder.ecs.register::<BorderRadiusComponent>();
    builder.ecs.register::<ColorComponent>();
    builder.ecs.register::<FontFamilyComponent>();

    // Create the dispatcher
    let mut dispatcher = DispatcherBuilder::new()
        .with(ReferenceSystem, "value_sys", &[])
        .with(DebugSystem, "debug_sys", &[])
        .build();
    // setup() must be called before creating any entities, it will register
    // all Components and Resources that Systems depend on
    dispatcher.setup(&mut builder.ecs);

    // Load Data
    builder.loader.load(&mut builder.ecs).unwrap();

    // Fire dispatch to run data through the systems
    dispatcher.dispatch(&builder.ecs);
}
