use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn main() {
    let highest_values = match get_three_highest_number_from_file("./input.txt") {
        Ok(result) => result,
        Err(e) => panic!("{}", e)
    };
    println!("Part 1 result: {:?}", highest_values[0]);
    println!("Part 2 result: {}", highest_values[0] + highest_values[1] + highest_values[2])
}

fn get_three_highest_number_from_file(filename: &str) -> Result<[i32; 3], Error> {
    let file = match File::open(filename) {
        Ok(returned_file) => returned_file,
        Err(e) => {
            eprintln!("{}", e);
            return Err(e)
        }
    };

    let reader = BufReader::new(file);
    let mut bmap = BTreeMap::new();
    let mut acc: i32 = 0;

    for (index, line) in reader.lines().enumerate() {
        let line_content = match line {
            Ok(content) => content,
            Err(e) => {
                eprintln!("{}", e);
                return Err(e)
            }
        };

        if line_content == "" {
            bmap.insert(acc, index);
            acc = 0;
        } else {
            let new_value: i32 = match line_content.parse() {
                Ok(value) => value,
                Err(e) => {
                    let err: Error = Error::new(ErrorKind::Other, e);
                    return Err(err)
                }
            };
            acc += new_value;
        }
    };

    // let result = match bmap.last_key_value(){
    //     None => 0_i32,
    //     Some((val, _)) => *val
    // };

    println!("{:?}", bmap);

    let keys: Vec<i32> = bmap.into_keys().rev().collect();
    Ok([keys[0], keys[1], keys[2]])

    // Ok(result)
}

#[test]
fn test_result() {
    let highest_values = get_three_highest_number_from_file("./test.txt").expect("Panic");
    println!("Highest value: {:?}", highest_values[0]);
    println!("Highest 3 values: {}, {}, {}", highest_values[0], highest_values[1], highest_values[2]);
    println!("Sum of 3 values: {}", highest_values[0] + highest_values[1] + highest_values[2])
}

