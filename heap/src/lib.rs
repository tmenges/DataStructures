pub fn heapify<T>(heap: &mut [T])
where
    T: std::cmp::PartialOrd + Copy,
{
    let n = heap.len() - 1;

    let mut i = n / 2;
    while i > 0 {
        let mut k = i;
        let v = heap[i];
        let mut is_heap = false;

        while !is_heap && 2 * k <= n {
            let mut j = 2 * k;
            if j < n && heap[j] < heap[j + 1] {
                j += 1;
            }
            if v >= heap[j] {
                is_heap = true;
            } else {
                heap[k] = heap[j];
                k = j;
            }
        }
        heap[k] = v;
        i -= 1;
    }
}

pub fn make_heap<T>(source: &[T]) -> Vec<T>
where
    T: std::cmp::PartialOrd + Copy,
{
    let mut heap = source.to_vec();

    heapify(&mut heap);

    heap
}

pub fn remove<T>(heap: &mut Vec<T>) -> T
where
    T: std::cmp::PartialOrd + Copy,
{
    let result = heap[1];

    if let Some(val) = heap.pop() {
        if heap.len() > 1 {
            heap[1] = val;
        }
    }
    heapify(heap);

    result
}

pub fn sort<T>(heap: &mut Vec<T>)
where
    T: std::cmp::PartialOrd + Copy,
{
    let mut heap_size = heap.len() - 1;
    let mut n = heap_size - 1;

    while n > 0 {
        heap.swap(1, heap_size);
        heap_size -= 1;
        n -= 1;

        heapify(&mut heap[0..heap_size]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut data = vec![0, 2, 9, 7, 6, 5, 8];
        let answer = vec![0, 9, 6, 8, 2, 5, 7];
        heapify(&mut data);
        assert_eq!(data, answer);
    }

    #[test]
    fn test_delete() {
        let mut data = vec![0, 2, 9, 7, 6, 5, 8];
        let answer = [0, 9, 8, 7, 6, 5, 2];
        heapify(&mut data);

        let mut a = 1;
        while data.len() > 1 {
            println!("data.len() = {}, a = {}", data.len(), a);
            let top = remove(&mut data);

            println!("top = {}, answer[a] = {}", top, answer[a]);
            assert_eq!(top, answer[a]);
            a += 1;
        }
    }

    #[test]
    fn test_sort() {
        let mut data = vec![0, 2, 9, 7, 6, 5, 8];
        heapify(&mut data);

        let answer = vec![0, 2, 5, 6, 7, 8, 9];
        sort(&mut data);

        assert_eq!(data, answer);
    }
}
