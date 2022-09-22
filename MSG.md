
## Controller contract (Remote chain)

Initialize msg
```json
{
  "swap_contract": "osmo1xxxxxx",
  "default_timeout": 1200,
  "allowed_list": [
    {
      "denom": "ujuno",
      "channel": "channel-0"
    },
    {
      "denom": "ibc/XXXXXX",
      "channel": "channel-0"
    }
  ]
}
```

Execute
```json
{
  "swap": {
    "channel": "channel-2",
    "denom": "uosmo",
    "min_amount": "1"
  }
}
```
Funds `1000000ujuno`


## Host contract (osmosis chain)

Initialize msg
```json
{
  "swap_router": "osmo1xxxxxx",
  "default_timeout": 1200,
  "allowed_list": [
    {
      "denom": "ibc/xxxxxxx",
      "channel": "channel-42"
    },
    {
      "denom": "uosmo",
      "channel": "channel-42"
    }
  ]
}
```

IBC Packet (Swap msgs)
```json
{
    "sender": "juno1xxxxx",
    "amount": "1000000",
    "denom": "ujuno",
    "out_denom": "uosmo",
    "min_amount": "1",
    "sequence": "1244"
}
```
> sequence: IBC transfer sequence used to verify ibc-transfer is completed.


Execute
```json
{
  "complete_swap": {
    "channel": "channel-70",
    "sequence": "5"
  }
}
```
> Custom relayer needs to complete the swap when ibc-transfer is completed
