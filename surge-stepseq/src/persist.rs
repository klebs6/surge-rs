/*!
  | Here's one way you could implement saving
  | and loading sequences:
  |
  */

crate::ix!();

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// With this implementation, you can save
/// a sequence by calling
/// `my_sequence.save_sequence("my_sequence.txt")`,
/// and load a sequence by calling
/// `my_sequence.load_sequence("my_sequence.txt")`. 
///
/// This will read/write the sequence data to/from
/// a plain text file in a simple space-separated
/// format.
/// 
/// Of course, you could also choose to implement
/// other file formats (such as JSON or YAML) if
/// you prefer.
/// 
impl StepSequencer {

    /// Save the current sequence to a file at the
    /// given path.
    ///
    pub fn save_sequence<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {

        let mut file = File::create(path)?;

        for step in &self.steps {
            write!(file, "{} ", step)?;
        }

        write!(file, "{} ", self.loop_start)?;
        write!(file, "{} ", self.loop_end)?;
        write!(file, "{} ", self.shuffle)?;
        write!(file, "{:X}", self.trigmask)?;

        Ok(())
    }

    /// Load a sequence from a file at the given
    /// path.
    ///
    pub fn load_sequence<P: AsRef<Path>>(&mut self, path: P) -> Result<(), std::io::Error> {

        let mut file = File::open(path)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)?;

        let mut split = buffer.split_ascii_whitespace();

        for step in &mut self.steps {

            if let Some(val) = split.next() {
                *step = val.parse().unwrap_or(0.0);
            }
        }

        if let Some(val) = split.next() {
            self.loop_start = val.parse().unwrap_or(0);
        }

        if let Some(val) = split.next() {
            self.loop_end = val.parse().unwrap_or(0);
        }

        if let Some(val) = split.next() {
            self.shuffle = val.parse().unwrap_or(0.0);
        }

        if let Some(val) = split.next() {
            self.trigmask = u64::from_str_radix(val, 16).unwrap_or(0);
        }

        Ok(())
    }
}
