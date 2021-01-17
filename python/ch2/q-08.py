import time


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


class Node:
    def __init__(self, value):
        self.value = value
        self.next = None


# ll: LinkedList
def find_loop(head):
    slow = head
    fast = head

    while fast is not None or fast.next is not None:
        slow = slow.next
        fast = fast.next.next
        print(slow.value, fast.value)
        if slow == fast:
            break

    if fast is None or fast.next is None:
        print("There is no loop.")
        return

    slow = head
    count = 0
    while slow != fast:
        print(f"Position {count}:", slow.value, fast.value)
        time.sleep(0.5)
        slow = slow.next
        fast = fast.next
        count += 1
    print(f"Position {count}:", slow.value, fast.value)

    print(f"Loop found at position {count}, whose value is {slow.value}.")


def main():
    # ll = LinkedList()
    # ll.head = Node(1)
    # ll.head.next = Node(2)
    # ll.head.next.next = Node(3)
    # ll.head.next.next.next = Node(4)
    # ll.head.next.next.next.next = Node(5)
    # ll.head.next.next.next.next.next = Node(6)
    # ll.head.next.next.next.next.next.next = ll.head.next.next

    # find_loop(ll.head)

    ll = LinkedList()
    for i in range(4):
        ll.append(Node(i))
    ll.head.next.next.next.next = ll.head.next.next

    find_loop(ll.head)

    # node = ll.head
    # while node is not None:
    #     print(node.value)
    #     node = node.next


if __name__ == '__main__':
    main()
