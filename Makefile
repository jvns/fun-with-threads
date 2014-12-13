all:
	gcc -o counter_race counter_race.c -lpthread
	rustc -O rust_counter.rs
