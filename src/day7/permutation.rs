// A custom iterator for permutations with replacement
pub(super) struct PermutationsWithReplacement<I: Iterator> {
    data: Vec<I::Item>,
    indices: Vec<usize>,
    first: bool,
}

impl<I> PermutationsWithReplacement<I>
where
    I: Iterator,
    I::Item: Clone,
{
    pub(super) fn new(iter: I, length: usize) -> Self {
        let data: Vec<_> = iter.collect();
        let indices = vec![0; length];
        let first = true;
        PermutationsWithReplacement {
            data,
            indices,
            first,
        }
    }
}

impl<I> Iterator for PermutationsWithReplacement<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.indices.iter().map(|&i| self.data[i].clone()).collect());
        }

        let mut i = self.indices.len();
        loop {
            i = i.checked_sub(1)?;
            if self.indices[i] + 1 < self.data.len() {
                self.indices[i] += 1;
                for j in i + 1..self.indices.len() {
                    self.indices[j] = 0;
                }
                return Some(self.indices.iter().map(|&i| self.data[i].clone()).collect());
            }
            if i == 0 {
                return None;
            }
        }
    }
}
