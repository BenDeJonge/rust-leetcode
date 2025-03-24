// https://lucassardois.medium.com/generational-indices-guide-8e3c5f7fd594

// type Generation = usize;

pub struct Key {
    pub index: usize,
    pub generation: usize,
}

enum Entry<T> {
    Free { next_free: usize },
    Occupied { value: T },
}

struct GenEntry<T> {
    entry: Entry<T>,
    generation: usize,
}

#[derive(Default)]
pub struct GenVec<T> {
    /// The container that holds the actual data.
    data: Vec<GenEntry<T>>,
    free_head: usize,
    len: usize,
}

impl<T> GenVec<T> {
    pub fn new() -> Self {
        GenVec {
            data: Vec::new(),
            free_head: 0,
            len: 0,
        }
    }

    /// Inserting data returns a `Key` object which can be used to retrieve the data at a later point.
    pub fn insert(&mut self, value: T) -> Key {
        let key = if let Some(GenEntry { entry, generation }) = self.data.get_mut(self.free_head) {
            // Update the current key.
            if let Entry::Free { next_free } = entry {
                let key = Key {
                    index: self.free_head,
                    generation: *generation,
                };
                self.free_head = *next_free;
                *entry = Entry::Occupied { value };
                key
            } else {
                panic!("corrupt free list");
            }
        } else {
            // Insert the new key.
            let generation = 0;
            let key = Key {
                index: self.data.len(),
                generation,
            };
            let entry = Entry::Occupied { value };
            let gen_entry = GenEntry { entry, generation };
            self.data.push(gen_entry);
            self.free_head = key.index + 1;
            key
        };
        self.len += 1;
        key
    }

    pub fn get(&self, key: &Key) -> Option<&T> {
        self.data
            .get(key.index)
            .and_then(|GenEntry { entry, generation }| match entry {
                Entry::Occupied { value } => {
                    if *generation == key.generation {
                        Some(value)
                    } else {
                        None
                    }
                }
                _ => None,
            })
    }

    pub fn remove(&mut self, key: &Key) -> Option<T> {
        self.data
            .get_mut(key.index)
            .and_then(|GenEntry { entry, generation }| match entry {
                Entry::Occupied { .. } => {
                    if *generation == key.generation {
                        *generation += 1;
                        let mut output = Entry::Free {
                            next_free: self.free_head,
                        };
                        std::mem::swap(entry, &mut output);
                        self.free_head = key.index;
                        self.len -= 1;
                        match output {
                            Entry::Occupied { value } => return Some(value),
                            _ => unreachable!("swapped with occupied entry"),
                        }
                    }
                    None
                }
                _ => None,
            })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
pub mod test {
    use super::GenVec;

    #[test]
    fn gen_vec() {
        let mut gv = GenVec::new();

        // Insert
        let a = gv.insert("a");
        let b = gv.insert("b");
        let c = gv.insert("c");
        assert_eq!(gv.get(&a), Some(&"a"));
        assert_eq!(gv.get(&b), Some(&"b"));
        assert_eq!(gv.get(&c), Some(&"c"));
        assert_eq!(gv.len(), 3);

        // Remove
        gv.remove(&a);
        assert_eq!(gv.get(&a), None);
        assert_eq!(gv.len(), 2);

        // Re-insert
        let d = gv.insert("d");

        assert_eq!(a.index, d.index);
        assert_ne!(a.generation, d.generation);

        // Re-remove and re-re-insert
        gv.remove(&d);
        let e = gv.insert("e");
        assert_eq!(a.index, e.index);
        assert_ne!(a.generation, e.generation);
    }
}
