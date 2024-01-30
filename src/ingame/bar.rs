use bevy::prelude::*;

use crate::{loading::TextureAssets, GameState};

use super::{
    customer::{generate_customer, CustomerBundle},
    IngameState, Interactible, InteractibleAction, OnIngameScreen,
};

// The bar counter
pub struct BarPlugin;

// Constants // y: -850.
const BAR_CUSTOMER_SLOT_LEFT: Vec3 = Vec3::new(-700., -850., 3.);
const BAR_CUSTOMER_SLOT_MIDDLE: Vec3 = Vec3::new(0., -850., 2.);
const BAR_CUSTOMER_SLOT_RIGHT: Vec3 = Vec3::new(700., -850., 1.);

impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(OnEnter(GameState::Playing), setup_bar)
            .add_systems(
                Update,
                (
                    fill_customer_slots.run_if(in_state(IngameState::Running)),
                    spawn_customers_in_slots.run_if(in_state(IngameState::Running)),
                ),
            );
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Drink {
    Beer,
    Wine,
    Whiskey,
}

impl Drink {
    pub fn iterator() -> impl Iterator<Item = (Drink, Vec3)> {
        vec![
            (Drink::Beer, Vec3::new(750., -615., 13.)),
            (Drink::Wine, Vec3::new(400., -615., 12.)),
            (Drink::Whiskey, Vec3::new(50., -615., 11.)),
        ]
        .into_iter()
    }
}

#[derive(Component)]
struct Bar {
    customer_slots: BarCustomerSlots,
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            customer_slots: BarCustomerSlots::default(),
        }
    }
}

fn setup_bar(mut commands: Commands, textures: Res<TextureAssets>) {
    // Bar counter
    commands
        .spawn(SpriteBundle {
            texture: textures.bar.clone(),
            transform: Transform {
                translation: Vec3::new(0., -733., 10.),
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

    // Drink slots
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

// Customer slot
// CustomerBundle is a bundle of components that is used to spawn a customer
// bool is true if the customer is spawned

struct CustomerSlot {
    customer: Option<CustomerBundle>,
    spawned: bool,
}

impl Default for CustomerSlot {
    fn default() -> Self {
        Self {
            customer: None,
            spawned: false,
        }
    }
}

#[derive(Default)]
struct BarCustomerSlots {
    left: CustomerSlot,
    middle: CustomerSlot,
    right: CustomerSlot,
}

fn fill_customer_slots(mut bar_q: Query<&mut Bar>, textures: Res<TextureAssets>) {
    let mut bar = bar_q.single_mut();

    if bar.customer_slots.left.customer.is_none() {
        bar.customer_slots.left.customer = Some(generate_customer(
            Transform::from_translation(BAR_CUSTOMER_SLOT_LEFT) //
                .with_scale(Vec3::new(1.5, 1.5, 0.0)),
            textures.customer.clone(),
        ));
    }
    if bar.customer_slots.middle.customer.is_none() {
        bar.customer_slots.middle.customer = Some(generate_customer(
            Transform::from_translation(BAR_CUSTOMER_SLOT_MIDDLE) //
                .with_scale(Vec3::new(1.5, 1.5, 0.0)),
            textures.customer.clone(),
        ));
    }
    if bar.customer_slots.right.customer.is_none() {
        bar.customer_slots.right.customer = Some(generate_customer(
            Transform::from_translation(BAR_CUSTOMER_SLOT_RIGHT) //
                .with_scale(Vec3::new(1.5, 1.5, 0.0)),
            textures.customer.clone(),
        ));
    }
}

fn spawn_customers_in_slots(mut commands: Commands, mut bar_q: Query<&mut Bar>) {
    let mut bar = bar_q.single_mut();

    if bar.customer_slots.left.customer.is_some() && !bar.customer_slots.left.spawned {
        commands.spawn(bar.customer_slots.left.customer.take().unwrap());
        bar.customer_slots.left.spawned = true;
    }
    if bar.customer_slots.middle.customer.is_some() && !bar.customer_slots.middle.spawned {
        commands.spawn(bar.customer_slots.middle.customer.take().unwrap());
        bar.customer_slots.middle.spawned = true;
    }
    if bar.customer_slots.right.customer.is_some() && !bar.customer_slots.right.spawned {
        commands.spawn(bar.customer_slots.right.customer.take().unwrap());
        bar.customer_slots.right.spawned = true;
    }
}
