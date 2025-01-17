use bevy::prelude::*;

use crate::{despawn_screen, game_states::in_game::InGameState};

pub fn conrol_scene_plugin(app: &mut App) {
    app
        .add_systems(OnExit(InGameState::Control), despawn_screen::<OnControl>);
}

// Tag component used to tag entities added on the control scene
#[derive(Component)]
struct OnControl;