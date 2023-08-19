use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::rc::Rc;

use super::appdata::AppData;

#[derive(Debug, Default)]
pub struct Comp {
    pub comp: String,
    pub code: String,
    pub freq: u64,
}

impl PartialEq for Comp {
    fn eq(&self, other: &Self) -> bool {
        self.freq.eq(&other.freq)
    }
}

impl PartialOrd for Comp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.freq.partial_cmp(&other.freq)
    }
}

impl Eq for Comp {}

impl Ord for Comp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq.cmp(&other.freq)
    }
}

#[derive(Debug, Default)]
pub struct Primary {
    heap: BinaryHeap<Rc<RefCell<Comp>>>,
    queued: VecDeque<Rc<RefCell<Comp>>>,
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
        if self.input != 0 as char && self.input != ' ' {
            self.input
        } else {
            '_'
        }
    }

    pub fn set_input(&mut self, input: char) {
        self.input = input;
    }

    pub fn build_heap(&mut self, data: &AppData) {
        for component in data.compset.values() {
            let component = component.borrow();
            self.heap.push(Rc::new(RefCell::new(Comp {
                comp: component.comp.clone(),
                code: component.code.clone(),
                freq: component.freq,
            })));
        }
    }

    pub fn get_current(&mut self, data: &AppData) -> Option<Rc<RefCell<Comp>>> {
        if self.queued.is_empty() {
            // 空隊列, 從堆中取
            if self.heap.is_empty() {
                // 空堆則建堆
                self.build_heap(data);
            }
            // 從堆中取出若幹
            for _ in 0..8 {
                if let Some(comp) = self.heap.pop() {
                    self.queued.push_back(comp);
                } else {
                    break;
                }
            }
        }
        self.queued.front().and_then(|comp| Some(comp.clone()))
    }

    pub fn next(&mut self) {
        self.queued.pop_front().and_then(|comp| {
            if self.get_elapsed() < 3000 {
                // 三秒内答出, 權重减半
                let freq = &mut comp.borrow_mut().freq;
                *freq = if *freq > 1 { *freq / 2 } else { *freq };
            }
            // 當前元素回堆, 並取出一個新元素
            self.heap.push(comp);
            self.heap.pop().and_then(|comp| {
                self.queued.push_back(comp);
                None::<()>
            });
            None::<()>
        });
    }
}
