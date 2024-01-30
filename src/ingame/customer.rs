use bevy::prelude::*;
use rand::seq::{IteratorRandom, SliceRandom};

use crate::{loading::TextureAssets, ScaleByAssetResolution};

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
    state: CustomerState,
}

// ToDo Implement timers for customer states
enum CustomerState {
    Waiting,
    Drinking,
    Leaving,
}

#[derive(Bundle)]
pub struct CustomerBundle {
    customer: Customer,
    pub sprite_bundle: SpriteBundle,
    interactible: Interactible,
    marker: OnIngameScreen,
}

impl CustomerBundle {
    pub fn new(name: &str, drink: Drink, texture: Handle<Image>, transform: Transform) -> Self {
        Self {
            customer: Customer {
                name: name.to_string(),
                drink,
                state: CustomerState::Waiting,
            },
            sprite_bundle: SpriteBundle {
                texture: texture,
                transform,
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

pub fn generate_random_customer(textures: &Res<TextureAssets>) -> CustomerBundle {
    info!("Generating random customer"); // ToDo Remove
    let mut rng = rand::thread_rng();
    let name = format!("{}", CUSTOMER_NAMES.choose(&mut rng).unwrap_or(&"John"),);
    let drink = Drink::iterator().choose(&mut rng).unwrap().0;
    let texture = CustomerAssets::iterator()
        .choose(&mut rng)
        .unwrap()
        .get_texture(textures);
    let transform = Transform {
        translation: Vec3::new(0.0, 0.0, 0.0),
        scale: ScaleByAssetResolution::Res720p.scale(),
        ..Default::default()
    };

    CustomerBundle::new(&name, drink, texture, transform)
}

pub const CUSTOMER_NAMES: [&str; 10] = [
    "John", "Jane", "Jack", "Jill", "James", "Jenny", "Jasper", "Jade", "Jared", "Jasmine",
];

enum CustomerAssets {
    Customer1,
}

impl CustomerAssets {
    pub fn get_texture(&self, textures: &Res<TextureAssets>) -> Handle<Image> {
        match self {
            CustomerAssets::Customer1 => textures.customer1.clone(),
        }
    }

    fn iterator() -> impl Iterator<Item = Self> {
        vec![CustomerAssets::Customer1].into_iter()
    }
}
