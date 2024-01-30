use bevy::prelude::*;

use super::bar::Drink;

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
    sprite: Handle<Image>,
}

impl CustomerBundle {
    pub fn new(name: &str, drink: Drink, sprite: Handle<Image>) -> Self {
        Self {
            customer: Customer {
                name: name.to_string(),
                drink,
            },
            sprite,
        }
    }
}
