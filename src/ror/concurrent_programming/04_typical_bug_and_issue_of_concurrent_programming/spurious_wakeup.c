#include <pthread.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <unistd.h>

pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t cond = PTHREAD_COND_INITIALIZER;

void handler(int sig) { printf("received signal: %d\n", sig); }
 
int main(int argc, char *argv[]) {
  pid_t pid = getpid();
  printf("pid: %d\n", pid);

  signal(SIGUSR1, handler);

  pthread_mutex_lock(&mutex);
  if (pthread_cond_wait(&cond, &mutex) != 0) {
    perror("pthread cond  wait");
    exit(1);
  }

  printf("sprious wake up \n");
  pthread_mutex_unlock(&mutex);

  return 0;
}
