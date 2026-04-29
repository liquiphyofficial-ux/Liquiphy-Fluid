"""Safe config loader for public demo placeholders."""

from __future__ import annotations

import os


REQUIRED_DRY_RUN = "true"


def load_config() -> dict[str, str]:
    dry_run = os.getenv("DRY_RUN", REQUIRED_DRY_RUN).lower()
    if dry_run != REQUIRED_DRY_RUN:
        raise RuntimeError("This public demo must run with DRY_RUN=true.")

    return {
        "jito_endpoint": os.getenv("JITO_ENDPOINT", "placeholder"),
        "block_engine_url": os.getenv("JITO_BLOCK_ENGINE_URL", "placeholder"),
        "rpc_url": os.getenv("RPC_URL", "placeholder"),
        "wss_url": os.getenv("WSS_URL", "placeholder"),
        "dry_run": dry_run,
    }


if __name__ == "__main__":
    config = load_config()
    for key in sorted(config):
        value = config[key]
        display = value if value == "true" or value == "placeholder" else "<redacted>"
        print(f"{key}={display}")

