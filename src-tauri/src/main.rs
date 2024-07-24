// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use serde::{Deserialize, Serialize};
/// ボード
#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    columns: Vec<Column>,
}
/// カラム
#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    id: i64,
    title: String,
    cards: Vec<Card>,
}
/// カード
#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    id: i64,
    title: String,
    description: Option<String>,
}
/// カードの位置
#[derive(Debug, Serialize, Deserialize)]
pub struct CardPos {
    #[serde(rename = "columnId")]
    column_id: i64,
    position: i64,
}
impl Column {
    pub fn new(id: i64, title: &str) -> Self {
        Column {
            id,
            title: title.to_string(),
            cards: Vec::new(),
        }
    }
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}
impl Card {
    pub fn new(id: i64, title: &str, description: Option<&str>) -> Self {
        Card {
            id,
            title: title.to_string(),
            description: description.map(ToString::to_string),
        }
    }
}
// ボードのデータを作成して返すハンドラ
#[tauri::command]
fn get_board() -> Result<Board, String> {
    let mut col0 = Column::new(0, "バックログ");
    col0.add_card(Card::new(0, "かんばんボードを追加する", Some("react-kanbanを使用する")));
    let col1 = Column::new(1, "開発中");
    let board = Board { columns: vec![col0, col1] };
    Ok(board)
}
/// カードの追加直後に呼ばれるハンドラ
#[tauri::command]
async fn handle_add_card(card: Card, pos: CardPos) -> Result<(), String> {
    // IPCで受信したデータをデバッグ表示する
    println!("handle_add_card ----------");
    dbg!(&card);
    dbg!(&pos);
    Ok(())
}
/// カードの移動直後に呼ばれるハンドラ
#[tauri::command]
async fn handle_move_card(card: Card, from: CardPos, to: CardPos) -> Result<(), String> {
    println!("handle_move_card ----------");
    dbg!(&card);
    dbg!(&from);
    dbg!(&to);
    Ok(())
}
/// カードの削除直後に呼ばれるハンドラ
#[tauri::command]
async fn handle_remove_card(card: Card, column_id: i64) -> Result<(), String> {
    println!("handle_remove_card ----------");
    dbg!(&card);
    dbg!(&column_id);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        // ハンドラを登録する。（元々あったgreetハンドラは削除した）
        .invoke_handler(tauri::generate_handler![
            get_board,
            handle_add_card,
            handle_move_card,
            handle_remove_card
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// fn main() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
