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
# yaml-language-server: $schema=https://raw.githubusercontent.com/suzuki-hoge/rec_lint/refs/tags/v0.0.3/schema/rec_lint_config.schema.json

include_extensions:
  - .java

exclude_dirs:
  - build
```

`src/main/java/.rec_lint.yaml`

```yaml
# yaml-language-server: $schema=https://raw.githubusercontent.com/suzuki-hoge/rec_lint/refs/tags/v0.0.3/schema/rec_lint.schema.json

rule:
  - label: println の禁止
    type: forbidden_texts
    keywords: [ System.out.println ]
    message: デバッグ残りは削除し、必要な出力は Logger を使うこと

  - label: public class の JavaDoc は必須
    type: require_java_doc
    java_doc:
      class: public
    message: JavaDoc を記述すること
```

`src/main/java/db/.rec_lint.yaml`

```yaml
# yaml-language-server: $schema=https://raw.githubusercontent.com/suzuki-hoge/rec_lint/refs/tags/v0.0.3/schema/rec_lint.schema.json

rule:
  - label: http 処理の禁止
    type: forbidden_patterns
    keywords: [ "import.*http" ]
    message: DB 処理と HTTP 処理は分離し、HTTP 処理は src/main/java/controller に実装すること
    match:
      - pattern: file_ends_with
        keywords: [ Command.java, Query.java ]
        cond: or

guideline:
  - message: N + 1 問題が発生するクエリがないか確認すること
    match:
      - pattern: file_ends_with
        keywords: [ Query.java ]
```

## サブコマンド

### show

指定ディレクトリで有効なルールを表示する

```
$ rec_lint show src/main/java
[ rule ] src/main/java: println の禁止
[ rule ] src/main/java: public class の JavaDoc は必須
```

下位ディレクトリは上位ディレクトリの設定を継承する

```
$ rec_lint show src/main/java/db
[ rule ] src/main/java: println の禁止
[ rule ] src/main/java: public class の JavaDoc は必須
[ rule ] src/main/java/db: http 処理の禁止
[ guideline ] src/main/java/db: N + 1 問題が発生するクエリがないか確認すること
```

### validate

ファイルをルールに基づいて検証する

```
$ rec_lint validate src/main/java/db/UserQuery.java
JavaDoc を記述すること (class UserQuery): src/main/java/db/UserQuery.java:7:1
```

ディレクトリを指定した場合はそれ以下のすべてのファイルを検証する

```
$ rec_lint validate src/main/java/db
DB 処理と HTTP 処理は分離し、HTTP 処理は src/main/java/controller に実装すること: src/main/java/db/PlanQuery.java:4:1
DB 処理と HTTP 処理は分離し、HTTP 処理は src/main/java/controller に実装すること: src/main/java/db/UserCommand.java:4:1
JavaDoc を記述すること (class UserQuery): src/main/java/db/UserQuery.java:7:1
デバッグ残りは削除し、必要な出力は Logger を使うこと: src/main/java/db/PlanQuery.java:11:9
デバッグ残りは削除し、必要な出力は Logger を使うこと: src/main/java/db/UserCommand.java:11:9
```

オプション:

- `-s, --sort <rule|file>` - 出力のソート順（デフォルト: rule）

### guideline

指定ディレクトリのガイドラインチェックリストを表示する

```
$ rec_lint guideline src/main/java/db
[ guideline ] src/main/java/db: N + 1 問題が発生するクエリがないか確認すること
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
DB 処理と HTTP 処理は分離し、HTTP 処理は src/main/java/controller に実装すること: src/main/java/db/PlanQuery.java:4:1
DB 処理と HTTP 処理は分離し、HTTP 処理は src/main/java/controller に実装すること: src/main/java/db/UserCommand.java:4:1
JavaDoc を記述すること (class Authenticator): src/main/java/Authenticator.java:5:1
JavaDoc を記述すること (class UserQuery): src/main/java/db/UserQuery.java:7:1
デバッグ残りは削除し、必要な出力は Logger を使うこと: src/main/java/PlanService.java:10:9
デバッグ残りは削除し、必要な出力は Logger を使うこと: src/main/java/db/PlanQuery.java:11:9
デバッグ残りは削除し、必要な出力は Logger を使うこと: src/main/java/db/UserCommand.java:11:9
```

`--sort file` は特定ファイルを修正したい場合に向いている

```
$ rec_lint validate --sort file src/main/java
src/main/java/Authenticator.java:5:1: JavaDoc を記述すること (class Authenticator)
src/main/java/PlanService.java:10:9: デバッグ残りは削除し、必要な出力は Logger を使うこと
src/main/java/db/PlanQuery.java:4:1: DB 処理と HTTP 処理は分離し、HTTP 処理は src/main/java/controller に実装すること
src/main/java/db/PlanQuery.java:11:9: デバッグ残りは削除し、必要な出力は Logger を使うこと
src/main/java/db/UserCommand.java:4:1: DB 処理と HTTP 処理は分離し、HTTP 処理は src/main/java/controller に実装すること
src/main/java/db/UserCommand.java:11:9: デバッグ残りは削除し、必要な出力は Logger を使うこと
src/main/java/db/UserQuery.java:7:1: JavaDoc を記述すること (class UserQuery)
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
