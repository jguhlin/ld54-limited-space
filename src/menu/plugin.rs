use bevy::prelude::*;

use crate::settings::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Menu state
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, (menu, text_input).run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

fn text_input(
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<Input<KeyCode>>,
    mut seed: ResMut<Seed>,
) {
    // TODO: Rate limit this...
    if kbd.pressed(KeyCode::Back) {
        seed.raw.pop();
    }

    for ev in evr_char.iter() {
        // ignore control (special) characters
        if !ev.char.is_control() {
            seed.raw.push(ev.char);
        }
    }
}

#[derive(Component)]
struct SeedInput;

#[derive(Component)]
struct MenuBase;

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let mut node_ec = commands.spawn((
        NodeBundle {
            style: Style {
                // center button
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        MenuBase,
    ));

    // TODO: Add volume slider
    node_ec.with_children(|parent| {
        parent
            .spawn(TextBundle::from_section(
                "Limited Space",
                TextStyle {
                    font_size: 40.0,
                    font: font.clone(),
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ))
            .insert(SeedInput);
        parent
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(150.),
                    height: Val::Px(65.),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Play",
                    TextStyle {
                        font_size: 40.0,
                        font: font.clone(),
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            });
    });
}

fn menu(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut seed_input: Query<&mut Text, With<SeedInput>>,
    seed: Res<Seed>,
) {
    let mut seed_input = seed_input.get_single_mut().unwrap();
    seed_input.sections[0].value = seed.raw.clone();

    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(GameState::BuildMap);
                // TODO: Generate actual seed and place it in the struct...
                // Reinitialize rng with it as well.
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<MenuBase>>) {
    let e = menu.get_single().unwrap();
    commands.entity(e).despawn_recursive();
}
