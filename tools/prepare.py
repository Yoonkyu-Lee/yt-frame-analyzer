"""Prepare a YouTube video for frame-based analysis.

    python tools/prepare.py <url> [--mask x0,y0,x1,y1] [--th 0.004] [--segments 8]

Steps (each skipped if its output already exists, so re-runs are cheap):
  1. meta.json        title / uploader / date / duration / description
  2. video/video.mp4  best video-only stream up to 1440p (code must be legible)
  3. frames/          1 fps, full resolution
  4. diff.json        masked frame-to-frame change score
  5. keyframes/       frames whose accumulated change vs. the last kept frame > --th,
                      plus the last frame before every scene cut (resized to 1568px wide)
  6. segments/segN_frames.txt  keyframes split into N time-ordered chunks (1-frame overlap)

--mask hides a moving region (e.g. webcam overlay) from change detection, in
normalized coordinates. Without it, a webcam makes every frame look "changed".
Everything goes to work/<video-id>/ (cache, not committed).
"""
import argparse, glob, json, os, subprocess, sys

import cv2
import imageio_ffmpeg

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
FF = imageio_ffmpeg.get_ffmpeg_exe()
SMALL_W, SMALL_H, OUT_W, PIX_TH = 576, 360, 1568, 40


def ytdlp(*args):
    return subprocess.run([sys.executable, "-m", "yt_dlp", *args],
                          check=True, capture_output=True, text=True, encoding="utf-8").stdout


def small(path, mask):
    g = cv2.imread(path, cv2.IMREAD_GRAYSCALE)
    g = cv2.resize(g, (SMALL_W, SMALL_H), interpolation=cv2.INTER_AREA)
    if mask:
        x0, y0, x1, y1 = mask
        g[int(SMALL_H * y0):int(SMALL_H * y1), int(SMALL_W * x0):int(SMALL_W * x1)] = 0
    return g


def changed(a, b):
    return float((cv2.absdiff(a, b) > PIX_TH).mean())


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("url")
    ap.add_argument("--mask", help="x0,y0,x1,y1 normalized region to ignore (webcam)")
    ap.add_argument("--th", type=float, default=0.004, help="accumulated change to keep a keyframe")
    ap.add_argument("--segments", type=int, default=8)
    ap.add_argument("--max-height", type=int, default=1440)
    a = ap.parse_args()
    mask = tuple(map(float, a.mask.split(","))) if a.mask else None

    vid = ytdlp("--skip-download", "--print", "id", a.url).strip()
    work = os.path.join(ROOT, "work", vid)
    os.makedirs(work, exist_ok=True)
    print("work dir:", work)

    meta_p = os.path.join(work, "meta.json")
    if not os.path.exists(meta_p):
        m = json.loads(ytdlp("--skip-download", "-j", a.url))
        keep = ["id", "title", "uploader", "upload_date", "duration_string", "webpage_url", "description", "chapters"]
        json.dump({k: m.get(k) for k in keep}, open(meta_p, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
    meta = json.load(open(meta_p, encoding="utf-8"))
    print(f"[1] {meta['title']} | {meta['uploader']} | {meta['duration_string']}")

    video = os.path.join(work, "video", "video.mp4")
    if not os.path.exists(video):
        os.makedirs(os.path.dirname(video), exist_ok=True)
        ytdlp("-f", f"bv*[height<={a.max_height}][ext=mp4]/bv*[height<={a.max_height}]",
              "--ffmpeg-location", FF, "-o", video, a.url)
    print("[2] video:", video)

    fdir = os.path.join(work, "frames")
    if not glob.glob(os.path.join(fdir, "f_*.jpg")):
        os.makedirs(fdir, exist_ok=True)
        subprocess.run([FF, "-hide_banner", "-loglevel", "error", "-i", video, "-vf", "fps=1",
                        "-q:v", "2", os.path.join(fdir, "f_%04d.jpg")], check=True)
    files = sorted(glob.glob(os.path.join(fdir, "f_*.jpg")))
    print("[3] frames:", len(files))

    # diff + keyframes are always recomputed (cheap) so --mask / --th can be tuned
    smalls = [small(p, mask) for p in files]
    diffs = [1.0] + [changed(smalls[i], smalls[i - 1]) for i in range(1, len(smalls))]
    json.dump([{"t": i, "file": os.path.basename(p), "diff": round(d, 5)} for i, (p, d) in enumerate(zip(files, diffs))],
              open(os.path.join(work, "diff.json"), "w"), indent=0)
    print("[4] frames with change >0.1%:", sum(d > 0.001 for d in diffs))

    keep = {0, len(files) - 1}
    for t in range(1, len(files)):
        if diffs[t] > 0.2:  # scene cut: keep the state just before and after
            keep |= {t - 1, t}
    last = smalls[0]
    for t in range(1, len(files)):
        if t in keep or changed(smalls[t], last) > a.th:
            keep.add(t)
            last = smalls[t]
    kdir = os.path.join(work, "keyframes")
    os.makedirs(kdir, exist_ok=True)
    for old in glob.glob(os.path.join(kdir, "k_*.jpg")):
        os.remove(old)
    names = []
    for t in sorted(keep):
        im = cv2.imread(files[t])
        h, w = im.shape[:2]
        im = cv2.resize(im, (OUT_W, int(h * OUT_W / w)), interpolation=cv2.INTER_AREA)
        name = f"k_{t // 60:02d}m{t % 60:02d}s.jpg"
        cv2.imwrite(os.path.join(kdir, name), im, [cv2.IMWRITE_JPEG_QUALITY, 90])
        names.append(name)
    print("[5] keyframes:", len(names))

    sdir = os.path.join(work, "segments")
    os.makedirs(sdir, exist_ok=True)
    # a previous run with more segments would leave segK files pointing at old keyframes
    for old in glob.glob(os.path.join(sdir, "seg*_frames.txt")):
        os.remove(old)
    n, C = len(names), a.segments
    for i in range(C):
        lo, hi = i * n // C, (i + 1) * n // C
        seg = names[max(0, lo - 1):hi]
        open(os.path.join(sdir, f"seg{i + 1}_frames.txt"), "w").write("\n".join(seg))
        print(f"[6] seg{i + 1}: {seg[0]} .. {seg[-1]} ({len(seg)})")


if __name__ == "__main__":
    main()
