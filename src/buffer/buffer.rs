use crate::buffer::chunk::{Chunk, CHUNK_CAPACITY};

pub struct Buffer {
    file_name: String,
    root_chunk: Box<Chunk>,
    
    number_of_lines: usize,
    cursor_line_pos: usize,
    cursor_character_pos: usize,
}

impl Buffer {
    pub fn new(file_name: String) -> Self {
        let empty_str = ['\0'; CHUNK_CAPACITY];
        Self {
            file_name,
            root_chunk: Box::from(Chunk::new(empty_str, 0, 0)),
            number_of_lines: 0,
            cursor_line_pos: 0,
            cursor_character_pos: 0,
        }
    }
    
    pub fn get_file_name(&self) -> &str {
        &self.file_name
    }
    
    pub fn insert_char(&mut self, c: char, line: usize, pos: usize)  {
        
    }
}
