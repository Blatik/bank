#!/usr/bin/env bash
set -euo pipefail

# setup_github_and_secrets.sh
# Creates GitHub repo from current directory (using gh CLI), pushes, and sets CF secrets.
# Requirements: gh (GitHub CLI), git, jq (optional)

usage(){
  cat <<EOF
Usage: $0 <repo-name> [--public]

Env vars (preferred):
  CF_API_TOKEN    (required) Cloudflare API token to store as GitHub secret
  CF_ACCOUNT_ID   (required) Cloudflare Account ID to store as GitHub secret

Example:
  CF_API_TOKEN="..." CF_ACCOUNT_ID="..." $0 my-org/my-repo --public
EOF
  exit 1
}

if ! command -v gh >/dev/null 2>&1; then
  echo "gh (GitHub CLI) not found. Install from https://cli.github.com/" >&2
  exit 1
fi

if [ "$#" -lt 1 ]; then
  usage
fi

REPO="$1"
PUBLIC_FLAG="--private"
shift
if [ "${1-}" = "--public" ]; then
  PUBLIC_FLAG="--public"
fi

if [ -z "${CF_API_TOKEN-}" ] || [ -z "${CF_ACCOUNT_ID-}" ]; then
  echo "Please export CF_API_TOKEN and CF_ACCOUNT_ID environment variables before running." >&2
  usage
fi

echo "Creating GitHub repo $REPO and pushing current directory..."

# Create repo and push
gh repo create "$REPO" $PUBLIC_FLAG --source=. --remote=origin --push --confirm

echo "Setting GitHub Actions secrets: CF_API_TOKEN and CF_ACCOUNT_ID"
echo "$CF_API_TOKEN" | gh secret set CF_API_TOKEN --repo "$REPO" --body -
echo "$CF_ACCOUNT_ID" | gh secret set CF_ACCOUNT_ID --repo "$REPO" --body -

echo "Done. The repository has been created and secrets stored. CI will run on push to main/master." 

echo "If you need to trigger a manual deploy, run:"
echo "  gh workflow run -R $REPO build-and-deploy"
