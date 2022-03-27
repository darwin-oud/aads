use super::aux::*;

/// Sorts slice `ar` using merge sort algorithm. This function is *O*(*n* \* log*n*) worst-case.
/// Requires temporary buffer with greater or equals size as main buffer
#[allow(dead_code)]
pub fn merge<T: Copy, F: Fn(&T, &T) -> bool>(ar: &mut [T], temp: &mut[T], comp: &F) {
    if sort_if_lt_5(ar, comp) { return; }
    let mid = ar.len() / 2;

    merge(&mut ar[0..mid], &mut temp[0..mid], comp);
    merge(&mut ar[mid..], &mut temp[mid..], comp);

    let mut idx_1 = 0;
    let mut idx_2 = mid;
    let mut idx_temp = 0;
    // merge two parts of initial slice in temp slice
    while idx_1 < mid && idx_2 < ar.len() {
        if comp(&ar[idx_1], &ar[idx_2]) {
            temp[idx_temp] = ar[idx_2];
            idx_2 += 1;
        } else {
            temp[idx_temp] = ar[idx_1];
            idx_1 += 1;
        }
        idx_temp += 1;
    }
    // copy rest from parts if exists
    if idx_1 >= mid {
        ar[idx_2..].iter_mut().zip(temp[idx_temp..].iter_mut()).map(|(x, y)| *y = *x).count();
    } else if idx_2 >= ar.len() {
        ar[idx_1..mid].iter_mut().zip(temp[idx_temp..].iter_mut()).map(|(x, y)| *y = *x).count();
    }
    // copy back to initial slice
    ar.iter_mut().zip(temp.iter_mut()).map(|(x, y)| *x = *y).count();
}

//----------------------------------- tests -----------------------------------------
#[test]
fn small_sets() {
    let mut sets = get_small_sets();
    for (i, v) in sets.iter_mut().enumerate() {
        let mut temp = vec![0; v.len()];
        let mut v_sorted = v.clone();
        v_sorted.sort();
        merge(v, &mut temp, &comparer);
        assert_eq!(*v, v_sorted, "heap sort failed with small set idx: {}.", i);
    }
}

#[test]
fn large_sets() {
    for sz in LONG_SETS_SIZES {
        let mut temp = vec![0; sz];
        let mut v = get_random_vec(sz);
        let mut v_sorted = v.clone();
        v_sorted.sort();
        merge(&mut v, &mut temp, &comparer);
        assert_eq!(v, v_sorted, "heap sort failed with large set size: {}.", sz);
    }
}