#!/bin/sh

export DOCKER_BUILDKIT=1
export COMPOSE_DOCKER_CLI_BUILD=1
docker-compose -f ./docker-compose-app.yaml -f ./docker-compose-infrastructure.yaml "$@"
