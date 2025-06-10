build:
	cargo build-sbf

test:
	cargo test --features test

get-program-key:
	solana address -k target/deploy/p_multisig-keypair.json

deploy:
	solana program deploy ./target/deploy/p_multisig.so --program-id ./target/deploy/p_multisig-keypair.json