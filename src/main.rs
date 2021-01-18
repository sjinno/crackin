mod ch_01;
mod ch_10;

fn main() {
    // println!("{}", ch_01::palindfrome_permutation("abcbacdddff"));

    // let arr1 = vec![1, 3, 4, 9, 19, 0, 0, 0, 0, 0, 0, 0, 0];
    // let arr2 = vec![2, 3, 4, 5, 7, 8, 11, 13];
    // println!("{:?}", ch_10::merge_sort(arr1, arr2));

    let test_cases = [
        ("pale", "pal"),
        ("pales", "pale"),
        ("pale", "bale"),
        ("pale", "bake"),
    ];
    for test in test_cases.iter() {
        println!("{}", ch_01::is_one_away(test.0, test.1));
    }
}
