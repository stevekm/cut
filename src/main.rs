extern crate clap;
use clap::{Arg, App};
use std::io::{self, BufReader, BufRead};
use std::fs::{self, File};
use std::convert::TryFrom;

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

// whether to split on text character fields or bytes (not implemented yet)
#[derive(Debug)]
enum OperatingMode {
    Fields
}

#[derive(Debug)]
struct Config {
    mode: OperatingMode,
    input: String,
    fields: FieldList,
    delimiter: String,
}

// a list of the fields to output from each line,
#[derive(Debug)]
struct FieldList {
    fields: Vec<FieldValue>
}

#[derive(Debug)]
enum FieldValue {
    Value(Value), // a single field value
    Range1(Range1), // a range that starts at the given index and extends to the end of the line
    Range2(Range2) // a start and stop range
}

#[derive(Debug)]
struct Value {
    value: u32
}

#[derive(Debug)]
struct Range1 {
    start: u32
}

#[derive(Debug)]
struct Range2 {
    start: u32,
    stop: u32,
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

    // do not allow 0 values
    if values.iter().any(|v| v.clone() == 0) {
        panic!("Zero value not allowed in field range")
    }

    values
}


fn split_line<'a>(line: &'a str, delimiter: &'a str) -> Vec<&'a str> {
    // split lines of text from the input file
    let output = line.split(delimiter).collect();
    output
}

fn subset_line_parts<'a>(parts: &'a [&str], indexes: &[u32]) -> Vec<&'a str> {
    // get only the indexed vector elements
    let output = indexes.iter().map(|index| parts[usize::try_from(*index).unwrap()]).collect::<Vec<&str>>();
    output
}

fn fields_to_indexes(fields: Vec<u32>) -> Vec<u32> {
    // convert the field numbers into array indexes
    let indexes = fields.iter().map(|&f| f - 1).collect::<Vec<u32>>();
    indexes
}

// for refactored method for outputting lines
fn make_field_list(fields_str: &str) -> FieldList {
    let mut fields = Vec::new();

    // first split comma separated components
    let fields_parts = fields_str.split(",").collect::<Vec<&str>>();
    for part in fields_parts {
        // check for '-' indicating a range
        if part.contains("-"){
            let subparts = part.split("-").filter(|&i|i != "").collect::<Vec<&str>>();
            if subparts.len() == 2 {
                // a Range with a defined Start and Stop
                // println!("{:?}",subparts);
                let start = parse_int(&subparts[0]);
                let stop = parse_int(&subparts[1]);
                fields.push(FieldValue::Range2(Range2 { start: start, stop: stop } ));
            } else if subparts.len() == 1 {
                // a Range that goes to the end of the line
                let start = parse_int(&subparts[0]);
                fields.push(FieldValue::Range1(Range1 { start: start } ));
            } else {
                panic!("Could not parse '{}'", part);
            }
        } else {
            let parsed = parse_int(&part);
            fields.push(FieldValue::Value( Value { value: parsed } ));
        }
    }

    let output = FieldList { fields: fields };
    output
}

fn indexes_to_print(fields: &FieldList, delimiter: &str, line: &str) -> Vec<u32> {
    let mut indexes = Vec::new();
    let num_fields = line.matches(delimiter).count();

    for field in fields.fields {
        match field {
            FieldValue::Value(value) => {
                if value =< num_fields {
                    indexes.push(value)
                }
            },
            FieldValue::Range1(start) => {
                if start < num_fields {
                    for num in (std::ops::Range {start: start, end: num_fields + 1}){
                        indexes.push(num);
                    }
                } else if start == num_fields {
                    indexes.push(start);
                }
            },
            FieldValue::Range2(start, stop) => {
                if start < num_fields {
                    if stop =< num_fields {
                        for num in (std::ops::Range {start: start, end: stop + 1}){
                            indexes.push(num);
                        }
                    } else {
                        for num in (std::ops::Range {start: start, end: num_fields + 1}){
                            indexes.push(num);
                        }
                    }
                } else if start == num_fields {
                    indexes.push(start);
                }
            }
        }
    }
    indexes
}
//
//
// fn format_output_line(fields: &FieldList, delimiter: &str, line: &str) -> {
//
// }

