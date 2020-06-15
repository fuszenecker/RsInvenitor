use std::env;
use std::error::Error;
use std::fs;
use std::result::Result;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 3 {
        let file_name = &args[1];

        let max_step = if args.len() == 4 {
            usize::from_str_radix(&args[3], 10)?
        } else {
            usize::max_value()
        };

        let text: String = fs::read_to_string(file_name)
            .unwrap_or_else(|_| panic!("Documentum nomine {} inveniri non potest.", &file_name))
            .chars()
            .filter(|ch| ch.is_alphabetic())
            .collect();

        let uppercase_text: String = text.to_uppercase();

        let string: String = args[2].chars().filter(|ch| ch.is_alphabetic()).collect();

        let mut output_file_name = String::from(file_name);
        output_file_name.push_str(".p");
        fs::write(output_file_name, uppercase_text.as_bytes())?;

        let uppercase_string: String = string.to_uppercase();
        println!(
            "In documento nomine \"{}\" textum \"{}\" invenio, gradus maximus est {}.",
            file_name, uppercase_string, max_step
        );

        let mut findings = invenitor::find_string(&uppercase_text, &uppercase_string, max_step);

        findings.sort_by_key(|&(_, c)| c);

        for (offset, step) in findings {
            println!("Positio: {}, gradus: {}.", offset, step);
        }

        Ok(())
    } else {
        println!("Sic utar: invenitor <nomen documenti> <textus inveniendus>");
        Ok(())
    }
}
