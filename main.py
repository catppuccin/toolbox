#!/usr/bin/env python3
# vim:ft=python:fenc=utf-8:fdm=marker

from PIL import Image, ImageOps, ImageDraw
import argparse
import os

# argparse {{{
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
parser.add_argument("-r", "--radius", help="Radius", type=int, default=50)
parser.add_argument(
    "-o", "--output", help="Output file", type=str, default="out/res.png"
)
parser.add_argument("latte", help="Latte screenshot")
parser.add_argument("frappe", help="Frappe screenshot")
parser.add_argument("macchiato", help="Macchiato screenshot")
parser.add_argument("mocha", help="Mocha screenshot")
args = parser.parse_args()
# }}}

# parse hex code into [r, g, b], stripping # if present
def parse_hex(hex):
    hex = hex.lstrip("#")
    return int(hex[0:2], 16), int(hex[2:4], 16), int(hex[4:6], 16)


def gen_masked(source, mask):
    img = Image.open(source)

    output = ImageOps.fit(img, mask.size, centering=(0.5, 0.5))
    output.putalpha(mask)

    return output


def round_mask(image, radius=40):
    rounded = Image.new("L", image.size, 0)
    draw = ImageDraw.Draw(rounded)
    draw.rounded_rectangle([(0, 0), image.size], radius, fill=255)
    image.putalpha(rounded)
    return image


if __name__ == "__main__":
    # parse the 4 screenshots into an array
    imgs = [args.latte, args.frappe, args.macchiato, args.mocha]
    # input size shorthands
    w, h = Image.open(args.latte).size

    # create the diagonal masks
    masks = []
    slices = [
        [0, 0, 0, h, w / 3, 0],
        [0, h, w / 3, 0, (w / 3) * 2, 0, w / 3, h],
        [w / 3, h, (w / 3) * 2, 0, w, 0, (w / 3) * 2, h],
        [(w / 3) * 2, h, w, 0, w, h],
    ]
    for slice in slices:
        img = Image.new("L", (w, h), 0)
        draw = ImageDraw.Draw(img)
        draw.polygon(slice, fill=255)
        masks.append(img)

    # make the composite image
    final = Image.new("RGBA", (w, h), (0, 0, 0))
    for i, arg in enumerate(imgs):
        masked = gen_masked(arg, masks[i])
        final.paste(masked, (0, 0), masked)

    # put it on a coloured background, if `--background` is passed
    if args.background:
        m = args.margin

        final = round_mask(final, args.radius)

        bg = Image.new("RGB", (w + m, h + m), parse_hex(args.background))
        bg.paste(final, (int(m / 2), int(m / 2)), final)
        bg = round_mask(bg, args.radius)

        final = bg

    if args.preview:
        final.show()
    else:
        basedir = os.path.dirname(os.path.abspath(args.output))
        if not os.path.exists(basedir):
            os.makedirs(basedir)
        final.save(args.output)
