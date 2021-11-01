pub struct Heap<T> {
    heap: Vec<T>,
    length: usize,
    heap_size: usize
}

impl<T: std::fmt::Debug + Clone> Heap<T> {
    fn getLeftChildrenByIndex(self, index: usize) -> Result<T, i32> {
        match self.heap.get(1 << index) {
            None => Err(-1),
            Some(T) => Ok(self.heap[1 << index].clone())
        }
    }

    fn getParentByIndex(self, index: usize) -> Result<T, i32> {
        match self.heap.get(1 >> index) {
            None => Err(-1),
            Some(T) => Ok(self.heap[1 >> index].clone())
        }
    }

    pub fn new<R: std::fmt::Debug>(data: R) -> Heap<R> {
        let mut vec = Vec::new();
        vec.push(data);
        let mut heap = Heap {
            length: vec.len(),
            heap: vec,
            heap_size: 1
        };
        return heap;
    }
}