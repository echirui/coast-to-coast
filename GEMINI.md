# GEMINI.md

## プロジェクト概要

ブロックを自陣から敵陣まで繋げるゲーム
囲碁やブロックスに似ている

## 技術スタック

- **開発言語** Rust
- **フロントエンド**: Bevy


## 開発のルール
.gitignoreは、追加のみ行ってください。個人情報やパスワードを記載したファイルがアップロードされてしまうので厳守してください。
テスト駆動開発を実施します。red-green-refactorを忠実に実行してください。
issue をタスクに分割してtodoリストを作成して少しずつ実装してください。
ブランチ戦略はGithub-flowを使ってください。feature/<任意のタイトル>というブランチを作成して
開発試験が終わったら、PRを送ってマージします。


### コミット
`commitして`という指示があった場合、変更をステージングしてからコミットします。
コミットメッセージは英語で記述してください。

### コミットメッセージの作成
複数行のコミットメッセージを作成する場合、以下の手順でコミットしてください。
1. コミットメッセージを一時ファイルに書き込みます。
   `write_file(content="<コミットメッセージ>", file_path="tmp/commit_message.txt")`
2. 一時ファイルを指定してコミットします。
   `git commit -F tmp/commit_message.txt`
3. コミット後、一時ファイルを削除します。
   `rm tmp/commit_message.txt`

### Issueの確認
`issueを確認して`という指示があった場合、`gh issue view <issue番号>`または`gh issue list`を使用します。

### Issueの登録
`issueを登録して`という指示があった場合、以下の手順でissueを登録します。
issueのタイトルと本文は日本語で記述してください。
1. issueの本文を一時ファイルに書き込みます。
   `write_file(content="<issue本文>", file_path="tmp/issue_body.md")`
2. 一時ファイルを指定してissueを登録します。
   `gh issue create --title "<issueタイトル>" --body-file tmp/issue_body.md`
3. issue登録後、一時ファイルを削除します。
   `rm tmp/issue_body.md`
