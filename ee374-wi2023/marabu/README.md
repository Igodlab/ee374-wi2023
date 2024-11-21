# Marabu [[EE-374](https://ee374.stanford.edu/protocol)]

Run backend client/node server in a terminal:

```bash
cargo watch -q -c -w src/ -x run 
```

Run initial (local) tests in another terminal:

```bash
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```
which should act as a complement quick test step to connect to our client
