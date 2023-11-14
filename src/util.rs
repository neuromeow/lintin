use std::io::BufRead;

pub fn read_lines_from_buf_reader<R: BufRead>(buf_reader: R) -> Vec<String> {
    buf_reader.lines().map(|line| line.unwrap()).collect()
}
