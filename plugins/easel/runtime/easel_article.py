"""Easel-owned validation of a Markdown article and its local image assets."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path


def inspect(article: Path) -> int:
    source = article.expanduser().resolve()
    if not source.is_file():
        raise FileNotFoundError(source)
    body = source.read_text(encoding="utf-8")
    title = next((line[2:].strip() for line in body.splitlines() if line.startswith("# ")), "")
    images = []
    missing = []
    for raw in re.findall(r"!\[[^]]*\]\(([^)]+)\)", body):
        image = (source.parent / raw).resolve()
        if not image.is_file():
            missing.append(str(image))
        images.append(str(image))
    result = {
        "stage": "ready" if title and not missing else "error",
        "article": str(source),
        "title": title,
        "imageCount": len(images),
        "images": images,
        "missingImages": missing,
        "richArticleSupported": True,
        "canPublishRequestedArticle": bool(title and not missing),
        "publishCommand": "openmuse easel zhihu publish-article --article <path> --exec" if images else "openmuse easel zhihu publish --title <title> --desc <body> --exec",
        "blockingIssue": None,
    }
    print(json.dumps(result, ensure_ascii=False), flush=True)
    return 0 if title and not missing else 1


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=["inspect"])
    parser.add_argument("--article", required=True)
    args = parser.parse_args()
    return inspect(Path(args.article))


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception as error:
        print(json.dumps({"stage": "error", "error": str(error)}, ensure_ascii=False), file=sys.stderr)
        sys.exit(1)
