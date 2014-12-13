all:
	gcc -o counter_race counter_race.c -lpthread
	rustc rust_counter.rs
