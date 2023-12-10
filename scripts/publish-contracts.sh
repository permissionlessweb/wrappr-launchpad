cd contracts/factories/wrappr-factory && cargo publish && cd ../../..
sleep 10
cd contracts/collections/wrappr721-base && cargo publish && cd ../../..
sleep 15
cd contracts/collections/wrappr721-metadata-onchain && cargo publish && cd ../../..
sleep 10
cd contracts/collections/wrappr721-nt && cargo publish && cd ../../..
sleep 10
cd contracts/collections/wrappr721-updatable && cargo publish && cd ../../..
sleep 10
cd contracts/splits && cargo publish && cd ../..
sleep 10
cd contracts/minters/wrappr-minter && cargo publish && cd ../../..
sleep 10
cd contracts/whitelists/whitelist && cargo publish && cd ../../..
sleep 10
cd contracts/factories/vending-factory && cargo publish && cd ../../..
sleep 15
cd contracts/minters/vending-minter && cargo publish && cd ../../..
sleep 15
cd contracts/factories/open-edition-factory && cargo publish && cd ../../..
sleep 15
cd contracts/minters/open-edition-minter && cargo publish && cd ../../..
sleep 10
cd contracts/whitelists/whitelist-immutable && cargo publish && cd ../../..
sleep 15
cd contracts/whitelists/whitelist-flex && cargo publish && cd ../../..
sleep 10
cd contracts/minters/vending-minter-wl-flex && cargo publish && cd ../../..

cd contracts/sg-eth-airdrop && cargo publish && cd ../..
sleep 15
cd test-suite && cargo publish && cd ..
