fn sort_1<T, F: Fn(&T, &T) -> bool>(_ar: &mut [T], _comp: &F) {}

fn sort_2<T, F: Fn(&T, &T) -> bool>(ar: &mut [T], comp: &F) {
    if comp(&ar[0], &ar[1]) { ar.swap(0, 1); }
}

fn sort_3<T, F: Fn(&T, &T) -> bool>(ar: &mut [T], comp: &F) {
    if comp(&ar[0], &ar[1]) { ar.swap(0, 1); }
    if comp(&ar[1], &ar[2]) { 
        ar.swap(1, 2);
        if comp(&ar[0], &ar[1]) { ar.swap(0, 1); }
    }
    
}

fn sort_4<T, F: Fn(&T, &T) -> bool>(ar: &mut [T], comp: &F) {
    sort_3(ar, comp);
    if comp(&ar[2], &ar[3]) {
        ar.swap(2,3);
        if comp(&ar[1], &ar[2]) {
            ar.swap(1,2);
            if comp(&ar[0], &ar[1]) { ar.swap(0, 1); }
        }
    }
}

pub fn sort_if_lt_5<T, F: Fn(&T, &T) -> bool>(ar: &mut [T], comp: &F) -> bool {
    let fns: [fn(&mut [T], &F); 5]  = [sort_1, sort_1, sort_2, sort_3, sort_4];
    match ar.len() {
        0..=4 => { fns[ar.len()](ar, comp); true }
        _ => false,
    }
}

//-------------------- for tests ---------------------------
use std::iter;
use rand::Rng;

#[allow(dead_code)]
pub fn get_random_vec(size: usize) -> Vec<i64> {
    iter::repeat_with(|| rand::thread_rng().gen_range(-100..100)).take(size).collect()
}

#[allow(dead_code)]
pub fn comparer(lhs: &i64, rhs: &i64) -> bool { lhs > rhs }

#[allow(dead_code)]
pub fn get_small_sets() -> Vec<Vec<i64>> {
    vec![
        vec![], 
        vec![0],
        vec![-1, -100],
        vec![4, -1, -100],
        vec![6, 0, -100, -1000],
        vec![7, 2, 5, 78, -11],
        vec![5, 78, -1, 5, 22, 42],
        vec![-100, -99, -98, -97, -96, -95, -94],
        vec![100, 99, 98, 97, 96, 95, 94],
        vec![6, 1, -2, -100, 45, 12, 90, -22, -1, 4, 9, 23, 3]
    ]
}

#[allow(dead_code)]
pub static LONG_SETS_SIZES: [usize; 12] = [11, 13, 20, 25, 42, 78, 100, 151, 253, 421, 526, 1000];