use bevy::prelude::*;

use crate::despawn_screen;

pub fn pause_scene_plugin(app: &mut App) {
    app.init_state::<PauseState>()
        .add_systems(OnEnter(PauseState::Pause), (
            setup_pause_ui,
        ))
        .add_systems(Update, pause_button_system.run_if(in_state(PauseState::Pause)))
        .add_systems(OnEnter(PuaseButtonState::Pressed), flip_to_pause_node)
        .add_systems(OnEnter(PuaseButtonState::NotPressed), flip_to_not_pause_node)
        .add_systems(OnExit(PauseState::Pause), despawn_screen::<OnPause>);
        //.add_systems(OnExit(PuaseButtonState::NotPressed), despawn_screen::<PauseButtonState>);
}
//TODO 1.[pause時にセーブしたらsavesディレクトリに[数字].ronファイルを生成するようにする]

// Tag component used to tag entities added on the new_game_loading scene
#[derive(Component)]
struct OnPause;

#[derive(Component)]
pub struct PauseButtonMarker;

#[derive(Component)]
struct PauseButtonPauseMarker;

#[derive(Component)]
struct PauseButtonNotPauseMarker;

//
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PauseState {
    Pause,
    #[default]
    Disabled,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PuaseButtonState{
    Pressed,
    #[default]
    NotPressed,
}

//Color
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.5, 0.5, 0.5);
const UI_BORDER_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const UI_BACKGROUND_COLOR: Color = Color::Srgba(Srgba::new(0.2, 0.2, 0.2, 0.8));

fn setup_pause_ui(mut commands: Commands) {
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
            OnPause,PauseButtonPauseMarker,
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(60.0),
                    height: Val::Px(30.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::new(
                        Val::Px(3.0),
                        Val::Px(3.0),
                        Val::Px(3.0),
                        Val::Px(3.0),
                    ),
                    ..default()
                },
                BorderColor(UI_BORDER_COLOR),
                BackgroundColor(UI_BACKGROUND_COLOR),
                Button,PauseButtonMarker
            ));
        });
    println!("Pause ui is spawned");
}

fn pause_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, (With<Button>, With<PauseButtonMarker>)),
    >,
    pause_button_state: Res<State<PuaseButtonState>>,
    mut next_pause_button_state: ResMut<NextState<PuaseButtonState>>
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                match *pause_button_state.get() {
                    PuaseButtonState::Pressed => {
                        next_pause_button_state.set(PuaseButtonState::NotPressed)
                    }
                    PuaseButtonState::NotPressed => {
                        next_pause_button_state.set(PuaseButtonState::Pressed)
                    }
                }
            },
            Interaction::Hovered => *background_color = HOVERED_BUTTON.into(),
            Interaction::None => {
                match *pause_button_state.get() {
                    PuaseButtonState::Pressed => {
                        *background_color = BackgroundColor(Color::srgb(1.0, 0.0, 0.0));
                    }
                    PuaseButtonState::NotPressed => {
                        *background_color = BackgroundColor(Color::srgb(0.0, 0.0, 1.0));
                    }
                }
            },
        }
    }
}

fn flip_to_pause_node(
    mut commands: Commands,
    pause_entities: Query<Entity, With<PauseButtonPauseMarker>>,
    pause_button_entities: Query<Entity, With<PauseButtonMarker>>
) {
    let added_node = commands.spawn((
        Node{
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::WHITE),
        PauseButtonNotPauseMarker,
    )).with_children(|parent|{
        parent.spawn((
            Node {
                width: Val::Px(60.0),
                height: Val::Px(30.0),
                padding: UiRect::all(Val::Px(10.0)),
                justify_items: JustifyItems::End,
                border: UiRect::new(
                    Val::Px(3.0),
                    Val::Px(3.0),
                    Val::Px(3.0),
                    Val::Px(3.0),
                ),
                ..default()
            },
            BorderColor(UI_BORDER_COLOR),
            BackgroundColor(UI_BACKGROUND_COLOR),
            Button,PauseButtonMarker
        ));
    }).id();

    for pause_entity in pause_button_entities.iter(){
        commands.entity(pause_entity).despawn();
    }

    for pause_entity in pause_entities.iter(){
        commands.entity(pause_entity).insert_children(0, &[added_node]);
    }
}

fn flip_to_not_pause_node(
    mut commands: Commands,
    pause_entities: Query<Entity, With<PauseButtonPauseMarker>>,
    not_pause_button_entities: Query<Entity, With<PauseButtonNotPauseMarker>>
) {
    let added_node = commands.spawn((
        Node {
            width: Val::Px(60.0),
            height: Val::Px(30.0),
            padding: UiRect::all(Val::Px(10.0)),
            border: UiRect::new(
                Val::Px(3.0),
                Val::Px(3.0),
                Val::Px(3.0),
                Val::Px(3.0),
            ),
            ..default()
        },
        BorderColor(UI_BORDER_COLOR),
        BackgroundColor(UI_BACKGROUND_COLOR),
        Button,PauseButtonMarker
    )).id();

    if !not_pause_button_entities.is_empty() {
        for pause_entity in not_pause_button_entities.iter(){
            commands.entity(pause_entity).despawn();
        }
    }

    for pause_entity in pause_entities.iter(){
        commands.entity(pause_entity).insert_children(0, &[added_node]);
    }
}