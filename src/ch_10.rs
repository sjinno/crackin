// 10.1 Sorted Merge
pub fn merge_sort(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    use std::cmp::Ordering;
    let (len1, len2) = (arr1.len(), arr2.len());
    let (mut idx1, mut idx2) = (len1 - len2 - 1, len2 - 1);
    let mut to_change = len1 - 1;

    while idx1 != 0 || idx2 != 0 {
        let (a, b) = (arr1[idx1], arr2[idx2]);
        match a.cmp(&b) {
            Ordering::Greater => {
                arr1[to_change] = a;
                idx1 -= 1;
            }
            Ordering::Less => {
                arr1[to_change] = b;
                idx2 -= 1
            }
            Ordering::Equal => {
                arr1[to_change] = b;
                idx2 -= 1
            }
        }
        to_change -= 1;

        println!("{} {} {}", idx1, idx2, to_change);
    }

    let (a, b) = (arr1[idx1], arr2[idx2]);
    if a < b {
        arr1[to_change] = b;
    } else if a > b {
        arr1[to_change] = a;
        arr1[0] = b;
    } else {
        arr1[to_change] = a;
    }

    arr1
}
