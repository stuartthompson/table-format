pub struct ContentIterator {
    parts: Vec<String>,
    next_part_ix: usize
}

impl Iterator for ContentIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if self.next_part_ix < self.parts.len() {
            // Get the next line part
            let line_part = self.parts[self.next_part_ix].to_string();

            // Increment the line part counter
            self.next_part_ix += 1;

            // Return the line part
            Some(line_part)
        } else {
            None
        }
    }
}

impl ContentIterator {
    pub fn new(parts: Vec<String>) -> ContentIterator {
        ContentIterator {
            parts,
            next_part_ix: 0
        }
    }
}
