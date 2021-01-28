# query-node

The query-node project contains an input schema (schema.graphql) and mappings for the Joystream `content-directory` runtime module.

## Code generation

We use Hydra-cli to generate a graphql server and a block indexer for joystream chain:

```bash
$ cd query-node
$ yarn build
```

## Starting services

To start services defined in the project docker-compose.yml, you should run docker-compose from the project root folder to use the correct .env file

## Run mapping processor

Before running mappings make sure indexer(`yarn indexer:start`) and indexer-api-server (mappings get the chain data from this graphql server) are both running:

```bash
yarn processor:start
```

## Query data

Once processor start to store event data you will be able to query this data from `http://localhost:4002/graphql`.

```graphql
query {
  channels {
    handle
  }
}
```

## Dev workflow

Setup the development environment:
```
# optional - clear the environment
docker stop $(docker ps -aq) # make sure all docker containers are stopped (Warning: this command stops ALL containers on your machine!)
docker-compose down -v # remove all docker volumes
rm query-node/generated -r # remove any previously generated query-node code

# services setup
yarn workspace query-node-root build # install dependencies and build query node
docker-compose up -d db # start db container
yarn workspace query-node-root db:migrate # create db from schema
docker-compose up -d joystream-node # start node
docker-compose up -d redis # start caching db
docker-compose up -d indexer-api-gateway # startup api gateway
docker stop joystream_indexer_1 # turn off indexer running in docker (started as dependency in the previous command)

# run indexer in the first terminal
yarn workspace query-node-root indexer:start # start indexer

# run processor in the second terminal
DEBUG=* yarn workspace query-node-root processor:start

# continue in the third terminal
yarn workspace @joystream/cd-schemas initialize:dev # initialize Joystream schemas
yarn workspace @joystream/cd-schemas example:createChannel # create example channel - this change will take effect in ~90 sec
yarn workspace query-node-root server:start:dev # start graphql server


```

How to make change in input schema:
```
# first make a change to schema.graphql
# stop the indexer instance

docker-compose up -V -d db # start new clear db
rm query-node/generated -r # remove any previously generated query-node code
yarn workspace query-node-root build
yarn workspace query-node-root db:migrate # recreate db

# restart indexer
yarn workspace query-node-root indexer:start # start indexer
```
