
#include <stdlib.h>
#include <stdio.h>
#include <math.h>
#include <time.h>

int main() {
	int iterations;
	double x, y, length, pi;
 	// Total number of points inside the circle
	int circle_points = 0;

	printf("Monte-Carlo Approximation\n");
	printf("Enter the number of iterations: \n");
	printf("Iterations: ");
	scanf("%d", &iterations);



	for (int i = 0; i<iterations; i++) {
		x = rand() / (double) RAND_MAX;
		y = rand() / (double) RAND_MAX;

		if ( x*x + y*y <= 1) {
			circle_points++;
		}
	}

	pi = 4.0 * ((double) circle_points / (double) iterations);

	printf("Approximation of pi with %d iterations.\n", iterations);
	printf("pi = %f", pi );

}
