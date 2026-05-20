ifeq ($(OS),Windows_NT)
	EXE_NAME := questra.exe
else
	EXE_NAME := questra
endif

.PHONY: clean

clean:
	cargo clean
	rm -rf ./q.zip

target/release/$(EXE_NAME):
	cargo build --release

# use `make clean q.zip' to always produce a fresh zip
q.zip: target/release/$(EXE_NAME)
	zip q.zip -j ./target/release/$(EXE_NAME)
	zip q.zip -r ./res
