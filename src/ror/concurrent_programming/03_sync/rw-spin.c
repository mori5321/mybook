#include <stdbool.h>
#include <stdatomic.h>

// => Writerがいないことを Readerロック取得前後でチェックすりゃいいんだな。なるほど。
void rwlock_read_acquire(int *rcnt, volatile int *wcnt) {
  for (;;) {
    
    while(*wcnt); // Writerがいるなら待機

    __sync_fetch_and_add(rcnt, 1);

    // あらためてWriterがいないかチェックし、いなければReaderロック取得完了(breakする)
    // ロックをwhileで表す = スピンロック
    if (*wcnt == 0) 
      break; 
    
    // Writer がいたらロックを破棄
    __sync_fetch_and_sub(rcnt, 1);
  }
}

// Read用ロック解放関数
void rwlock_read_release(int *rcnt) {
  __sync_fetch_and_sub(rcnt, 1);
}

bool test_and_set(bool *lock) {
  return __sync_lock_test_and_set(lock, 1);
}

void tas_release(bool *lock) {
  __sync_lock_release(lock);
}

void spinlock_acquire(bool *lock) {
  for (;;) {
    while (*lock);
    if (!test_and_set(lock))
      break;
  }
}


void spinlock_release(bool *lock) {
  tas_release(lock);
}

void rwlock_write_acquire(bool *lock, volatile int *rcnt, volatile int *wcnt) {
  __sync_fetch_and_add(wcnt, 1);

  while (*rcnt); // Readerがいるなら待機
  
  spinlock_acquire(lock);
}

void rwlock_write_release(bool *lock, volatile int *wcnt) {
  __sync_fetch_and_sub(wcnt, 1);
  spinlock_release(lock);
}


int rcnt = 0;
int wcnt = 0;

bool lock = false;

void reader() {
  for (;;) {
    rwlock_read_acquire(&rcnt, &wcnt);
    // クリティカルセクション(読み込みのみ)
    rwlock_read_release(&rcnt);
  }
}

void writer() {
  for (;;) {
    rwlock_write_acquire(&lock, &rcnt, &wcnt);
    // クリティカルセクション(書き込み)
    rwlock_write_release(&lock, &wcnt);
  }
}
