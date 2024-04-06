use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: i32,
    pub posted: String,
    pub sender: String,
    pub content: String,
}

static DATA_FILENAME: &str = "data.json";

// 一覧
pub fn get_all() -> Vec<Message> {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let mut json_data: Vec<Message> = serde_json::from_str(&file).unwrap();
    json_data.sort_by(|a, b| b.posted.cmp(&a.posted));

    json_data
}


// 詳細
pub fn get(id: i32) -> Message { // 関数名：get 引数：i32型 id return： Message型
    // DATA_FILENAME で指定されたファイルの内容を読み込み、file 変数に格納
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    // file 変数に格納された JSON データを serde_json クレートを使用して Vec<Message> 型にデシリアライズし、json_data 変数に格納
    let mut json_data = Vec<Message> = serde_json::from_str(&file).unwrap();
    // message 変数を初期化する
    let mut message = Message {id: 0,
            posted: "".to_string(),
            sender: "".to_string(),
            content: "".to_string()
        };
    // json_data 配列の中で id と一致する要素を検索し、見つかった場合は message 変数をその要素の内容で更新
    if let Some(index) = json_data.iter().position(|item| item.id == id) {
        message = json_data[index].clone();
    }
    // message 変数を返す
    message
}

// 新規作成
pub fn create(mut message: Message) -> Message {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let mut json_data: Vec<Message> = serde_json::from_str(&file).unwrap();
    let mut max = 0;
    for item in &json_data {
        max = std::cmp::max(item.id, max);
    }
    message.id = max + 1;
    println!("Create: {} {} {} {}",
        message.id,
        message.posted,
        message.sender,
        message.content
    );
    json_data.push(message);
    let json_str = serde_json::to_string(&json_data).unwrap();
    // シリアライズされた JSON 文字列を DATA_FILENAME ファイルに書き込む
    // 変数名がアンダースコアのみなのは、返ってきた値を使用しないため。その場合こういう変数名にする。逆に使用しない変数があるとコンパイル時ワーニングがでる
    let _ = fs::write(DATA_FILENAME, json_str);
    json_data.pop().unwrap()
}

// 更新
pub fn update(message: &Message) {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let mut json_data: Vec<Message> = serde_json::from_str(&file).unwrap();
    if let Some(index) = json_data.iter().position(|item| item.id == message.id) {
        json_data[index] = message.clone();
        let json_str = serde_json::to_string(&json_data).unwrap();
        let _ = fs::write(DATA_FILENAME, json_str);
    }
}
