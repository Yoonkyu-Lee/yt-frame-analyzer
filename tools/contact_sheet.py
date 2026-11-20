"""Make a 4x3 contact sheet of evenly spaced frames to inspect a video's layout.
    python tools/contact_sheet.py work/<id> [n=12]  -> work/<id>/contact.jpg"""
import sys, glob, os
from PIL import Image, ImageDraw
work = sys.argv[1]; n = int(sys.argv[2]) if len(sys.argv) > 2 else 12
files = sorted(glob.glob(os.path.join(work, "frames", "f_*.jpg")))
pick = [files[int(i * (len(files) - 1) / (n - 1))] for i in range(n)]
tw, th = 640, 400
sheet = Image.new("RGB", (tw * 4, th * ((n + 3) // 4)), "white")
for i, p in enumerate(pick):
    im = Image.open(p); im.thumbnail((tw, th))
    t = int(os.path.basename(p)[2:6]) - 1
    ImageDraw.Draw(im).text((8, 8), f"{t//60}:{t%60:02d}", fill=(255, 0, 0))
    sheet.paste(im, ((i % 4) * tw, (i // 4) * th))
sheet.save(os.path.join(work, "contact.jpg"), quality=85)
print(len(files), "frames ->", os.path.join(work, "contact.jpg"))
