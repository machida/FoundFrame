#!/usr/bin/env bash
set -eu

find src-tauri/migrations -maxdepth 1 -type f | sort
