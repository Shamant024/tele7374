CC = gcc
RUSTC = rustc
RUSTLIB_PATH = /usr/lib/rustlib/aarch64-unknown-linux-gnu/lib

LIBS = $(RUSTLIB_PATH)/libstd-4e83e61126e325db.so \
       $(RUSTLIB_PATH)/libcore-b44f001c1cf81c71.rlib \
       $(RUSTLIB_PATH)/liballoc-cef78af8159b3a12.rlib \
       $(RUSTLIB_PATH)/libcompiler_builtins-c1515b5a0806bd44.rlib

all: main

main: main.o module1.o module2.o
	$(CC) $(CFLAGS) -o main main.o module1.o module2.o $(LIBS) -ldl -lpthread -lm

main.o: main.rs
	$(RUSTC) main.rs --emit=obj

module1.o: module1.rs
	$(RUSTC) module1.rs --emit=obj --crate-type=lib

module2.o: module2.rs
	$(RUSTC) module2.rs --emit=obj --crate-type=lib

clean: 
	rm -f *.o main
