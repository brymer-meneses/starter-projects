#include <iostream>
#include <random>
#include <cmath>

int main() {
    int iterations = 0;
    int numbers_inside = 0;
    
    // Initialize Random Number
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<> dis(0.0, 1.0);

    std::cout << "Monte-Carlo Approximation of pi" << std::endl;
    std::cout << "Enter the number of iterations: " << std::endl;

    std::cin >> iterations;
    for (int i = 0; i < iterations; i++) {
        float x = dis(gen); // Generate random x
        float y = dis(gen); // Generate random y 

        float distance = std::sqrt(x*x + y*y);
        if (distance < 1) {
            numbers_inside++;

        }

    }

    float approx_pi = ((float) 4)*((float)numbers_inside/ (float) iterations);
  
    std::cout << "Approximation of pi with " << iterations << " iterations = " << approx_pi << std::endl;


    return 0;
}