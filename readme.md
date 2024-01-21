# Introducing Currrychain

Curry is the future*. This repo lays the foundation for a decentralized financial system secured with cryptography. Insert other marketing stuff here.

*It's not actually the future. BTW this whole project is satire and I am building it for fun and practice. 

**Curry** is the cryptocurrency powing your transactions on the currychain. Everybody needs some curry. Without curry, you will not be able to make memecoins or NFTs on the currychain.**


**We do not (currently) support tokens or NFTs. 

## Keys and Transactions

Private keys are generated with the `bls_signatures` crate. A `Key` struct defines a convenient structure for the private key, public key, and address. 

The address is derived from the public key. The final form of the address is yet to be decided. But it will likely start with `curry` and contain a checksum of some kind. 

A `Transaction` started with this structure:

```Rust
pub struct Transaction {
    pub from: String,
    pub signature: String,
    pub to: String,
    pub sender_public_key: String,
    pub hash: String,
    pub amount: u128,
}
```

The combination of `from, to, amount` forms a message for a digital signature signed with the senders private key. The `from` is an address.

This is added to the `pending_transactions` collection to be verified on block confirmation.

Verification checks the `sender_public_key` for verifying the transaction.  
