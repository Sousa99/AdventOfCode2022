
use std::collections::{VecDeque, HashSet};

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================

type BufferElem = char;
type Buffer = Vec<char>;

struct BufferWindow<'a> {
    capacity: usize,
    vector: VecDeque<&'a BufferElem>,
}

pub struct Device {
    start_transmission_marker_size: usize,
    start_message_marker_size: usize,
    buffer: Buffer,
}

// =============================================== AUXILIARY FUNCTIONS ===============================================



// ================================================= IMPLEMENTATIONS =================================================

impl<'a> BufferWindow<'a> {

    fn new(capacity: usize) -> BufferWindow<'a> {
        BufferWindow {
            capacity,
            vector: VecDeque::with_capacity(capacity)
        }
    }

    fn push_elem(&mut self, elem: &'a BufferElem) {

        self.vector.push_back(elem);
        while self.vector.len() > self.capacity {
            self.vector.pop_front();
        }
    }

    fn develop_set(&self) -> HashSet<&'a BufferElem> {

        return self.vector.iter()
            .map(|elem| *elem)
            .collect();
    }
}

impl Device {

    pub fn new(buffer: Buffer) -> Device {
        Device {
            start_transmission_marker_size: 4,
            start_message_marker_size: 14,
            buffer
        }
    }

    fn find_marker(&self, size: usize) -> Option<usize> {

        let mut buffer_window: BufferWindow = BufferWindow::new(size);
        for (buffer_index, buffer_elem) in self.buffer.iter().enumerate() {

            buffer_window.push_elem(&buffer_elem);
            if buffer_window.develop_set().len() == size {
                return Some(buffer_index + 1);
            }

        }

        return None;
    }

    pub fn find_marker_start_transmission(&self) -> Option<usize> {
        return self.find_marker(self.start_transmission_marker_size);        
    }

    pub fn find_marker_start_message(&self) -> Option<usize> {
        return self.find_marker(self.start_message_marker_size);        
    }
}