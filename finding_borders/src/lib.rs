

use std::collections::HashSet;


// ALGORITHM 1 : prefix function (KMP failure links)
pub fn borders_prefix_function(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    if n == 0 {
        return Vec::new();
    }
    let mut pi = vec![0usize; n];
    for i in 1..n {
        let mut j = pi[i - 1];
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        pi[i] = j;
    }
    let mut res = Vec::new();
    let mut k = pi[n - 1];
    while k > 0 {
        res.push(k);
        k = pi[k - 1];
    }
    res.reverse();
    res
}


// ALGORITHM 2 : rolling hash + HashSet (Membership Structures)
pub fn borders_hash_membership(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    if n == 0 {
        return Vec::new();
    }

    const B1: u64 = 131;
    const M1: u64 = 1_000_000_007;
    const B2: u64 = 137;
    const M2: u64 = 998_244_353;

    let mut pref1 = vec![0u64; n + 1];
    let mut pref2 = vec![0u64; n + 1];
    let mut pow1 = vec![1u64; n + 1];
    let mut pow2 = vec![1u64; n + 1];

    for i in 0..n {
        pref1[i + 1] = (pref1[i] * B1 + s[i] as u64) % M1;
        pref2[i + 1] = (pref2[i] * B2 + s[i] as u64) % M2;
        pow1[i + 1] = (pow1[i] * B1) % M1;
        pow2[i + 1] = (pow2[i] * B2) % M2;
    }

    let sub1 = |l: usize, r: usize| -> u64 {
        (pref1[r] + M1 - (pref1[l] * pow1[r - l]) % M1) % M1
    };
    let sub2 = |l: usize, r: usize| -> u64 {
        (pref2[r] + M2 - (pref2[l] * pow2[r - l]) % M2) % M2
    };

    let mut prefixes: HashSet<(usize, u64, u64)> = HashSet::with_capacity(n);
    for len in 1..n {
        prefixes.insert((len, sub1(0, len), sub2(0, len)));
    }

    let mut res = Vec::new();
    for len in 1..n {
        let key = (len, sub1(n - len, n), sub2(n - len, n));
        if prefixes.contains(&key) {
            res.push(len);
        }
    }
    res
}
