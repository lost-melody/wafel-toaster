use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::rc::Rc;

use super::appdata::AppData;
use super::appdata::Comp;

#[derive(Debug, Default)]
pub struct Primary {
    heap: BinaryHeap<Rc<RefCell<Comp>>>,
    current: Option<Rc<RefCell<Comp>>>,
    input: char,
    elapsed: u64,
}

impl Primary {
    pub fn tick(&mut self, elapsed: u64) {
        self.elapsed += elapsed;
    }

    pub fn get_elapsed(&self) -> u64 {
        self.elapsed
    }

    pub fn reset_elapsed(&mut self) {
        self.elapsed = 0;
    }

    pub fn get_input(&mut self) -> char {
        self.input
    }

    pub fn set_input(&mut self, input: char) {
        self.input = input;
    }

    pub fn build_heap(&mut self, data: &AppData) {
        for component in data.compset.values() {
            self.heap.push(component.clone());
        }
    }

    pub fn get_current(&mut self, data: &AppData) -> Option<Rc<RefCell<Comp>>> {
        if let Some(ref current) = self.current {
            Some(current.clone())
        } else {
            if self.heap.is_empty() {
                self.build_heap(data);
                self.current = self.heap.pop();
            }
            self.current.clone()
        }
    }

    pub fn next(&mut self) {
        if let Some(ref current) = self.current {
            {
                let freq = &mut current.borrow_mut().freq;
                if *freq > 1 {
                    *freq -= *freq >> 2;
                }
            }
            self.heap.push(current.clone());
            self.current = self.heap.pop();
        }
    }
}
