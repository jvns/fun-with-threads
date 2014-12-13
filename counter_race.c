#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#define NUM_THREADS     20

int counter;

void *increment(int amount) {
    // Oh no this is a race!
    counter += amount;
}

void *AddThings(void *threadid)
{
   for (int i = 0; i < 100000; i++) {
       increment(1);
   }
   printf("Hello World! It's me, thread #%ld! I'm done.\n", (long)threadid);
   pthread_exit(NULL);
}

int main (int argc, char *argv[])
{
   pthread_t threads[NUM_THREADS];
   long t;
   for(t = 0; t<NUM_THREADS; t++){
      int rc = pthread_create(&threads[t], NULL, AddThings, (void *)t);
      if (rc){
         printf("ERROR; return code from pthread_create() is %d\n", rc);
         exit(1);
      }
   }
   // Wait for threads to finish
   for (t = 0; t < NUM_THREADS; t++)
       pthread_join(threads[t], NULL);
   printf("Final value of counter is: %d\n", counter);
   pthread_exit(NULL);
}
