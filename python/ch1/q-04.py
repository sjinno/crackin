def does_str_make_palindrome_permutation(s) -> bool:
    if len(s) < 3:
        return True

    new_s = s.replace(" ", "").lower()
    each_letter_count = {}
    for char in new_s:
        if char in each_letter_count:
            each_letter_count[char] += 1
        else:
            each_letter_count[char] = 1

    count = 0
    for (_key, val) in each_letter_count.items():
        if val % 2 == 1:
            count += 1
        if count > 1:
            return False

    return True


def main():
    print(does_str_make_palindrome_permutation("ab"))
    print(does_str_make_palindrome_permutation("abb"))
    print(does_str_make_palindrome_permutation("abcba"))
    print(does_str_make_palindrome_permutation("kjdks"))
    print(does_str_make_palindrome_permutation("sddfgsagf"))
    print(does_str_make_palindrome_permutation("askdjaskjdlksajdsad"))
    print(does_str_make_palindrome_permutation("ffffffffffffffffff"))


if __name__ == '__main__':
    main()
