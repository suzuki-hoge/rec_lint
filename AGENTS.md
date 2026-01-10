# Repository Guidelines

## 大方針

- すべてに日本語で対応すること
- `git` の操作は `status` や `diff` や `log` のような確認のみ許可する
- 指示の先頭に `[ask]` が付く場合、ファイルを変更せず質問に答えること
- 指示の先頭に `[plan]` が付く場合、ファイルを変更せず計画を作成すること
- セッション開始時に `rec_lint desc` を実行し rec_lint について理解すること
- `test-projects` 以外の `.rec_lint.yaml` は変更してはならない

## コーディング

- ファイルを作成・変更するときは該当するディレクトリを `rec_lint show <DIR>` で確認し、rule と guideline を確認すること
- 実装完了時に `make fix` が成功すること
- 実装完了時に差分のあるディレクトリを `rec_lint guideline <DIR>` で確認し、セルフレビューして改善すること

## ドキュメントとスキーマ

- `schema/` は YAML Language Server のために公開される
- `docs/readme/` は `tools/readme-generator/` が `README.md` を生成するために使う
- `docs/schema/*.md` は `tools/schema-tools/` が `schema/` から生成する

## ルールの追加と変更

1. `schema/` を追記・変更する
2. `src/` を追記・変更する
3. `test-projects/` を追記・変更する
4. `tests/` を追記・変更する
5. 必要なら `docs/readme/README_TEMPLATE.md` を追記・変更する
6. `make doc` でドキュメントを更新する
