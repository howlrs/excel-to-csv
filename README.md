# generate readme
ディレクトリパスを渡すとディレクトリ内以下ファイルを走査し、リスト化、GEMINI AIにドキュメント化を依頼し、結果をディレクトリ内に保存します。

ドキュメント生成プロンプトは、当アプリ`/public/prompt.txt`に保存しています。ご利用に合わせ適宜修正・変更してください。

## サポートファイル
- txt
- 各プログラムファイル
- json, yaml, tomlなど設定ファイル
- docx
  
## 除外
- 画像及び動画
- EXCEL
- PDF

## Env vars
- GOOGLE_GEMINI_API_KEY: Google gemini AIにアクセスするための鍵（必須
- GOOGLE_GEMINI_MODEL: Google gemini AIのモデルを設定する（デフォルト: gemini-2.0-flash-exp