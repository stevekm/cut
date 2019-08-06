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


// #[derive(Debug)]
// enum OperatingMode {
//     // whether to split on text character fields or bytes (not implemented yet)
//     Fields
// }

// #[derive(Debug)]
// struct Config {
//     mode: OperatingMode,
//     input: String,
//     // fields: FieldList,
//     delimiter: String,
// }

#[derive(Debug)]
enum FieldRange {
    Value(u32), // a single field value
    Range1(u32), // a range that starts at the given index and extends to the end of the line
    Range2((u32, u32)) // a start and stop range
}

impl FieldRange {
    fn get_field_range<'a>(&'a self, max: &'a u32) -> (&'a u32, &'a u32) {
        // get the start and stop values based on the range type and max value
        use FieldRange::*;

        match &self {
            &Value(start) => {
                if start <= max {
                    (start, start)
                } else {
                    (&0, &0) // empty line; remove later
                }
            },
            &Range1(start) => {
                if start <= max {
                    (start, max)
                } else {
                    (&0, &0) // empty line; remove later
                }
            },
            &Range2((start, stop)) => {
                if start <= max {
                    if stop <= max {
                        (start, stop)
                    } else {
                        (start, max)
                    }
                } else {
                    (&0, &0) // empty line; remove later
                }
            },
        }
    }
}

#[derive(Debug)]
struct FieldList {
    // a list of the fields to output from each line,
    fields: Vec<FieldRange>
}

impl FieldList {
    fn get_fields<'a>(&'a self, max: &'a u32) -> Vec<u32> {
        // return the list of fields to output from the input file
        let mut values = Vec::new();
        for field in &self.fields {
            let (start, stop) = field.get_field_range(&max);
            for num in (std::ops::Range {start: start.clone(), end: stop.clone() + 1 }){
                values.push(num);
            }
        }
        // do not include 0 values
        let values = values.into_iter().filter(|e| e.clone() != 0).collect::<Vec<u32>>();
        values
    }

    fn get_indexes(&self, length: &u32) -> Vec<u32> {
        // return the index values for the split line parts which should be returned
        let fields = self.get_fields(&length);
        let indexes = fields.iter().map(|&f| f - 1).collect::<Vec<u32>>();
        indexes
    }
}


