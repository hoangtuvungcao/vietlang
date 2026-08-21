#!/usr/bin/env bash
set -euo pipefail

url="${VIETLANG_LOAD_URL:-http://127.0.0.1:18080/health}"
requests="${VIETLANG_LOAD_REQUESTS:-1000}"
concurrency="${VIETLANG_LOAD_CONCURRENCY:-32}"
duration="${VIETLANG_SOAK_SECONDS:-0}"

run_batch() {
  seq 1 "$requests" | xargs -P "$concurrency" -I '{}' \
    curl --fail --silent --show-error --max-time 10 "$url" >/dev/null
}

started="$(date +%s)"
completed=0
while :; do
  run_batch
  completed=$((completed + requests))
  now="$(date +%s)"
  if [ "$duration" -eq 0 ] || [ $((now - started)) -ge "$duration" ]; then
    break
  fi
done
elapsed=$(( $(date +%s) - started ))
if [ "$elapsed" -lt 1 ]; then elapsed=1; fi
echo "PASS requests=$completed elapsed_s=$elapsed throughput_rps=$((completed / elapsed)) concurrency=$concurrency"
