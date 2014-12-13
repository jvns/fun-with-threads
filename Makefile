all:
	gcc -o counter_race counter_race.c -lpthread
	./counter_race
