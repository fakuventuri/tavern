use std::time::Duration;

use bevy::prelude::*;
use rand::{seq::SliceRandom, Rng};

use crate::{loading::TextureAssets, remove_value_from_vec, GameState, ScaleByAssetResolution};

use super::{
    customer::{generate_random_customer, Customer, CustomerBundle},
    ActiveInteractibleActions, ClickedInteractible, CustomersStats, DrinkInHand,
    IgnoredInteractibleActions, IngameState, InteractibleAction, InteractibleBundle,
    MainCameraIngame, MoveCameraTo, OnIngameScreen,
};

// The bar counter
pub struct BarPlugin;

// Constants
/// The y position of the customer when they are at the bar
pub const BAR_CUSTOMER_TARGET_Y: f32 = -850.; // y: -850.
/// The y position of the customer when they not visible
pub const BAR_CUSTOMER_HIDDEN_Y: f32 = -1400.;
const SLOT_LEFT_SPAWN_POINT: Vec3 = Vec3::new(-700., BAR_CUSTOMER_HIDDEN_Y, 3.); // z = 3.
const SLOT_MIDDLE_SPAWN_POINT: Vec3 = Vec3::new(0., BAR_CUSTOMER_HIDDEN_Y, 2.);
const SLOT_RIGHT_SPAWN_POINT: Vec3 = Vec3::new(700., BAR_CUSTOMER_HIDDEN_Y, 1.);

impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(OnEnter(GameState::Playing), setup_bar)
            .add_systems(
                Update,
                (
                    handle_bar_interactible_click.run_if(in_state(IngameState::Running)),
                    spawn_customers_in_slots.run_if(in_state(IngameState::Running)),
                    spawn_customer.run_if(in_state(IngameState::Running)),
                ),
            );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Drink {
    Beer,
    Wine,
    Whiskey,
}

impl Drink {
    pub fn iterator() -> impl Iterator<Item = (Drink, Vec3)> {
        vec![
            (Drink::Beer, Vec3::new(750., -615., 14.)),
            (Drink::Wine, Vec3::new(400., -615., 13.)),
            (Drink::Whiskey, Vec3::new(50., -615., 12.)),
        ]
        .into_iter()
    }
}

#[derive(Component)]
pub struct Bar {
    customer_slots: BarCustomerSlots,
    // customer_queue: Vec<CustomerBundle>,
    customer_spawn_timer: Timer,
}

impl Bar {
    pub fn remove_customer(&mut self, slot_marker: &CustomerSlotMarker) {
        match slot_marker {
            CustomerSlotMarker::Left => {
                self.customer_slots.left.spawned = false;
            }
            CustomerSlotMarker::Middle => {
                self.customer_slots.middle.spawned = false;
            }
            CustomerSlotMarker::Right => {
                self.customer_slots.right.spawned = false;
            }
        }
    }
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
        .insert(InteractibleBundle::new(InteractibleAction::Bar))
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
            .insert(InteractibleBundle::new(InteractibleAction::Barrel(drink)))
            .insert(OnIngameScreen);

        commands
            .spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: match drink {
                            Drink::Beer => "Beer",
                            Drink::Wine => "Wine",
                            Drink::Whiskey => "Whiskey",
                        }
                        .to_string(),
                        style: TextStyle {
                            font_size: 55.,
                            color: Color::BLACK,
                            ..Default::default()
                        },
                    }],
                    linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                    alignment: TextAlignment::Center,
                },
                transform: Transform {
                    translation: barrel_pos + Vec3::new(-50., 25., 1.),
                    ..Default::default()
                },
                ..Default::default()
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

