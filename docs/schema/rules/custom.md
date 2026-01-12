# CustomRule

[← ルール一覧に戻る](../rec_lint.schema.md#rule-types)

任意のコマンドを実行してファイルを検証する

## CustomRule

任意のコマンドを実行してファイルを検証する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| exec | string | o | ファイルに対して実行するコマンド<br>終了コード 0 の場合はエラーなし扱い<br>エラー時は実行コマンドの標準出力がエラーメッセージとして表示される<br>e.g. `ruby path/to/your/checker.rb {path}`<br>e.g. `docker run your:image {path}` |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](./common.md#matchitem)[] | - | show と validation で対象とするファイルの条件<br>複数指定時は and で結合 |

