

#include <stdio.h>
#include <stdlib.h>

struct cipherJob {
	int key, decision;
	char message[100];
};

typedef struct cipherJob CipherJob;
CipherJob getInput() {
	CipherJob userInput;

	printf("Caesar Cipher v1.0\n");
	printf("What do you want to do?\n");
	printf("\t [1] - Encode\n");
	printf("\t [2] - Decode\n");

	printf("Decision: ");
	scanf("%d", &userInput.decision);
	printf("\nEnter key: ");
	scanf("%d", &userInput.key);

	printf("%S", userInput.decision ? "Decryption" : "Encryption");

	

	return userInput;
}


int main() {
	CipherJob userInput = getInput();

	printf("decision: %d\n", userInput.decision);
	printf("key: %d\n", userInput.key);
	
	return 0;
}
