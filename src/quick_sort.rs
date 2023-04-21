pub fn sort<T: PartialOrd>(array: &mut Vec<T>) {
    quick_sort(array, 0, (array.len() - 1) as isize);
}

fn quick_sort<T: PartialOrd>(array: &mut Vec<T>, lo: isize, hi: isize) {
    if lo >= hi {
        return;
    }
    let p = partition(array, lo, hi);
    quick_sort(array, lo, p - 1);
    quick_sort(array, p + 1, hi);
}

fn partition<T: PartialOrd>(array: &mut Vec<T>, lo: isize, hi: isize) -> isize {
    let pivot = hi;
    let mut i = lo;
    for j in lo..hi {
        if array[j as usize] < array[pivot as usize] {
            array.swap(i as usize, j as usize);
            i += 1;
        }
    }
    array.swap(i as usize, hi as usize);
    i
}
