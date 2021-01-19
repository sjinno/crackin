use std::collections::HashMap;

// 1.4 Palindrome Permutation
#[allow(dead_code)]
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
#[allow(dead_code)]
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
        count == 1
    } else {
        count == 2
    }
}

// 1.6 Compressed String
#[allow(dead_code)]
pub fn compress_string(s: &str) -> String {
    use std::char;
    let length = s.len();
    let mut compressed_string = Vec::<(char, usize)>::with_capacity(length);
    let mut idx = 0;
    // let mut count = 1;
    let mut iter = s.chars();
    compressed_string.push((iter.next().unwrap(), 1));
    // while let Some(c) = iter.next() {
    //     if c == compressed_string[idx].0 {
    //         compressed_string[idx].1 += 1;
    //     } else {
    //         compressed_string.push((c, 1));
    //         idx += 1;
    //     }
    // }
    for c in iter {
        if c == compressed_string[idx].0 {
            compressed_string[idx].1 += 1;
        } else {
            compressed_string.push((c, 1));
            idx += 1;
        }
    }

    if length == compressed_string.len() * 2 {
        return s.to_string();
    }

    // println!("{:?}", compressed_string);
    compressed_string
        .iter()
        .map(|(c, n)| format!("{}{}", c, n))
        .collect::<String>()
}

// 1.7 Rotate Matrix
#[allow(dead_code)]
pub fn rotate_matrix(mut mtx: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let size = mtx.len() - 1;
    let (mut low, mut up) = (0, size);
    while low < up {
        let (mut a, mut b) = (low, up);

        while a < up && b > low {
            let tmp1 = mtx[a][up];
            let tmp2 = mtx[up][b];
            let tmp3 = mtx[b][low];
            mtx[a][up] = mtx[low][a];
            mtx[up][b] = tmp1;
            mtx[b][low] = tmp2;
            mtx[low][a] = tmp3;
            a += 1;
            b -= 1;
        }

        low += 1;
        up -= 1;
    }

    mtx
}
