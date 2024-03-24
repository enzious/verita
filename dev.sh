#!/bin/bash

case "$1" in
  start)
    pushd ./dev
    docker compose up -d
    ;;

  stop)
    pushd ./dev
    docker compose down
    ;;

  destroy)
    pushd ./dev
    docker compose down
    rm -r verita-db-data
    ;;

  *)
    echo "Invalid action."
    exit 1
    ;;
esac
