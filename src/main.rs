use std::env;
use std::result::{ Result };
use std::error::{ Error };
use std::fs;

fn compare_strings(text: &String, string: &String, offset: usize, step: usize) -> bool {
    for (index, ch) in string.chars().enumerate() {
        let fch = text.chars().nth(offset + index * step).unwrap();

        if ch != fch {
            return false
        }
    }

    true
}

fn find_string(text: &String, string: &String) -> Vec<(usize, usize)> {
    let length = text.len();
    let mut findings: Vec<(usize, usize)> = Vec::new();

    for offset in 0..length {
        let max_step = (length - offset - 1) / (string.len() - 1);

        for step in 1 ..= max_step {
            if compare_strings(text, string, offset, step) {
                findings.push((offset, step));
            }
        }
    }

    findings
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let file_name = &args[1];
        let string = &args[2];

        println!("In documento nomine \"{}\" textum \"{}\" invenio.", file_name, string);

        let text = fs::read_to_string(file_name)
            .expect("Documentum nomine invenire non possum.");

        let findings = find_string(&text, &string);

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
        assert_eq!(true, compare_strings(&String::from("AAABBBCCC"), &String::from("AAA"), 0, 1));
    }

    #[test]
    fn compare_strings_should_fail() {
        assert_eq!(false, compare_strings(&String::from("AAABBBCCC"), &String::from("AAB"), 0, 1));
    }

    #[test]
    fn compare_strings_offset() {
        assert_eq!(true, compare_strings(&String::from("AAABBBCCC"), &String::from("AAB"), 1, 1));
    }

    #[test]
    fn compare_strings_distant() {
        assert_eq!(true, compare_strings(&String::from("ALMAFA"), &String::from("AA"), 3, 2));
    }

    #[test]
    fn find_strings_trivial() {
        assert_eq!(vec![(0,1)], find_string(&String::from("AAA"), &String::from("AAA")));
    }

    #[test]
    fn find_strings_with_offset() {
        assert_eq!(vec![(2,1)], find_string(&String::from("BBAAA"), &String::from("AAA")));
    }

    #[test]
    fn find_strings_with_offset_and_step() {
        assert_eq!(vec![(2,2)], find_string(&String::from("BBACACA"), &String::from("AAA")));
    }

    #[test]
    fn find_strings_toolong() {
        assert_eq!(0, find_string(&String::from("BBACACA"), &String::from("AAABBB")).len());
    }
}