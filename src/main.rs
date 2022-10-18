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
struct IdentityComponent(String);
impl Component for IdentityComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct ValueComponent(String);
impl Component for ValueComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct ReferenceComponent(String);
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
struct ReferenceSystem;
impl<'a> System<'a> for ReferenceSystem {
    type SystemData = (WriteStorage<'a, ValueComponent>, ReadStorage<'a, ReferenceComponent>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut value, reference) = data;

        println!("VALUE SYSTEM TICK");
        // Loop over all components that have a reference.
        for (value,reference) in (&mut value, &reference).join() {
            // println!("value: {:?}, reference: {:?}", value, reference);
            // Follow the reference (get token by name), and Set "value" to the referenced value.
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
        for (identity, value) in (&identity, &value).join() {
            println!("token: {:?}, value: {:?}", identity, value);
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
        .with(IdentityComponent("mint.0".to_string()))
        .with(ValueComponent("#0E100E".to_string()))
        .with(ColorComponent{})
        .with(TokenComponent{})
        .build();
    
    let entity2 = state.ecs
        .create_entity()
        .with(IdentityComponent("background".to_string()))
        .with(ValueComponent("{mint.0}".to_string()))
        .with(ReferenceComponent("{mint.0}".to_string()))
        .with(ColorComponent{})
        .with(TokenComponent{})
        .build();

    dispatcher.dispatch(&state.ecs);
}
