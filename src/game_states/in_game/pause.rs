pub mod pause01;
pub mod pause02;
pub mod save_data;

use bevy::prelude::*;

use crate::{despawn_screen, SelectedStory};
use crate::game_states::main_menu::setting_button;

pub fn pause_scene_plugin(app: &mut App) {
    app.init_state::<PauseState>()
        .add_systems(OnEnter(PauseState::Pause), pause01::setup_pause_ui)
        .add_systems(
            OnEnter(PuaseButtonState::Pressed),
            (pause02::flip_to_pause_node, pause01::flip_ui_to_not_visible),
        )
        .add_systems(
            OnEnter(PuaseButtonState::NotPressed),
            (pause01::flip_to_not_pause_node, pause01::flip_ui_to_visible)
                .run_if(in_state(PauseState::Pause)),
        )
        .add_systems(
            Update,
            (
                pause01::pause_button_system,
                pause02::in_pause_button_system,
                pause02::in_pause_in_load_button_system,
            )
                .run_if(in_state(PauseState::Pause)),
        )
        .add_systems(Update, (
            setting_button::<SelectedStory>,
        ))
        .add_systems(OnExit(PauseState::Pause), despawn_screen::<OnPause>);
}

// Tag component used to tag entities added on the new_game_loading scene
#[derive(Component)]
pub struct OnPause;

#[derive(Component)]
pub struct PauseButtonMarker;

#[derive(Component)]
pub struct PauseButtonPauseMarker;

#[derive(Component)]
pub struct PauseButtonNotPauseMarker;
#[derive(Component)]
pub struct FlipVisibilityMarker;

//
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PauseState {
    Pause,
    #[default]
    Disabled,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PuaseButtonState {
    Pressed,
    #[default]
    NotPressed,
}

#[derive(Component)]
enum InPauseButtonAction {
    Save,
    Load,
}
