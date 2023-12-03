use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut calibration_sum = 0;
    let path = "calibration_doc.txt";
    let file = fs::File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut count = 0;

    for line in reader.lines(){
        let line = line?;
        calibration_sum = calibration_sum + get_calibration_value(line,  &mut count)
    }
    // let contents = fs::read_to_string(&file).expect("Failed to read file");
    println!("Docs: \n{}", calibration_sum);
    Ok(())
}

fn get_calibration_value(line: String, count: &mut i32) -> i32{
    let mut calibration_value = 0;
    let mut startFound = false;
    let mut endFound = false;
    let mut start = 0;
    let mut end = line.len() - 1;
    let mut start_char = line.chars().nth(start).unwrap();
    let mut end_char = line.chars().nth(end).unwrap();
    while ! (startFound && endFound) {
 
        if ! startFound {
            start_char = line.chars().nth(start).unwrap();
        }
        if ! endFound {
            end_char = line.chars().nth(end).unwrap();
        }

        if start_char.is_numeric() && !startFound{
            println!("Docs: running val: \n{}", calibration_value);
            println!("Docs: start \n{}", start_char);
            calibration_value = calibration_value + ((start_char as i32 - '0' as i32))*10;
            startFound = true;
        }
        if end_char.is_numeric() && !endFound{
            println!("Docs: running val \n{}", calibration_value);
            println!("Docs: end \n{}", end_char);
            calibration_value = calibration_value + (end_char as i32 - '0' as i32);
            endFound = true;
        }
        start+=1;
        if end > 0 {
            end-=1;
        }
    }
    return calibration_value;
}
