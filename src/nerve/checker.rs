use strsim::jaro;


/// Checks if two strings are similar given a threshold from 0 to 1 where 0
/// is different and 1 is equal.
///
/// Note that the current implementation uses the [Jaro distance](http://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance).
pub fn is_similar<'a>(a: &str, b: &'a str, threshold: f64)
    -> Result<(&'a str, f64), (&'a str, f64)> {
    let distance = jaro(a, b);
    let tuple = (b, distance);

    if distance >= threshold {
        Ok(tuple)
    } else {
        Err(tuple)
    }
}


/// Checks if a string has any similars in a set of strings for the given
/// threshold. Check `is_similar()` for implementation details.
pub fn is_similar_to_any<'a>(a: &str, v: &[&'a str], threshold: f64)
    -> Vec<Result<(&'a str, f64), (&'a str, f64)>> {
    v.iter()
     .map(|&b| is_similar(a, b, threshold))
     .collect()
}


/// Filters a list of strings based on how similar are to an initial string
/// for the given threshold. Check `is_similar()` for implementation details.
pub fn filter_by_similar<'a>(input: &str, references: &Vec<&'a str>, threshold: f64) -> Vec<&'a str> {
    let xs = is_similar_to_any(input, references, threshold);

    xs.iter()
      .filter_map(|&x| x.ok())
      .map(|(x, _)| x)
      .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_similar_test() {
        assert!(is_similar("RFC: iOS Secret Keeping", "iOS Secret Keeping", 0.6).is_ok());
        assert!(is_similar("RFC: iOS Secret Keeping", "Developer certificate of origin", 0.6).is_err());
    }

    #[test]
    fn is_similar_to_all_test() {
        let expected = vec!["iOS Secret Keeping"];
        let actual = "RFC: iOS Secret Keeping";

        assert!(is_similar_to_any(actual, &expected, 0.6).iter()
                .all(|x| x.is_ok()));
    }

    #[test]
    fn is_different_to_some_test() {
        let expected = vec!["iOS Secret Keeping", "Developer certificate of origin"];
        let actual = "RFC: iOS Secret Keeping";

        assert!(is_similar_to_any(actual, &expected, 0.6).iter()
                .any(|x| x.is_err()));
    }


    #[test]
    fn similar_to_all() {
        let expected = vec!["iOS Secret Keeping"];
        assert_eq!(filter_by_similar("RFC: iOS Secret Keeping", &expected, 0.6), vec!["iOS Secret Keeping"]);
    }

    #[test]
    fn similar_to_some() {
        let expected = vec!["iOS Secret Keeping", "Developer certificate of origin"];
        assert_eq!(filter_by_similar("RFC: iOS Secret Keeping", &expected, 0.6), vec!["iOS Secret Keeping"]);
    }

}
