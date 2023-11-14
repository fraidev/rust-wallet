create wallet;

```rust
fn main() {
    wallet = Wallet::<Tezos::TZ1>::generate();
    public_key = wallet.public_key();
    // tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx
    private_key = wallet.private_key();
    // edpkvGfJn3CN3J5x9oYkxJhGzJWxkK4TqGK1zJzgk6u1Z4qZ8b9
    public_key_hash = wallet.public_key_hash();
    // edskRcYmKvUvjNyKtQJhVUa1xXJH3xZLZ5Q8zrjBBP1jQgk3iY9qoW


    wallet = Wallet::<Tezos::TZ1>::import("edskRcYmKvUvjNyKtQJhVUa1xXJH3xZLZ5Q8zrjBBP1jQgk3iY9qoW");
    public_key = wallet.public_key();
    // edpkvGfJn3CN3J5x9oYkxJhGzJWxkK4TqGK1zJzgk6u1Z4qZ8b9
    public_key_hash = wallet.public_key_hash();
    // tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx
}
```
