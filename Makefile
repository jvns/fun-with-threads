all:
	gcc -std=c99 -o counter_race counter_race.c -lpthread
	gcc -std=c99 -o counter_with_mutex counter_with_mutex.c -lpthread
	rustc -O rust_counter.rs
	rustc -O rust_counter_atomics.rs
