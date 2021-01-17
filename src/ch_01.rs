use std::collections::HashMap;

// 1.4 Palindrome Permutation
pub fn palindfrome_permutation(s: &str) -> bool {
    if s.len() < 3 {
        return true;
    }

    let mut map = HashMap::<char, usize>::new();
    s.chars()
        .into_iter()
        .for_each(|c| *map.entry(c).or_insert(0) += 1);
    // println!("{:?}", map);

    let mut count = 0;
    for (_key, val) in map {
        if val % 2 == 1 {
            count += 1;
        }
        if count > 1 {
            return false;
        }
    }

    true
}
