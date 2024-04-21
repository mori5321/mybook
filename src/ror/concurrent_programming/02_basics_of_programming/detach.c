
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#define NUM_THREADS 10

// Pthreadsではスレッド用の関数の型は void*型の値を受取*void型の値を返す関数でなければならない。
// void*型から実際の型にキャストする。
void *thread_func(void *arg) {
  int id = (int)arg;

  for (int i = 0; i < 10; i++) {
    printf("id = %d, i = %d\n", id, i);
    sleep(1);
  }

  return "finished!";
}


int main(int argc, char *argv[]) {
  pthread_attr_t attr;
  if (pthread_attr_init(&attr) != 0) {
    perror("pthread_attr_init");
    return -1;
  }

  if (pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED) != 0) {
    perror("pthread_attr_setdetachstate");
    return -1;
  }

  pthread_t th;
  if (pthread_create(&th, &attr, thread_func, (void *)0) != 0) {
    perror("pthread_create");
    return -1;
  }

  if (pthread_attr_destroy(&attr) != 0) {
    perror("pthread_attr_destroy");
    return -1;
  }
}
