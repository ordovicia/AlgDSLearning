Node* removeDuplicates(Node *head)
          {
              if (head == NULL)
                  return head;
              
              for (Node* node = head; node->next != NULL; node = node->next) {
                  int now_data = node->data;
                  Node* next = node->next;
                  while (next->data == now_data) {
                      if (next->next == NULL) {
                          node->next = NULL;
                          return head;
                      }
                      next = next->next;
                  }
                  node->next = next;
              }
              
              return head;
          }