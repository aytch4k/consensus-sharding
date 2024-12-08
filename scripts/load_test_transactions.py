import requests
import random
import time

ROUTER_URL = "http://localhost:8080/route"

def generate_transaction(transaction_id):
    """
    Generate a mock transaction payload.
    """
    return {
        "transaction_id": f"tx-{transaction_id}",
        "sender_id": f"user-{random.randint(1, 100)}",
        "data": "Sample transaction data"
    }

def send_transaction(transaction):
    """
    Send a transaction to the router endpoint.
    """
    try:
        response = requests.post(ROUTER_URL, json=transaction)
        print(f"Transaction {transaction['transaction_id']} response: {response.status_code}")
        print(response.json())
    except Exception as e:
        print(f"Error sending transaction {transaction['transaction_id']}: {e}")

def main():
    for i in range(1, 51):  # Generate 50 test transactions
        transaction = generate_transaction(i)
        send_transaction(transaction)
        time.sleep(0.1)  # Simulate 100ms delay between transactions

if __name__ == "__main__":
    main()
