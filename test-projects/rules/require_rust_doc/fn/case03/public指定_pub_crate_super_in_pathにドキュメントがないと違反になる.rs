pub(crate) fn missing_crate() {}

pub(super) fn missing_super() {}

pub(in crate::foo) fn missing_in_path() {}

/// documented function
pub(crate) fn documented_crate() {}

fn private_fn() {}
