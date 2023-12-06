use std::str::FromStr;

/// Parse a whitespace-delimited string into a vector of objects of type T.
pub fn parse_str<T>(s: &str) -> Result<Vec<T>, <T as FromStr>::Err> where T: FromStr {
    let mut v: Vec<T> = vec!();
    let mut parse_result: Result<T, <T as FromStr>::Err>;
    for sub in s.split_whitespace() {
        parse_result = sub.parse::<T>();
        match parse_result {
            Ok(value) => v.push(value),
            Err(e) => return Err(e)
        }
    }
    Ok(v)
}

/// Splits a string into two parts, one before the colon and one after.
/// Assumings the string is of a format (note the space after the colon):
///     <prefix>: <data>
/// Returns a tuple (prefix, data).
pub(crate) fn split_prefix(s: &str) -> (&str, &str) {
    let colon_i = s.find(':').expect("Could not find colon in string.");
    (&s[0..colon_i], &s[colon_i+2..])
}
