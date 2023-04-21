pub fn sort<T: PartialOrd>(array: &mut Vec<T>) {
    loop {
        let mut swapped = false;
        for i in 0..array.len() - 1 {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
