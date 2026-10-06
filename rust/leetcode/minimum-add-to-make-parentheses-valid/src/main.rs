fn main() {
    println!("Hello, world!");
}

pub fn min_add_to_make_valid(s: String) -> i32 {
    let mut count: i32 = 0;
    let mut cant_be_closed: i32 = 0;

    for str_part in s.chars() {
        if str_part == '(' {
            if count < 0 {
                cant_be_closed = cant_be_closed + count.abs();
                count = 0;
            }
            count = count + 1;
        } else if str_part == ')' {
            count = count - 1
        }
    }
    count.abs() + cant_be_closed
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_braket() {
        let result = min_add_to_make_valid(String::from("())"));
        assert_eq!(result, 1)
    }

    #[test]
    fn all_left() {
        let result = min_add_to_make_valid(String::from("((("));
        assert_eq!(result, 3)
    }

    #[test]
    fn all_right() {
        let result = min_add_to_make_valid(String::from(")))"));
        assert_eq!(result, 3)
    }

    #[test]
    fn with_text() {
        let result = min_add_to_make_valid(String::from("(asdfsdfa))"));
        assert_eq!(result, 1)
    }

    #[test]
    fn with_text_outside_left() {
        let result = min_add_to_make_valid(String::from("a(asdfsdfa)"));
        assert_eq!(result, 2)
    }

    #[test]
    fn with_text_outside_right() {
        let result = min_add_to_make_valid(String::from("(asdfsdfa)a"));
        assert_eq!(result, 2)
    }

    #[test]
    fn with_text_outside_both() {
        let result = min_add_to_make_valid(String::from("a(asdfsdfa)a"));
        assert_eq!(result, 2)
    }

    #[test]
    fn cancle_out_simple_count_left() {
        let result = min_add_to_make_valid(String::from("()))(("));
        assert_eq!(result, 4)
    }

    #[test]
    fn cancle_out_simple_count_right() {
        let result = min_add_to_make_valid(String::from("))()(("));
        assert_eq!(result, 4)
    }
}
