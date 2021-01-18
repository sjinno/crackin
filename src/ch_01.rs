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

// 1.5 One Away
pub fn is_one_away(s1: &str, s2: &str) -> bool {
    let (l1, l2) = (s1.len(), s2.len());
    let rem = l1.rem_euclid(l2);

    // If the remainder of (l1 % l2) is not 1,
    // l1, or 0, then return false.
    match rem {
        r if r == 1 || r == l1 || r == 0 => {}
        _ => return false,
    }

    let mut map = HashMap::<char, isize>::new();
    // Count the number of each character in s1 and
    // store it in hash map/hash table.
    s1.chars()
        .into_iter()
        .for_each(|c| *map.entry(c).or_insert(0) += 1);
    println!("BEFORE: {:#?}", map);

    // Iterate through s2 and if char exists in hashmap,
    // then subtract 1 from the existing count.
    // If not, increase count by 1.
    let mut count = 0;
    for ch in s2.chars() {
        if let Some(c) = map.get_mut(&ch) {
            *c -= 1;
        } else {
            // *map.entry(ch).or_insert(0) += 1;
            count += 1;
        }
    }
    println!("AFTER: {:#?}", map);
    println!("===============");

    // let mut count = 0;
    for val in map.values() {
        count += val.abs();
    }

    if rem == 1 || rem == l1 {
        return count == 1;
    } else {
        return count == 2;
    }
}
