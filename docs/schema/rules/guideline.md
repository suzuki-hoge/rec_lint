# GuidelineItem

[← ルール一覧に戻る](../rec_lint.schema.md#rule-types)

guideline に列挙するレビューガイドラインの定義

## GuidelineItem

guideline に列挙するレビューガイドラインの定義

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| message | string | o | guideline で表示するメッセージ |
| match | [matchItem](./common.md#matchitem)[] | - | 対象とするファイルの条件<br>複数指定時は and で結合 |

