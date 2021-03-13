#include <iostream>
#include <vector>
#include <algorithm>

int main() {
    using namespace std;

    int length = 5;
    vector <float> numbers;

    int i = 1;
    while (i <= 5) {
        float temp;
        cout << "number " << i << ": ";
        cin >> temp;
        numbers.push_back(temp);

        i++;
    }

    sort(numbers.begin(), numbers.end());
    cout << "Smallest Number : " << numbers[0] << endl;
    cout << "2nd Smallest Number :" << numbers[1] << endl;

    return 0;
}