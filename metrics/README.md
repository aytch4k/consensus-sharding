metrics/
Purpose: Monitoring and performance visualization configurations.
Files:
prometheus.yml: Prometheus configuration for collecting metrics.
grafana-dashboard.json: Grafana dashboard definition for visualizing TPS and latency.


// Prometheus Configuration:
scrape_interval: Prometheus queries metrics every 15 seconds.
Targets: Each shard node and the transaction router are defined as scrape targets.

//Grafana Dashboard Panels:
Total Transactions Processed:
Aggregates processed_transactions_total metric across all shards.
Displays the cumulative count of processed transactions.

Transactions Per Second (TPS):
Computes the rate of processed_transactions_total over a 1-minute interval.
Visualizes TPS trends across the network.

Shard Latency:
Displays processing latency (shard_processing_latency_milliseconds) for each shard.

Failed Transactions:
Shows the total count of failed transactions from failed_transactions_total.