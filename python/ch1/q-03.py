def urlify_str(s) -> str:
    start, end = s.startswith(" "), s.endswith(" ")
    if start and end:
        return "%20" + "%20".join(s.split()) + "%20"
    if start and not end:
        return "%20" + "%20".join(s.split())
    if not start and end:
        return "%20".join(s.split()) + "%20"
    return "%20".join(s.split())


def solution2(s) -> str:
    length = len(s)
    new_str = ""
    for i in range(length-1):
        curr, next = s[i], s[i+1]
        if curr == " " and next == " ":
            continue
        elif curr == " " and next != " ":
            new_str += "%20"
        else:
            new_str += s[i]
    return new_str if s[length-1] != " " else new_str + "%20"


def main():
    print(urlify_str("   Hello,   world!   "))
    print(urlify_str("   Hello,   world!"))
    print(urlify_str("Hello,   world!   "))
    print(urlify_str("Hello,   world!"))
    print()
    print(solution2("   Hello,   world!   "))
    print(solution2("   Hello,   world!"))
    print(solution2("Hello,   world!   "))
    print(solution2("Hello,   world!"))


if __name__ == '__main__':
    main()
