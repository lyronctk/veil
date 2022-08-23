cargo run --release -- \
  --private-key $HACKLODGE_PRIVATE_KEY \
  --backup-address $HACKLODGE_BACKUP_ADDRESS \
  --contract-address $HACKLODGE_CONTRACT_ADDRESS \
  --min-gas 10 \
  --max-gas 100 \
  --gas-step 10 \
  --nonce 1 \
  --output-path "not-your-private-keys.csv" \
  --erc20-addresses "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984" \
  "0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9" \
  "0xA808B22ffd2c472aD1278088F16D4010E6a54D5F"

