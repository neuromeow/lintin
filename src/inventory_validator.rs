use std::io::BufRead;

pub fn validate_inventory<R: BufRead>(reader: R) -> Vec<String> {
    let mut inventory_errors = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(_text) => {
                // As a stub for future checks.
                // And if an error occurs, it's description should also be added to `inventory_errors`.
            }
            Err(err) => {
                inventory_errors.push(format!("{} Line reading error: {}", index + 1, err));
            }
        }
    }
    inventory_errors
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
