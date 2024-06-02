provenanced tx wasm migrate \
    tp1tl5f4u2jsqsjkne3yv5saau84xa999hkgxytaaz4uj648pt8a3pstj7ldp \
    698 \
    '{}' \
    --from "$feebucket" \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --broadcast-mode block \
    --testnet \
    --yes \
    --gas 4000000 \
    --gas-prices 26905nhash \
    --node=https://rpc.test.provenance.io:443 \
	--output json | jq '.txhash, .code, .raw_log'
