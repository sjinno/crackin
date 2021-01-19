mod ch_01;
mod ch_10;

fn main() {
    // println!("{}", ch_01::palindfrome_permutation("abcbacdddff"));

    // let arr1 = vec![1, 3, 4, 9, 19, 0, 0, 0, 0, 0, 0, 0, 0];
    // let arr2 = vec![2, 3, 4, 5, 7, 8, 11, 13];
    // println!("{:?}", ch_10::merge_sort(arr1, arr2));

    // 1.5
    // let test_cases = [
    //     ("pale", "pal"),
    //     ("pales", "pale"),
    //     ("pale", "bale"),
    //     ("pale", "bake"),
    // ];
    // for test in test_cases.iter() {
    //     println!("{}", ch_01::is_one_away(test.0, test.1));
    // }

    // // 1.6
    // let s = "aabcccccaaa";
    // println!("{}", ch_01::compress_string(s));
    // println!("{}", ch_01::compress_string("aabbcc"));

    // // 1.7
    // let m1 = vec![vec![1, 2], vec![3, 4]];

    // let m2 = vec![
    //     vec![1, 2, 3, 4, 5],
    //     vec![6, 7, 8, 9, 10],
    //     vec![11, 12, 13, 14, 15],
    //     vec![16, 17, 18, 19, 20],
    //     vec![21, 22, 23, 24, 25],
    // ];
    // let mm1 = ch_01::rotate_matrix(m1);
    // let mm2 = ch_01::rotate_matrix(m2);
    // print_mtx(mm1);
    // print_mtx(mm2);

    let m1 = vec![vec![1, 0], vec![2, 3]];

    let m2 = vec![vec![1, 2, 3], vec![4, 0, 6], vec![7, 8, 9]];

    let mm1 = ch_01::zero_matrix(m1);
    let mm2 = ch_01::zero_matrix(m2);

    print_mtx(mm1);
    print_mtx(mm2);
}

#[allow(dead_code)]
fn print_mtx(mtx: Vec<Vec<usize>>) {
    for row in mtx {
        println!("{:?}", row);
    }
}
