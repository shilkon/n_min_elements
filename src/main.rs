use std::collections::BinaryHeap;

fn n_min<T, D>(mut it: T, n: usize) -> impl Iterator<Item = D> where
    T: Iterator<Item = D> + Clone,
    D: Ord {
    let mut heap = BinaryHeap::with_capacity(n);
    for i in 0..n {
        heap.push(match it.next() {
            Some(x) => x,
            None => break
        });
    }
    it.for_each(|x| {
        if x < *heap.peek().unwrap() {
            heap.pop();
            heap.push(x);
        }
    });
    heap.into_iter()
}

fn main() {
    let mut vec = vec![11, 43, 32, 3, 89, 12, -15, 21];
    //let mut vec = vec![11, 43, 32];
    for i in n_min(vec.iter(), 3) {
        print!("{i} ")
    }
    vec.push(1);
}
