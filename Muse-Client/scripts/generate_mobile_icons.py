#!/usr/bin/env python3
"""Generate the mobile launcher icons from the macOS app icon.

The macOS asset is a rounded tile inset inside a 1024x1024 canvas: the artwork
occupies 968x968 with a 28px transparent margin and rounded corners. iOS and
Android both expect a square, fully opaque icon and apply their own mask, so a
straight downscale of that asset would ship transparent corners (rejected for
the iOS marketing icon, and visible as dark corners elsewhere).

This script crops the margin away and extends each row's edge colour into the
rounded corners before downscaling. Extending per row rather than filling with
one flat colour keeps the tile's vertical gradient intact, so the filled corner
area is indistinguishable from the original artwork.

Usage:
    python3 scripts/generate_mobile_icons.py
"""

from __future__ import annotations

from pathlib import Path

from PIL import Image

REPO = Path(__file__).resolve().parent.parent
SOURCE = (
    REPO
    / 'app/openmuse_host/macos/Runner/Assets.xcassets/AppIcon.appiconset'
    / 'app_icon_1024.png'
)
ANDROID_DIR = REPO / 'app/openmuse_mobile/android/app/src/main/res'
IOS_DIR = REPO / 'app/openmuse_mobile/ios/Runner/Assets.xcassets/AppIcon.appiconset'

# Android launcher icons, one file per density bucket.
ANDROID_TARGETS = {
    'mipmap-mdpi/ic_launcher.png': 48,
    'mipmap-hdpi/ic_launcher.png': 72,
    'mipmap-xhdpi/ic_launcher.png': 96,
    'mipmap-xxhdpi/ic_launcher.png': 144,
    'mipmap-xxxhdpi/ic_launcher.png': 192,
}

# iOS app icon set. The nibble/point sizes follow the asset catalogue: the
# filename encodes the point size and the scale factor.
IOS_TARGETS = {
    'Icon-App-20x20@1x.png': 20,
    'Icon-App-20x20@2x.png': 40,
    'Icon-App-20x20@3x.png': 60,
    'Icon-App-29x29@1x.png': 29,
    'Icon-App-29x29@2x.png': 58,
    'Icon-App-29x29@3x.png': 87,
    'Icon-App-40x40@1x.png': 40,
    'Icon-App-40x40@2x.png': 80,
    'Icon-App-40x40@3x.png': 120,
    'Icon-App-60x60@2x.png': 120,
    'Icon-App-60x60@3x.png': 180,
    'Icon-App-76x76@1x.png': 76,
    'Icon-App-76x76@2x.png': 152,
    'Icon-App-83.5x83.5@2x.png': 167,
    'Icon-App-1024x1024@1x.png': 1024,
}

# A pixel counts as a source for the corner fill once it is this opaque; softer
# pixels along the rounded edge stay antialiased and get composited on top.
OPAQUE = 250


def square_tile(source: Path) -> Image.Image:
    """Crop the transparent margin and return an opaque, full-bleed square."""
    tile = Image.open(source).convert('RGBA')
    bbox = tile.getchannel('A').getbbox()
    if bbox is None:
        raise SystemExit(f'{source} has no visible pixels')
    tile = tile.crop(bbox)

    width, height = tile.size
    if width != height:
        raise SystemExit(f'{source} is not square after cropping: {tile.size}')

    pixels = tile.load()
    background = Image.new('RGB', (width, height))
    background_pixels = background.load()

    previous: tuple[int, int, int] | None = None
    for y in range(height):
        first = last = None
        for x in range(width):
            if pixels[x, y][3] >= OPAQUE:
                if first is None:
                    first = x
                last = x
        if first is None:
            # Defensive: a row with no opaque pixel reuses the row above.
            if previous is None:
                raise SystemExit(f'{source} has an empty first row')
            for x in range(width):
                background_pixels[x, y] = previous
            continue
        for x in range(first):
            background_pixels[x, y] = pixels[first, y][:3]
        for x in range(first, last + 1):
            background_pixels[x, y] = pixels[x, y][:3]
        for x in range(last + 1, width):
            background_pixels[x, y] = pixels[last, y][:3]
        previous = pixels[last, y][:3]

    # Composite the original tile over the filled background so the rounded
    # edge keeps its antialiasing.
    return Image.alpha_composite(background.convert('RGBA'), tile).convert('RGB')


def write_icons(tile: Image.Image) -> list[Path]:
    written: list[Path] = []
    for directory, targets in ((ANDROID_DIR, ANDROID_TARGETS), (IOS_DIR, IOS_TARGETS)):
        for name, size in sorted(targets.items()):
            target = directory / name
            if not target.parent.is_dir():
                raise SystemExit(f'missing icon directory: {target.parent}')
            # RGB output: iOS rejects an alpha channel on the marketing icon.
            resized = tile.resize((size, size), Image.LANCZOS)
            resized.save(target, format='PNG', optimize=True)
            written.append(target)
    return written


def main() -> None:
    if not SOURCE.is_file():
        raise SystemExit(f'missing source icon: {SOURCE}')
    tile = square_tile(SOURCE)
    written = write_icons(tile)
    print(f'source: {SOURCE.relative_to(REPO)} ({tile.size[0]}x{tile.size[1]} tile)')
    for path in written:
        size = Image.open(path).size
        mode = Image.open(path).mode
        print(f'  {path.relative_to(REPO)}  {size[0]}x{size[1]}  {mode}')
    print(f'{len(written)} icons written')


if __name__ == '__main__':
    main()
