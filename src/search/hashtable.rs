pub type HashType = u64;

pub struct Hashtable<T: Copy + Default + PartialEq> {
    entries: Vec<T>,
}

impl<T: Copy + Default + PartialEq> Hashtable<T> {
    #[must_use]
    pub fn new(megabytes: usize) -> Self {
        let mut tt = Hashtable { entries: vec![] };
        tt.resize(megabytes);
        tt
    }

    #[must_use]
    pub fn poll(&self, key: HashType) -> T {
        let idx = self.get_idx(key);
        self.entries[idx]
    }

    pub fn add(&mut self, key: HashType, entry: &T) {
        let idx = self.get_idx(key);
        self.entries[idx] = *entry;
    }

    #[must_use]
    pub fn hashfull(&self) -> Option<i32> {
        let size = std::cmp::min(self.entries.len(), 1000);

        if size == 0 {
            return None;
        }

        let mut filled = 0;
        for i in 0..size {
            filled += (self.entries[i] != T::default()) as i32;
        }

        Some(filled)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn resize(&mut self, megabytes: usize) {
        let num_entries = (megabytes as usize * 1024 * 1024) / std::mem::size_of::<T>();
        self.entries.resize(num_entries, T::default());
    }

    pub fn clear(&mut self) {
        self.entries.fill(T::default());
    }

    fn get_idx(&self, key: HashType) -> usize {
        key as usize % self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Hashtable;

    #[test]
    fn basic() {
        let mut tt = Hashtable::<i32>::new(1);

        for i in 0..tt.len() {
            let key = i as u64;
            assert_eq!(tt.poll(key), 0);
        }

        for i in 0..4 * tt.len() {
            let key = i as u64;

            tt.add(key, &1);
            assert_eq!(tt.poll(key), 1);

            tt.add(key, &2);
            assert_eq!(tt.poll(key), 2);
        }
    }
}
