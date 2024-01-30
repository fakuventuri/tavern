use bevy::prelude::*;

use crate::{loading::TextureAssets, GameState};

use super::{customer::CustomerBundle, Interactible, InteractibleAction, OnIngameScreen};

// The bar counter
pub struct BarPlugin;

// Constants
const BAR_SLOT_LEFT: Vec3 = Vec3::new(-300., -600., 0.);
const BAR_SLOT_MIDDLE: Vec3 = Vec3::new(0., -600., 0.);
const BAR_SLOT_RIGHT: Vec3 = Vec3::new(300., -600., 0.);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Drink {
    Beer,
    Wine,
    Whiskey,
}

impl Drink {
    pub fn iterator() -> impl Iterator<Item = (Drink, Vec3)> {
        vec![
            (Drink::Beer, Vec3::new(750., -615., 4.)),
            (Drink::Wine, Vec3::new(400., -615., 3.)),
            (Drink::Whiskey, Vec3::new(50., -615., 2.)),
        ]
        .into_iter()
    }
}

impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(OnEnter(GameState::Playing), setup_bar);
    }
}

#[derive(Component)]
struct Bar {
    customer_slots: BarCustomerSlots,
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            customer_slots: BarCustomerSlots {
                left: None,
                middle: None,
                right: None,
            },
        }
    }
}

struct BarCustomerSlots {
    left: Option<CustomerBundle>,
    middle: Option<CustomerBundle>,
    right: Option<CustomerBundle>,
}

fn setup_bar(mut commands: Commands, textures: Res<TextureAssets>) {
    // Bar counter
    commands
        .spawn(SpriteBundle {
            texture: textures.bar.clone(),
            transform: Transform {
                translation: Vec3::new(0., -600., 0.), // y: -540.
                scale: Vec3::new(1.5, 1.5, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Bar::default())
        .insert(Interactible {
            action: InteractibleAction::EnterBar,
        })
        .insert(OnIngameScreen);

    // Bar slots
    for (drink, barrel_pos) in Drink::iterator() {
        commands
            .spawn(SpriteBundle {
                texture: textures.barrel.clone(),
                transform: Transform {
                    translation: barrel_pos,
                    scale: Vec3::new(1.5, 1.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Interactible {
                action: InteractibleAction::Barrel(drink),
            })
            .insert(OnIngameScreen);
    }
}
