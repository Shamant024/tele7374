CC = gcc
RUSTC = rustc

all: main

main: main.rs foo.o
	$(RUSTC) main.rs -o main -C linker=$(CC) -C link-arg=foo.o

foo.o: foo.c
	$(CC) $(CFLAGS) -c foo.c -o foo.o

clean:
	rm -f *.o main
