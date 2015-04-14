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
        比较简单的做法，但速度有点慢。
        先将 n 个 ListNodes 划分，划分到 2 个时合并。
        速度慢的原因猜测：难道因为递归调用效率低？
        """
        n = len(lists)
        if n == 0:
            return None
        if n == 1:
            return lists[0]
        elif n == 2:
            return self.mergeTwo(lists[0], lists[1])
        else:
            l1 = self.mergeKLists(lists[0:n/2])
            l2 = self.mergeKLists(lists[n/2:n])
            return self.mergeTwo(l1,l2)

    def mergeTwo(self, ListNode1, ListNode2):
        sorted_list = []
        while ListNode1 and ListNode2:
            if ListNode1.val < ListNode2.val:
                sorted_list.append(ListNode1)
                ListNode1 = ListNode1.next
            else:
                sorted_list.append(ListNode2)
                ListNode2 = ListNode2.next
        while ListNode1:
            sorted_list.append(ListNode1)
            ListNode1 = ListNode1.next
        while ListNode2:
            sorted_list.append(ListNode2)
            ListNode2 = ListNode2.next
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

