# Indy playground

[Hyperledger Indy](https://www.hyperledger.org/projects/hyperledger-indy) playground.

## Instruction

1. Build indy pools docker image:

```
docker build -t indypool --build-arg pool_id=<your_ip_addr> -f indy-pool.dockerfile
```

2. Run indy pools docker image:

```
docker run --name=indy -p <your_ip_addr>:9701-9708:9701-9708 -itd indy-pool
```

3. Extract the pool genesis

```
docker exec -it indy cat /var/lib/indy/sandbox/pool_transactions_genesis > pool_genesis.json
```

4. Build and run the code

```
cargo run
```
