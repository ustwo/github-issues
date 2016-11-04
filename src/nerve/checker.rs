//! This module implements string similarity keeping track of the id/number of
//! the original Issue.

use strsim::jaro;

type IssueField = (u32, String);

type FieldDistance<'a> = (&'a str, Distance);
type Distance = f64;
type DistanceResult<'a> = Result<FieldDistance<'a>, FieldDistance<'a>>;


/// Checks if two strings are similar given a threshold from 0 to 1 where 0
/// is different and 1 is equal.
///
/// Note that the current implementation uses the [Jaro distance](http://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance).
///
/// ```
/// use nerve::checker::similar;
///
/// match similar("foo", "bar", 0.5) {
///     Ok((s, distance)) => assert!(distance >= 0.5),
///     Err((s, distance)) => assert!(distance < 0.5),
/// }
/// ```
pub fn similar<'a>(a: &str, b: &'a str, threshold: f64) -> DistanceResult<'a> {
    let distance = jaro(a, b);
    let res = (b, distance.clone());

    if distance >= threshold {
        Ok(res)
    } else {
        Err(res)
    }
}


/// Checks if a string has any similars in a set of strings for the given
/// threshold. Check `similar()` for implementation details.
pub fn similar_to_any<'a>(a: &'a str, v: &'a [IssueField], threshold: f64)
    -> Vec<(u32, DistanceResult<'a>)> {
    v.iter()
     .map(|&(id, ref b)| (id, similar(a, b, threshold)))
     .collect()
}


/// Filters a list of strings based on how similar are to an initial string
/// for the given threshold. Check `similar()` for implementation details.
pub fn filter_by_similar(input: &str, references: &[IssueField], threshold: f64) -> Vec<IssueField> {
    let xs = similar_to_any(input, references, threshold);

    xs.iter()
      .filter_map(|&(id, x)| {
        match x {
            Err(_) => None,
            Ok((s, _)) => Some((id, s.to_string()))
        }
      })
      .collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn similar_test() {
        assert!(similar("RFC: iOS Secret Keeping", "iOS Secret Keeping", 0.6).is_ok());
        assert!(similar("RFC: iOS Secret Keeping", "Developer certificate of origin", 0.6).is_err());
    }

    #[test]
    fn similar_to_all_test() {
        let expected = vec![(1, "iOS Secret Keeping".to_string())];
        let actual = "RFC: iOS Secret Keeping";

        assert!(similar_to_any(actual, &expected, 0.6).iter()
                .all(|&(_, x)| x.is_ok()));
    }

    #[test]
    fn is_different_to_some_test() {
        let expected = vec![ (1, "iOS Secret Keeping".to_string())
                           , (2, "Developer certificate of origin".to_string())
                           ];
        let actual = "RFC: iOS Secret Keeping";

        assert!(similar_to_any(actual, &expected, 0.6).iter()
                .any(|&(_, x)| x.is_err()));
    }

    #[test]
    fn similar_to_all() {
        let expected = vec![(1, "iOS Secret Keeping".to_string())];
        assert_eq!(filter_by_similar("RFC: iOS Secret Keeping", &expected, 0.6),
                   vec![(1, "iOS Secret Keeping".to_string())]);
    }

    #[test]
    fn similar_to_some() {
        let expected = vec![ (1, "iOS Secret Keeping".to_string())
                           , (2, "Developer certificate of origin".to_string())
                           ];
        assert_eq!(filter_by_similar("RFC: iOS Secret Keeping", &expected, 0.6),
                   vec![(1, "iOS Secret Keeping".to_string())]);
    }

}
