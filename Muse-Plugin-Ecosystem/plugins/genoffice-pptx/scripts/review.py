"""Apply a market review decision after checking a pending submission."""

import argparse
import getpass
import json
import os
from urllib.request import Request, urlopen


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("submission_id")
    parser.add_argument("action", choices=["approve", "reject", "delist", "relist"])
    parser.add_argument("--registry", required=True, help="Private market admin origin")
    parser.add_argument("--note", required=True)
    parser.add_argument("--token", default=os.environ.get("MARKET_REVIEWER_TOKEN"))
    args = parser.parse_args()
    token = args.token or getpass.getpass("Reviewer token: ")
    if not token:
        parser.error("Reviewer token required")
    body = json.dumps({"note": args.note}).encode()
    url = f"{args.registry.rstrip('/')}/internal/submissions/{args.submission_id}/{args.action}"
    request = Request(url, data=body, method="POST", headers={
        "Authorization": f"Bearer {token}",
        "Content-Type": "application/json",
    })
    with urlopen(request, timeout=30) as response:
        print(response.read().decode())


if __name__ == "__main__":
    main()
