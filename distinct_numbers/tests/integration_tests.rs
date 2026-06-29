use distinct_numbers::{count_distinct_avl, count_distinct_hashset};

// the problem's own example: [2 3 2 2 3] has 2 distinct values
#[test]
fn example_case() {
    let v = vec![2, 3, 2, 2, 3];
    assert_eq!(count_distinct_hashset(&v), 2);
    assert_eq!(count_distinct_avl(&v), 2);
}

// a single value is trivially 1 distinct
#[test]
fn single_value() {
    let v = vec![7];
    assert_eq!(count_distinct_hashset(&v), 1);
    assert_eq!(count_distinct_avl(&v), 1);
}

// everything the same -> still just 1 distinct
#[test]
fn all_same() {
    let v = vec![5; 1000];
    assert_eq!(count_distinct_hashset(&v), 1);
    assert_eq!(count_distinct_avl(&v), 1);
}

// everything different -> distinct count equals the length
#[test]
fn all_distinct() {
    let v: Vec<i64> = (1..=1000).collect();
    assert_eq!(count_distinct_hashset(&v), 1000);
    assert_eq!(count_distinct_avl(&v), 1000);
}

// already-sorted input would wreck an UNbalanced bst (it degrades to a line),
// the AVL has to stay fast and correct here
#[test]
fn sorted_input_stays_balanced() {
    let v: Vec<i64> = (1..=200_000).collect();
    assert_eq!(count_distinct_avl(&v), 200_000);
    assert_eq!(count_distinct_hashset(&v), 200_000);
}

// the big values the constraints allow (up to 1e9)
#[test]
fn large_values() {
    let v = vec![1_000_000_000, 1, 1_000_000_000, 999_999_999];
    assert_eq!(count_distinct_hashset(&v), 3);
    assert_eq!(count_distinct_avl(&v), 3);
}

// the real safety net: both algorithms must always agree on random data
#[test]
fn both_algorithms_agree() {
    let mut state: u64 = 0xdead_beef_cafe_1234;
    let mut v: Vec<i64> = Vec::with_capacity(50_000);
    for _ in 0..50_000 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        // small modulus on purpose so we force lots of duplicates
        v.push((state % 5_000) as i64 + 1);
    }
    assert_eq!(count_distinct_hashset(&v), count_distinct_avl(&v));
}
