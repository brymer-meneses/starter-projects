#include <tuple> 
#include <string>
#include <iostream>

using namespace std;

tuple <string, int, int> test() {
    return make_tuple("hello", 1, 2);
}

int main() {
    int firstnum = 0;
    int secondnum = 0 ;
    string text = "initial";

    cout << "first number: " << firstnum <<endl;
    cout << "second number: " << secondnum << endl;
    cout << "text: " << text << endl;

    tie(text, firstnum, secondnum) = test();

    cout << "first number: " << firstnum <<endl;
    cout << "second number: " << secondnum << endl;
    cout << "text: " << text << endl;
    return 0;
}