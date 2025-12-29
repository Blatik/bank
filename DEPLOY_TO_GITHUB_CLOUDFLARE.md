# Deploy to GitHub & Cloudflare

This guide explains how to push the repository to GitHub and deploy automatically to Cloudflare using the provided GitHub Actions workflow (`.github/workflows/deploy.yml`).

Required GitHub Actions secrets:

- `CF_API_TOKEN` — Cloudflare API token with `Workers:Edit` (and/or `Pages` as needed).
- `CF_ACCOUNT_ID` — Cloudflare account id.

1) Initialize repo locally and push to GitHub (replace `<your-remote-url>`):

```bash
git init
git add .
git commit -m "Initial commit: add workflow for Cloudflare deploy"
git branch -M main
git remote add origin <your-remote-url>
git push -u origin main
```

2) Add secrets to GitHub repository Settings → Secrets and variables → Actions:

- `CF_API_TOKEN` = your API token
- `CF_ACCOUNT_ID` = your account id

3) What the workflow does

- On push to `main` or `master`, the workflow:
  - checks out code
  - sets up Node (for `wrangler`) and Rust
  - builds the Rust backend (`/backend`) and frontend (`/frontend`) — adjust if you use non-Rust frontend
  - installs `wrangler` and runs `wrangler publish` (uses `wrangler.toml`)

4) Local manual publish (optional)

```bash
# Login interactively
wrangler login

# Or publish with token
export CF_API_TOKEN="<your-token>"
wrangler publish --api-token "$CF_API_TOKEN"
```

Notes & troubleshooting

- If your frontend is a Yew/Trunk app, install `trunk` in CI or change the workflow to `cargo install trunk` before building.
- If the Worker code lives in a subdirectory, set `working-directory` in the workflow or move `wrangler.toml` to the correct place.
- Confirm `wrangler.toml` has the correct `account_id` or rely on the `CF_ACCOUNT_ID` secret.

If you want, I can:

- create the GitHub repo for you and push (I will need credentials or a PAT), or
- further customize the CI build steps to match exact build commands in `/frontend` and `/backend`.

Helper scripts

I added two helper scripts in `./scripts/`:

- `setup_github_and_secrets.sh` — creates a GitHub repo using `gh`, pushes current directory, and sets `CF_API_TOKEN` and `CF_ACCOUNT_ID` secrets. Requires `gh` CLI.
- `publish_with_wrangler.sh` — runs `wrangler publish` using `CF_API_TOKEN` env var.

Example usage (from repo root):

```bash
# Export required env vars
export CF_API_TOKEN="<your-cloudflare-token>"
export CF_ACCOUNT_ID="<your-account-id>"

# Create repo and push (replace my-org/my-repo)
./scripts/setup_github_and_secrets.sh my-org/my-repo --public

# Optionally publish locally
./scripts/publish_with_wrangler.sh
```

