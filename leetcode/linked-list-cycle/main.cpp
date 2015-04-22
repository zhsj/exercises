#include <iostream>
using namespace std;
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};


/// https://leetcode.com/problems/linked-list-cycle/
///
/// Given a linked list, determine if it has a cycle in it.
/// Follow up:
/// Can you solve it without using extra space?
class Solution {
    public:
        /// 两个指针 p1, p2
        /// p1 每次向后走一格，p2 每次向后走两格
        /// 如果有圈，那么 p1 会追上 p2
        bool hasCycle(ListNode *head) {
            ListNode *p1 = head, *p2 = head;
            while(p2!=NULL && p2->next!=NULL && p2->next!=NULL){
                p1 = p1->next;
                p2 = p2->next->next;
                if(p1 == p2) return true;
            }
            return false;
        }
};

int main(int argc, char const* argv[])
{
    Solution solution;
    ListNode* a1 = new ListNode(1);
    ListNode *a2 = new ListNode(2);
    a1->next = a2;
    cout << solution.hasCycle(a1) << endl;;
    a2->next = a1;
    cout << solution.hasCycle(a1) << endl;;
    return 0;
}
