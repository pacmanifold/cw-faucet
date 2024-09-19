# Prefunded CosmWasm Faucet

This contract implements a prefunded faucet for one denom. The denom in question MUST be sent to the contract on instantiation and cannot later be changed to another denom. The contract implements [cw-ownable](https://github.com/larry0x/cw-plus-plus/tree/main/packages/ownable) and the owner is set to `info.sender` in the instantiation. The two parameters `wait_time` and `claim_size` are updatable using `ExecuteMsg::UpdateConfig`.
