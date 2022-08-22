cargo run --release -- \
  --private-key $HACKLODGE_PRIVATE_KEY \
  --backup-address $HACKLODGE_BACKUP_ADDRESS \
  --contract-address $HACKLODGE_CONTRACT_ADDRESS \
  --min-gas 10 \
  --max-gas 100 \
  --gas-step 10 \
  --nonce 0
