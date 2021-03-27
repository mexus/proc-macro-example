use the_macro::concatenate;

const ID: &str = concatenate!("Something", "-", false, "_", 14);

/// Returns ID.
pub fn kek() -> &'static str {
    ID
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_id() {
        let expected = concatenate!("Hm!");
        assert_ne!(expected, ID);
    }
}
