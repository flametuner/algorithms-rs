use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub fn sort<T: PartialOrd + Clone + Send + Copy + Debug + 'static>(array: &mut Vec<T>) {
    let mut tmp = array.clone();
    let counter = Arc::new(AtomicUsize::new(0));
    split_merge(array, 0, array.len(), &mut tmp, Some(counter));
}

fn split_merge<T: PartialOrd + Clone + Send + Copy + Debug + 'static>(
    array: &mut Vec<T>,
    begin: usize,
    end: usize,
    tmp: &mut Vec<T>,
    counter: Option<Arc<AtomicUsize>>,
) {
    if end - begin < 2 {
        return;
    }

    let middle = (end + begin) / 2;

    let mut handle = None;

    if let Some(counter) = &counter {
        if counter.load(Ordering::SeqCst) < 1 {
            // if counter.load(Ordering::SeqCst) < num_cpus::get() {
            counter.fetch_add(1, Ordering::SeqCst);
            let mut split = array.drain(middle..end).collect::<Vec<_>>();
            let mut tmp = tmp.drain(middle..end).collect::<Vec<_>>();

            let counter = counter.clone();

            handle = Some(thread::spawn(move || {
                println!("New thread");
                let length = split.len();
                split_merge(&mut tmp, 0, length, &mut split, Some(counter));
                return (split, tmp);
            }));
        }
    }

    if let Some(handle) = handle {
        split_merge(tmp, begin, middle, array, counter);
        let (split, tmp_a) = handle.join().unwrap();
        array.extend(split);
        tmp.extend(tmp_a);
    } else {
        split_merge(tmp, begin, middle, array, None);
        split_merge(tmp, middle, end, array, None);
    }

    merge(array, tmp, begin, middle, end);
}

fn merge<T: PartialOrd + Clone + Copy + Debug>(
    array: &mut Vec<T>,
    tmp: &mut Vec<T>,
    begin: usize,
    middle: usize,
    end: usize,
) {
    let mut i = begin;
    let mut j = middle;
    for k in begin..end {
        if i < middle && (j >= end || tmp[i] <= tmp[j]) {
            array[k] = tmp[i].clone();
            i += 1;
        } else {
            array[k] = tmp[j].clone();
            j += 1;
        }
    }
}
