use super::aux::*;

/// Sorts slice `ar` using heap sort algorithm. This function is *O*(*n* \* log*n*) worst-case.
#[allow(dead_code)]
pub fn heap<T, F: Fn(&T, &T) -> bool>(ar: &mut [T], comp: &F) {
    if sort_if_lt_5(ar, comp) { return; }
    // Sift down the root of binary search tree
    let sift_down = |ar: &mut [T]| {
        let mut i = 0;
        while i < ar.len() {
            let l = 2 * i + 1;
            let mut r = 2 * i + 2;
            if l >= ar.len() { break; }
            if r >= ar.len() { r = i; } 
            if comp(&ar[l], &ar[i]) || comp(&ar[r], &ar[i]) {
                let swap_idx = if comp(&ar[l], &ar[r]) { l } else { r };
                ar.swap(i, swap_idx);
                i = swap_idx;
            } else {
                break;
            }
        }
    };
    // Sift up the last element in binary search tree
    let sift_up = |ar: &mut[T]| {
        let mut i = ar.len() - 1;
        while i > 0 {
            let parent = (i - 1) / 2;
            if comp(&ar[i], &ar[parent]) {
                ar.swap(parent, i);
                i = parent;
            } else {
                break;
            }
        }
    };
    // Build binary search tree
    for i in 1..ar.len() {
        sift_up(&mut ar[0..i+1]);
    }
    // Sort elements
    for i in (1..ar.len()).rev() {
        ar.swap(0, i);
        sift_down(&mut ar[0..i]);
    }
}

//----------------------------------- tests -----------------------------------------
#[test]
fn small_sets() {
    let mut sets = get_small_sets();
    for (i, v) in sets.iter_mut().enumerate() {
        let mut v_sorted = v.clone();
        v_sorted.sort();
        heap(v, &comparer);
        assert_eq!(*v, v_sorted, "heap sort failed with small set idx: {}.", i);
    }
}

#[test]
fn large_sets() {
    for sz in LONG_SETS_SIZES {
        let mut v = get_random_vec(sz);
        let mut v_sorted = v.clone();
        v_sorted.sort();
        heap(&mut v, &comparer);
        assert_eq!(v, v_sorted, "heap sort failed with large set size: {}.", sz);
    }
}