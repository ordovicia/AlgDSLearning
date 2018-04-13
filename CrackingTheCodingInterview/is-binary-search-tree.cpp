/* Hidden stub code will pass a root argument to the function below. Complete the function to solve the challenge. Hint: you may want to write one or more helper functions.

The Node struct is defined as follows:
   struct Node {
      int data;
      Node* left;
      Node* right;
   }
*/

bool checkBST(Node* root)
{
    bool left_ok = true, right_ok = true;
    if (root->left != nullptr)
        left_ok = checkBSTRecurse(root->left, 0, root->data - 1);
    if (root->right != nullptr)
        right_ok = checkBSTRecurse(root->right, root->data + 1, 10000);
    return left_ok && right_ok;
}

bool checkBSTRecurse(Node* node, int min, int max)
{
    if (node->data < min || node->data > max)
        return false;

    bool left_ok = true, right_ok = true;
    if (node->left != nullptr)
        left_ok = checkBSTRecurse(node->left, min, node->data - 1);
    if (node->right != nullptr)
        right_ok = checkBSTRecurse(node->right, node->data + 1, max);

    return left_ok && right_ok;
}
