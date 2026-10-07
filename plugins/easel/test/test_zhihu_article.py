import contextlib
import io
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch


sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "runtime"))
from easel_zhihu_article import _receipt_path, parse_article, publish  # noqa: E402


class ZhihuArticleTest(unittest.TestCase):
    def test_preserves_interleaved_local_images_and_removes_markdown_controls(self):
        with tempfile.TemporaryDirectory() as root:
            directory = Path(root)
            (directory / "figure.png").write_bytes(b"png")
            article = directory / "article.md"
            article.write_text(
                "# 标题\n\n## 章节\n\n**正文**\n\n![图注](figure.png)\n\n后文\n",
                encoding="utf-8",
            )
            title, blocks = parse_article(article)
            self.assertEqual(title, "标题")
            self.assertEqual([block.kind for block in blocks], ["text", "text", "image", "text"])
            self.assertEqual(blocks[0].value, "章节")
            self.assertEqual(blocks[1].value, "正文")
            self.assertEqual(blocks[2].value, str((directory / "figure.png").resolve()))

    def test_missing_image_blocks_publication(self):
        with tempfile.TemporaryDirectory() as root:
            article = Path(root) / "article.md"
            article.write_text("# 标题\n![图](missing.png)\n", encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "图片不可用"):
                parse_article(article)

    def test_existing_submission_is_not_published_again(self):
        with tempfile.TemporaryDirectory() as root:
            workspace = Path(root)
            article = workspace / "article.md"
            (workspace / "figure.png").write_bytes(b"png")
            article.write_text("# 标题\n![图](figure.png)\n", encoding="utf-8")
            receipt = _receipt_path(workspace, article)
            receipt.parent.mkdir()
            receipt.write_text(json.dumps({"url": "https://zhuanlan.zhihu.com/p/123"}))
            output = io.StringIO()
            with patch.dict(os.environ, {"OPENMUSE_PLUGIN_WORKSPACE": root}):
                with contextlib.redirect_stdout(output):
                    self.assertEqual(publish(article, execute=True), 0)
            self.assertEqual(json.loads(output.getvalue())["stage"], "already_submitted")


if __name__ == "__main__":
    unittest.main()
