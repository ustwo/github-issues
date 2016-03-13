pub fn is_repopath(value: String) -> Result<(), String> {
    if value.split("/").collect::<Vec<&str>>().len() != 2 {
        return Err(String::from("<repopath> must have the form <owner>/<repo>.  e.g. ustwo/github-issues"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_repopath() {
        assert!(is_repopath("foo/bar".to_owned()).is_ok());
    }

    #[test]
    fn invalid_repopath() {
        assert!(is_repopath("foo_bar".to_owned()).is_err());
        assert!(is_repopath("foo/bar/baz".to_owned()).is_err());
    }

}
