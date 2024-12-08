import hashlib
import requests

def hash_sender_id(sender_id):
    """
    Generate a hash of the sender ID to determine the shard.
    """
    return int(hashlib.sha256(sender_id.encode()).hexdigest(), 16)

def forward_transaction_to_shard(transaction, shard_url):
    """
    Forward a transaction to the appropriate shard and return the response.
    """
    try:
        response = requests.post(f"{shard_url}/process", json=transaction)
        return response.json(), response.status_code
    except requests.exceptions.RequestException as e:
        return {"status": "failed", "reason": str(e)}, 500
