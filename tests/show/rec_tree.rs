#[path = "../common/mod.rs"]
mod common;

#[test]
fn dir1指定のときdir1のルールが表示される() {
    let dir = common::test_project_path("rec_tree/dir1");
    let result = rec_lint::commands::show::run(&dir).unwrap();
    common::assert_output(
        &result,
        r#"
            [ rule ] dir1: dir1-rule
            [ guideline ] dir1: dir1-guideline
        "#,
    );
}

#[test]
fn dir2指定のときdir1のルールが表示される() {
    let dir = common::test_project_path("rec_tree/dir1/dir2");
    let result = rec_lint::commands::show::run(&dir).unwrap();
    common::assert_output(
        &result,
        r#"
            [ rule ] dir1: dir1-rule
            [ guideline ] dir1: dir1-guideline
        "#,
    );
}

#[test]
fn dir3指定のときdir1とdir3のルールが表示される() {
    let dir = common::test_project_path("rec_tree/dir1/dir2/dir3");
    let result = rec_lint::commands::show::run(&dir).unwrap();
    common::assert_output(
        &result,
        r#"
            [ rule ] dir1: dir1-rule
            [ rule ] dir1/dir2/dir3: dir3-rule
            [ guideline ] dir1: dir1-guideline
            [ guideline ] dir1/dir2/dir3: dir3-guideline
        "#,
    );
}

#[test]
fn file1指定のときdir1のルールが表示される() {
    let file = common::test_project_path("rec_tree/dir1/file1.txt");
    let result = rec_lint::commands::show::run(&file).unwrap();
    common::assert_output(
        &result,
        r#"
            [ rule ] dir1: dir1-rule
            [ guideline ] dir1: dir1-guideline
        "#,
    );
}

#[test]
fn file2指定のときdir1のルールが表示される() {
    let file = common::test_project_path("rec_tree/dir1/dir2/file1.txt");
    let result = rec_lint::commands::show::run(&file).unwrap();
    common::assert_output(
        &result,
        r#"
            [ rule ] dir1: dir1-rule
            [ guideline ] dir1: dir1-guideline
        "#,
    );
}

#[test]
fn file3指定のときdir1とdir3のルールが表示される() {
    let file = common::test_project_path("rec_tree/dir1/dir2/dir3/file1.txt");
    let result = rec_lint::commands::show::run(&file).unwrap();
    common::assert_output(
        &result,
        r#"
            [ rule ] dir1: dir1-rule
            [ rule ] dir1/dir2/dir3: dir3-rule
            [ guideline ] dir1: dir1-guideline
            [ guideline ] dir1/dir2/dir3: dir3-guideline
        "#,
    );
}
