CC = gcc
RUSTC = rustc

all: main

main: main.o module1.o module2.o
	$(CC) $(CFLAGS) -o main main.o module1.o module2.o /usr/lib/rustlib/arm-unknown-linux-gnueabihf/lib/lib*.so

main.o: main.rs
	$(RUSTC) main.rs --emit=obj

module1.o: module1.rs
	$(RUSTC) module1.rs --emit=obj --crate-type=lib

module2.o: module2.rs
	$(RUSTC) module2.rs --emit=obj --crate-type=lib

clean: 
	rm -f *.o main
