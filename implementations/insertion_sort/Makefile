test_all: test_rs

rs_insertion_sort:
	rustc insertion_sort.rs -o rs_insertion_sort

test_rs: rs_insertion_sort
	./rs_insertion_sort

clean:
	rm ./rs_insertion_sort