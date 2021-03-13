#include <iostream>

using namespace std;



int main() {
    
    float x,y, ans;
    char op;
    cout << "Enter the first number: ";
    cin >> x;
    cout << "Add: + \n";
    cout << "Subtract: - \n";
    cout << "Multiply: * \n";
    cout << "Divide: / \n";
    cout << "Choose an operator: ";
    cin >> op;
    cout << "Enter the second number: ";
    cin >> y;
    
    switch (op) {
    case '+':
        ans = x + y;
        break;
    case '-':
        ans = x - y;
        break;
    case '*':
        ans = x * y;
        break;
    case '/':
        if (x) {
            ans = x / y;
            break;
        }
    default:
        cerr << "Invalid Input" << endl;
        return 0;
    return 0;
    }
    cout << "Result: " << ans << endl;
}