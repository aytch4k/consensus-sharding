shard-config/
Purpose: Stores shard-specific configuration files for customizing each shard node.
Files:
shard1-config.json: Configuration for Shard 1.
shard2-config.json: Configuration for Shard 2.
shard3-config.json: Configuration for Shard 3.

Key Configuration Parameters
shard_id:
A unique identifier for the shard.

port:
The port on which the shard listens for API requests.

validators:
The number of validators participating in the shard's PBFT consensus.

max_transactions_per_block:
The maximum number of transactions a block can contain.

block_time_milliseconds:
The time interval (in milliseconds) between block proposals.

state_storage:
The type of storage backend for shard state data (e.g., RocksDB, LevelDB).