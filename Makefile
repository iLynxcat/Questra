.PHONY: clean

clean:
	cargo clean
	rm -rf ./.pack
	rm -rf ./q.zip

target/release/questra:
	cargo build --release

# use `make clean q.zip' to always produce a fresh zip
q.zip: target/release/questra
	@if [[ ! -d .pack ]]; then mkdir .pack; fi
	cp ./target/release/questra ./.pack/questra
	cp -r ./target/release/res ./.pack/
	zip -r q.zip ./.pack/
	@rm -rf .pack || exit 1
