
#include <stdio.h>
#include <stdlib.h>

float add(float x, float y) {
    return x + y;
}

float subtract(float x, float y) {
    return x - y;
}

float multiply(float x, float y) {
    return x * y;
}

float divide(float x, float y) {
    if (y == 0) {
        printf("Error division by zero\n");
        exit(0);
    } else {
        return x / y;
    }
}

int main()
{
    float num1, num2;
    int choice;

    float result;

    printf("Calculator v1.0\n");
    printf("What do you want to do?\n");
    printf("\t [1] - Add\n");
    printf("\t [2] - Multiply\n");
    printf("\t [3] - Subtract\n");
    printf("\t [4] - Divide\n");

    printf("Decision: ");
    scanf("%d", &choice);

    printf("First Number: ");
    scanf("%f", &num1);
    printf("Second Number: ");
    scanf("%f", &num2);

    if (choice == 1) {
        result = add(num1, num2);
    } else if (choice == 2) {
        result = multiply(num1, num2);
    } else if (choice == 3) {
        result = subtract(num1, num2);
    } else if (choice == 4) {
        result = divide(num1, num2);
    } else {
        printf("Invalid Choice");
        exit(0);
    }

    printf("Result = %f", result);

    return 0;
}

