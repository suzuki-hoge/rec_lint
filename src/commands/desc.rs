use anyhow::Result;

pub fn run() -> Result<Vec<String>> {
    Ok(r#"
rec_lint は 設定ファイル ( .rec_lint.yaml ) をもとにコードチェックを行うリンタです
各ディレクトリの設定ファイルは、それより上位ディレクトリの設定ファイルを継承します

rec_lint show <DIR> は、そのディレクトリに適用されるルールとガイドラインが表示されます

rec_lint validate <DIR|FILE>... は、そのディレクトリに適用されているルールをファイルが満たしているか検証します

rec_lint guideline <DIR> は、そのディレクトリの実装ガイドラインが表示されます
"#
    .trim()
    .split("\n")
    .map(|line| line.to_string())
    .collect())
}
