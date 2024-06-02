
provenanced tx wasm execute \
    tp1tl5f4u2jsqsjkne3yv5saau84xa999hkgxytaaz4uj648pt8a3pstj7ldp \
    '{
    "close_deposit": {}
}
' \
    --from $feebucket \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 26905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq '.txhash, .code, .raw_log'
