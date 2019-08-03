extern crate clap;
use clap::{Arg, App};
use std::io::{self, BufReader, BufRead};
use std::fs::{self, File};

struct Reader {
    input: String
}

impl Reader {
    fn get(&self) -> Box<BufRead> {
        let reader: Box<BufRead> = match self.input.as_str() {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(fs::File::open(self.input.as_str()).unwrap()))
        };
        return reader
    }
}

fn parse_int(number_str: &str) -> u32 {
    // convert string to int
    let number = number_str.parse::<u32>().expect(format!("Failed to parse {}", number_str).as_str());
    number
}


fn get_fields(fields_str: &str) -> Vec<u32> {
    // parse out the field indexes from the CLI arg
    let mut values = Vec::new();

    // first split comma separated components
    let fields_parts = fields_str.split(",").collect::<Vec<&str>>();
    for part in fields_parts{
        if part.contains("-"){
            // check for '-' indicating a range
            let subparts = part.split("-").collect::<Vec<&str>>();
            if subparts.len() != 2 {
                panic!("Could not parse '{}'", part);
            }
            let start = parse_int(&subparts[0]);
            let mut end = parse_int(&subparts[1]);
            end = end + 1;
            for num in (std::ops::Range {start: start, end: end}){
                values.push(num);
            }
        } else {
            let parsed = parse_int(&part);
            values.push(parsed);
        }

    }

    values
}


fn split_line<'a>(line: &'a str, delimiter: &'a str) -> Vec<&'a str> {
    // split lines of text from the input file
    let output = line.split(delimiter).collect();
    output
}

fn main()  {
    let matches = App::new("cut")
                        .about("GNU cut clone")
                        .arg(Arg::with_name("inputFile")
                           .help("The input file to use")
                           .index(1))
                        .arg(Arg::with_name("fields")
                            .help("The fields to output")
                          .short("f"))
                      .arg(Arg::with_name("delimiter")
                          .help("Field delimiter")
                        .short("d"))
                        .get_matches();

    let inputFile = matches.value_of("inputFile").unwrap_or("-");
    let fields = matches.value_of("fields").unwrap_or("0");
    let delimiter = matches.value_of("delimiter").unwrap_or("\t");
    let reader = Reader { input: inputFile.to_string() };

    for line in reader.get().lines() {
        match line {
            Ok(l) => {
                let parts = split_line(&l, delimiter);
                println!("{:?}", parts);
            }
            Err(e) => println!("error parsing line: {:?}", e),
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn parse_int1(){
        assert_eq!(parse_int("100"), 100)
    }

    #[test]
    fn split_lines_tab(){
        let input = "foo\tbar";
        let expected_output = vec!["foo", "bar"];
        let delimiter = "\t";
        let output = split_line(input, delimiter);
        assert_eq!(output, expected_output)
    }

    #[test]
    fn split_lines_comma(){
        let input = "foo,bar";
        let expected_output = vec!["foo", "bar"];
        let delimiter = ",";
        let output = split_line(input, delimiter);
        assert_eq!(output, expected_output)
    }

    #[test]
    fn split_lines_pipe(){
        let input = "foo|bar";
        let expected_output = vec!["foo", "bar"];
        let delimiter = "|";
        let output = split_line(input, delimiter);
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_get_fields_0(){
        let fields_str = "0";
        let expected_output = vec![0];
        let output = get_fields(fields_str);
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_get_fields_multi(){
        let fields_str = "1,2";
        let expected_output = vec![1,2];
        let output = get_fields(fields_str);
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_get_fields_range(){
        let fields_str = "1-3";
        let expected_output = vec![1,2,3];
        let output = get_fields(fields_str);
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_get_fields_range_multi(){
        let fields_str = "1-3,4,7,9-12";
        let expected_output = vec![1,2,3,4,7,9,10,11,12];
        let output = get_fields(fields_str);
        assert_eq!(output, expected_output)
    }

}
