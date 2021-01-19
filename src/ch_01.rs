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

// 1.8 Zero Matrix: Write an algorithm such that
// if an element in an MxM matrix is 0,
// its entire row and column are set to 0.
// I don't really know what the question is asking...
#[allow(dead_code)]
pub fn zero_matrix(mut mtx: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut to_change: Vec<(usize, usize)> = vec![];
    let size = mtx.len();
    for r in 0..size {
        for c in 0..size {
            if mtx[r][c] == 0 {
                to_change.push((r, c));
            }
        }
    }

    for chg in to_change {
        println!("{:?}", chg);
        for i in 0..size {
            mtx[chg.0][i] = 0;
            mtx[i][chg.1] = 0;
        }
    }
    println!("======");

    mtx
}

// 1.9 String Rotation
#[allow(dead_code)]
pub fn is_rotation(s1: &str, s2: &str) -> bool {
    let (len1, len2) = (s1.len(), s2.len());
    if len1 != len2 {
        return false;
    }

    let mut chars1 = s1.chars();
    let mut chars2 = s2.chars();

    let first = chars1.next().unwrap();
    let mut to_append = vec![];

    while let Some(c) = chars2.next() {
        if c != first {
            to_append.push(c);
            continue;
        } else {
            break;
        }
    }

    // println!("{:?}", chars1);
    // println!("{:?}", chars2);
    // println!("{:?}", to_append);
    let mut new = chars2.collect::<Vec<char>>();
    new.extend(to_append);

    new == chars1.collect::<Vec<char>>()
}

fn is_substring(s1: &str, s2: &str) -> bool {
    let (len1, len2) = (s1.len(), s2.len());
    let chars1 = s1.chars().collect::<Vec<_>>();
    let chars2 = s2.chars().collect::<Vec<_>>();
    if len1 > len2 {
        let iter = chars1.windows(len2);
        for s in iter {
            if s == chars2 {
                return true;
            }
        }
    } else {
        let iter = chars2.windows(len1);
        for s in iter {
            if s == chars1 {
                return true;
            }
        }
    }
    false
}