fn parse_int(number_str: &str) -> u32 {
    // convert string to int
    let number = number_str.parse::<u32>().expect(format!("Failed to parse {}", number_str).as_str());
    number
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

fn make_field_list(fields_str: &str) -> FieldList {
    // generate a field list object from the CLI arg
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
                fields.push(FieldRange::Range2((start, stop)));
            } else if subparts.len() == 1 {
                // a Range that goes to the end of the line
                let start = parse_int(&subparts[0]);
                fields.push(FieldRange::Range1(start));
            } else {
                panic!("Could not parse '{}'", part);
            }
        } else {
            let parsed = parse_int(&part);
            fields.push(FieldRange::Value(parsed));
        }
    }

    let output = FieldList { fields: fields };
    output
}

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
    let field_list = make_field_list(&fields.to_string());

    // main program loop
    for line in reader.get().lines() {
        match line {
            Ok(l) => {
                // todo: put this in function with tests
                let parts = split_line(&l, &delimiter);
                let length = u32::try_from(parts.len()).unwrap();
                let indexes = field_list.get_indexes(&length);
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
    fn test_parse_int1(){
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
    fn test_field_list1_range2(){
        let max = 10;
        let mut fields = Vec::new();
        fields.push(FieldRange::Range2((1, 3)));
        let range = fields[0].get_field_range(&max);
        let expected_output = (&1,&3);
        assert_eq!(range, expected_output)
    }

    #[test]
    fn test_field_list2_range2(){
        let max = 2;
        let mut fields = Vec::new();
        fields.push(FieldRange::Range2((1, 4)));
        let range = fields[0].get_field_range(&max);
        let expected_output = (&1,&2);
        assert_eq!(range, expected_output)
    }

    #[test]
    fn test_field_list2_range3(){
        let max = 4;
        let mut fields = Vec::new();
        fields.push(FieldRange::Range2((5, 7)));
        let range = fields[0].get_field_range(&max);
        let expected_output = (&0,&0);
        assert_eq!(range, expected_output)
    }

    #[test]
    fn test_field_list2_range1(){
        let max = 4;
        let mut fields = Vec::new();
        fields.push(FieldRange::Range1(5));
        let range = fields[0].get_field_range(&max);
        let expected_output = (&0, &0);
        assert_eq!(range, expected_output)
    }

    #[test]
    fn test_field_list3_range1(){
        let max = 4;
        let mut fields = Vec::new();
        fields.push(FieldRange::Range1(2));
        let range = fields[0].get_field_range(&max);
        let expected_output = (&2, &4);
        assert_eq!(range, expected_output)
    }

    #[test]
    fn test_field_list4_range1(){
        let max = 4;
        let mut fields = Vec::new();
        fields.push(FieldRange::Range1(4));
        let range = fields[0].get_field_range(&max);
        let expected_output = (&4, &4);
        assert_eq!(range, expected_output)
    }

    #[test]
    fn test_field_list1_value(){
        let max = 4;
        let mut fields = Vec::new();
        fields.push(FieldRange::Value(4));
        let range = fields[0].get_field_range(&max);
        let expected_output = (&4, &4);
        assert_eq!(range, expected_output)
    }

    #[test]
    fn test_field_list2_value(){
        let max = 4;
        let mut fields = Vec::new();
        fields.push(FieldRange::Value(1));
        let range = fields[0].get_field_range(&max);
        let expected_output = (&1, &1);
        assert_eq!(range, expected_output)
    }

    #[test]
    fn test_field_list4_value(){
        let max = 4;
        let mut fields = Vec::new();
        fields.push(FieldRange::Value(5));
        let range = fields[0].get_field_range(&max);
        let expected_output = (&0, &0);
        assert_eq!(range, expected_output)
    }

    #[test]
    fn test_field_list1(){
        let max = 10;
        let mut fields = Vec::new();
        fields.push(FieldRange::Range2((1, 3)));
        let field_list = FieldList { fields: fields };
        let output = field_list.get_fields(&max);
        let expected_output = vec![1,2,3];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_field_list2(){
        let max = 10;
        let mut fields = Vec::new();
        fields.push(FieldRange::Range2((8, 10)));
        let field_list = FieldList { fields: fields };
        let output = field_list.get_fields(&max);
        let expected_output = vec![8,9,10];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_field_list3(){
        let max = 10;
        let mut fields = Vec::new();
        fields.push(FieldRange::Range2((9, 11)));
        let field_list = FieldList { fields: fields };
        let output = field_list.get_fields(&max);
        let expected_output = vec![9,10];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_field_list4(){
        let max = 10;
        let mut fields = Vec::new();
        fields.push(FieldRange::Range1(7));
        let field_list = FieldList { fields: fields };
        let output = field_list.get_fields(&max);
        let expected_output = vec![7,8,9,10];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_field_list5(){
        let max = 10;
        let mut fields = Vec::new();
        fields.push(FieldRange::Range1(11));
        let field_list = FieldList { fields: fields };
        let output = field_list.get_fields(&max);
        let expected_output = vec![];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_field_list6(){
        let max = 10;
        let mut fields = Vec::new();
        fields.push(FieldRange::Value(11));
        let field_list = FieldList { fields: fields };
        let output = field_list.get_fields(&max);
        let expected_output = vec![];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_field_list7(){
        let max = 10;
        let mut fields = Vec::new();
        fields.push(FieldRange::Value(1));
        let field_list = FieldList { fields: fields };
        let output = field_list.get_fields(&max);
        let expected_output = vec![1];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_field_list_indexes1(){
        let max = 5;
        let mut fields = Vec::new();
        fields.push(FieldRange::Value(1));
        let field_list = FieldList { fields: fields };
        let output = field_list.get_indexes(&max);
        let expected_output = vec![0];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_field_list_indexes2(){
        let max = 5;
        let mut fields = Vec::new();
        fields.push(FieldRange::Value(6));
        let field_list = FieldList { fields: fields };
        let output = field_list.get_indexes(&max);
        let expected_output = vec![];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_field_list_indexes3(){
        let max = 5;
        let mut fields = Vec::new();
        fields.push(FieldRange::Range1(6));
        let field_list = FieldList { fields: fields };
        let output = field_list.get_indexes(&max);
        let expected_output = vec![];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_field_list_indexes4(){
        let max = 5;
        let mut fields = Vec::new();
        fields.push(FieldRange::Range1(3));
        let field_list = FieldList { fields: fields };
        let output = field_list.get_indexes(&max);
        let expected_output = vec![2,3,4];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_field_list_indexes5(){
        let max = 5;
        let mut fields = Vec::new();
        fields.push(FieldRange::Range2((2,4)));
        let field_list = FieldList { fields: fields };
        let output = field_list.get_indexes(&max);
        let expected_output = vec![1,2,3];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_field_list_indexes6(){
        let max = 5;
        let mut fields = Vec::new();
        fields.push(FieldRange::Range2((2,6)));
        let field_list = FieldList { fields: fields };
        let output = field_list.get_indexes(&max);
        let expected_output = vec![1,2,3,4];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_field_list_indexes7(){
        let max = 5;
        let mut fields = Vec::new();
        fields.push(FieldRange::Range2((6,8)));
        let field_list = FieldList { fields: fields };
        let output = field_list.get_indexes(&max);
        let expected_output = vec![];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_make_field_list1(){
        let fields_str = "1";
        let length = 10;
        let field_list = make_field_list(&fields_str);
        let output = field_list.get_fields(&length);
        let expected_output = vec![1];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_make_field_list2(){
        let fields_str = "2-";
        let length = 5;
        let field_list = make_field_list(&fields_str);
        let output = field_list.get_fields(&length);
        let expected_output = vec![2,3,4,5];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_make_field_list3(){
        let fields_str = "2-4";
        let length = 5;
        let field_list = make_field_list(&fields_str);
        let output = field_list.get_fields(&length);
        let expected_output = vec![2,3,4];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_make_field_list4(){
        let fields_str = "2-4,8,11-14";
        let length = 25;
        let field_list = make_field_list(&fields_str);
        let output = field_list.get_fields(&length);
        let expected_output = vec![2,3,4,8,11,12,13,14];
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_make_field_list5(){
        let fields_str = "4,11-";
        let length = 14;
        let field_list = make_field_list(&fields_str);
        let output = field_list.get_fields(&length);
        let expected_output = vec![4,11,12,13,14];
        assert_eq!(output, expected_output)
    }

}
