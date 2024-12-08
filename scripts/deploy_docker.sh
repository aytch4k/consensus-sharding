#!/bin/bash

# Script to build and deploy Docker containers

echo "Building Docker images..."
docker-compose build

echo "Starting Docker containers..."
docker-compose up -d

echo "Deployment complete! Use 'docker-compose ps' to check running containers."
