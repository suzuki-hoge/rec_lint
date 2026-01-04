# rec_lint

任意のディレクトリに設定ファイルを作成できるリンタ

親ディレクトリで定義したルールは子ディレクトリに継承される

## インストール

```
$ cargo install --path .
```

ルート設定ファイルを作成（プロジェクトルートで実行）

```
$ rec_lint init
```

ルールファイルを作成

```
$ rec_lint add
```

または

```
$ rec_lint add sub-dir
```

## 設定ファイル

- [.rec_lint_config.yaml](docs/rec_lint_config.schema.md) - ルートディレクトリに配置（プロジェクト全体の設定）
- [.rec_lint.yaml](docs/rec_lint.schema.md) - 各ディレクトリに配置（ルール定義）

## 設定例

`.rec_lint_config.yaml`

```yaml
{{exec: cat docs/sample/.rec_lint_config.yaml}}
```

`src/main/java/.rec_lint.yaml`

```yaml
{{exec: cat docs/sample/src/main/java/.rec_lint.yaml}}
```

`src/main/java/db/.rec_lint.yaml`

```yaml
{{exec: cat docs/sample/src/main/java/db/.rec_lint.yaml}}
```

## サブコマンド

### show

指定ディレクトリで有効なルールを表示する

```
$ rec_lint show src/main/java
{{exec: cargo run --quiet -- show docs/sample/src/main/java}}
```

下位ディレクトリは上位ディレクトリの設定を継承する

```
$ rec_lint show src/main/java/db
{{exec: cargo run --quiet -- show docs/sample/src/main/java/db}}
```

### validate

ファイルをルールに基づいて検証する

```
$ rec_lint validate src/main/java/db/UserQuery.java
{{exec: cargo run --quiet -- validate docs/sample/src/main/java/db/UserQuery.java}}
```

ディレクトリを指定した場合はそれ以下のすべてのファイルを検証する

```
$ rec_lint validate src/main/java/db
{{exec: cargo run --quiet -- validate docs/sample/src/main/java/db}}
```

オプション:

- `-s, --sort <rule|file>` - 出力のソート順（デフォルト: rule）

### guideline

指定ディレクトリのガイドラインチェックリストを表示する

```
$ rec_lint guideline src/main/java/db
{{exec: cargo run --quiet -- guideline docs/sample/src/main/java/db}}
```

## 活用ノウハウ

### 開発フローの中でフックして自動的にコードを改善する

コミットフローや AI Agent への指示に `rec_lint validate <PATH>` を入れておけば、気付かぬうちに意図しない設計のままコードが量産されるのを回避できる

`rec_lint show <DIR>` は人間 / AI Agent を問わず実装の指針として参考にできる

`rec_lint guideline <DIR>` は人間が実装の指針にできるほか、自動検証するのが難しい内容を AI Agent にセルフレビューさせるなどの応用が可能

すべての設定において `message` を自由に設定できるため、メッセージ自体を AI Agent への次のプロンプトにすることで自動的な改善サイクルを構築できる

### validate --sort の活用

`rec_lint validate` は `--sort <rule|file>` でエラーメッセージの出力順を指定できる

`--sort rule` は特定ルールごとに修正したい場合に向いている

```
$ rec_lint validate --sort rule src/main/java
{{exec: cargo run --quiet -- validate -s rule docs/sample/src/main/java}}
```

`--sort file` は特定ファイルを修正したい場合に向いている

```
$ rec_lint validate --sort file src/main/java
{{exec: cargo run --quiet -- validate -s file docs/sample/src/main/java}}
```

### Yaml Language Server の利用

設定ファイルの冒頭にスキーマを指定すると YAML の読み書き時にスキーマ情報と説明が得られる

`.rec_lint_config.yaml`:

```yaml
# yaml-language-server: $schema=https://raw.githubusercontent.com/suzuki-hoge/rec_lint/refs/tags/v0.0.3/schema/rec_lint_config.schema.json
```

`.rec_lint.yaml`:

```yaml
# yaml-language-server: $schema=https://raw.githubusercontent.com/suzuki-hoge/rec_lint/refs/tags/v0.0.3/schema/rec_lint.schema.json
```

- Idea 系エディタ: 標準サポート
- VSCode: [YAML Language Support by Red Hat](https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml) を追加

### type: custom の利用

プリセットの `type: forbidden_texts` などでカバーできないケースをバリデーションしたい場合は `type: custom` で rec_lint 処理フロー中から任意のコマンドを実行できる

詳細は [docs/rec_lint.schema.md](docs/rec_lint.schema.md) を参照
