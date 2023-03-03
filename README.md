# Blocklist 

This service is allowing to query IP addresses and check if they are on a blocklist or not.
It opens one `GET` endpoint at port `3000` on the following path `/ips/` which expects an ip address (or any other String) as a paramter. 

For example:
`http://localhost:3000/ips/192.168.178.1`

## Install
```bash
$ git clone git@github.com:gruberb/blocklist.git
$ cd blocklist
```
## Run

```bash
$ cargo run 
```

## Example usage

1. Run the server via `$ cargo run`
2. Open another terminal window and query the endpoint:

```bash
$ curl localhost:3000/ips/185.56.83.83
```

## Example output Server
```bash
$ cargo run
   Compiling blocklist v0.1.0 (/Users/gruberbastian/CodingIsFun/Code Challenges/Nomic/blocklist)
    Finished dev [unoptimized + debuginfo] target(s) in 1.07s
     Running `target/debug/blocklist`
2023-03-04T11:39:57.628613Z  INFO blocklist::blocklist: Fetch latest blocklist
2023-03-04T11:39:58.849870Z  INFO blocklist::store: Update in-memory blocklist store
2023-03-04T11:39:58.861350Z  INFO blocklist: Start the server at 127.0.0.1:3000
2023-03-04T11:39:58.861868Z  INFO blocklist: Check for pending scheduler tasks
2023-03-04T11:40:28.139669Z  INFO blocklist: Check for IP address id="185.56.83.83"
```

## Example output Client
```bash
$ curl localhost:3000/ips/185.56.83.83
true⏎
```
