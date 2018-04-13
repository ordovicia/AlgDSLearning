#include <iostream>
#include <stack>
#include <queue>

using namespace std;

class Solution
{
public:
    Solution() = default;

    void pushCharacter(char c)
    {
        m_stack.push(c);
    }

    void enqueueCharacter(char c)
    {
        m_queue.push(c);
    }

    char popCharacter()
    {
        auto ret = m_stack.top();
        m_stack.pop();
        return ret;
    }

    char dequeueCharacter()
    {
        auto ret = m_queue.front();
        m_queue.pop();
        return ret;
    }

private:
    stack<char> m_stack;
    queue<char> m_queue;
};
