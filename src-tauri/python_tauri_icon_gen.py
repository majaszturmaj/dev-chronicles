#!/usr/bin/env python3
from PIL import Image, ImageDraw, ImageFont
from pathlib import Path
import shutil
import subprocess

OUT = Path("icons")
OUT.mkdir(parents=True, exist_ok=True)

# sizes we will generate (px)
SIZES = [16, 32, 64, 128, 256, 512, 1024]

def create_icon_image(size: int, color="#4A90E2") -> Image.Image:
    img = Image.new("RGBA", (size, size), color)
    d = ImageDraw.Draw(img)
    text = "DC"
    # try to load a decent font, fall back to default
    try:
        font = ImageFont.truetype("DejaVuSans-Bold.ttf", int(size * 0.45))
    except Exception:
        font = ImageFont.load_default()
    # center text (small offset to look visually centered)
    text_w, text_h = d.textsize(text, font=font)
    pos = ((size - text_w) // 2, (size - text_h) // 2 - int(size * 0.03))
    d.text(pos, text, fill="white", font=font)
    return img

print("Generating PNGs...")
images = {}
for s in SIZES:
    img = create_icon_image(s)
    images[s] = img
    p = OUT / f"{s}x{s}.png"
    img.save(p)
    print("Saved:", p)

# Create exact files required by tauri.conf.json
# 32x32.png (already created), 128x128.png, 128x128@2x.png (256x256), 256x256.png
shallow_files = {
    "32x32.png": images[32],
    "128x128.png": images[128],
    "128x128@2x.png": images[256],
    "256x256.png": images[256],
}
for name, img in shallow_files.items():
    path = OUT / name
    img.save(path)
    print("Saved:", path)

# Create ICO (Windows) with multiple sizes
ico_path = OUT / "icon.ico"
# For ICO include several sizes (Pillow handles packaging)
images_for_ico = [images[16], images[32], images[64], images[128], images[256]]
# Pillow needs a base image to save ICO; we'll use the largest and provide sizes list
images_for_ico[-1].save(ico_path, format="ICO", sizes=[(16,16),(32,32),(64,64),(128,128),(256,256)])
print("Saved:", ico_path)

# Create .iconset folder (macOS style) for conversion to .icns
iconset_dir = OUT / "icon.iconset"
if iconset_dir.exists():
    shutil.rmtree(iconset_dir)
iconset_dir.mkdir()

# mapping of iconset filenames -> pixel size
iconset_map = {
    "icon_16x16.png": 16,
    "icon_16x16@2x.png": 32,
    "icon_32x32.png": 32,
    "icon_32x32@2x.png": 64,
    "icon_128x128.png": 128,
    "icon_128x128@2x.png": 256,
    "icon_256x256.png": 256,
    "icon_256x256@2x.png": 512,
    "icon_512x512.png": 512,
    "icon_512x512@2x.png": 1024,
}

for fname, px in iconset_map.items():
    img = create_icon_image(px)
    dest = iconset_dir / fname
    img.save(dest)
    print("Saved:", dest)

# Try to auto-create .icns using png2icns if available (linux)
icns_path = OUT / "icon.icns"
try:
    # png2icns usage: png2icns <output.icns> <icon.iconset dir>
    res = subprocess.run(["png2icns", str(icns_path), str(iconset_dir)], capture_output=True, text=True)
    if res.returncode == 0:
        print("Created icon.icns via png2icns:", icns_path)
    else:
        print("png2icns returned non-zero (or not present). stdout/stderr:")
        print(res.stdout)
        print(res.stderr)
        print("You can convert icon.iconset -> icon.icns on macOS with:")
        print("  iconutil -c icns icons/icon.iconset")
except FileNotFoundError:
    print("png2icns not found. To create .icns on a Mac, run:")
    print("  iconutil -c icns icons/icon.iconset")
    print("Or install a Linux tool such as png2icns if available for your distro.")

print("All done. Files created in:", OUT.resolve())

