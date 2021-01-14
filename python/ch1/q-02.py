def is_permutation(s1, s2) -> bool:
    len1, len2 = len(s1), len(s2)
    if len1 != len2:
        return False

    count1, count2 = 0, 0
    for char in s1:
        for i in s1:
            if i == char:
                count1 += 1
        for j in s2:
            if j == char:
                count2 += 1
        if count1 != count2:
            return False

    return True


def main():
    print(is_permutation("aaaaaaaaaaabaaaaaaaaaaaa",
                         "aaaaaaaaaaaaaaaaaabaaaaa"))


if __name__ == "__main__":
    main()
