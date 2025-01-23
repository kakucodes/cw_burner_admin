# CW Burner Admin

A specialized CosmWasm contract designed to serve as a tokenfactory admin that only allows burning tokens, preventing any minting operations. This contract is particularly useful for managing deflationary token mechanisms or implementing controlled token burning strategies.

## Features

- **Secure Token Burning**: Allows burning of tokenfactory tokens while preventing minting capabilities
- **Balance Tracking**: Maintains a record of all burned token amounts per denom
- **Multi-Token Support**: Can handle multiple token denominations simultaneously
- **Permissionless Burns**: Any user can trigger burns of tokens held by the contract
- **Query Support**: Provides query endpoints to check total burned amounts per denom

## Execute Messages

The contract supports the following execute messages:

#### BurnBalance

Burns all tokens of a specific denomination held by the contract:

```rust
ExecuteMsg::BurnBalance {
    denom: String
}
```

This message is payable, allowing users to send tokens along with the message for immediate burning.

## Query Messages

#### AmountBurned

Queries the total amount of tokens burned for a specific denomination:

```rust
QueryMsg::AmountBurned {
    denom: String
}
```

Returns a `Coin` struct containing the total burned amount and denomination.

## Setup as Token Admin

To use this contract as a token burner:

1. Either deploy a new instance of the contract or use an already instantiated instance
2. Create your tokenfactory denomination
3. Transfer admin rights of your token to the contract address using `MsgChangeAdmin`
4. Send tokens to the contract
5. Call `BurnBalance` to burn the tokens

## Security Considerations

- Once admin rights are transferred to this contract, they cannot be transferred again
- The contract cannot mint new tokens, only burn existing ones
- Anyone can trigger burns of tokens held by the contract
- The contract maintains an accurate tally of all burned tokens
