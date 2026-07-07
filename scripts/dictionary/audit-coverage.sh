#!/usr/bin/env bash
set -eu

root_dir="$(cd "$(dirname "$0")/../.." && pwd)"

audit_file() {
  local file_path="$1"

  awk '
    BEGIN {
      total = 0
    }

    /^  - slug:/ {
      total += 1
    }

    /time_context:/ {
      if ($0 ~ /(^|[^a-z_])early_morning([^a-z_]|$)/) early_morning += 1
      if ($0 ~ /(^|[^a-z_])morning([^a-z_]|$)/) morning += 1
      if ($0 ~ /(^|[^a-z_])noon([^a-z_]|$)/) noon += 1
      if ($0 ~ /(^|[^a-z_])afternoon([^a-z_]|$)/) afternoon += 1
      if ($0 ~ /(^|[^a-z_])late_afternoon([^a-z_]|$)/) late_afternoon += 1
      if ($0 ~ /(^|[^a-z_])evening([^a-z_]|$)/) evening += 1
      if ($0 ~ /(^|[^a-z_])night([^a-z_]|$)/) night += 1
    }

    /weather:/ {
      if ($0 ~ /(^|[^a-z_])clear([^a-z_]|$)/) clear += 1
      if ($0 ~ /(^|[^a-z_])cloudy([^a-z_]|$)/) cloudy += 1
      if ($0 ~ /(^|[^a-z_])rain([^a-z_]|$)/) rain += 1
      if ($0 ~ /(^|[^a-z_])drizzle([^a-z_]|$)/) drizzle += 1
      if ($0 ~ /(^|[^a-z_])humid([^a-z_]|$)/) humid += 1
      if ($0 ~ /(^|[^a-z_])snow([^a-z_]|$)/) snow += 1
    }

    /seasonality:/ {
      if ($0 ~ /(^|[^a-z_])spring([^a-z_]|$)/) spring += 1
      if ($0 ~ /(^|[^a-z_])summer([^a-z_]|$)/) summer += 1
      if ($0 ~ /(^|[^a-z_])autumn([^a-z_]|$)/) autumn += 1
      if ($0 ~ /(^|[^a-z_])winter([^a-z_]|$)/) winter += 1
    }

    END {
      printf "  entries: %d\n", total
      printf "  time_context: early_morning=%d morning=%d noon=%d afternoon=%d late_afternoon=%d evening=%d night=%d\n",
        early_morning, morning, noon, afternoon, late_afternoon, evening, night
      printf "  weather: clear=%d cloudy=%d rain=%d drizzle=%d humid=%d snow=%d\n",
        clear, cloudy, rain, drizzle, humid, snow
      printf "  seasonality: spring=%d summer=%d autumn=%d winter=%d\n",
        spring, summer, autumn, winter
    }
  ' "$file_path"
}

echo "FoundFrame dictionary coverage audit"
echo

for country_dir in "$root_dir"/dictionaries/countries/*; do
  country_code="$(basename "$country_dir")"
  echo "[$country_code]"

  for file_name in moments.yaml places.yaml object-details.yaml; do
    file_path="$country_dir/$file_name"
    if [ -f "$file_path" ]; then
      echo "- $file_name"
      audit_file "$file_path"
    fi
  done

  echo
done

echo "Heuristic only: counts come from inline YAML lists and help spot thin combinations quickly."
