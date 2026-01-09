#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

fn sort_dir() -> std::path::PathBuf {
    common::test_project_path("sort")
}

#[test]
fn sort_fileはファイルパス順でソートされる() {
    let dir = sort_dir();
    let result = rec_lint::commands::validate::run(&[dir], SortMode::File).unwrap();

    // --sort file: ファイル名 → 行番号 → ルール名 の順でソート
    // 3種のルール (forbidden_texts, forbidden_patterns, require_japanese_comment) が混在
    common::assert_output(
        &result,
        r#"
            a_first.rs:1:4: TODO禁止
            a_first.rs:3:4: FIXMEパターン禁止
            b_second.kt:1:1: コメントを日本語にしてください [ found: english comment line 1 ]
            b_second.kt:3:1: コメントを日本語にしてください [ found: FIXME: line 3 ]
            b_second.kt:3:8: FIXMEパターン禁止
            c_third.java:1:1: コメントを日本語にしてください [ found: TODO: line 1 ]
            c_third.java:1:4: TODO禁止
            c_third.java:3:1: コメントを日本語にしてください [ found: another english line 3 ]
        "#,
    );
}

#[test]
fn sort_ruleはルール名順でソートされる() {
    let dir = sort_dir();
    let result = rec_lint::commands::validate::run(&[dir], SortMode::Rule).unwrap();

    // --sort rule: ルール名 → ファイル名 → 行番号 の順でソート
    // 3種のルール (forbidden_texts, forbidden_patterns, require_japanese_comment) が混在
    common::assert_output(
        &result,
        r#"
            FIXMEパターン禁止: a_first.rs:3:4
            FIXMEパターン禁止: b_second.kt:3:8
            TODO禁止: a_first.rs:1:4
            TODO禁止: c_third.java:1:4
            コメントを日本語にしてください: b_second.kt:1:1 [ found: english comment line 1 ]
            コメントを日本語にしてください: b_second.kt:3:1 [ found: FIXME: line 3 ]
            コメントを日本語にしてください: c_third.java:1:1 [ found: TODO: line 1 ]
            コメントを日本語にしてください: c_third.java:3:1 [ found: another english line 3 ]
        "#,
    );
}
