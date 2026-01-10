# 共通定義

[← トップに戻る](../rec_lint.schema.md)

## RuleBase

ルールの共通フィールド

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| label | string | - | show で表示するラベル |
| message | string | - | validation で違反しているときに表示するメッセージ |
| match | [matchItem](#matchitem)[] | - | show と validation で対象とするファイルの条件<br>複数指定時は and で結合 |

## MatchItem

ファイルマッチ条件の定義

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| pattern | [matchPattern](#matchpattern) | o | マッチパターンの種類 |
| keywords | string[] | o | マッチ対象のキーワード |
| cond | [matchCond](#matchcond) | - | 省略時は and |

## MatchPattern

マッチパターンの種類

| 値 | 説明 |
|----|------|
| `file_starts_with` | ファイル名が指定文字列で始まる |
| `file_ends_with` | ファイル名が指定文字列で終わる |
| `path_contains` | パスに指定文字列が含まれる |
| `file_not_starts_with` | ファイル名が指定文字列で始まらない |
| `file_not_ends_with` | ファイル名が指定文字列で終わらない |
| `path_not_contains` | パスに指定文字列が含まれない |

## MatchCond

keywords の結合条件

| 値 | 説明 |
|----|------|
| `and` | すべての keyword に一致 |
| `or` | いずれかの keyword に一致 |

## Visibility

Doc コメントを強制する対象の可視性

| 値 | 説明 |
|----|------|
| `public` | その言語における public のコードのみ検証 |
| `all` | すべての可視性を検証 |

## TestRequireLevel

テスト存在検証レベル

| 値 | 説明 |
|----|------|
| `exists` | テストが存在すること |
| `all_public` | 全 public メソッド/関数がテストで呼ばれること |

## ExternalFileTestConfig

外部ファイルテスト存在検証設定 (PHPUnit, Kotest)

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| test_directory | string | - | テストディレクトリのパス |
| test_file_suffix | string | - | テストファイル名のサフィックス |
| require | [testRequireLevel](#testrequirelevel) | - | 検証レベル |

## SameFileTestConfig

同一ファイルテスト存在検証設定 (Rust unit test)

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| require | [testRequireLevel](#testrequirelevel) | - | 検証レベル |

