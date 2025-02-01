use bevy::prelude::*;
use std::fs::File;

use crate::{
    despawn_screen,
    game_states::in_game::{
        DisplayImage, ImageData, InGameState, NovelGameStates, StoryDataList, StoryImageList,
        StorySceneData, WallPaperData, WallpaperAssets, Wallpapers,
    }, GameState,
};

use super::{ImageAssets, StoryWallPaperList};

pub fn loading_game_plugin(app: &mut App) {
    app
    .add_systems(
        OnEnter(InGameState::LoadingGame),
        (
            game_state_to_ingame,
            reset_datas,
            deser_text_new_game,
            deser_image_new_game,
            deser_wallpaper_image,
            load_chara_image,
            load_wallpaper_image,
            in_game_state_to_control,
        )
            .chain(),
    )
    .add_systems(
        OnExit(InGameState::LoadingGame),
        despawn_screen::<OnContinueGameLoading>,
    );
}

// Tag component used to tag entities added on the start_game_loading scene
#[derive(Component)]
struct OnContinueGameLoading;
fn game_state_to_ingame(
    mut game_state: ResMut<NextState<GameState>>
) {
    game_state.set(GameState::InGame);
}

fn reset_datas(mut commands: Commands) {
    commands.insert_resource(StoryDataList::default());
    commands.insert_resource(StoryImageList::default());
    commands.insert_resource(ImageAssets::default());
    commands.insert_resource(StoryWallPaperList::default());
    commands.insert_resource(WallpaperAssets::default());
}

fn deser_text_new_game(
    mut data_list: ResMut<StoryDataList>,
    novel_game_states: Res<NovelGameStates>,
) {
    //[story data].ronを開く//
    let path = format!("assets/text_data/{}.ron", novel_game_states.story);
    let file = File::open(path).expect("fail opening file");

    //ストーリーデータのdeserialize
    let story_scene_datas: Vec<StorySceneData> = match ron::de::from_reader(file) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load StorySceneData: {}", e);
            std::process::exit(1);
        }
    };
    //TODO 3.[(ストーリ名、データリスト)のハッシュマップに入れる必要ある？]
    data_list
        .story_data_list
        .insert(novel_game_states.story.to_string(), story_scene_datas);
}

fn deser_image_new_game(
    mut image_list: ResMut<StoryImageList>,
    novel_game_states: Res<NovelGameStates>,
) {
    //[image data].ronを開く//
    let image_data_path = format!(
        "assets/image_data/asset_image/{}.ron",
        novel_game_states.story
    );
    let image_data_file = File::open(image_data_path).expect("fail opening image_data_file");
    let display_image_path = format!(
        "assets/image_data/image_position/{}.ron",
        novel_game_states.story
    );
    let display_image_file =
        File::open(display_image_path).expect("fail opening display_image_file");

    //image_dataのdeserialize
    let vec_image_data: Vec<ImageData> = match ron::de::from_reader(image_data_file) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load StorySceneData: {}", e);
            std::process::exit(1);
        }
    };

    //display_imageのdeserialize
    let vec_display_image: Vec<DisplayImage> = match ron::de::from_reader(display_image_file) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load StorySceneData: {}", e);
            std::process::exit(1);
        }
    };

    image_list.story_data_list.insert(
        novel_game_states.story.to_string(),
        (vec_image_data, vec_display_image),
    );
}

fn load_chara_image(
    mut image_assets: ResMut<ImageAssets>,
    image_list: Res<StoryImageList>,
    asset_server: Res<AssetServer>,
) {
    for (_, (image_datas, _)) in image_list.story_data_list.iter() {
        for image_data in image_datas.iter() {
            let handle = asset_server.load(format!(
                "image/charactor/{}/{}.png",
                image_data.chara, image_data.face
            ));
            image_assets.images.insert(image_data.image_id, handle);
        }
    }
}

fn deser_wallpaper_image(
    mut data_list: ResMut<StoryWallPaperList>,
    novel_game_states: Res<NovelGameStates>,
) {
    //[wallpaper data].ronを開く
    let image_data_path = format!(
        "assets/wallpaper_data/wallpaper_states/{}.ron",
        novel_game_states.story
    );
    let image_data_file = File::open(image_data_path).expect("fail opening file");
    let wallpaper_path = format!(
        "assets/wallpaper_data/wallpaper_assets/{}.ron",
        novel_game_states.story
    );
    let wallpaper_file = File::open(wallpaper_path).expect("fail opening file");

    //背景画像の遷移データのdeserialize
    let vec_wallpaper_data: Vec<WallPaperData> = match ron::de::from_reader(image_data_file) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load WallPaperData: {}", e);
            std::process::exit(1);
        }
    };

    //背景画像関連データのdeserialize
    let vec_image_data: Vec<Wallpapers> = match ron::de::from_reader(wallpaper_file) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load Wallpapers: {}", e);
            std::process::exit(1);
        }
    };

    //TODO 3.[(ストーリ名、データリスト)のハッシュマップに入れる必要ある？]
    data_list.story_data_list.insert(
        novel_game_states.story.to_string(),
        (vec_wallpaper_data, vec_image_data),
    );
}

fn load_wallpaper_image(
    mut wallpaper_assets: ResMut<WallpaperAssets>,
    image_list: Res<StoryWallPaperList>,
    asset_server: Res<AssetServer>,
) {
    for (_, (_, wallpapers)) in image_list.story_data_list.iter() {
        for wallpaper in wallpapers.iter() {
            let handle =
                asset_server.load(format!("image/background/{}.png", wallpaper.wallpaper_name));
            wallpaper_assets.images.insert(wallpaper.image_id, handle);
        }
    }
}

fn in_game_state_to_control(mut in_game_state: ResMut<NextState<InGameState>>) {
    in_game_state.set(InGameState::Control);
}
