use std::time::Duration;

use bevy::prelude::*;
use rand::{seq::SliceRandom, Rng};

use crate::{loading::TextureAssets, GameState, ScaleByAssetResolution};

use super::{
    customer::{generate_random_customer, CustomerBundle},
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
                    spawn_customers_in_slots.run_if(in_state(IngameState::Running)),
                    spawn_customer.run_if(in_state(IngameState::Running)),
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
    // customer_queue: Vec<CustomerBundle>,
    customer_spawn_timer: Timer,
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            customer_slots: BarCustomerSlots::default(),
            customer_spawn_timer: Timer::from_seconds(1., TimerMode::Once),
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
                scale: ScaleByAssetResolution::Res720p.scale(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Bar::default())
        .insert(Interactible {
            action: InteractibleAction::EnterBar,
        })
        .insert(OnIngameScreen);

    // Barrel Slots
    for (drink, barrel_pos) in Drink::iterator() {
        commands
            .spawn(SpriteBundle {
                texture: textures.barrel.clone(),
                transform: Transform {
                    translation: barrel_pos,
                    scale: ScaleByAssetResolution::Res720p.scale(),
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

#[derive(Default)]
struct CustomerSlot {
    customer: Option<CustomerBundle>,
    spawned: bool,
}

impl CustomerSlot {
    fn is_full(&self) -> bool {
        self.customer.is_some() || self.spawned
    }
}

#[derive(Default)]
struct BarCustomerSlots {
    left: CustomerSlot,
    middle: CustomerSlot,
    right: CustomerSlot,
}

impl BarCustomerSlots {
    fn is_full(&self) -> bool {
        self.left.is_full() && self.middle.is_full() && self.right.is_full()
    }

    fn get_random_empty_slot(&mut self) -> Option<&mut CustomerSlot> {
        let mut rng = rand::thread_rng();
        let mut slots = vec![&mut self.left, &mut self.middle, &mut self.right];
        slots.shuffle(&mut rng);

        slots.into_iter().find(|slot| !slot.is_full())
    }
}

fn spawn_customer(mut bar_q: Query<&mut Bar>, textures: Res<TextureAssets>, time: Res<Time>) {
    let mut bar = bar_q.single_mut();

    if !bar.customer_slots.is_full() {
        if bar.customer_spawn_timer.tick(time.delta()).just_finished() {
            if let Some(slot) = bar.customer_slots.get_random_empty_slot() {
                slot.customer = Some(generate_random_customer(&textures));
            }
            bar.customer_spawn_timer.reset();
            let rand_duration = rand::thread_rng().gen_range(2..5);
            bar.customer_spawn_timer
                .set_duration(Duration::from_secs(rand_duration));
        }
    }
}

fn spawn_customers_in_slots(mut commands: Commands, mut bar_q: Query<&mut Bar>) {
    let mut bar = bar_q.single_mut();

    if bar.customer_slots.left.customer.is_some() && !bar.customer_slots.left.spawned {
        let mut customer = bar.customer_slots.left.customer.take().unwrap();
        customer.sprite_bundle.transform.translation = BAR_CUSTOMER_SLOT_LEFT;
        commands.spawn(customer);
        bar.customer_slots.left.spawned = true;
    }
    if bar.customer_slots.middle.customer.is_some() && !bar.customer_slots.middle.spawned {
        let mut customer = bar.customer_slots.middle.customer.take().unwrap();
        customer.sprite_bundle.transform.translation = BAR_CUSTOMER_SLOT_MIDDLE;
        commands.spawn(customer);
        bar.customer_slots.middle.spawned = true;
    }
    if bar.customer_slots.right.customer.is_some() && !bar.customer_slots.right.spawned {
        let mut customer = bar.customer_slots.right.customer.take().unwrap();
        customer.sprite_bundle.transform.translation = BAR_CUSTOMER_SLOT_RIGHT;
        commands.spawn(customer);
        bar.customer_slots.right.spawned = true;
    }
}
