const INTERNAL_CONSTANT: &str = "internal";

fn private_helper() -> i32 {
    42
}

pub(crate) fn crate_only_function() -> i32 {
    private_helper()
}

pub(super) fn super_only_function() -> i32 {
    100
}
