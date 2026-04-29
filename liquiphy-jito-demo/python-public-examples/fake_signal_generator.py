"""Public-safe fake signal generator.

No network calls. No secrets. No production strategy.
"""

from __future__ import annotations

import json
from datetime import datetime, timezone


def build_fake_signal() -> dict[str, object]:
    return {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "source": "mock-shredstream",
        "mint_alias": "DEMO_MINT_A",
        "signal": "demo_watch",
        "score": 64,
        "dry_run": True,
        "notes": "Synthetic signal only. Not trading advice.",
    }


if __name__ == "__main__":
    print(json.dumps(build_fake_signal(), indent=2))

