use super::aux::*;

/// Sorts slice `ar` using bubble sort algorithm. This function is *O*(*n*^2) worst-case.
#[allow(dead_code)]
pub fn insertion<T, F: Fn(&T, &T) -> bool>(ar: &mut [T], comp: &F) {
    if sort_if_lt_5(ar, comp) { return; }
    for i in 1..ar.len() {
        let mut j = i;
        while j > 0 && comp(&ar[j-1], &ar[j]) {
            ar.swap(j-1, j);
            j -= 1;
        }
    }
}

//----------------------------------- tests -----------------------------------------
#[test]
fn small_sets() {
    let mut sets = get_small_sets();
    for (i, v) in sets.iter_mut().enumerate() {
        let mut v_sorted = v.clone();
        v_sorted.sort();
        insertion(v, &comparer);
        assert_eq!(*v, v_sorted, "insertion sort failed with small set idx: {}.", i);
    }
}

#[test]
fn large_sets() {
    for sz in LONG_SETS_SIZES {
        let mut v = get_random_vec(sz);
        let mut v_sorted = v.clone();
        v_sorted.sort();
        insertion(&mut v, &comparer);
        assert_eq!(v, v_sorted, "insertion sort failed with large set size: {}.", sz);
    }
}