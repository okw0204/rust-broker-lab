# 作業管理：GitHub Issues

仕様と作業チケットは `okw0204/rust-broker-lab` の [GitHub Issues](https://github.com/okw0204/rust-broker-lab/issues) で管理する。

## 操作の基本

GitHub の操作には `gh` CLI を使う。以下のコマンドはこのリポジトリ内で実行する。セッション内で最初に Issue を作成する前に、`gh repo view --json nameWithOwner` で接続先を確認する。別のリポジトリを指していたら、書き込む前に本人へ確認する。

- 作成：`gh issue create --title "..." --body-file -`。複数行の本文はヒアドキュメントで渡す。
- 取得：`gh issue view <number> --json number,title,body,labels,comments`
- 一覧：`gh issue list --state open --json number,title,body,labels,comments`。必要に応じて `--label`・`--state` で絞る。
- コメント：`gh issue comment <number> --body "..."`
- ラベル追加・削除：`gh issue edit <number> --add-label "..."` / `--remove-label "..."`
- 完了：理由を `gh issue comment` で記録してから `gh issue close` で閉じる。すでに閉じた Issue にも、説明は独立したコメントとして残す。

## 親子関係と依存関係

- 親子関係は GitHub の sub-issue を使う。利用できない場合は、親の本文に子のタスクリストを置き、子の本文冒頭に `Part of #<parent>` を記す。
- 作業の依存関係は GitHub のネイティブな Issue dependencies を正本にする。利用できない場合は、本文冒頭の `Blocked by: #<number>, #<number>` で記録する。
- すべての依存先が閉じたチケットを着手可能とする。ネイティブ依存関係を読む場合も、閉じた依存先を未解決として数えない。

## スキルからの指示の読み替え

- 「作業管理先へ公開する」：このリポジトリに GitHub Issue を作成する。
- 「対象チケットを取得する」：指定された Issue の本文・ラベル・コメントを読む。
- PRs as a request surface: no

## 大きな調査の作業管理

`wayfinder` を使う場合、全体の見取り図を一つの Issue に置き、`wayfinder:map` ラベルを付ける。各判断は子 Issue とし、`wayfinder:research`・`wayfinder:prototype`・`wayfinder:grilling`・`wayfinder:task` のいずれかで種類を示す。これらのラベルは必要になった時点で用意する。

着手時に自分を担当者へ割り当てる。次の作業は、見取り図の順で、未完了・依存先がすべて完了・担当者未設定の子 Issue から選ぶ。解決したら回答をコメントし、Issue を閉じ、見取り図へ判断の要約とリンクを追記する。

## 学習用リポジトリでの扱い

チケットが着手可能でも、学習者が解く課題を AI の自動実装対象にはしない。AI による準備作業と学習課題を区別し、`AGENTS.md` の学習支援ルールに従う。
