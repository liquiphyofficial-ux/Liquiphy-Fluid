"""Generate fake Fluid dashboard data for a public demo."""

from __future__ import annotations

import json


def dashboard_payload() -> dict[str, object]:
    return {
        "system": "Liquiphy + Fluid public demo",
        "window": "15m",
        "events_observed": 128,
        "events_parsed": 121,
        "dry_run_sends": 7,
        "fake_outcomes": {
            "simulated_accepted": 5,
            "simulated_rejected": 2,
        },
        "warning": "Synthetic dashboard data only.",
    }


if __name__ == "__main__":
    print(json.dumps(dashboard_payload(), indent=2))

