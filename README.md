# PDF READER
PDFファイルから構造化したCSVデータ化、結果をディレクトリ内に保存します。

ドキュメント生成プロンプトは、当アプリ`/public/prompt.txt`に保存しています。ご利用に合わせ適宜修正・変更してください。

請求書・領収書の文字起こしを想定しています。複数ファイルに対応していますが、データがGEMINIのLIMIT TOKEN以下である必要があります。

## サポートファイル
- PDF

## 出力形式
- CSVファイルをOS固有のドキュメントディレクトリに出力します。
- データは主にCSVです。しかし、構造化が入れ子になっている場合JSON形式で出力されます。必要な形式を選択してください。
  
## Env vars
- GOOGLE_GEMINI_API_KEY: Google gemini AIにアクセスするための鍵（必須
- GOOGLE_GEMINI_MODEL: Google gemini AIのモデルを設定する（デフォルト: gemini-2.0-flash-exp