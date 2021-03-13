#include <iostream>
#include <string>
#include <tuple>
using namespace std;


string decode(string input, int key) {
    for (int i=0; i <input.length(); i++) {
        if (isupper(input[i])) {
            input[i] = char((input[i] - key-65)%26 +65);
        } else {
            input[i] = char((input[i] - key -97)%26 + 97);
        }
    }
    return input;
}

string encode(string input, int key) {
    for (int i=0; i <input.length(); i++) {
        if (isupper(input[i])) {
            input[i] = char((input[i] + key-65)%26 +65);
        } else {
            input[i] = char((input[i] + key -97)%26 + 97);
        }
    }
    return input;
}

int main() {

    string input;
    int key;
    char decision;
    bool quit = false;

    while (!quit) {
    
    cout << "Caesar Cipher v1.0" << endl;
    cout << "What do you want to do? \n" << endl;
    cout << "1 - Encode" << endl;
    cout << "2 - Decode" << endl;
    cout << "q - Quit" << endl;

    
    cout << "Decision: ";
    cin >> decision;

    if (decision == 'q') {
        break;
    }

    cout << "Text: ";
    cin >> input;

    cout << "key: ";
    cin >> key;



  
    if (decision == '1') {
        cout << "Result: " <<encode(input, key) << endl;
        cout << "\n" << endl;


    } else if (decision == '2') {
        cout << "Result: "<< decode(input, key) << endl;
        cout << "\n" << endl;

    } else {
        cout << "invalid decision" << endl;
        cout << "\n" << endl;
    }


    }
    
}

