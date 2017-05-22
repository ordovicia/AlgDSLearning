Node* insert(Node* head, int data)
{
    if (head == nullptr) {
        head = new Node{data};
        return head;
    }

    auto node = head;
    while (node->next != nullptr) {
        node = node->next;
    }
    node->next = new Node{data};
    return head;
}
