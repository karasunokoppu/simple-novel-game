use bevy::prelude::*;

use crate::{despawn_screen, game_states::in_game::InGameState};

pub fn pause_scene_plugin(app: &mut App) {
    app.add_systems(OnExit(InGameState::Pause), despawn_screen::<OnPause>);
}

// Tag component used to tag entities added on the new_game_loading scene
#[derive(Component)]
struct OnPause;
