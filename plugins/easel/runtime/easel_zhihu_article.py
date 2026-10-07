"""Easel-owned Markdown-to-Zhihu article publisher.

The Host sees only this declared CLI contribution. Browser selectors, profile
and platform readback are intentionally kept in the optional plugin.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class ArticleBlock:
    kind: str
    value: str
    caption: str = ""


def _display_text(line: str) -> str:
    if line == "---" or re.fullmatch(r"\|[\s:|-]+\|", line):
        return ""
    if line.startswith("|") and line.endswith("|"):
        line = " · ".join(cell.strip() for cell in line.strip("|").split("|"))
    line = re.sub(r"^#{2,6}\s+", "", line)
    line = re.sub(r"^>\s*", "", line)
    line = re.sub(r"^[-*]\s+", "• ", line)
    line = re.sub(r"\[([^]]+)\]\(([^)]+)\)", r"\1（\2）", line)
    return line.replace("**", "").replace("`", "").strip("*")


def parse_article(path: Path) -> tuple[str, list[ArticleBlock]]:
    article = path.expanduser().resolve()
    source = article.read_text(encoding="utf-8")
    title = next((line[2:].strip() for line in source.splitlines() if line.startswith("# ")), "")
    if not title:
        raise ValueError("文章缺少一级标题")
    blocks: list[ArticleBlock] = []
    image = re.compile(r"^!\[([^]]*)\]\(([^)]+)\)$")
    for raw in source.splitlines():
        line = raw.strip()
        if not line or line == f"# {title}":
            continue
        match = image.fullmatch(line)
        if match:
            media = (article.parent / match.group(2)).resolve()
            if not media.is_file() or media.suffix.lower() not in {".png", ".jpg", ".jpeg", ".webp"}:
                raise ValueError(f"图片不可用：{media}")
            blocks.append(ArticleBlock("image", str(media), match.group(1)))
        else:
            rendered = _display_text(line)
            if rendered:
                blocks.append(ArticleBlock("text", rendered))
    if not any(block.kind == "image" for block in blocks):
        raise ValueError("当前命令只接受含图片的图文稿；纯文本使用 zhihu publish")
    return title, blocks


def _editor_image_count(page) -> int:
    return page.locator(".public-DraftEditor-content img, [contenteditable=true] img").count()


def _upload_image(page, media: str, before: int) -> None:
    # Zhihu's persistent hidden image input feeds its asset library. It does
    # not reliably insert into the article. Use the editor's upload dialog and
    # explicitly insert the uploaded asset after its thumbnail is ready.
    trigger = page.get_by_role("button", name="图片", exact=True)
    trigger.click(timeout=10000)
    file_input = page.locator('input[type=file][accept="image/*"]').last
    file_input.wait_for(state="attached", timeout=10000)
    file_input.set_input_files(media)
    page.get_by_text(re.compile(r"已上传\s*1\s*张图片")).wait_for(timeout=30000)
    page.wait_for_timeout(2000)
    page.get_by_role("button", name="插入图片", exact=True).click(timeout=10000)
    page.wait_for_function(
        "count => document.querySelectorAll('.public-DraftEditor-content img, [contenteditable=true] img').length > count",
        arg=before,
        timeout=45000,
    )


def _receipt_path(workspace: Path, article: Path) -> Path:
    digest = hashlib.sha256(article.read_bytes()).hexdigest()
    return workspace / "state" / f"zhihu-article-{digest}.json"


def publish(article: Path, execute: bool, force: bool = False) -> int:
    title, blocks = parse_article(article)
    images = [block for block in blocks if block.kind == "image"]
    if not execute:
        print(json.dumps({
            "stage": "preview_required",
            "title": title,
            "imageCount": len(images),
            "blocks": [{"kind": b.kind, "value": b.caption if b.kind == "image" else b.value} for b in blocks],
            "effect": "network.publish",
        }, ensure_ascii=False), flush=True)
        return 0

    workspace = Path(os.environ["OPENMUSE_PLUGIN_WORKSPACE"]).resolve()
    receipt = _receipt_path(workspace, article.expanduser().resolve())
    if receipt.is_file() and not force:
        previous = json.loads(receipt.read_text(encoding="utf-8"))
        print(json.dumps({"stage": "already_submitted", **previous}, ensure_ascii=False), flush=True)
        return 0
    from playwright.sync_api import sync_playwright

    sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "shared" / "scripts"))
    import content_guard

    content_guard.guard_or_die(
        [title, "\n".join(block.value for block in blocks if block.kind == "text")],
        exec_mode=True,
        allow_unsafe=False,
        label="知乎图文发布内容",
    )
    os.environ["PLAYWRIGHT_BROWSERS_PATH"] = str(workspace / "runtime" / "browsers")
    profile = workspace / "profiles" / "ZhihuProfile"
    profile.mkdir(parents=True, exist_ok=True)
    with sync_playwright() as playwright:
        browser = playwright.chromium.launch_persistent_context(
            str(profile), headless=True, locale="zh-CN", args=["--no-proxy-server"]
        )
        try:
            page = browser.pages[0] if browser.pages else browser.new_page()
            page.goto("https://zhuanlan.zhihu.com/write", wait_until="domcontentloaded", timeout=30000)
            title_input = page.locator(".WriteIndex-titleInput textarea, textarea[placeholder*='标题']").first
            title_input.wait_for(timeout=15000)
            editor = page.locator(".public-DraftEditor-content, [contenteditable=true]").first
            editor.wait_for(timeout=15000)
            title_input.fill(title)
            uploaded = 0
            for block in blocks:
                if block.kind == "text":
                    editor.click()
                    page.keyboard.press("ControlOrMeta+End")
                    page.keyboard.insert_text(block.value)
                    page.keyboard.press("Enter")
                    continue
                before = _editor_image_count(page)
                _upload_image(page, block.value, before)
                uploaded += 1
                if block.caption:
                    editor.click()
                    page.keyboard.press("ControlOrMeta+End")
                    page.keyboard.press("Enter")
                    page.keyboard.insert_text(block.caption)
                    page.keyboard.press("Enter")
                print(json.dumps({"stage": "image_uploaded", "count": uploaded, "total": len(images)}, ensure_ascii=False), flush=True)

            observed = _editor_image_count(page)
            if observed < len(images):
                raise RuntimeError(f"发布前图片校验失败：需要 {len(images)} 张，编辑器只有 {observed} 张")
            body = editor.inner_text()
            first_line = next((block.value for block in blocks if block.kind == "text"), "")
            if first_line and first_line[:24] not in body:
                raise RuntimeError("发布前正文校验失败")
            print(json.dumps({"stage": "submitting", "title": title, "imageCount": observed}, ensure_ascii=False), flush=True)
            page.get_by_role("button", name="发布", exact=True).click(timeout=10000)
            page.wait_for_url(re.compile(r"https://zhuanlan\.zhihu\.com/p/\d+(?:\?.*)?$"), timeout=30000)
            url = page.url
            receipt.parent.mkdir(parents=True, exist_ok=True)
            saved = {"url": url, "title": title, "imageCount": observed}
            temporary = receipt.with_suffix(".tmp")
            temporary.write_text(json.dumps(saved, ensure_ascii=False), encoding="utf-8")
            temporary.replace(receipt)
            page.wait_for_timeout(2000)
            published_images = page.locator("article img, .Post-RichText img, .RichText img").count()
            verified = published_images >= len(images)
            print(json.dumps({
                "stage": "published" if verified else "submitted",
                "url": url,
                "imageCount": observed,
                "readbackImageCount": published_images,
                "readback": "verified" if verified else "unavailable",
                "message": "知乎已返回文章地址；网页回读受限，请在知乎 App 核查" if not verified else "",
            }, ensure_ascii=False), flush=True)
            return 0
        finally:
            browser.close()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--article", required=True)
    parser.add_argument("--exec", action="store_true")
    parser.add_argument("--force", action="store_true", help="明确允许重复提交同一份文章")
    args = parser.parse_args()
    return publish(Path(args.article), args.exec, args.force)


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception as error:
        print(json.dumps({"stage": "error", "error": str(error)}, ensure_ascii=False), file=sys.stderr, flush=True)
        sys.exit(1)
