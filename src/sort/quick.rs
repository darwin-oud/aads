use super::aux::*;

/// Sorts slice `ar` using quick sort algorithm. This function is *O*(*n* \* log*n*) worst-case.
#[allow(dead_code)]
pub fn quick<T, F: Fn(&T, &T) -> bool>(ar: &mut [T], comp: &F) {
    if sort_if_lt_5(ar, comp) { return; }
    let mut l = 1;
    let mut r = ar.len()-1;
    let pivot = 0;

    while l < r {
        while l < ar.len() && comp(&ar[pivot], &ar[l]) { l += 1; }
        while r > 1 && comp(&ar[r], &ar[pivot]) { r-= 1; }
        if l < r { 
            ar.swap(l, r); 
            l += 1;
            r -= 1;
        }
    }
    for i in 1..l {
        ar.swap(i-1, i);
    }
    if r > 0 { quick(&mut ar[0..=r], comp); }
    if l < ar.len() - 1 { quick(&mut ar[l..], comp); }
}

//----------------------------------- tests -----------------------------------------
#[test]
fn small_sets() {
    let mut sets = get_small_sets();
    for (i, v) in sets.iter_mut().enumerate() {
        let mut v_sorted = v.clone();
        v_sorted.sort();
        quick(v, &comparer);
        assert_eq!(*v, v_sorted, "quick sort failed with small set idx: {}.", i);
    }
}

#[test]
fn large_sets() {
    for sz in LONG_SETS_SIZES {
        let mut v = get_random_vec(sz);
        let mut v_sorted = v.clone();
        v_sorted.sort();
        quick(&mut v, &comparer);
        assert_eq!(v, v_sorted, "quick sort failed with large set size: {}.", sz);
    }
}