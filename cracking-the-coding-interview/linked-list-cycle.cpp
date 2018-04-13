/*
Detect a cycle in a linked list. Note that the head pointer may be 'NULL' if the list is empty.

A Node is defined as:
    struct Node {
        int data;
        struct Node* next;
    }
*/

#include <unordered_set>

bool has_cycle(Node* head)
{
    if (head == nullptr)
        return false;

    std::unordered_set<Node*> visited;
    for (; head->next != nullptr; head = head->next) {
        if (visited.count(head) != 0)
            return true;
        visited.insert(head);
    }

    return false;
}
