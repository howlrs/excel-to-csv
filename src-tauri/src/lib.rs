use std::fs;
use std::io::Write;
use std::path::Path;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde_json::json;

mod documents;

#[tauri::command]
async fn command_run(paths: Vec<String>) -> Result<String, String> {
    if paths.is_empty() {
        return Err("No file paths provided".to_string());
    }

    // バインドされるpublicディレクトリのファイルを読み込みプロンプトとする
    let prompt = {
        // 引数がない場合は、ファイルを読み込みプロンプトとする
        let current_dir = std::env::current_dir().unwrap();
        println!("Current directory is {}", current_dir.display());
        let target_file = current_dir.join("public/prompt.txt");
        std::fs::read_to_string(target_file).unwrap()
    };

    let single_dir_path = paths.first().unwrap();

    // ディレクトリであるか検証
    let dir = Path::new(&single_dir_path);
    if !dir.is_dir() {
        eprintln!("Error: {} is not a directory", single_dir_path);
        return Err("Error: not a directory".to_string());
    }

    // ディレクトリ内を走査しファイルをリストする
    let files = documents::scan::files(dir);
    // content length
    let content_length = files.len();
    let content_size = files.iter().map(|s| s.len()).sum::<usize>();
    let content_size_mb = content_size as f64 / 1024.0 / 1024.0;
    println!(
        "Found {} files, content size: {:.3} MB",
        content_length, content_size_mb
    );
    // ファイルのテキスト化
    let contents_str = files
        .iter()
        .fold(String::new(), |acc, s| acc + format!("{}\n", s).as_str());

    //  モデルを取得
    let model = get_model();
    // リクエストを送信
    let body = json!({
    "contents":
        json!([{
            // roleがuserの場合はuser、それ以外はmodel as assistant
            "role": "user",
            "parts": [
                {
                    "text": format!("{}\n{}\ncontent length: {}, content size: {:.2}MB",prompt, contents_str, content_length, content_size_mb),
                },
            ],
        }])
    });

    println!("Sending to server...");

    let result = documents::request::request(model.as_str(), body).await;
    match result {
        Ok(json) => {
            let content = match documents::request::get_content(&json) {
                Ok(content) => content,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    return Err(err);
                }
            };

            // ファイルに保存
            let time_at = chrono::Local::now();
            let docname = format!("doc-{}.md", time_at.format("%Y%m%d"));
            // ターゲットディレクトリ下にファイルを作成
            let target_dir_and_filename = format!("{}/{}", dir.display(), docname);
            println!("Saving to {}", target_dir_and_filename);
            let path = Path::new(&target_dir_and_filename);
            let mut file = fs::File::create(path).expect("Failed to create file");
            file.write_all(content.as_bytes())
                .expect("Failed to write to file");
            Ok(content)
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(err)
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![command_run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_model() -> String {
    match std::env::var("GOOGLE_GEMINI_MODEL") {
        Ok(val) => val,
        Err(_) => "gemini-2.0-flash-exp".to_string(),
    }
}
