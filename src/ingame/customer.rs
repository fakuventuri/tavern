use bevy::prelude::*;
use rand::seq::{IteratorRandom, SliceRandom};

use crate::{loading::TextureAssets, ScaleByAssetResolution};

use super::{
    bar::{Bar, CustomerSlotMarker, Drink, BAR_CUSTOMER_HIDDEN_Y, BAR_CUSTOMER_TARGET_Y},
    ClickedInteractible, CustomersStats, DrinkInHand, IngameState, InteractibleAction,
    InteractibleBundle, InteractionSpriteColors, OnIngameScreen,
};

pub struct CustomerPlugin;

const CUSTOMER_SLIDE_SPEED: f32 = 810.;
const CUSTOMER_DRINKING_DURATION: f32 = 3.;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(
                Update,
                customers_system.run_if(in_state(IngameState::Running)),
            );
    }
}

#[derive(Component)]
pub struct Customer {
    name: String,
    state: CustomerState,
    drink: Drink,
}

#[derive(Debug, PartialEq)]
pub enum CustomerState {
    Spawning,
    Waiting(Timer),
    Drinking(Timer),
    Leaving,
}

#[derive(Bundle)]
pub struct CustomerBundle {
    customer: Customer,
    pub sprite_bundle: SpriteBundle,
    interactible_bundle: InteractibleBundle,
    marker: OnIngameScreen,
}

impl CustomerBundle {
    pub fn new(name: &str, drink: Drink, texture: Handle<Image>, transform: Transform) -> Self {
        Self {
            customer: Customer {
                name: name.to_string(),
                state: CustomerState::Spawning,
                drink,
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
            interactible_bundle: InteractibleBundle::new(InteractibleAction::Customer),
            marker: OnIngameScreen,
        }
    }
}

fn customers_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Transform,
        // &mut Sprite,
        &mut InteractionSpriteColors,
        &mut Customer,
        &CustomerSlotMarker,
        Option<&ClickedInteractible>,
    )>,
    customers_stats: Res<CustomersStats>,
    mut bar_q: Query<&mut Bar>,
    mut drink_in_hand: ResMut<DrinkInHand>,
) {
    for (
        entity,
        mut transform,
        mut interaction_sprite_colors,
        mut customer,
        slot_marker,
        clicked,
    ) in query.iter_mut()
    {
        match &mut customer.state {
            CustomerState::Spawning => {
                transform.translation.y += CUSTOMER_SLIDE_SPEED * time.delta_seconds();
                if transform.translation.y >= BAR_CUSTOMER_TARGET_Y {
                    customer.state = CustomerState::Waiting(Timer::from_seconds(
                        customers_stats.customers_wait_duration,
                        TimerMode::Once,
                    ));
                }
            }
            CustomerState::Waiting(timer) => {
                // Turn redder the closer the timer is to finishing
                interaction_sprite_colors.normal =
                    Color::rgb(1., 1. * timer.percent_left(), 1. * timer.percent_left());
                interaction_sprite_colors.highlight =
                    Color::rgb(1.3, 1.3 * timer.percent_left(), 1.3 * timer.percent_left());

                transform.translation.y -= 10. * time.delta_seconds();

                // 180. = oscillation frequency // 4. = oscillation range
                transform.translation.x += (timer.percent().powf(2.) * 180.).cos()
                    * (180. / std::f32::consts::PI) // In parentesis = convert to radians
                    * 4.
                    * timer.percent().powf(2.)
                    * time.delta_seconds();

                if timer.tick(time.delta()).just_finished() {
                    customer.state = CustomerState::Leaving;
                } else if clicked.is_some() {
                    commands.entity(entity).remove::<ClickedInteractible>(); // Reset clicked
                    if let Some(drink) = drink_in_hand.0.take() {
                        if drink == customer.drink {
                            // ToDo Add money and rep. Get drink value from drink
                            transform.translation.y = BAR_CUSTOMER_TARGET_Y;
                            interaction_sprite_colors.normal = Color::rgb(0.6, 1., 0.6);
                            interaction_sprite_colors.highlight = Color::rgb(0.9, 1.3, 0.9);
                            customer.state = CustomerState::Drinking(Timer::from_seconds(
                                CUSTOMER_DRINKING_DURATION,
                                TimerMode::Once,
                            ));
                        }
                    }
                }
            }
            CustomerState::Drinking(timer) => {
                // ToDo Drinking animation using Sin with the timer as input
                // 180. = oscillation frequency // 4. = oscillation range
                transform.translation.y += (timer.percent() * 45.).cos()
                * (180. / std::f32::consts::PI) // In parentesis = convert to radians
                * 2.
                * time.delta_seconds();

                if timer.tick(time.delta()).just_finished() {
                    customer.state = CustomerState::Leaving;
                }
            }
            CustomerState::Leaving => {
                transform.translation.y -= CUSTOMER_SLIDE_SPEED * time.delta_seconds();
                if transform.translation.y <= BAR_CUSTOMER_HIDDEN_Y {
                    commands.entity(entity).despawn_recursive();
                    bar_q.single_mut().remove_customer(slot_marker);
                }
            }
        }
    }
}

pub fn generate_random_customer(textures: &Res<TextureAssets>) -> CustomerBundle {
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
