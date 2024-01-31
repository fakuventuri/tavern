use bevy::prelude::*;
use rand::seq::{IteratorRandom, SliceRandom};

use crate::{loading::TextureAssets, ScaleByAssetResolution};

use super::{
    bar::{Bar, CustomerSlotMarker, Drink, BAR_CUSTOMER_HIDDEN_Y, BAR_CUSTOMER_TARGET_Y},
    ClickedInteractible, CustomersStats, DrinkInHand, IngameState, InteractibleAction,
    InteractibleBundle, InteractionSpriteColors, OnIngameScreen, PlayerStats,
};

pub struct CustomerPlugin;

const CUSTOMER_SLIDE_SPEED: f32 = 810.;
const CUSTOMER_DRINKING_DURATION: f32 = 1.;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(
                Update,
                (
                    customers_system.run_if(in_state(IngameState::Running)),
                    handle_order_popup.run_if(in_state(IngameState::Running)),
                ),
            );
    }
}

#[derive(Component)]
struct OrderPopup(Timer);

#[derive(Bundle)]
struct OrderPopupBundle {
    order_popup_marker: OrderPopup,
    text_2d_bundle: Text2dBundle,
    marker: OnIngameScreen,
}

impl OrderPopupBundle {
    fn new(drink: Drink, translation: Vec3, color: Color, extra_z: f32, duration: f32) -> Self {
        Self {
            order_popup_marker: OrderPopup(Timer::from_seconds(duration, TimerMode::Once)),
            text_2d_bundle: Text2dBundle {
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
                            color,
                            ..Default::default()
                        },
                    }],
                    linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                    alignment: TextAlignment::Center,
                },
                transform: Transform {
                    translation: Vec3::new(translation.x, -325., 12. + extra_z),
                    ..Default::default()
                },

                ..Default::default()
            },
            marker: OnIngameScreen,
        }
    }
}

#[derive(Component)]
pub struct Customer {
    _name: String,
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
                _name: name.to_string(),
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
    mut player_stats: ResMut<PlayerStats>,
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
                    // Reset streak on failed drink delivery
                    player_stats.streak = 0;
                    customer.state = CustomerState::Leaving;
                } else if clicked.is_some() {
                    commands.entity(entity).remove::<ClickedInteractible>(); // Reset clicked

                    // Show order popup on customer click
                    spawn_popup(
                        &mut commands,
                        &customer,
                        &transform,
                        &interaction_sprite_colors,
                        1.,
                        1.,
                    );

                    if let Some(drink) = drink_in_hand.0.take() {
                        if drink == customer.drink {
                            transform.translation.y = BAR_CUSTOMER_TARGET_Y;
                            interaction_sprite_colors.normal = Color::rgb(0.6, 1., 0.6);
                            interaction_sprite_colors.highlight = Color::rgb(0.9, 1.3, 0.9);
                            customer.state = CustomerState::Drinking(Timer::from_seconds(
                                CUSTOMER_DRINKING_DURATION,
                                TimerMode::Once,
                            ));

                            // Show order popup on Successful drink delivery
                            spawn_popup(
                                &mut commands,
                                &customer,
                                &transform,
                                &interaction_sprite_colors,
                                10.,
                                CUSTOMER_DRINKING_DURATION,
                            );

                            // Add money, streak and reputation
                            player_stats.money +=
                                drink.get_price() * (player_stats.streak as f64 / 2.).max(1.);
                            player_stats.streak += 1;
                            if player_stats.streak > player_stats.highest_streak {
                                player_stats.highest_streak = player_stats.streak;
                            }
                            player_stats.reputation_progress += 1;
                        }
                    }
                }
            }
            CustomerState::Drinking(timer) => {
                // 20. = oscillation frequency // 2. = oscillation range
                transform.translation.y += (timer.percent() * 20.).cos()
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

// ToDo find a better way that dont spawn multiple popups and or dont collide in z axis
fn spawn_popup(
    commands: &mut Commands,
    customer: &Customer,
    transform: &Transform,
    interaction_sprite_colors: &InteractionSpriteColors,
    extra_z: f32,
    duration: f32,
) {
    // Background
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(200., 100.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                transform.translation.x,
                -325.,
                11. + extra_z,
            )),
            ..default()
        })
        .insert(OnIngameScreen)
        .insert(OrderPopup(Timer::from_seconds(duration, TimerMode::Once)));
    // Popup text
    commands.spawn(OrderPopupBundle::new(
        customer.drink,
        transform.translation,
        interaction_sprite_colors.normal,
        extra_z,
        duration,
    ));
}

fn handle_order_popup(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut OrderPopup, &mut Transform)>,
) {
    for (entity, mut order_popup, mut _transform) in query.iter_mut() {
        if order_popup.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        } else {
            // _transform.translation.y += 100. * time.delta_seconds();
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
