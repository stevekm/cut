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



// a list of the fields to output from each line,
// #[derive(Debug)]
// struct FieldList {
//     fields: Vec<FieldValue>
// }
//
// #[derive(Debug)]
// enum FieldValue {
//     Value(Value), // a single field value
//     Range1(Range1), // a range that starts at the given index and extends to the end of the line
//     Range2(Range2) // a start and stop range
// }
//
// #[derive(Debug)]
// struct Value {
//     value: u32
// }
//
// #[derive(Debug)]
// struct Range1 {
//     start: u32
// }
//
// #[derive(Debug)]
// struct Range2 {
//     start: u32,
//     stop: u32,
// }

fn parse_int(number_str: &str) -> u32 {
    // convert string to int
    let number = number_str.parse::<u32>().expect(format!("Failed to parse {}", number_str).as_str());
    number
}


// fn get_fields(fields_str: &str) -> Vec<u32> {
//     // parse out the field indexes from the CLI arg
//     let mut values = Vec::new();
//
//     // first split comma separated components
//     let fields_parts = fields_str.split(",").collect::<Vec<&str>>();
//     for part in fields_parts{
//         if part.contains("-"){
//             // check for '-' indicating a range
//             let subparts = part.split("-").collect::<Vec<&str>>();
//             if subparts.len() != 2 {
//                 panic!("Could not parse '{}'", part);
//             }
//             let start = parse_int(&subparts[0]);
//             let mut end = parse_int(&subparts[1]);
//             end = end + 1;
//             for num in (std::ops::Range {start: start, end: end}){
//                 values.push(num);
//             }
//         } else {
//             let parsed = parse_int(&part);
//             values.push(parsed);
//         }
//
//     }
//
//     // do not allow 0 values
//     if values.iter().any(|v| v.clone() == 0) {
//         panic!("Zero value not allowed in field range")
//     }
//
//     values
// }


fn split_line<'a>(line: &'a str, delimiter: &'a str) -> Vec<&'a str> {
    // split lines of text from the input file
    let output = line.split(delimiter).collect();
    output
}

// fn subset_line_parts<'a>(parts: &'a [&str], indexes: &[u32]) -> Vec<&'a str> {
//     // get only the indexed vector elements
//     let output = indexes.iter().map(|index| parts[usize::try_from(*index).unwrap()]).collect::<Vec<&str>>();
//     output
// }

// fn fields_to_indexes(fields: Vec<u32>) -> Vec<u32> {
//     // convert the field numbers into array indexes
//     let indexes = fields.iter().map(|&f| f - 1).collect::<Vec<u32>>();
//     indexes
// }

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

