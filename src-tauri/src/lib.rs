use std::path::Path;

use calamine::{open_workbook, Reader, Xlsx};

#[tauri::command]
async fn command_parse_excel(paths: Vec<String>) -> Result<String, String> {
    if paths.is_empty() {
        return Err("No file paths provided".to_string());
    }

    let first = paths.first().unwrap();

    // ファイルであるか検証
    let filepath = Path::new(&first).to_path_buf();
    if filepath.is_dir() {
        return Err("Error: Not a file".to_string());
    }

    // excelファイルであることを確認する
    if filepath.extension().unwrap() != "xlsx" {
        return Err("Error: Not an excel file".to_string());
    }

    let mut workbook: Xlsx<_> = match open_workbook(&filepath) {
        Ok(workbook) => workbook,
        Err(err) => {
            eprintln!("Error: {}", err);
            return Err("Cannot open workbook".to_string());
        }
    };

    let mut rows = Vec::new();
    if let Ok(r) = workbook.worksheet_range("Sheet1") {
        for (i, row) in r.rows().enumerate() {
            rows.push(format!(
                "{},{}",
                i,
                row.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .as_str()
            ));
        }
    }

    Ok(rows.join("\n"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![command_parse_excel])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
