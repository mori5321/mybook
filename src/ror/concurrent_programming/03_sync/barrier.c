#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>
#include <pthread.h>



void barrier(volatile int *cnt, int max) {
  __sync_fetch_and_add(cnt, 1);
  while (*cnt < max);
}

volatile int num = 0;

void *worker(void *arg) {
  // sleep
  printf("Thread is starting\n");
  
  srand(time(NULL));
  int sleep_time = rand() % 10;

  sleep(sleep_time);

  barrier(&num, 10);

  printf("Thread is done\n");

  return 0;
}

int main(int argc, char *argv[]) {
  pthread_t threads[10];
  for (int i = 0; i < 10; i++) {
    if (pthread_create(&threads[i], NULL, worker, NULL) != 0) {
      perror("pthread_create");
      return -1;
    };
  }

  for (int i = 0; i < 10; i++) {
    if (pthread_join(threads[i], NULL) != 0) {
      perror("pthread_join");
      return -1;
    }
  }

  return 0;
}
