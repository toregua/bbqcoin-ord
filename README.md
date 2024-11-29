# Bbqcoin Ord

ℹ️ This is a fork/based on [verydogelabs/wonky-ord-dogecoin](https://github.com/verydogelabs/wonky-ord-dogecoin)

You can see a running version here : [BBQ-ORD.COM](https://bbq-ord.com/) - Credits to Danny Glova


## API documentation
You can find the API documentation [here](openapi.yaml).
Most convenient way to view the API documentation is to use the [Swagger Editor](https://editor.swagger.io/).
You can import the `openapi.yaml` file and view the API documentation via Import URL: `https://raw.githubusercontent.com/toregua/ord-bbqcoin/main/openapi.yaml`.

## TL;DR How to run

### Preqrequisites
You will have to launch your own Bbqcoin node and have it fully synced. 

You can use the following way to set up your own Bbqcoin node:
1. Manually by following this documentation [install a bbqcoin node manually on unix](https://github.com/bbqcoin-community/bbqcoin/blob/master/doc/build-unix.md).
2. Using docker (recommanded way) following this repo and documentation [run a bbqcoin node inside a docker container](https://github.com/toregua/bbqcoin-node)

### Ord Parameters

`--index-transactions` will store transaction data, this is currently needed for `--index-bbq20` and furthermore helps
for a better performance for the API.

`--index-bbq20` will store bbqscriptions data with users balance, tick list & tick holders

`--index-bunes` will store bunes data (Bune is the same concept as RUNE on BTC on DUNE on Doge)

`--nr-parallel-requests` will configure how many parallel requests while indexing are sent to your RPC Server - 16 is
recommended for default node settings.

### Env variables

To have the indexer running well you have to specify some env variables.

First you have to create a `.env`  file (you can copy paste the `.env.example` to create it)

`FIRST_INSCRIPTION_HEIGHT` for now i have set the value to 0 to be sure to handle all inscriptions. If you have the right value, you can indicate it, which will improve indexing speed.

`FIRST_BUNE_HEIGHT` for now i am not sure anyone had already deployed a BUNE on bbqcoin so for now 0 also

`RPC_URL` it is here the more important part because you have to specify your node rpc url (http://user:pass@127.0.0.1:22555 for example)


## Start the bbq ord indexer / server in Docker
I recommand to use a docker image to run the ord indexer / server.

### Prerequisites Docker
1. Use ubuntu linux or a similar distribution
2. Install bbqcoin node and have it fully synced (recommanded way is [run a bbqcoin node inside a docker container](https://github.com/toregua/bbqcoin-node))
3. Install docker and docker-compose (Ubuntu)[https://docs.docker.com/engine/install/ubuntu/]
4. Clone this repository using git clone
5. Navigate inside the cloned repository folder

### Build the Docker image
```shell
docker-compose build
```
### Start the ord in a docker container in background
```shell
docker-compose up -d
```
### Logs access
```shell
docker-compose logs -f --tail 200
```

### Stop the ord in a docker container
When stopping the ord in a container it is important to add a timeout.
If no timeout is add, the process cannot close the database properly and the next start will take ages or fail.

```shell
docker-compose stop -t 600
docker-compose down
```