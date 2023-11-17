use std::io::BufRead;

pub fn validate_inventory<R: BufRead>(reader: R) -> Vec<String> {
    let mut found_errors = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(_text) => {
                // As a stub for future checks. If an error occurs, add its description to `found_errors`.
                // You can use `index` here which contains the line number.
            }
            Err(err) => {
                // If there was an error reading a line, add its description along with line number to `found_errors`.
                found_errors.push(format!("Error reading line {}: {}", index + 1, err));
            }
        }
    }
    found_errors
}