// fn indexes_to_print(fields: &FieldList, delimiter: &str, line: &str) -> Vec<u32> {
//     let mut indexes = Vec::new();
//     let num_fields = line.matches(delimiter).count();
//
//     for field in fields.fields {
//         match field {
//             FieldValue::Value(value) => {
//                 if value =< num_fields {
//                     indexes.push(value)
//                 }
//             },
//             FieldValue::Range1(start) => {
//                 if start < num_fields {
//                     for num in (std::ops::Range {start: start, end: num_fields + 1}){
//                         indexes.push(num);
//                     }
//                 } else if start == num_fields {
//                     indexes.push(start);
//                 }
//             },
//             FieldValue::Range2(start, stop) => {
//                 if start < num_fields {
//                     if stop =< num_fields {
//                         for num in (std::ops::Range {start: start, end: stop + 1}){
//                             indexes.push(num);
//                         }
//                     } else {
//                         for num in (std::ops::Range {start: start, end: num_fields + 1}){
//                             indexes.push(num);
//                         }
//                     }
//                 } else if start == num_fields {
//                     indexes.push(start);
//                 }
//             }
//         }
//     }
//     indexes
// }
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
    let field_list = make_field_list(&fields.to_string());
    println!("{:?}", field_list);

    //
    // let max = 4;
    // let mut fields = Vec::new();
    // fields.push(FieldRange::Value(4));
    // let range = fields[0].get_field_range(&max);
    // println!("{:?}", range);

    // let indexes = fields_to_indexes(get_fields(fields));


    // let field_list = make_field_list(&"1,4-".to_string());
    // println!("{:?}", field_list);
    // println!("{:?}", field_list.fields);
    // println!("{:?}", field_list.fields[0]);
    // println!("{:?}", field_list.fields[0].value);
    //
    // let config = Config {
    //     mode: OperatingMode::Fields,
    //     input: inputFile.to_string(),
    //     delimiter: delimiter.to_string(),
    //     fields: field_list,
    // };
    // println!("{:?}", config.fields.fields);

    // for line in reader.get().lines() {
    //     match line {
    //         Ok(l) => {
    //             let parts = split_line(&l, &delimiter);
    //             let subset = subset_line_parts(&parts, &indexes);
    //             let output = subset.join(delimiter);
    //             println!("{}", output);
    //         }
    //         Err(e) => println!("error parsing line: {:?}", e),
    //     }
    // }
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






    // #[test]
    // fn test_get_fields_multi(){
    //     let fields_str = "1,2";
    //     let expected_output = vec![1,2];
    //     let output = get_fields(fields_str);
    //     assert_eq!(output, expected_output)
    // }

    // #[test]
    // fn test_get_fields_range(){
    //     let fields_str = "1-3";
    //     let expected_output = vec![1,2,3];
    //     let output = get_fields(fields_str);
    //     assert_eq!(output, expected_output)
    // }
    //
    // #[test]
    // fn test_get_fields_range_multi(){
    //     let fields_str = "1-3,4,7,9-12";
    //     let expected_output = vec![1,2,3,4,7,9,10,11,12];
    //     let output = get_fields(fields_str);
    //     assert_eq!(output, expected_output)
    // }

    // #[test]
    // fn test_subset_line_parts(){
    //     let parts = vec!["foo", "bar", "baz"];
    //     let indexes = vec![1,2]; // not the same as fields; off by 1
    //     let expected_output = vec!["bar", "baz"];
    //     let output = subset_line_parts(&parts, &indexes);
    //     assert_eq!(output, expected_output)
    // }

    // #[test]
    // fn test_fields_to_indexes(){
    //     let fields = vec![1,2,3];
    //     let expected_output = vec![0u32,1u32,2u32];
    //     let output = fields_to_indexes(fields);
    //     assert_eq!(output, expected_output)
    // }

    // #[test]
    // fn test_subset_line_fields(){
    //     let fields_str = "1-3,5";
    //     let fields = get_fields(fields_str);
    //     let indexes = fields_to_indexes(fields);
    //     let parts = vec!["foo", "bar", "baz", "buzz", "fuzz", "waz"];
    //     let expected_output = vec!["foo", "bar", "baz", "fuzz"];
    //     let output = subset_line_parts(&parts, &indexes);
    //     assert_eq!(output, expected_output)
    // }



    // #[test]
    // fn test_cut_line1(){
    //     let fields_str = "1-3,5";
    //     let delimiter = "\t";
    //     let indexes = fields_to_indexes(get_fields(fields_str));
    //     let input = "foo\tbar\tbaz\tbuzz\tfuzz\twaz";
    //     let expected_output = "foo\tbar\tbaz\tfuzz";
    //     let output = subset_line_parts(&split_line(&input, &delimiter), &indexes).join(delimiter);
    //     assert_eq!(output, expected_output)
    // }

    // #[test]
    // fn test_cut_line2(){
    //     let fields_str = "1-3,5";
    //     let delimiter = ",";
    //     let indexes = fields_to_indexes(get_fields(fields_str));
    //     let input = "foo,bar,baz,buzz,fuzz,waz";
    //     let expected_output = "foo,bar,baz,fuzz";
    //     let output = subset_line_parts(&split_line(input, delimiter), &indexes).join(delimiter);
    //     assert_eq!(output, expected_output)
    // }

    // #[test]
    // #[should_panic]
    // fn test_field_0(){
    //     let fields_str = "0";
    //     let indexes = get_fields(fields_str);
    // }

    // #[test]
    // fn test_indexes_to_print(){
    //     let fields_str = "1,4-";
    //     let input = "foo\tbar\tbaz\tbuzz\tfuzz\twaz";
    //     let expected_output = vec![1,4,5,6];
    //     let field_list = make_field_list(&fields_str);
    //     let output = indexes_to_print();
    //     // let expected_output = "foo\tbuzz\tfuzz\twaz";
    //     // let expected_output = FieldList {
    //     //     fields: vec![
    //     //     FieldValue::Value(Value {value: 1}),
    //     //     FieldValue::Range1(Range1 {start: 4})
    //     //     ]
    //     // };
    //     // let field_list = make_field_list(&fields_str)
    //     // let output = format_output_line();
    //     // println!("{:?}", output);
    //     // assert_eq!(output.fields[1], expected_output.fields[1])
    //     // assert_eq!(output.fields[0].value, 1)
    // }
    // fn test_field_range_dash(){
        // let fields_str = "1,4-";
    //     let delimiter = "\t";
    //     let input = "foo\tbar\tbaz\tbuzz\tfuzz\twaz";
    //     let indexes = indexes_to_print(fields_str, delimiter, input);

    //     let output = format_output_line(indexes, input);
    //     assert_eq!(output, expected_output)
    // }

}
