pub fn sort<T: PartialOrd + Clone>(array: &mut Vec<T>) {
    let mut array_b = array.clone();
    split_merge(&mut array_b, 0, array.len(), array);
}

fn split_merge<T: PartialOrd + Clone>(
    array_b: &mut Vec<T>,
    begin: usize,
    end: usize,
    array_a: &mut Vec<T>,
) {
    if end - begin < 2 {
        return;
    }
    let middle = (end + begin) / 2;
    split_merge(array_a, begin, middle, array_b);
    split_merge(array_a, middle, end, array_b);
    merge(array_b, begin, middle, end, array_a);
}

fn merge<T: PartialOrd + Clone>(
    array_a: &mut Vec<T>,
    begin: usize,
    middle: usize,
    end: usize,
    array_b: &mut Vec<T>,
) {
    let mut i = begin;
    let mut j = middle;
    for k in begin..end {
        if i < middle && (j >= end || array_a[i] <= array_a[j]) {
            array_b[k] = array_a[i].clone();
            i += 1;
        } else {
            array_b[k] = array_a[j].clone();
            j += 1;
        }
    }
}
