use bevy::prelude::*;

use crate::despawn_screen;

pub fn pause_scene_plugin(app: &mut App) {
    app
    .init_state::<PauseState>()
    .add_systems(OnExit(PauseState::Disabled), despawn_screen::<OnPause>);
}
//TODO 1.[セーブしたらsavesディレクトリに[数字].ronファイルを生成するようにする]
// Tag component used to tag entities added on the new_game_loading scene
#[derive(Component)]
struct OnPause;

//
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PauseState{
    Pause,
    #[default]
    Disabled,
}