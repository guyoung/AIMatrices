use std::fmt::Debug;

#[derive(Default, Clone)]
pub struct ReuseList<T> {
    items: Vec<Option<T>>,
    slots: Vec<usize>,
    last_slot_idx: usize,
    len: usize,
    slot_size: usize,
}

impl<T: Debug> Debug for ReuseList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReuseList")
            .field("items", &self.items)
            .field("slots", &self.slots)
            .finish()
    }
}

impl<T> ReuseList<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    //is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    //create a with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            slots: Vec::with_capacity(capacity >> 2),
            last_slot_idx: 0,
            len: 0,
            slot_size: 0,
        }
    }

    pub fn append(&mut self, item: T) -> usize {
        if self.slot_size > 0 {
            //reuse empty slot if valid
            let slot = self.slots[self.last_slot_idx - 1];
            if slot > 0 {
                self.items[slot - 1] = Some(item);
                self.slots[self.last_slot_idx - 1] = 0;
                if self.last_slot_idx > 1 {
                    self.last_slot_idx -= 1;
                }

                self.len += 1;
                return slot - 1;
            }
        }
        //no valid empty slots, append to end
        self.items.push(Some(item));
        self.len += 1;
        self.items.len() - 1
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.items.len() {
            return None;
        }

        let item = self.items[index].take();
        if item.is_some() {
            if self.slot_size > 0 && self.slots[self.last_slot_idx - 1] == 0 {
                self.slots[self.last_slot_idx - 1] = index + 1;
            } else {
                self.slots.push(index + 1);
                self.last_slot_idx += 1;
                self.slot_size += 1;
            }
            self.len -= 1;
        }
        item
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.items.len() {
            None
        } else {
            self.items[index].as_ref()
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.items.len() {
            None
        } else {
            self.items[index].as_mut()
        }
    }

    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter().filter_map(|x| x.as_ref())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items.iter_mut().filter_map(|x| x.as_mut())
    }

    //implement clear
    pub fn clear(&mut self) {
        self.items.clear();
        self.slots.clear();
        self.last_slot_idx = 0;
        self.len = 0;
        self.slot_size = 0;
    }

    pub fn optimize(&mut self) {
        let mut new_items = Vec::with_capacity(self.len);

        for item in self.items.iter_mut() {
            let a = item.take();
            if a.is_some() {
                new_items.push(a);
            }
        }
        self.items = new_items;
        self.slots.clear();
        self.last_slot_idx = 0;
        self.slot_size = 0;
    }
}
