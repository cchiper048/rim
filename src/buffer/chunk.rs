pub const CHUNK_CAPACITY: usize = 4;

pub struct Chunk {
    data: [char; CHUNK_CAPACITY],
    line_from: usize,
    line_to: usize,
    last_line_length: usize,
    length: usize,
    
    left_chunk: Option<Box<Chunk>>,
    right_chunk: Option<Box<Chunk>>,
    height: usize,
}

#[derive(Debug)]
pub enum ChunkEditError {
    Full,
    OutOfBounds,
}

#[derive(Debug)]
pub enum FindChunkError {
    ChunkNotFound,
}

impl Chunk {
    pub fn new(data: [char; CHUNK_CAPACITY], line_from: usize, line_to: usize) -> Self {
        Self {
            data,
            line_from,
            line_to,
            last_line_length: 0,
            length: 0,

            left_chunk: None,
            right_chunk: None,
            height: 1,
        }
    }

    pub fn insert_char(&mut self, c: char, index: usize) -> Result<(), ChunkEditError> {
        if self.length == CHUNK_CAPACITY {
            return Err(ChunkEditError::Full);
        }
        if index > self.length {
            return Err(ChunkEditError::OutOfBounds);
        }

        self.data.copy_within(index..self.length, index + 1);
        self.data[index] = c;
        self.length += 1;

        if c == '\n' {
            self.line_to += 1;
        }

        Ok(())
    }

    pub fn remove_char(&mut self, index: usize) -> Result<char, ChunkEditError> {
        if index > self.length {
            return Err(ChunkEditError::OutOfBounds);
        }

        if self.data[index] == '\n' {
            self.line_to -= 1;
        }

        let removed_char = self.data[index];
        self.data.copy_within(index+1..self.length, index);
        self.length -= 1;
        self.data[self.length] = '\0';

        Ok(removed_char)
    }

    pub fn get_data(&self) -> String {
        self.data.iter().collect::<String>()
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_balance_factor(&self) -> isize {
        let left_height= self.left_chunk.as_ref().map(|c| c.get_height()).unwrap_or(0) as isize;
        let right_height = self.right_chunk.as_ref().map(|c| c.get_height()).unwrap_or(0) as isize;

        left_height - right_height
    }

    pub fn change_line_to(&mut self, delta: isize) {
        self.line_to += delta as usize;
    }

    pub fn change_line_from(&mut self, delta: isize) {
        self.line_from += delta as usize;
    }

    pub fn find_chunk_by_line(&mut self, s_line: usize) -> Result<&mut Chunk, FindChunkError> {
        let mut curr_chunk = self;

        while curr_chunk.line_from != s_line {
            if curr_chunk.line_from < s_line {
                if let Some(left) = curr_chunk.left_chunk.as_deref_mut() {
                    curr_chunk = left;
                    continue;
                } else {
                    return Err(FindChunkError::ChunkNotFound);
                }
            }

            if let Some(right) = curr_chunk.right_chunk.as_deref_mut() {
                curr_chunk = right;
                continue;
            } else {
                return Err(FindChunkError::ChunkNotFound);
            }
        }

        Ok(curr_chunk)
    }
}
