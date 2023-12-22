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
    fn test_validate_inventory_line_reading_errors_do_not_exist() {
        let valid_data = b"Valid line\n";
        let cursor = Cursor::new(valid_data);
        let inventory_errors = validate_inventory(cursor);
        assert!(inventory_errors.is_empty());
    }

    #[test]
    fn test_validate_inventory_line_reading_errors_exist() {
        let corrupted_data = &[0, 159, 146, 150];
        let cursor = Cursor::new(corrupted_data);
        let inventory_errors = validate_inventory(cursor);
        assert_eq!(inventory_errors.len(), 1);
        assert!(inventory_errors[0].starts_with("1 Line reading error:"));
    }
}
