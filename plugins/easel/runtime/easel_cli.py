"""Easel-owned deterministic media pipeline exposed through OpenMuse CLI."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont


def _font(size: int) -> ImageFont.FreeTypeFont | ImageFont.ImageFont:
    for candidate in ("/System/Library/Fonts/Supplemental/Arial.ttf", "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", "C:/Windows/Fonts/arial.ttf"):
        if Path(candidate).is_file():
            return ImageFont.truetype(candidate, size)
    return ImageFont.load_default()


def _cover(path: Path) -> None:
    canvas = Image.new("RGB", (1920, 1080), (11, 18, 31))
    draw = ImageDraw.Draw(canvas)
    for y in range(1080):
        draw.line((0, y, 1920, y), fill=(11 + y // 110, 18 + y // 100, 31 + y // 55))
    draw.rounded_rectangle((180, 170, 1740, 910), radius=42, fill=(21, 31, 48), outline=(64, 100, 135), width=3)
    icon = Image.open(Path(__file__).with_name("openmuse-icon.png")).convert("RGBA")
    icon.thumbnail((250, 250))
    canvas.paste(icon, (835, 260), icon)
    draw.text((960, 585), "OpenMuse", anchor="mm", font=_font(100), fill=(244, 249, 255))
    draw.text((960, 695), "CREATE · EXPLORE · SHARE", anchor="mm", font=_font(34), fill=(155, 198, 228))
    draw.rounded_rectangle((760, 800, 1160, 812), radius=6, fill=(83, 193, 234))
    path.parent.mkdir(parents=True, exist_ok=True)
    canvas.save(path)


def process(input_path: Path, output_path: Path, cover_path: Path | None) -> int:
    source = input_path.expanduser().resolve()
    destination = output_path.expanduser().resolve()
    if not source.is_file():
        raise FileNotFoundError(source)
    if destination == source:
        raise ValueError("output must differ from input")
    destination.parent.mkdir(parents=True, exist_ok=True)
    cover = cover_path.expanduser().resolve() if cover_path else destination.with_suffix(".cover.png")
    if cover_path is None:
        _cover(cover)
    elif not cover.is_file():
        raise FileNotFoundError(cover)
    probe = subprocess.run(["ffprobe", "-v", "error", "-select_streams", "a", "-show_entries", "stream=index", "-of", "csv=p=0", str(source)], capture_output=True, text=True, check=True)
    has_audio = bool(probe.stdout.strip())
    video_filter = "[0:v]fps=30,trim=duration=2,setpts=PTS-STARTPTS,scale=1920:1080,format=yuv420p[c];[1:v]fps=30,scale=1920:1080:force_original_aspect_ratio=decrease,pad=1920:1080:(ow-iw)/2:(oh-ih)/2,setsar=1,format=yuv420p,setpts=PTS-STARTPTS[v];[2:a]atrim=duration=2,asetpts=PTS-STARTPTS[ca]"
    if has_audio:
        video_filter += ";[1:a]aresample=48000,asetpts=PTS-STARTPTS[a];[c][ca][v][a]concat=n=2:v=1:a=1[outv][outa]"
    else:
        video_filter += ";[c][v]concat=n=2:v=1:a=0[outv]"
    command = ["ffmpeg", "-hide_banner", "-y", "-loop", "1", "-t", "2", "-i", str(cover), "-i", str(source), "-f", "lavfi", "-t", "2", "-i", "anullsrc=channel_layout=stereo:sample_rate=48000", "-filter_complex", video_filter, "-map", "[outv]"]
    if has_audio:
        command += ["-map", "[outa]", "-c:a", "aac", "-b:a", "192k"]
    command += ["-c:v", "libx264", "-preset", "medium", "-crf", "22", "-movflags", "+faststart", str(destination)]
    print(json.dumps({"stage": "rendering", "input": str(source), "cover": str(cover), "output": str(destination)}, ensure_ascii=False), flush=True)
    subprocess.run(command, check=True)
    print(json.dumps({"stage": "ready", "input": str(source), "cover": str(cover), "video": str(destination)}, ensure_ascii=False), flush=True)
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    commands = parser.add_subparsers(dest="command", required=True)
    video = commands.add_parser("process")
    video.add_argument("--input", required=True)
    video.add_argument("--output", required=True)
    video.add_argument("--cover")
    args = parser.parse_args()
    if args.command == "process":
        return process(Path(args.input), Path(args.output), Path(args.cover) if args.cover else None)
    return 2


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception as error:
        print(json.dumps({"stage": "error", "error": str(error)}, ensure_ascii=False), file=sys.stderr)
        sys.exit(1)
