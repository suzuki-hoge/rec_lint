# テストルールの変名と新規追加

## 概要

### Part 1: 既存ルールの名前変更（Rust 実装変更なし）

| 変更前 | 変更後 |
|--------|--------|
| `require_japanese_phpunit_test` | `require_japanese_phpunit_test_name` |
| `require_japanese_kotest_test` | `require_japanese_kotest_test_name` |
| `require_japanese_rust_test` | `require_japanese_rust_test_name` |

### Part 2: 新規ルール追加

| ルール | 説明 |
|--------|------|
| `require_phpunit_test` | PHPUnit テストファイルの存在検証 |
| `require_kotest_test` | Kotest テストファイルの存在検証 |
| `require_rust_test` | Rust テストの存在検証 |

---

## Part 1: 名前変更の変更対象ファイル

### 1.1 JSON Schema
**ファイル:** `schema/rec_lint.schema.json`

変更箇所:
- Line ~302: `"const": "require_japanese_phpunit_test"` → `"require_japanese_phpunit_test_name"`
- Line ~318: `"const": "require_japanese_kotest_test"` → `"require_japanese_kotest_test_name"`
- Line ~340: `"const": "require_japanese_rust_test"` → `"require_japanese_rust_test_name"`
- 各ルールの title と description も更新

### 1.2 Rust ルール定義
**ファイル:** `src/rule/mod.rs`

変更箇所 (文字列のみ):
- Line ~302: `"require_japanese_phpunit_test"` → `"require_japanese_phpunit_test_name"`
- Line ~305: `"require_japanese_kotest_test"` → `"require_japanese_kotest_test_name"`
- Line ~308: `"require_japanese_rust_test"` → `"require_japanese_rust_test_name"`

### 1.3 ドキュメント
**ファイル:** `docs/rec_lint.schema.md`

自動生成のため編集しない

### 1.4 README.md
- ルール名の更新（存在すれば）

---

## Part 2: 新規ルール追加の変更対象ファイル

### 2.1 設定構造

#### PHP/Kotlin 用設定
```yaml
phpunit_test:
  test_directory: "tests"        # テストディレクトリ
  require: file_exists           # file_exists | all_public
  suffix: Test
```

#### Rust 用設定
```yaml
rust_test:
  unit:
    require: exists              # exists | all_public
  integration:
    test_directory: "tests"      # 統合テストディレクトリ
    require: exists              # exists | all_public
  suffix: _test
```

unit と integration はそれぞれ任意

---

### 2.2 変更ファイル一覧

| ファイル | 変更内容 |
|---------|---------|
| `schema/rec_lint.schema.json` | 3つの新ルール定義 + phpunitTestConfig, kotestTestConfig, rustTestConfig |
| `src/rule/parser.rs` | RawPhpUnitTestConfig, RawKotestTestConfig, RawRustTestConfig 構造体 |
| `src/rule/mod.rs` | Rule enum に新しい variant 追加、convert_rule に対応追加 |
| `src/validate/test/mod.rs` | PhpUnitTestConfig, KotestTestConfig, RustTestConfig 型定義、TestExistenceViolation 型 |
| `src/validate/test/phpunit.rs` | validate_existence() 関数追加 |
| `src/validate/test/kotest.rs` | validate_existence() 関数追加 |
| `src/validate/test/rust.rs` | validate_existence() 関数追加 |
| `src/commands/validate.rs` | 新ルールの呼び出し追加 |

---

### 2.3 JSON Schema 追加内容

#### phpunitTestConfig
```json
{
  "phpunitTestConfig": {
    "title": "PhpUnitTestConfig",
    "description": "PHPUnit テスト存在検証設定",
    "type": "object",
    "required": ["test_directory", "require"],
    "properties": {
      "test_directory": {
        "description": "テストディレクトリのパス",
        "type": "string"
      },
      "require": {
        "description": "検証レベル",
        "$ref": "#/definitions/testRequireLevel"
      }
    }
  }
}
```

#### kotestTestConfig
```json
{
  "kotestTestConfig": {
    "title": "KotestTestConfig",
    "description": "Kotest テスト存在検証設定",
    "type": "object",
    "required": ["test_directory", "require"],
    "properties": {
      "test_directory": { "type": "string" },
      "require": { "$ref": "#/definitions/testRequireLevel" }
    }
  }
}
```

#### rustTestConfig
```json
{
  "rustTestConfig": {
    "title": "RustTestConfig",
    "description": "Rust テスト存在検証設定",
    "type": "object",
    "properties": {
      "unit": {
        "type": "object",
        "properties": {
          "require": { "$ref": "#/definitions/testRequireLevelRust" }
        }
      },
      "integration": {
        "type": "object",
        "properties": {
          "test_directory": { "type": "string" },
          "require": { "$ref": "#/definitions/testRequireLevelRust" }
        }
      }
    }
  }
}
```

