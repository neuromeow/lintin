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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_validate_inventory_reading_errors_do_not_exist() {
        let cursor = Cursor::new(b"Valid line\n");
        let found_errors = validate_inventory(cursor);
        assert!(found_errors.is_empty());
    }

    #[test]
    fn test_validate_inventory_reading_errors_exist() {
        let corrupt_data: &[u8] = &[0, 159, 146, 150];
        let cursor = Cursor::new(corrupt_data);
        let result = validate_inventory(cursor);
        assert_eq!(result.len(), 1);
        assert!(result[0].starts_with("Error reading line 1:"));
    }
}
