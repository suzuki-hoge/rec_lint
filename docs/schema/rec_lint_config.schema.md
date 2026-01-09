# .rec_lint_config.yaml ドキュメント

rec_lint のルート設定ファイル

## トップレベル

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| include_extensions | string[] | - | 検証対象とする拡張子のリスト<br>ドット付きで指定<br>未指定の場合は全ての拡張子が対象<br>これより下位の設定で include しても、これ以外は対象にならない<br>e.g. `.java`<br>e.g. `.kt`<br>e.g. `.rs` |
| exclude_dirs | string[] | - | 検証対象から除外するディレクトリ名<br>e.g. `node_modules`<br>e.g. `build` |

