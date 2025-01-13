use ignore::WalkBuilder;
use std::fs;
use std::path::Path;

pub fn files(dir: &Path) -> Vec<String> {
    let mut str = Vec::new();
    let walker = WalkBuilder::new(dir)
        .follow_links(true) // シンボリックリンクも追跡
        .standard_filters(true) // デフォルトフィルター (デフォルトで .gitignore 等を適用)
        .build();

    for entry in walker
    // エラーハンドリング
    {
        // ここでファイルのフィルタリングを行い、処理するファイルを選定する
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if is_document_or_source_code(path) {
                    println!("Found file: {}", path.display());
                    // ドキュメントかどうか判定し、処理対象にするか決める
                    // read file to string
                    let content = {
                        match path.extension() {
                            Some(ext) if ext == "docx" => {
                                match crate::documents::docx::read(path) {
                                    Ok(content) => content,
                                    Err(err) => {
                                        eprintln!("Error: {}", err);
                                        continue;
                                    }
                                }
                            }
                            _ => match fs::read_to_string(path) {
                                Ok(content) => content,
                                Err(err) => {
                                    eprintln!("Error: {}", err);
                                    continue;
                                }
                            },
                        }
                    };
                    let filepath = path.to_str().unwrap();
                    str.push(format!(
                        "{}\n
                    ```{}
                    {}
                    ```
                    \n\n------------\n\n",
                        filepath,
                        path.extension().unwrap().to_str().unwrap(),
                        content
                    ));
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
    str
}

/// ignore漏れを防ぐため、ファイルの拡張子で判定
fn is_document_or_source_code(file_path: &Path) -> bool {
    matches!(
        file_path.extension().and_then(|s| s.to_str()),
        Some("md")
            | Some("txt")
            | Some("rs")
            | Some("py")
            | Some("js")
            | Some("ts")
            | Some("jsx")
            | Some("tsx")
            | Some("html")
            | Some("css")
            | Some("c")
            | Some("cpp")
            | Some("java")
            | Some("php")
            | Some("go")
            | Some("rb")
            | Some("sh")
            | Some("bat")
            | Some("ps1")
            | Some("psm1")
            | Some("json")
            | Some("yaml")
            | Some("yml")
            | Some("toml")
            | Some("xml")
            | Some("csv")
            | Some("tsv")
            | Some("sql")
            | Some("sqlite")
            | Some("db")
            | Some("log")
            | Some("ini")
            | Some("cfg")
            | Some("conf")
            | Some("properties")
            | Some("env")
            | Some("envrc")
            | Some("env.sample")
            | Some("env.example")
            | Some("env.local")
            | Some("env.development")
            | Some("env.test")
            | Some("env.staging")
            | Some("env.production")
            | Some("env.ci")
            | Some("env.dev")
            | Some("env.prod")
            | Some("env.stage")
            | Some("env.stg")
            // [support]ドキュメント
            | Some("docx")
    )
}
