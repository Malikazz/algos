use std::cmp::max;

fn main() {
    println!("Hello, world!");
}

struct ShorterLonger<'a> {
    short: &'a str,
    long: &'a str,
    pat: &'a str,
}

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    // Get the shorter string
    let mut shorter_longer: ShorterLonger;
    if str1.len() < str2.len() {
        shorter_longer = ShorterLonger {
            short: &str1[..],
            long: &str2[..],
            pat: &str1[..],
        };
    } else {
        shorter_longer = ShorterLonger {
            short: &str2[..],
            long: &str1[..],
            pat: &str2[..],
        };
    };

    while shorter_longer.pat.len() > 0 {
        if (shorter_longer.long
            == shorter_longer
                .pat
                .repeat(shorter_longer.long.len() / shorter_longer.pat.len())
            && shorter_longer.long.len() % shorter_longer.pat.len() == 0)
            && (shorter_longer.short
                == shorter_longer
                    .pat
                    .repeat(shorter_longer.short.len() / shorter_longer.pat.len())
                && shorter_longer.short.len() % shorter_longer.pat.len() == 0)
        {
            return String::from(shorter_longer.pat);
        }
        shorter_longer.pat = &shorter_longer.pat[1..];
    }
    String::from(shorter_longer.pat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_string_01() {
        assert_eq!(
            gcd_of_strings(String::from("ABCABC"), String::from("ABC")),
            "ABC"
        );
    }

    #[test]
    fn test_gcd_string_02() {
        assert_eq!(
            gcd_of_strings(String::from("ABABAB"), String::from("ABAB")),
            "AB"
        );
    }

    #[test]
    fn test_gcd_string_03() {
        assert_eq!(
            gcd_of_strings(String::from("LEET"), String::from("CODE")),
            ""
        );
    }
}
