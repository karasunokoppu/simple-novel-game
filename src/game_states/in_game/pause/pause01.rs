use bevy::prelude::*;

use crate::game_states::in_game::pause::{
    FlipVisibilityMarker, OnPause, PauseButtonMarker, PauseButtonNotPauseMarker,
    PauseButtonPauseMarker, PauseButtonState,
};

//Pauseボタンを押す前と押したときの処理

//Color
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.5, 0.5, 0.5);
const UI_BORDER_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const UI_BACKGROUND_COLOR: Color = Color::Srgba(Srgba::new(0.2, 0.2, 0.2, 0.8));

pub fn setup_pause_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 1000.0),
            OnPause,
            PauseButtonPauseMarker,
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(60.0),
                    height: Val::Px(30.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::new(Val::Px(3.0), Val::Px(3.0), Val::Px(3.0), Val::Px(3.0)),
                    ..default()
                },
                BorderColor(UI_BORDER_COLOR),
                BackgroundColor(UI_BACKGROUND_COLOR),
                Button,
                PauseButtonMarker,
            ));
        });
    println!("Pause ui is spawned");
}

//PauseButtonStateがNotPressedのときに、表示されるウィンドウ
pub fn flip_to_not_pause_node(
    mut commands: Commands,
    pause_entities: Query<Entity, With<PauseButtonPauseMarker>>,
    not_pause_button_entities: Query<Entity, With<PauseButtonNotPauseMarker>>,
) {
    let added_node = commands
        .spawn((
            Node {
                width: Val::Px(60.0),
                height: Val::Px(30.0),
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::new(Val::Px(3.0), Val::Px(3.0), Val::Px(3.0), Val::Px(3.0)),
                ..default()
            },
            BorderColor(UI_BORDER_COLOR),
            BackgroundColor(UI_BACKGROUND_COLOR),
            Button,
            OnPause,
            PauseButtonMarker,
        ))
        .id();

    if !not_pause_button_entities.is_empty() {
        for pause_entity in not_pause_button_entities.iter() {
            commands.entity(pause_entity).despawn();
        }
    }

    for pause_entity in pause_entities.iter() {
        commands
            .entity(pause_entity)
            .insert_children(0, &[added_node]);
    }
}

pub fn pause_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            (With<Button>, With<PauseButtonMarker>),
        ),
    >,
    pause_button_state: Res<State<PauseButtonState>>,
    mut next_pause_button_state: ResMut<NextState<PauseButtonState>>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                match *pause_button_state.get() {
                    PauseButtonState::Pressed => {
                        next_pause_button_state.set(PauseButtonState::NotPressed)
                    }
                    PauseButtonState::NotPressed => {
                        next_pause_button_state.set(PauseButtonState::Pressed)
                    }
                }
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *background_color = UI_BACKGROUND_COLOR.into();
            }
        }
    }
}

pub fn flip_ui_to_visible(
    mut test_ui: Query<(&mut Node, &FlipVisibilityMarker), With<FlipVisibilityMarker>>,
) {
    for (mut node, _) in test_ui.iter_mut() {
        node.display = Display::Flex;
    }
}
pub fn flip_ui_to_not_visible(
    mut test_ui: Query<(&mut Node, &FlipVisibilityMarker), With<FlipVisibilityMarker>>,
) {
    println!("{}", test_ui.iter().count());
    for (mut node, _) in test_ui.iter_mut() {
        node.display = Display::None;
    }
}
