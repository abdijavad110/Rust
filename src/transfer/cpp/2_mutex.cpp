#include<stdio.h>
#include<pthread.h>
#include<immintrin.h>


pthread_mutex_t lock;


void *dumb_deposit(void* ptr) {
	int* bal;
	bal = (int*) ptr;
	for (int i=0; i<100000; i++) {
		pthread_mutex_lock(&lock);
		*bal += 1;
		pthread_mutex_unlock(&lock);
	}
}

int main() {
	pthread_t t1, t2;
	int *balance;
	*balance = 0;

	pthread_mutex_init(&lock, NULL)
	pthread_create(&t1, NULL, dumb_deposit, (void*) balance);
	pthread_create(&t2, NULL, dumb_deposit, (void*) balance);
	pthread_join(t1, NULL);
	pthread_join(t2, NULL);

	printf("%d\n", *balance);
}
