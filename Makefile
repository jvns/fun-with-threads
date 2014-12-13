all:
	gcc -o counter_race counter_race.c -lpthread
	gcc -o counter_with_mutex counter_with_mutex.c -lpthread
	rustc -O rust_counter.rs
