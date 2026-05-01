#!/usr/bin/env bash
set -e

IMAGE_NAME="baltimore-homicide-dashboard"
CONTAINER_NAME="baltimore-homicide-dashboard-container"
HOST_PORT="3838"
CONTAINER_PORT="3838"

echo "Building Docker image: ${IMAGE_NAME}"
docker build -t "${IMAGE_NAME}" .

if [ "$(docker ps -aq -f name=^${CONTAINER_NAME}$)" ]; then
  echo "Removing existing container: ${CONTAINER_NAME}"
  docker rm -f "${CONTAINER_NAME}"
fi

echo "Starting dashboard container..."
docker run -d \
  --name "${CONTAINER_NAME}" \
  -p "${HOST_PORT}:${CONTAINER_PORT}" \
  "${IMAGE_NAME}"

echo "Dashboard running at http://localhost:${HOST_PORT}"
echo "To view logs: docker logs -f ${CONTAINER_NAME}"
echo "To stop it: docker stop ${CONTAINER_NAME}"
