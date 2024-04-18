#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER; // Mutex用共有変数

int counter = 0; // クリティカルセクションで操作する変数
                                                  

void *some_func(void *arg) {
  if (pthread_mutex_lock(&mutex) != 0) {
    perror("pthread_mutex_lock");
    exit(-1);
  }

  counter = counter + 1;

  if (pthread_mutex_unlock(&mutex) != 0) {
    perror("pthread_mutex_unlock");
    exit(-1);
  }

  return NULL;
}

int main(int argc, char *argv[]) {
  pthread_t th1, th2;

  if (pthread_create(&th1, NULL, some_func, NULL) != 0) {
    perror("pthread_create");
    return -1;
  }

  if (pthread_create(&th2, NULL, some_func, NULL) != 0) {
    perror("pthread_create");
    return -1;
  }

  if (pthread_join(th2, NULL) != 0) {
    perror("pthread_join");
    return -1;
  }

  if (pthread_join(th1, NULL) != 0) {
    perror("pthread_join");
    return -1;
  }

  printf("counter = %d\n", counter);


  if (pthread_mutex_destroy(&mutex) != 0) {
    perror("pthread_mutex_destroy");
    return -1;
  }

  return 0;
}