fn main()  {
    let matches = App::new("cut")
                        .about("GNU cut clone")
                        .arg(Arg::with_name("inputFile")
                            .help("The input file to use")
                            .index(1))
                        .arg(Arg::with_name("fields")
                            .takes_value(true)
                            .required(true)
                            .help("The fields to output")
                            .short("f"))
                      .arg(Arg::with_name("delimiter")
                          .help("Field delimiter")
                        .short("d"))
                        .get_matches();

    let inputFile = matches.value_of("inputFile").unwrap_or("-");
    let fields = matches.value_of("fields").unwrap_or("1");
    let delimiter = matches.value_of("delimiter").unwrap_or("\t");
    let reader = Reader { input: inputFile.to_string() };
    let indexes = fields_to_indexes(get_fields(fields));

    let field_list = make_field_list(&fields.to_string());
    // let field_list = make_field_list(&"1,4-".to_string());
    // println!("{:?}", field_list);
    // println!("{:?}", field_list.fields);
    // println!("{:?}", field_list.fields[0]);
    // println!("{:?}", field_list.fields[0].value);
    //
    let config = Config {
        mode: OperatingMode::Fields,
        input: inputFile.to_string(),
        delimiter: delimiter.to_string(),
        fields: field_list,
    };
    // println!("{:?}", config.fields.fields);

    for line in reader.get().lines() {
        match line {
            Ok(l) => {
                let parts = split_line(&l, &delimiter);
                let subset = subset_line_parts(&parts, &indexes);
                let output = subset.join(delimiter);
                println!("{}", output);
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
        let output = split_line(&input, &delimiter);
        assert_eq!(output, expected_output)
    }

    #[test]
    fn split_lines_comma(){
        let input = "foo,bar";
        let expected_output = vec!["foo", "bar"];
        let delimiter = ",";
        let output = split_line(&input, &delimiter);
        assert_eq!(output, expected_output)
    }

    #[test]
    fn split_lines_pipe(){
        let input = "foo|bar";
        let expected_output = vec!["foo", "bar"];
        let delimiter = "|";
        let output = split_line(&input, &delimiter);
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

    #[test]
    fn test_subset_line_parts(){
        let parts = vec!["foo", "bar", "baz"];
        let indexes = vec![1,2]; // not the same as fields; off by 1
        let expected_output = vec!["bar", "baz"];
        let output = subset_line_parts(&parts, &indexes);
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_fields_to_indexes(){
        let fields = vec![1,2,3];
        let expected_output = vec![0u32,1u32,2u32];
        let output = fields_to_indexes(fields);
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_subset_line_fields(){
        let fields_str = "1-3,5";
        let fields = get_fields(fields_str);
        let indexes = fields_to_indexes(fields);
        let parts = vec!["foo", "bar", "baz", "buzz", "fuzz", "waz"];
        let expected_output = vec!["foo", "bar", "baz", "fuzz"];
        let output = subset_line_parts(&parts, &indexes);
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_join_tab(){
        let input = vec!["foo","bar"];
        let expected_output = "foo\tbar";
        let output = input.join("\t");
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_join_comma(){
        let input = vec!["foo","bar"];
        let expected_output = "foo,bar";
        let output = input.join(",");
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_cut_line1(){
        let fields_str = "1-3,5";
        let delimiter = "\t";
        let indexes = fields_to_indexes(get_fields(fields_str));
        let input = "foo\tbar\tbaz\tbuzz\tfuzz\twaz";
        let expected_output = "foo\tbar\tbaz\tfuzz";
        let output = subset_line_parts(&split_line(&input, &delimiter), &indexes).join(delimiter);
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_cut_line2(){
        let fields_str = "1-3,5";
        let delimiter = ",";
        let indexes = fields_to_indexes(get_fields(fields_str));
        let input = "foo,bar,baz,buzz,fuzz,waz";
        let expected_output = "foo,bar,baz,fuzz";
        let output = subset_line_parts(&split_line(input, delimiter), &indexes).join(delimiter);
        assert_eq!(output, expected_output)
    }

    #[test]
    #[should_panic]
    fn test_field_0(){
        let fields_str = "0";
        let indexes = get_fields(fields_str);
    }

    #[test]
    fn test_indexes_to_print(){
        let fields_str = "1,4-";
        let input = "foo\tbar\tbaz\tbuzz\tfuzz\twaz";
        let expected_output = vec![1,4,5,6];
        let field_list = make_field_list(&fields_str);
        let output = indexes_to_print();
        // let expected_output = "foo\tbuzz\tfuzz\twaz";
        // let expected_output = FieldList {
        //     fields: vec![
        //     FieldValue::Value(Value {value: 1}),
        //     FieldValue::Range1(Range1 {start: 4})
        //     ]
        // };
        // let field_list = make_field_list(&fields_str)
        // let output = format_output_line();
        // println!("{:?}", output);
        // assert_eq!(output.fields[1], expected_output.fields[1])
        // assert_eq!(output.fields[0].value, 1)
    }
    // fn test_field_range_dash(){
        // let fields_str = "1,4-";
    //     let delimiter = "\t";
    //     let input = "foo\tbar\tbaz\tbuzz\tfuzz\twaz";
    //     let indexes = indexes_to_print(fields_str, delimiter, input);

    //     let output = format_output_line(indexes, input);
    //     assert_eq!(output, expected_output)
    // }

}
