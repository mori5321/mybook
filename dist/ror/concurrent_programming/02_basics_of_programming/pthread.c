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
  pthread_t v[NUM_THREADS]; // スレッド用のハンドラを保存する配列を定義する

  for (int i = 0; i < NUM_THREADS; i++) {
    if (pthread_create(&v[i], NULL, thread_func, (void *)i) != 0) {
      perror("pthread_create");
      return 01;
    }
  }

  // for loop で待つのが正しいの?
  for (int i = 0; i < NUM_THREADS; i++) {
    char *ptr;
    printf("loop %d\n", i);
    if (pthread_join(v[i], (void **)&ptr) == 0) {
      printf("msg = %s\n", ptr);
    } else {
      perror("pthread_join");
      return -1;
    }
  }

  printf("Hello");

  return 0;
}
