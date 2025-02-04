use bevy::prelude::*;
use std::{fs::{self, File},path::Path, io::{Read, Write}};

use crate::game_states::in_game::NovelGameStates;
use crate::game_states::main_menu::LoadDataEvent;

pub fn save_data(
    data: &NovelGameStates,
){
    let file_count: u32 = match count_ron_files_in_save_dir() {
        Ok(count) => {println!("count is {}", count);count as u32},
        Err(e) => panic!("{e}")
    };
    let file_name: u32 = file_count + 1;
    let file_path = format!("saves/{}.ron", file_name);
    let save_strings = ron::to_string(data).expect("Failed to serialize data");
    let mut file = File::create(file_path).unwrap();

    file.write_all(save_strings.as_bytes()).unwrap();

}

pub fn load_data(
    mut data: ResMut<NovelGameStates>,
    mut save_data_iter: EventReader<LoadDataEvent>,
){
    for load_data_index in save_data_iter.read(){
        let file_path = format!("saves/{}.ron", load_data_index.message);
        let mut file = File::open(file_path).unwrap();
        let mut buffer = String::new();

        file.read_to_string(&mut buffer).unwrap();
        let load_data: NovelGameStates = ron::from_str(&buffer).expect("Failed to deserialize data");

        data.current_story_id = load_data.current_story_id;
        data.next_story_id = load_data.next_story_id;
        data.story = load_data.story;
    }
}

fn count_ron_files_in_save_dir() -> std::io::Result<usize> {
    let save_dir = Path::new("saves");

    if !save_dir.exists() || !save_dir.is_dir() {
        return Ok(0); // ディレクトリが存在しない場合は0を返す
    }

    let count = fs::read_dir(save_dir)?
        .filter_map(Result::ok) // 読み込みに失敗したファイルをスキップ
        .filter(|entry| {
            entry.path().extension().map_or(false, |ext| ext == "ron") // .ron拡張子を持つか確認
        })
        .count();

    Ok(count)
}