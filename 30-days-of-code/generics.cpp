/**
*    Name: printArray
*    Print each element of the generic vector on a new line. Do not return anything.
*    @param A generic vector
**/

// Write your code here
template <typename Type>
void printArray(const vector<Type>& ary) {
    for (const auto& e : ary) {
        cout << e << endl;
    }
}