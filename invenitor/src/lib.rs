use std::cmp::min;

fn compare_strings(text: &[char], string: &str, offset: usize, step: usize) -> bool {
    for (index, ch) in string.chars().enumerate() {
        if offset + index * step >= text.len() {
            return false;
        } else {
            let fch = text[offset + index * step];

            if ch != fch {
                return false;
            }
        }
    }

    true
}

pub fn find_string(text: &str, string: &str, max_step: usize) -> Vec<(usize, usize)> {
    let chars: Vec<char> = text.chars().collect();
    let length = chars.len();
    let mut findings: Vec<(usize, usize)> = Vec::new();

    for offset in 0..length {
        let step_max = min((length - offset - 1) / (string.len() - 1), max_step);

        for step in 1..=step_max {
            if compare_strings(&chars, string, offset, step) {
                findings.push((offset, step));
            }
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_strings_trivial() {
        assert_eq!(
            true,
            compare_strings(
                &String::from("AAABBBCCC").chars().collect::<Vec<char>>(),
                &String::from("AAA"),
                0,
                1
            )
        );
    }

    #[test]
    fn compare_strings_should_fail() {
        assert_eq!(
            false,
            compare_strings(
                &String::from("AAABBBCCC").chars().collect::<Vec<char>>(),
                &String::from("AAB"),
                0,
                1
            )
        );
    }

    #[test]
    fn compare_strings_offset() {
        assert_eq!(
            true,
            compare_strings(
                &String::from("AAABBBCCC").chars().collect::<Vec<char>>(),
                &String::from("AAB"),
                1,
                1
            )
        );
    }

    #[test]
    fn compare_strings_distant() {
        assert_eq!(
            true,
            compare_strings(
                &String::from("ALMAFA").chars().collect::<Vec<char>>(),
                &String::from("AA"),
                3,
                2
            )
        );
    }

    #[test]
    fn find_strings_trivial() {
        assert_eq!(
            vec![(0, 1)],
            find_string(&String::from("AAA"), &String::from("AAA"), 100)
        );
    }

    #[test]
    fn find_strings_with_offset() {
        assert_eq!(
            vec![(2, 1)],
            find_string(&String::from("BBAAA"), &String::from("AAA"), 100)
        );
    }

    #[test]
    fn find_strings_with_offset_and_step() {
        assert_eq!(
            vec![(2, 2)],
            find_string(&String::from("BBACACA"), &String::from("AAA"), 100)
        );
    }

    #[test]
    fn find_strings_toolong() {
        assert_eq!(
            0,
            find_string(&String::from("BBACACA"), &String::from("AAABBB"), 100).len()
        );
    }

    #[test]
    fn min_1() {
        assert_eq!(5, min(10, 5));
    }
}
