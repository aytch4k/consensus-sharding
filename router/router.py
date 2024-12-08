from flask import Flask, request, jsonify
from prometheus_flask_exporter import PrometheusMetrics
import hashlib
import requests
import time

app = Flask(__name__)
metrics = PrometheusMetrics(app)

# Shard endpoints
shards = {
    1: "http://shard1:8001",
    2: "http://shard2:8002",
    3: "http://shard3:8003"
}

# Prometheus custom metrics
total_routed_transactions = metrics.counter(
    "total_routed_transactions",
    "Total transactions routed to shards"
)
failed_routing_transactions = metrics.counter(
    "failed_routing_transactions",
    "Total failed transaction routing attempts"
)
routing_latency = metrics.summary(
    "routing_latency_seconds",
    "Time spent routing transactions"
)

def get_shard_id(sender_id):
    hash_value = int(hashlib.sha256(sender_id.encode()).hexdigest(), 16)
    return (hash_value % len(shards)) + 1

@app.route('/route', methods=['POST'])
@total_routed_transactions.count_exceptions()
@routing_latency.time()
def route_transaction():
    data = request.json
    sender_id = data.get("sender_id")
    if not sender_id:
        failed_routing_transactions.inc()
        return jsonify({"status": "failed", "reason": "Missing sender_id"}), 400

    shard_id = get_shard_id(sender_id)
    shard_url = shards[shard_id]

    try:
        response = requests.post(f"{shard_url}/process", json=data)
        return jsonify({"status": "routed", "shard_id": shard_id, "response": response.json()}), response.status_code
    except Exception as e:
        failed_routing_transactions.inc()
        return jsonify({"status": "failed", "reason": str(e)}), 500

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8080)
