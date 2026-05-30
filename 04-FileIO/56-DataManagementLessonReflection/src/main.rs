use w3l1_lesson_reflections::{count_lines_reader, safe_divide};

fn main() {
    let data = b"line1\nline2\nline3\n";
    let count = count_lines_reader(&data[..]);
    println!("Lines: {}", count);

    match safe_divide(10.0, 2.0) {
        Ok(v) => println!("10/2 = {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
