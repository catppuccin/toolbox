#!/usr/bin/env ptython3

from PIL import Image, ImageOps, ImageDraw
import argparse
import os

parser = argparse.ArgumentParser()
parser.add_argument(
    "-p",
    "--preview",
    help="Preview the image (saves only as a temporary file)",
    action="store_true",
)
parser.add_argument(
    "-b",
    "--background",
    help="Draw on a solid background colour, specify the hex code",
    type=str,
)
parser.add_argument("-m", "--margin", help="Margin", type=int, default=40)
parser.add_argument(
    "-o", "--output", help="Output file", type=str, default="out/res.png"
)
parser.add_argument("latte", help="Latte screenshot")
parser.add_argument("frappe", help="Frappe screenshot")
parser.add_argument("macchiato", help="Macchiato screenshot")
parser.add_argument("mocha", help="Mocha screenshot")
args = parser.parse_args()

# parse the 4 screenshots into an array
imgs = [args.latte, args.frappe, args.macchiato, args.mocha]

w, h = Image.open(args.latte).size  # Get size

# parse hex code into [r, g, b], stripping # if present
def parse_hex(hex):
    hex = hex.lstrip("#")
    return int(hex[0:2], 16), int(hex[2:4], 16), int(hex[4:6], 16)


masks = []
slices = [
    [0, 0, 0, h, w / 3, 0],
    [0, h, w / 3, 0, (w / 3) * 2, 0, w / 3, h],
    [w / 3, h, (w / 3) * 2, 0, w, 0, (w / 3) * 2, h],
    [(w / 3) * 2, h, w, 0, w, h],
]
for slice in slices:
    img = Image.new("RGB", (w, h), (0, 0, 0))
    draw = ImageDraw.Draw(img)
    draw.polygon(slice, fill=(255, 255, 255))
    masks.append(img)


def gen_masked(source, mask):
    mask = mask.convert("L")
    img = Image.open(source)

    output = ImageOps.fit(img, mask.size, centering=(0.5, 0.5))
    output.putalpha(mask)

    return output


def round_corners(im, rad=15):
    circle = Image.new("L", (rad * 2, rad * 2), 0)
    draw = ImageDraw.Draw(circle)
    draw.ellipse((0, 0, rad * 2, rad * 2), fill=255)
    alpha = Image.new("L", im.size, "white")
    w, h = im.size
    alpha.paste(circle.crop((0, 0, rad, rad)), (0, 0))
    alpha.paste(circle.crop((0, rad, rad, rad * 2)), (0, h - rad))
    alpha.paste(circle.crop((rad, 0, rad * 2, rad)), (w - rad, 0))
    alpha.paste(circle.crop((rad, rad, rad * 2, rad * 2)), (w - rad, h - rad))
    im.putalpha(alpha)
    return im


final = Image.new("RGBA", (w, h), (0, 0, 0))
for i, arg in enumerate(imgs):
    masked = gen_masked(arg, masks[i])
    final.paste(masked, (0, 0), masked)


if args.background:
    m = args.margin
    bg = Image.new("RGBA", (w + m, h + m), parse_hex(args.background))
    bg.paste(round_corners(final), (int(m / 2), int(m / 2)), round_corners(final))
    final = bg

if args.preview:
    final.show()
else:
    basedir = os.path.dirname(os.path.abspath(args.output))
    if not os.path.exists(basedir):
        os.makedirs(basedir)
    final.save(args.output)
