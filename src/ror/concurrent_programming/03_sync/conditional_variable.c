#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t cond = PTHREAD_COND_INITIALIZER; // 条件変数の初期化。更新はMutexでロックを獲得したあとに行う

volatile bool ready = false;
char buf[256];

void* producer(void *arg) { // データの生成スレッド
  printf("producer ");
  fgets(buf, sizeof(buf), stdin);

  pthread_mutex_lock(&mutex);
  ready = true; // producer関数によるデータ生成が、consumerスレッドの生成より割きに行われる可能性を排除する。(より厳密には擬似覚醒という現象を防ぎたい)

  if (pthread_cond_broadcast(&cond) != 0) { // 待機中のすべてのスレッドに通知する
    perror("pthread_cond_broadcast");
    exit(1);
  }

  pthread_mutex_unlock(&mutex);
  return NULL;
}

void* consumer(void *arg) { // データの消費を行うスレッド
  pthread_mutex_lock(&mutex); // 条件変数を読み込むためのmutextロック獲得

  while (!ready) {
    // ここがわからん
    if (pthread_cond_wait(&cond, &mutex) != 0) { // 読み込みができない場合は待機。pthread_cond_waitをロックの解放と待機をアトミックに行うことが保証されている
      perror("pthread_cond_wait");
      exit(1);
    }
  }

  pthread_mutex_unlock(&mutex);

  printf("consumer %s", buf);
  return NULL;
}

int main(int argc, char *argv[]) {
  pthread_t pr, cn;
  pthread_create(&pr, NULL, producer, NULL);
  pthread_create(&cn, NULL, consumer, NULL);

  pthread_join(pr, NULL);
  pthread_join(cn, NULL);

  pthread_mutex_destroy(&mutex);

  if (pthread_cond_destroy(&cond) != 0) {
    perror("pthread_cond_destroy");
    exit(1);
  }

  return 0;
}