#### testRequireLevel (PHP/Kotlin 用)
```json
{
  "testRequireLevel": {
    "type": "string",
    "oneOf": [
      { "const": "file_exists", "description": "テストファイルが存在すること" },
      { "const": "all_public", "description": "全 public メソッドがテストで呼ばれること" }
    ]
  }
}
```

#### testRequireLevelRust
```json
{
  "testRequireLevelRust": {
    "type": "string",
    "oneOf": [
      { "const": "exists", "description": "テストが存在すること" },
      { "const": "all_public", "description": "全 pub 関数がテストで呼ばれること" }
    ]
  }
}
```

---

### 2.4 Rust 型定義

#### parser.rs に追加
```rust
#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawPhpUnitTestConfig {
    pub test_directory: Option<String>,
    pub require: Option<TestRequireLevel>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawKotestTestConfig {
    pub test_directory: Option<String>,
    pub require: Option<TestRequireLevel>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawRustTestConfig {
    pub unit: Option<RawRustUnitTestConfig>,
    pub integration: Option<RawRustIntegrationTestConfig>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawRustUnitTestConfig {
    pub require: Option<TestRequireLevelRust>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawRustIntegrationTestConfig {
    pub test_directory: Option<String>,
    pub require: Option<TestRequireLevelRust>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TestRequireLevel {
    FileExists,
    AllPublic,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TestRequireLevelRust {
    Exists,
    AllPublic,
}
```

#### mod.rs に Rule variant 追加
```rust
pub enum Rule {
    // 既存...
    PhpUnitTestExistence(TestExistenceRule<PhpUnitTestConfig>),
    KotestTestExistence(TestExistenceRule<KotestTestConfig>),
    RustTestExistence(TestExistenceRule<RustTestConfig>),
}
```

---

### 2.5 バリデーションロジック

#### テストファイルマッピング (PHP/Kotlin)

**2つの方式で検索し、両方が一致すれば「存在する」と判定:**

1. **namespace/package ベース:**
   - PHP: `namespace App\Service;` → `{test_directory}/App/Service/UserServiceTest.php`
   - Kotlin: `package com.example` → `{test_directory}/com/example/UserServiceTest.kt`

2. **ファイルパスベース:**
   - PHP: `src/App/Service/UserService.php` → `{test_directory}/App/Service/UserServiceTest.php`
   - Kotlin: `src/main/kotlin/com/example/UserService.kt` → `{test_directory}/com/example/UserServiceTest.kt`

**マッピングロジック:**
- suffix を拡張子の直前につける
- 実装ファイル名 `Foo.php` → テスト名 `Foo{suffix}.php`

#### all_public 検証ロジック

1. 実装ファイルから public メソッド名を抽出
2. 対応するテストファイルの内容を読み取り
3. 各 public メソッド名がテストファイル内で呼び出されているか確認
4. 呼び出されていないメソッドを violation として報告

#### Rust unit テスト検証

- 同一ファイル内に `#[test]` 属性付き関数があるか確認
- `all_public` の場合: pub fn がテスト内で呼び出されているか確認

#### Rust integration テスト検証

- `tests/` ディレクトリに対応するファイルがあるか確認
- `all_public` の場合: pub fn が統合テスト内で呼び出されているか確認

---

### 2.6 テストコード

#### 単体テスト追加

| ファイル | テスト内容 |
|---------|-----------|
| `src/rule/parser.rs` | 新しい Config 構造体のパーステスト |
| `src/validate/test/phpunit.rs` | PHP テスト存在検証テスト |
| `src/validate/test/kotest.rs` | Kotlin テスト存在検証テスト |
| `src/validate/test/rust.rs` | Rust テスト存在検証テスト |

#### 統合テスト追加

| ファイル | テスト内容 |
|---------|-----------|
| `tests/validate.rs` | 新ルールの E2E テスト |
| `tests/dummy_project/` | テスト用サンプルファイル追加 |

---

## 実装順序

### Phase 1: 名前変更（影響範囲が小さい）
1. `schema/rec_lint.schema.json` の type 定数変更
2. `src/rule/mod.rs` の文字列変更
3. `docs/rec_lint.schema.md` の更新

### Phase 2: 新ルールの型定義
1. `src/rule/parser.rs` に Config 構造体追加
2. `src/validate/test/mod.rs` に型定義追加
3. `src/rule/mod.rs` に Rule variant 追加

### Phase 3: JSON Schema 追加
1. Config 定義追加
2. ルール定義追加

### Phase 4: バリデーション実装
1. `src/validate/test/phpunit.rs` に存在検証追加
2. `src/validate/test/kotest.rs` に存在検証追加
3. `src/validate/test/rust.rs` に存在検証追加
4. `src/commands/validate.rs` に呼び出し追加

### Phase 5: テスト追加
1. 単体テスト
2. 統合テスト
3. ダミープロジェクト更新
