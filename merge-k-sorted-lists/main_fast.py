# -*- coding: utf-8 -*-
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    """
    https://leetcode.com/problems/merge-k-sorted-lists/
    Merge k sorted linked lists and return it as one sorted list. Analyze and describe its complexity.
    """
    # @param a list of ListNode
    # @return a ListNode
    def mergeKLists(self, lists):
        """
        这是一个比较 low 的做法，但是很快。首先它没有重复创建对象，
        然后直接用了 sort 来排序。
        """
        from operator import attrgetter
        sorted_list = []
        for head in lists:
            while head:
                sorted_list.append(head)
                head = head.next
        sorted_list.sort(key=attrgetter('val'))
        sorted_list.append(None)
        for i in range(0,len(sorted_list)-1):
            sorted_list[i].next = sorted_list[i+1]
        return sorted_list[0]

def make_list(l):
    result = [ListNode(i) for i in l]
    for i in range(0,len(result)-1):
        result[i].next = result[i+1]
    if result:
        return result[0]
    else:
        return None


def print_node(node):
    while node:
        print(node.val)
        node = node.next

if __name__ == '__main__':
    solution = Solution();
    x1 = make_list([1,4,7,8])
    x2 = make_list([2,3,5,7])
    t1 = solution.mergeKLists([x1,x2,x1,x1,x2])
    print_node(t1)

    t2 = solution.mergeKLists([])
    print_node(t2)
    t3 = solution.mergeKLists([{},{}])
    print_node(t3)

