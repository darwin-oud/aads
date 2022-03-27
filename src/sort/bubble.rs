use super::aux::*;

/// Sorts slice `ar` using bubble sort algorithm. This function is *O*(*n*^2) worst-case.
#[allow(dead_code)]
pub fn bubble<T, F: Fn(&T, &T) -> bool>(ar: &mut [T], comp: &F) {
    if sort_if_lt_5(ar, comp) { return; }
    let mut l = 0;
    let mut r = ar.len() - 1;
    while l < r {
        for i in l+1..=r {
            if comp(&ar[i-1], &ar[i]) {
                ar.swap(i-1, i);
            }
        }
        r -= 1;
        for i in (l+1..=r).rev() {
            if comp(&ar[i-1], &ar[i]) {
                ar.swap(i-1, i);
            } 
        }
        l += 1;
    }
}

//----------------------------------- tests -----------------------------------------
#[test]
fn small_sets() {
    let mut sets = get_small_sets();
    for (i, v) in sets.iter_mut().enumerate() {
        let mut v_sorted = v.clone();
        v_sorted.sort();
        bubble(v, &comparer);
        assert_eq!(*v, v_sorted, "bubble sort failed with small set idx: {}.", i);
    }
}

#[test]
fn large_sets() {
    for sz in LONG_SETS_SIZES {
        let mut v = get_random_vec(sz);
        let mut v_sorted = v.clone();
        v_sorted.sort();
        bubble(&mut v, &comparer);
        assert_eq!(v, v_sorted, "bubble sort failed with large set size: {}.", sz);
    }
}