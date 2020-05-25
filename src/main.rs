use std::env;
use std::result::{ Result };
use std::error::{ Error };
use std::fs;
use std::cmp::{ min };
use std::path::{ Path };

fn compare_strings(text: &Vec<char>, string: &String, offset: usize, step: usize) -> bool {
    for (index, ch) in string.chars().enumerate() {
        if offset + index * step >= text.len() {
            return false
        } else {
            let fch = text[offset + index * step];

            if ch != fch {
                return false
            }
        }
    }

    true
}

fn find_string(text: &String, string: &String, max_step: usize) -> Vec<(usize, usize)> {
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

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 3 {
        let file_name = &args[1];
        let mut max_step = usize::max_value();

        if args.len() == 4 {
            max_step = usize::from_str_radix(&args[3], 10)?;
        }

        let text: String = fs::read_to_string(file_name)
        .expect("Documentum nomine invenire non possum.")
        .chars()
        .filter(|ch| { ch.is_alphabetic() })
        .collect();

        let uppercase_text: String = text.to_uppercase();

        let string: String = args[2]
        .chars()
        .filter(|ch| { ch.is_alphabetic() })
        .collect();

        let mut output_file_name = String::from(file_name);
        output_file_name.push_str(".p");
        fs::write(output_file_name, uppercase_text.as_bytes())?;

        let uppercase_string: String = string.to_uppercase();
        println!("In documento nomine \"{}\" textum \"{}\" invenio, gradus maximus est {}.", file_name, uppercase_string, max_step);

        let findings = find_string(&uppercase_text, &uppercase_string, max_step);

        for (offset, step) in findings {
            println!("Distantia: {}, gradus: {}.", offset, step);
        }

        Ok(())
    } else {
        println!("Sic utar: invenitor <nomen documenti> <textus inveniendus>");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_strings_trivial() {
        assert_eq!(true, compare_strings(&String::from("AAABBBCCC").chars().collect(), &String::from("AAA"), 0, 1));
    }

    #[test]
    fn compare_strings_should_fail() {
        assert_eq!(false, compare_strings(&String::from("AAABBBCCC").chars().collect(), &String::from("AAB"), 0, 1));
    }

    #[test]
    fn compare_strings_offset() {
        assert_eq!(true, compare_strings(&String::from("AAABBBCCC").chars().collect(), &String::from("AAB"), 1, 1));
    }

    #[test]
    fn compare_strings_distant() {
        assert_eq!(true, compare_strings(&String::from("ALMAFA").chars().collect(), &String::from("AA"), 3, 2));
    }

    #[test]
    fn find_strings_trivial() {
        assert_eq!(vec![(0,1)], find_string(&String::from("AAA"), &String::from("AAA"), 100));
    }

    #[test]
    fn find_strings_with_offset() {
        assert_eq!(vec![(2,1)], find_string(&String::from("BBAAA"), &String::from("AAA"), 100));
    }

    #[test]
    fn find_strings_with_offset_and_step() {
        assert_eq!(vec![(2,2)], find_string(&String::from("BBACACA"), &String::from("AAA"), 100));
    }

    #[test]
    fn find_strings_toolong() {
        assert_eq!(0, find_string(&String::from("BBACACA"), &String::from("AAABBB"), 100).len());
    }

    #[test]
    fn min_1() {
        assert_eq!(5, min(10, 5));
    }
}