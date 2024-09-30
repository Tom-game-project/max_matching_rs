pub fn matching_permutations(arr: Vec<usize>, depth: usize) -> Vec<Vec<usize>> {
    if arr.len() == 1 {
        return vec![arr];
    }
    let mut rlist = Vec::new();

    for i in 0..arr.len() {
        let current = arr[i];
        let remaining = [&arr[..i], &arr[i + 1..]].concat();

        for p in matching_permutations(remaining, depth + 1) {
            rlist.push([vec![current], p].concat());
        }
    }
    rlist
}

pub struct Permutations<T> {
    data: Vec<T>,
    c: Vec<usize>,
    i: usize,
}

impl<T: Clone> Permutations<T> {
    pub fn new(data: Vec<T>) -> Self {
        let n = data.len();
        Permutations {
            c: vec![0; n],
            data,
            i: 0,
        }
    }
}

impl<T: Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            self.i += 1;
            return Some(self.data.clone());
        }

        while self.i < self.data.len() {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.data.swap(0, self.i);
                } else {
                    self.data.swap(self.c[self.i], self.i);
                }
                self.c[self.i] += 1;
                self.i = 1;
                return Some(self.data.clone());
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }
        None
    }
}
