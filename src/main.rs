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

fn split_fields(fields_str: &str) -> Vec<& u32> {
    let mut fields = Vec::new();
    fields
}

fn parse_int(number_str: &str) -> u32 {
    let number = number_str.parse::<u32>().expect(format!("Failed to parse {}", number_str).as_str());
    number
}


fn split_line<'a>(line: &'a str, delimiter: &'a str) -> Vec<&'a str> {
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
        println!("{:?}", line);
    }
    // if vec![1] == vec![0] {
    //     println!("yes");
    // } else {
    //     println!("no");
    // }

    // let number = parse_int("100e");
    // println!("{}", number)
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

    // #[test]
    // #[should_panic]
    // fn parse_bad_int(){
    //     parse_int("100e")
    // }

    // fn split_fields1() {
    //     let input = "1";
    //     let output = split_fields(input);
    //     let expected_output = vec![1];
    //     assert_eq!(output, expected_output);
    // }
}
