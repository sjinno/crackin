"""
    Given two sorted arrays A and B, considering that A has enough space to hold B,
    merge sort those two.

    e.g. Input: arr1 = [1, 3, 6, 0, 0, 0], arr2 = [2, 4, 5]
         Output: [1, 2, 3, 4, 5, 6]
"""


def merge_sort(arr1, arr2):
    len1, len2 = len(arr1), len(arr2)

    idx1 = len1 - len2 - 1  # initial index: end of index of arr1
    idx2 = len2 - 1
    change = len1 - 1

    while change != 0:
        a, b = arr1[idx1], arr2[idx2]
        if a > b:
            arr1[change] = a
            idx1 -= 1
        elif a < b:
            arr1[change] = b
            idx2 -= 1
        else:
            arr1[change] = b
            idx2 -= 1
        change -= 1

        if change == 0 and idx2 == 0:
            arr1[change] = arr2[idx2]

    return arr1


def main():
    arr1 = [1, 3, 6, 0, 0, 0]
    arr2 = [2, 4, 5]
    merged = merge_sort(arr1, arr2)
    print(merged)


if __name__ == "__main__":
    main()
