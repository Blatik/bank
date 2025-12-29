#!/usr/bin/env bash
set -euo pipefail

# publish_with_wrangler.sh
# Publishes Cloudflare Worker using wrangler and CF_API_TOKEN env var.

if ! command -v wrangler >/dev/null 2>&1; then
  echo "wrangler not found. Install via: npm install -g wrangler" >&2
  exit 1
fi

if [ -z "${CF_API_TOKEN-}" ]; then
  echo "Please export CF_API_TOKEN environment variable before running." >&2
  exit 1
fi

echo "Publishing with wrangler using provided CF_API_TOKEN..."
wrangler publish --api-token "$CF_API_TOKEN"

echo "Publish finished. Check worker URL in wrangler.toml or in Cloudflare dashboard." 
