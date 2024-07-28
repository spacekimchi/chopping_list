#!/usr/bin/env bash
set -x
set -eo pipefail

cleanup() {
    # Get the container ID of the running PostgreSQL container
    RUNNING_POSTGRES_CONTAINER=$(docker ps --filter 'name=chopping_list_pg' --format '{{.ID}}')
    if [[ -n $RUNNING_POSTGRES_CONTAINER ]]; then
        echo >&2 "Stopping and removing the running PostgreSQL container..."
        docker stop "${RUNNING_POSTGRES_CONTAINER}"
        docker rm "${RUNNING_POSTGRES_CONTAINER}"
        echo >&2 "PostgreSQL container removed."
    else
        echo >&2 "No running PostgreSQL container found."
    fi

    # Remove unused volumes
    echo >&2 "Removing unused Docker volumes..."
    docker volume prune -f

    # Remove unused images
    echo >&2 "Removing unused Docker images..."
    docker image prune -f

    echo >&2 "Cleanup complete."
}

# Run the cleanup function
cleanup
