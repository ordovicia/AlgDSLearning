

37

        int getHeight(Node* root){

38

            int left_height = root->left == NULL

39

                ? 0

40

                : 1 + getHeight(root->left);

41

            int right_height = root->right == NULL

42

                ? 0

43

                : 1 + getHeight(root->right);

44

            return max(left_height, right_height);

45

        }

46

​