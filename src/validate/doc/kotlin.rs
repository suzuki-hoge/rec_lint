use super::{DocKind, DocViolation, KotlinDocConfig};
use crate::rule::parser::Visibility;

/// Validate Kotlin file for missing KDoc
pub fn validate(content: &str, config: &KotlinDocConfig) -> Vec<DocViolation> {
    let mut violations = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Skip empty lines and comments
        if line.is_empty() || is_comment_line(line) {
            i += 1;
            continue;
        }

        // Check for block comment (skip non-kdoc comments)
        if line.starts_with("/*") && !line.starts_with("/**") {
            i = skip_block_comment(&lines, i);
            continue;
        }

        // Check if there's a KDoc before this line
        let has_kdoc = check_kdoc_before(&lines, i);

        // Check each element type independently (order matters for specificity)
        if let Some(v) = check_enum_class(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_sealed_class(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_sealed_interface(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_data_class(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_value_class(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_annotation_class(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_class(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_interface(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_object(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_typealias(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_function(line, i + 1, has_kdoc, config) {
            violations.push(v);
        }

        i += 1;
    }

    violations
}

fn skip_block_comment(lines: &[&str], start: usize) -> usize {
    let mut i = start;
    while i < lines.len() {
        if lines[i].contains("*/") {
            return i + 1;
        }
        i += 1;
    }
    lines.len()
}

fn check_kdoc_before(lines: &[&str], current: usize) -> bool {
    if current == 0 {
        return false;
    }

    let mut i = current - 1;

    // Skip annotations
    while i > 0 {
        let line = lines[i].trim();
        if line.starts_with('@') {
            i -= 1;
            continue;
        }
        break;
    }

    let line = lines[i].trim();

    // Check for end of KDoc
    if line.ends_with("*/") {
        if line.starts_with("/**") {
            return true;
        }
        while i > 0 {
            i -= 1;
            let prev = lines[i].trim();
            if prev.starts_with("/**") {
                return true;
            }
            if prev.starts_with("/*") && !prev.starts_with("/**") {
                return false;
            }
        }
    }

    false
}

fn is_comment_line(line: &str) -> bool {
    line.starts_with("//") || line.starts_with("/*") || line.starts_with("*")
}

fn check_visibility(line: &str, visibility: &Visibility) -> bool {
    let is_public = !line.contains("private ") && !line.contains("internal ") && !line.contains("protected ");
    match visibility {
        Visibility::Public => is_public,
        Visibility::All => true,
    }
}

// ============================================================================
// Individual element checkers
// ============================================================================

fn check_class(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.class.as_ref()?;

    // Check for " class " but exclude specific class types
    if !line.contains(" class ") && !line.starts_with("class ") {
        return None;
    }

    // Exclude other class types
    if line.contains("enum class")
        || line.contains("sealed class")
        || line.contains("data class")
        || line.contains("value class")
        || line.contains("annotation class")
    {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "class");
    Some(DocViolation { line: line_num, kind: DocKind::Class, name })
}

fn check_interface(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.interface.as_ref()?;

    if !line.contains(" interface ") && !line.starts_with("interface ") {
        return None;
    }

    // Exclude sealed interface
    if line.contains("sealed interface") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "interface");
    Some(DocViolation { line: line_num, kind: DocKind::Interface, name })
}

fn check_object(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.object.as_ref()?;

    if !line.contains(" object ") && !line.starts_with("object ") {
        return None;
    }

    // Exclude companion object
    if line.contains("companion object") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "object");
    Some(DocViolation { line: line_num, kind: DocKind::Object, name })
}

fn check_enum_class(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.enum_class.as_ref()?;

    if !line.contains("enum class ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "enum class");
    Some(DocViolation { line: line_num, kind: DocKind::EnumClass, name })
}

fn check_sealed_class(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.sealed_class.as_ref()?;

    if !line.contains("sealed class ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "sealed class");
    Some(DocViolation { line: line_num, kind: DocKind::SealedClass, name })
}

fn check_sealed_interface(
    line: &str,
    line_num: usize,
    has_kdoc: bool,
    config: &KotlinDocConfig,
) -> Option<DocViolation> {
    let visibility = config.sealed_interface.as_ref()?;

    if !line.contains("sealed interface ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "sealed interface");
    Some(DocViolation { line: line_num, kind: DocKind::SealedInterface, name })
}

fn check_data_class(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.data_class.as_ref()?;

    if !line.contains("data class ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "data class");
    Some(DocViolation { line: line_num, kind: DocKind::DataClass, name })
}

fn check_value_class(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.value_class.as_ref()?;

    if !line.contains("value class ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "value class");
    Some(DocViolation { line: line_num, kind: DocKind::ValueClass, name })
}

fn check_annotation_class(
    line: &str,
    line_num: usize,
    has_kdoc: bool,
    config: &KotlinDocConfig,
) -> Option<DocViolation> {
    let visibility = config.annotation_class.as_ref()?;

    if !line.contains("annotation class ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "annotation class");
    Some(DocViolation { line: line_num, kind: DocKind::AnnotationClass, name })
}

fn check_typealias(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.typealias.as_ref()?;

    if !line.contains("typealias ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "typealias");
    Some(DocViolation { line: line_num, kind: DocKind::Typealias, name })
}

fn check_function(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.function.as_ref()?;

    if !line.contains("fun ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    // Extract function name
    let fun_pos = line.find("fun ")?;
    let after_fun = &line[fun_pos + 4..];

    // Skip generic type parameter if present: fun <T> name
    let name_start = if after_fun.starts_with('<') { after_fun.find('>')? + 1 } else { 0 };

    let name_part = after_fun[name_start..].trim();
    let name = name_part.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect::<String>();

    if name.is_empty() {
        return None;
    }

    Some(DocViolation { line: line_num, kind: DocKind::Function, name })
}

fn extract_class_name(line: &str, keyword: &str) -> String {
    let parts: Vec<&str> = line.split(keyword).collect();
    if parts.len() < 2 {
        return String::new();
    }

    let after = parts[1].trim();
    after.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect()
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    // =========================================================================
    // クラス
    // =========================================================================

    #[test]
    fn KDocがないクラスは違反になる() {
        let content = "class MyClass {}";
        let config = KotlinDocConfig { class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Class);
        assert_eq!(violations[0].name, "MyClass");
    }

    #[test]
    fn KDocがあるクラスは違反にならない() {
        let content = "/** Doc */\nclass MyClass {}";
        let config = KotlinDocConfig { class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn public指定時はprivateクラスをスキップする() {
        let content = "private class MyClass {}";
        let config = KotlinDocConfig { class: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn クラス検査が無効の場合は違反にならない() {
        let content = "class MyClass {}";
        let config = KotlinDocConfig { class: None, ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // インターフェース
    // =========================================================================

    #[test]
    fn KDocがないインターフェースは違反になる() {
        let content = "interface MyInterface {}";
        let config = KotlinDocConfig { interface: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Interface);
        assert_eq!(violations[0].name, "MyInterface");
    }

    #[test]
    fn KDocがあるインターフェースは違反にならない() {
        let content = "/** Doc */\ninterface MyInterface {}";
        let config = KotlinDocConfig { interface: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn public指定時はprivateインターフェースをスキップする() {
        let content = "private interface MyInterface {}";
        let config = KotlinDocConfig { interface: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // オブジェクト
    // =========================================================================

    #[test]
    fn KDocがないobjectは違反になる() {
        let content = "object Singleton {}";
        let config = KotlinDocConfig { object: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Object);
        assert_eq!(violations[0].name, "Singleton");
    }

    #[test]
    fn KDocがあるobjectは違反にならない() {
        let content = "/** Doc */\nobject Singleton {}";
        let config = KotlinDocConfig { object: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // enum class
    // =========================================================================

    #[test]
    fn KDocがないenum_classは違反になる() {
        let content = "enum class Status { A, B }";
        let config = KotlinDocConfig { enum_class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::EnumClass);
        assert_eq!(violations[0].name, "Status");
    }

    #[test]
    fn KDocがあるenum_classは違反にならない() {
        let content = "/** Doc */\nenum class Status { A }";
        let config = KotlinDocConfig { enum_class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // sealed class
    // =========================================================================

    #[test]
    fn KDocがないsealed_classは違反になる() {
        let content = "sealed class Result";
        let config = KotlinDocConfig { sealed_class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::SealedClass);
        assert_eq!(violations[0].name, "Result");
    }

    #[test]
    fn KDocがあるsealed_classは違反にならない() {
        let content = "/** Doc */\nsealed class Result";
        let config = KotlinDocConfig { sealed_class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // sealed interface
    // =========================================================================

    #[test]
    fn KDocがないsealed_interfaceは違反になる() {
        let content = "sealed interface State";
        let config = KotlinDocConfig { sealed_interface: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::SealedInterface);
        assert_eq!(violations[0].name, "State");
    }

    #[test]
    fn KDocがあるsealed_interfaceは違反にならない() {
        let content = "/** Doc */\nsealed interface State";
        let config = KotlinDocConfig { sealed_interface: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // data class
    // =========================================================================

    #[test]
    fn KDocがないdata_classは違反になる() {
        let content = "data class User(val name: String)";
        let config = KotlinDocConfig { data_class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::DataClass);
        assert_eq!(violations[0].name, "User");
    }

    #[test]
    fn KDocがあるdata_classは違反にならない() {
        let content = "/** Doc */\ndata class User(val name: String)";
        let config = KotlinDocConfig { data_class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // value class
    // =========================================================================

    #[test]
    fn KDocがないvalue_classは違反になる() {
        let content = "value class Password(val value: String)";
        let config = KotlinDocConfig { value_class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::ValueClass);
        assert_eq!(violations[0].name, "Password");
    }

    #[test]
    fn KDocがあるvalue_classは違反にならない() {
        let content = "/** Doc */\nvalue class Password(val value: String)";
        let config = KotlinDocConfig { value_class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // annotation class
    // =========================================================================

    #[test]
    fn KDocがないannotation_classは違反になる() {
        let content = "annotation class MyAnnotation";
        let config = KotlinDocConfig { annotation_class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::AnnotationClass);
        assert_eq!(violations[0].name, "MyAnnotation");
    }

    #[test]
    fn KDocがあるannotation_classは違反にならない() {
        let content = "/** Doc */\nannotation class MyAnnotation";
        let config = KotlinDocConfig { annotation_class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // typealias
    // =========================================================================

    #[test]
    fn KDocがないtypealiasは違反になる() {
        let content = "typealias StringList = List<String>";
        let config = KotlinDocConfig { typealias: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Typealias);
        assert_eq!(violations[0].name, "StringList");
    }

    #[test]
    fn KDocがあるtypealiasは違反にならない() {
        let content = "/** Doc */\ntypealias StringList = List<String>";
        let config = KotlinDocConfig { typealias: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn public指定時はprivate_typealiasをスキップする() {
        let content = "private typealias StringList = List<String>";
        let config = KotlinDocConfig { typealias: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // 関数
    // =========================================================================

    #[test]
    fn KDocがない関数は違反になる() {
        let content = "fun doSomething() {}";
        let config = KotlinDocConfig { function: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Function);
        assert_eq!(violations[0].name, "doSomething");
    }

    #[test]
    fn KDocがある関数は違反にならない() {
        let content = "/** Doc */\nfun doSomething() {}";
        let config = KotlinDocConfig { function: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn public指定時はprivate関数をスキップする() {
        let content = "private fun doSomething() {}";
        let config = KotlinDocConfig { function: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn ジェネリック関数の名前を正しく抽出できる() {
        let content = "fun <T> process(item: T) {}";
        let config = KotlinDocConfig { function: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].name, "process");
    }

    // =========================================================================
    // エッジケース
    // =========================================================================

    #[test]
    fn アノテーション付きクラスのKDocを認識する() {
        let content = "/** Doc */\n@Serializable\nclass MyClass {}";
        let config = KotlinDocConfig { class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn 複数行KDocを認識する() {
        let content = "/**\n * Multi-line\n */\nclass MyClass {}";
        let config = KotlinDocConfig { class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn 通常のブロックコメントはKDocとして認識されない() {
        let content = "/* Not KDoc */\nclass MyClass {}";
        let config = KotlinDocConfig { class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn 複数行の通常コメントはKDocとして認識されない() {
        let content = "/*\n * Not KDoc\n */\nclass MyClass {}";
        let config = KotlinDocConfig { class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn 空の設定では違反が検出されない() {
        let content = "class MyClass {}\nfun foo() {}";
        let config = KotlinDocConfig::default();
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn public指定時はinternalクラスをスキップする() {
        let content = "internal class MyClass {}";
        let config = KotlinDocConfig { class: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }
}
