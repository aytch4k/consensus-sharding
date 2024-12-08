scripts/
Purpose: Helper scripts for running local tests, loading transactions, and deploying Docker containers.
Files:
run_local_test.sh: Shell script to start a local environment for testing.
load_test_transactions.py: Script to simulate transaction loads for performance testing.
deploy_docker.sh: Automates Docker deployment and setup.


esting Instructions
Deploy the Environment: 

Run the run_local_test.sh script:

'''
bash scripts/run_local_test.sh
'''
Monitor the Logs: Use Docker logs to monitor transactions being processed by shards:

'''
docker-compose logs -f
'''
Check Metrics: Access the /metrics endpoint for the router and shards to verify Prometheus metrics:

Router: http://localhost:8080/metrics
Shards: http://localhost:8001/metrics, 
        http://localhost:8002/metrics,
        http://localhost:8003/metrics, etc.