#[derive(Component)]
pub enum CustomerSlotMarker {
    Left,
    Middle,
    Right,
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

fn spawn_customer(
    mut bar_q: Query<&mut Bar>,
    textures: Res<TextureAssets>,
    time: Res<Time>,
    customers_stats: Res<CustomersStats>,
) {
    let mut bar = bar_q.single_mut();

    if !bar.customer_slots.is_full() {
        if bar.customer_spawn_timer.tick(time.delta()).just_finished() {
            if let Some(slot) = bar.customer_slots.get_random_empty_slot() {
                slot.customer = Some(generate_random_customer(&textures));
            }
            bar.customer_spawn_timer.reset();
            let rand_next_customer_time =
                rand::thread_rng().gen_range(customers_stats.customers_spawn_gap.clone());
            bar.customer_spawn_timer
                .set_duration(Duration::from_secs(rand_next_customer_time));
        }
    }
}

fn spawn_customers_in_slots(mut commands: Commands, mut bar_q: Query<&mut Bar>) {
    let mut bar = bar_q.single_mut();

    if bar.customer_slots.left.customer.is_some() && !bar.customer_slots.left.spawned {
        let mut customer = bar.customer_slots.left.customer.take().unwrap();
        customer.sprite_bundle.transform.translation = SLOT_LEFT_SPAWN_POINT;
        commands.spawn(customer).insert(CustomerSlotMarker::Left);
        bar.customer_slots.left.spawned = true;
    }
    if bar.customer_slots.middle.customer.is_some() && !bar.customer_slots.middle.spawned {
        let mut customer = bar.customer_slots.middle.customer.take().unwrap();
        customer.sprite_bundle.transform.translation = SLOT_MIDDLE_SPAWN_POINT;
        commands.spawn(customer).insert(CustomerSlotMarker::Middle);
        bar.customer_slots.middle.spawned = true;
    }
    if bar.customer_slots.right.customer.is_some() && !bar.customer_slots.right.spawned {
        let mut customer = bar.customer_slots.right.customer.take().unwrap();
        customer.sprite_bundle.transform.translation = SLOT_RIGHT_SPAWN_POINT;
        commands.spawn(customer).insert(CustomerSlotMarker::Right);
        bar.customer_slots.right.spawned = true;
    }
}

fn handle_bar_interactible_click(
    //
    mut commands: Commands,
    mut move_camera_to_q: Query<
        &mut MoveCameraTo,
        (With<MainCameraIngame>, Without<InteractibleAction>),
    >,
    interactibles_q: Query<
        (Entity, &InteractibleAction),
        (With<ClickedInteractible>, Without<Customer>),
    >,
    mut active_interactibles_q: Query<&mut ActiveInteractibleActions>,
    mut ignored_interactibles_q: Query<&mut IgnoredInteractibleActions>,
    mut drink_in_hand: ResMut<DrinkInHand>,
) {
    let mut move_camera_to = move_camera_to_q.single_mut();
    let mut active_interactibles = active_interactibles_q.single_mut();
    let mut ignored_interactibles = ignored_interactibles_q.single_mut();

    for (entity, interactible_action) in interactibles_q.iter() {
        commands.entity(entity).remove::<ClickedInteractible>(); // Reset clicked
        match *interactible_action {
            InteractibleAction::Bar => {
                move_camera_to.0 = Some(Vec2::new(0., -630.)); // -275. = One shelf height | -630. = Two shelf height

                // Deactivate Bar
                remove_value_from_vec(InteractibleAction::Bar, &mut active_interactibles.0);
                // Ignore Customer
                // ignored_interactibles.0.push(InteractibleAction::Customer);
                // Stop ignoring Barrels
                InteractibleAction::get_barrels().iter().for_each(|barrel| {
                    remove_value_from_vec(*barrel, &mut ignored_interactibles.0)
                });
                // Activate ExitBar
                active_interactibles.0.push(InteractibleAction::ExitBar);
            }
            InteractibleAction::ExitBar => {
                move_camera_to.0 = Some(Vec2::new(0., 0.));
                // Deactivate ExitBar
                remove_value_from_vec(InteractibleAction::ExitBar, &mut active_interactibles.0);
                // Ignore Barrels
                InteractibleAction::get_barrels()
                    .iter()
                    .for_each(|barrel| ignored_interactibles.0.push(*barrel));
                // Stop ignoring Customer
                // remove_value_from_vec(InteractibleAction::Customer, &mut ignored_interactibles.0);
                // Activate Bar
                active_interactibles.0.push(InteractibleAction::Bar);
            }
            InteractibleAction::Barrel(drink) => {
                drink_in_hand.0 = Some(drink);
            }
            InteractibleAction::Customer => {
                unreachable!("Customers should be ignored in this query")
            }
            InteractibleAction::_None => {}
        }
    }
}
