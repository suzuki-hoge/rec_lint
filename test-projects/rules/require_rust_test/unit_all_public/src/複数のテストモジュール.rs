pub fn create_user() {}
pub fn delete_user() {}

#[cfg(test)]
mod tests1 {
    use super::*;
    #[test]
    fn test_create_user() {}
}

#[cfg(test)]
mod tests2 {
    use super::*;
    #[test]
    fn test_delete_user() {}
}
