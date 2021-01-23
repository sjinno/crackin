class Node:
    def __init__(self, value):
        self.value = value
        self.next = None


class LinkedList:
    def __init__(self):
        self.head = None

    def append(self, Node):
        if self.head is None:
            self.head = Node
            return

        node = self.head
        while node.next is not None:
            node = node.next
        node.next = Node
        return

    def __str__(self):
        node = self.head
        while node.next is not None:
            print(f"{node.value}->", end="")
            node = node.next
        print(node.value, end="")
        return ""


def delete_duplicates(head):
    node = head
    ht = {}
    ht[node.value] = True

    while node.next is not None:
        while node.next.value in ht:
            node.next = node.next.next
            if node.next is None:
                return
        ht[node.next.value] = True
        node = node.next

    return


def main():
    ll = LinkedList()
    ll.append(Node(1))
    ll.append(Node(5))
    ll.append(Node(2))
    ll.append(Node(2))
    ll.append(Node(3))
    ll.append(Node(2))
    ll.append(Node(3))
    ll.append(Node(5))
    ll.append(Node(5))
    ll.append(Node(5))
    ll.append(Node(3))
    ll.append(Node(3))
    ll.append(Node(3))
    ll.append(Node(1))
    ll.append(Node(4))
    ll.append(Node(1))
    ll.append(Node(1))
    ll.append(Node(3))
    ll.append(Node(5))
    ll.append(Node(5))
    ll.append(Node(5))

    print("Before: ", ll)

    delete_duplicates(ll.head)

    print("After: ", ll)


if __name__ == '__main__':
    main()
