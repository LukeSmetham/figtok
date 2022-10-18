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
        for (value,reference) in (&mut value_store, &reference_store).join() {
            println!("get reference: {:?}", reference);

            // Find the referenced token.
            let matched = (&identity_store, &entities).join()
                .filter(|(identity, _)| identity.id == "mint.0".to_string())
                .collect::<Vec<(&IdentityComponent, Entity)>>();
            
            let (_, entity) = matched[0];
            // Here we need to somehow get the correct value from the referenced entity found above.
            value._current = "REPLACED".to_string();
        }
    }
}

struct DebugSystem;
impl<'a> System<'a> for DebugSystem {
    type SystemData = (ReadStorage<'a, IdentityComponent>, ReadStorage<'a, ValueComponent>);

    fn run(&mut self, data: Self::SystemData) {
        let (identity, value) = data;

        println!("DEBUG SYSTEM TICK");
        // Loop over all components that have a reference.
        for (id, value) in (&identity, &value).join() {
            println!("token: {:?}, value: {:?}", id, value);
            // Follow the reference (get token by name), and Set "value" to the referenced value.
        }
    }
}

// Main
pub struct State {
    ecs: World
}

fn main() {
    println!("Hello, world!");

    let mut state = State {
        ecs: World::new()
    };

    // Core Components
    state.ecs.register::<IdentityComponent>();
    state.ecs.register::<ValueComponent>();
    state.ecs.register::<ReferenceComponent>();
    
    // Entity Type Components
    state.ecs.register::<TokenComponent>();

    // Property Commponents
    state.ecs.register::<ColorComponent>();

    // Dispatcher
    let mut dispatcher = DispatcherBuilder::new()
        .with(ReferenceSystem, "value_sys", &[])
        .with(DebugSystem, "debug_sys", &[])
        .build();
    // setup() must be called before creating any entities, it will register
    // all Components and Resources that Systems depend on
    dispatcher.setup(&mut state.ecs);

    // Load Data
    let loader = Loader::new("./tokens/$metadata.json".to_string());
    loader.load().unwrap();

    let entity1 = state.ecs
        .create_entity()
        .with(IdentityComponent { id: "mint.0".to_string() })
        .with(ValueComponent { value: "#0E100E".to_string(), _current: "#0E100E".to_string() })
        .with(ColorComponent{})
        .with(TokenComponent{})
        .build();
    
    let entity2 = state.ecs
        .create_entity()
        .with(IdentityComponent { id: "background".to_string() })
        .with(ValueComponent { value: "{mint.0}".to_string(), _current: "{mint.0}".to_string() })
        .with(ColorComponent{})
        .with(TokenComponent{})
        .with(ReferenceComponent { token: "mint.0".to_string() })
        .build();

    dispatcher.dispatch(&state.ecs);
}
