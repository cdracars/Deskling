#!/usr/bin/env python3
"""Re-export the selected icon without changing its crop, colors or background.

Requires Pillow for asset maintenance only, not for building or running Deskling.
"""
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
SIZES = (16, 24, 32, 48, 64, 128, 256, 512)


def main():
    with Image.open(ROOT / "assets/icons/source/kipwick-icon.png") as source:
        if source.width != source.height:
            raise ValueError("The selected icon source must be square")
        for size in SIZES:
            target = ROOT / f"assets/icons/hicolor/{size}x{size}/apps/deskling.png"
            target.parent.mkdir(parents=True, exist_ok=True)
            source.resize((size, size), Image.Resampling.LANCZOS).save(target, optimize=True)
            print(target.relative_to(ROOT))


if __name__ == "__main__":
    main()
