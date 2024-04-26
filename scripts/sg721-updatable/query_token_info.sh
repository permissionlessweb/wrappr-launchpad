MSG=$(cat <<EOF
{
  "nft_info": {"token_id": "$1"}
}
EOF
)
echo $MSG $WRAPPR721

starsd q wasm contract-state smart $WRAPPR721 "$MSG"

