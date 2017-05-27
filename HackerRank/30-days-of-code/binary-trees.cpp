void levelOrder(Node* root)
{
    if (root == NULL)
        return;

    queue<Node*> q;
    q.push(root);
    while (not q.empty()) {
        Node* p = q.front();
        q.pop();
        cout << p->data << ' ';
        if (p->left != NULL)
            q.push(p->left);
        if (p->right != NULL)
            q.push(p->right);
    }
}
