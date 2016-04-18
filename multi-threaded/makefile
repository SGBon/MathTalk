RC=rustc
BIN=matmulti.exe
RCFLAGS= -o $(BIN)
SOURCE=matmul-multi.rs

run: build
	./$(BIN)

build:
	$(RC) $(RCFLAGS) $(SOURCE)

clean:
	rm -rf $(BIN)
