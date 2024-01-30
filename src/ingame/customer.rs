use bevy::prelude::*;
use rand::seq::{IteratorRandom, SliceRandom};

use super::{bar::Drink, Interactible, InteractibleAction, OnIngameScreen};

pub struct CustomerPlugin;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}

#[derive(Component)]
struct Customer {
    name: String,
    drink: Drink,
}

#[derive(Bundle)]
pub struct CustomerBundle {
    customer: Customer,
    sprite_bundle: SpriteBundle,
    interactible: Interactible,
    marker: OnIngameScreen,
}

impl CustomerBundle {
    pub fn new(
        name: &str,
        drink: Drink,
        texture: Handle<Image>,
        customer_transform: Transform,
    ) -> Self {
        Self {
            customer: Customer {
                name: name.to_string(),
                drink,
            },
            sprite_bundle: SpriteBundle {
                texture: texture,
                transform: customer_transform,
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::BottomCenter, // Ruins interaction_handle
                    ..Default::default()
                },
                ..Default::default()
            },
            interactible: Interactible {
                action: InteractibleAction::Customer,
            },
            marker: OnIngameScreen,
        }
    }
}

pub fn generate_customer(customer_transform: Transform, texture: Handle<Image>) -> CustomerBundle {
    let mut rng = rand::thread_rng();
    let name = format!("{}", CUSTOMER_NAMES.choose(&mut rng).unwrap_or(&"John"),);
    let drink = Drink::iterator().choose(&mut rng).unwrap().0;
    CustomerBundle::new(&name, drink, texture, customer_transform)
}

pub const CUSTOMER_NAMES: [&str; 10] = [
    "John", "Jane", "Jack", "Jill", "James", "Jenny", "Jasper", "Jade", "Jared", "Jasmine",
];
