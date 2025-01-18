use bevy::{prelude::*, state::commands};

use crate::despawn_screen;

use super::DrawUIState;


pub fn select_ui_plugin(app: &mut App) {
    app
    .add_systems(OnEnter(DrawUIState::Select), setup_select_ui)
    .add_systems(OnExit(DrawUIState::Select), despawn_screen::<OnSelectUI>);
}

#[derive(Component)]
struct OnSelectUI;

fn setup_select_ui(
    mut commands: Commands
) {
    commands.spawn((
        Node{
            ..default()
        },
        OnSelectUI
    ));
}