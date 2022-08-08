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
parser.add_argument("-u", "--outer", help="Outer Radius", type=int, default=None)
parser.add_argument(
    "-o", "--output", help="Output file", type=str, default="out/res.png"
)
parser.add_argument("latte", help="Latte screenshot")
parser.add_argument("frappe", help="Frappe screenshot")
parser.add_argument("macchiato", help="Macchiato screenshot")
parser.add_argument("mocha", help="Mocha screenshot")
args = parser.parse_args()
if args.outer is None:
    args.outer = args.radius
if not args.background:
    args.background = "#b4befe"
    args.margin = 0
# }}}

# parse hex code into [r, g, b], stripping # if present
def parse_hex(hex):
    hex = hex.lstrip("#")
    return int(hex[0:2], 16), int(hex[2:4], 16), int(hex[4:6], 16)


# generate anti-aliased slice masks
def gen_masks(w, h):
    # calculate the slices, 4x the original size for anti-aliasing
    w = w * 4
    h = h * 4
    slices_aa = [
        [0, 0, 0, h, w / 8, h, (w / 8) * 3, 0],
        [w / 8, h, (w / 8) * 3, 0, (w / 8) * 5, 0, (w / 8) * 3, h],
        [(w / 8) * 3, h, (w / 8) * 5, 0, (w / 8) * 7, 0, (w / 8) * 5, h],
        [(w / 8) * 5, h, (w / 8) * 7, 0, w, 0, w, h],
    ]
    masks = []
    for slice in slices_aa:
        img = Image.new("L", (w, h), 0)
        draw = ImageDraw.Draw(img)
        draw.polygon(slice, fill=255)
        masks.append(img)
    w = int(w / 4)
    h = int(h / 4)
    fmasks = [Image.new("L", (w, h), 255)]
    for index in range(1, 4):
        fmasks.append(gen_fmask(masks[0:(index)], w, h))
    return fmasks


def gen_masked(source, mask):
    img = Image.open(source)

    output = ImageOps.fit(img, mask.size, centering=(0.5, 0.5))
    output.putalpha(mask)

    return output


def anti_alias(img, output_size):
    """Cheap anti-aliasing."""
    return img.resize(output_size, Image.Resampling.LANCZOS)


def gen_fmask(masks, w, h):
    img = Image.new("L", (w * 4, h * 4), 0)
    for mask in masks:
        img.paste(mask, (0, 0), mask)
    return anti_alias(ImageOps.invert(img), (w, h))


def round_mask(image, radius=40):
    size = (w * 4, h * 4)
    rounded = Image.new("L", size, 0)
    draw = ImageDraw.Draw(rounded)
    draw.rounded_rectangle([(0, 0), size], radius, fill=255)

    # scale down for the output, cheap anti-aliasing
    rounded = rounded.resize(image.size, Image.Resampling.LANCZOS)
    image.putalpha(rounded)
    return image


if __name__ == "__main__":
    # parse the 4 screenshots into an array
    imgs = [args.latte, args.frappe, args.macchiato, args.mocha]
    # input size shorthands
    w, h = Image.open(args.latte).size

    # create the diagonal masks
    masks = gen_masks(w, h)

    # make the composite image
    final = Image.new("RGBA", (w, h), (0, 0, 0))
    for i, arg in enumerate(imgs):
        masked = gen_masked(arg, masks[i])
        final.paste(masked, (0, 0), masked)

    # put it on a coloured background, if `--background` is passed
    m = args.margin

    final = round_mask(final, args.radius)

    bg = Image.new("RGBA", (w + m, h + m), parse_hex(args.background))
    bg = round_mask(bg, args.outer)
    bg.paste(final, (int(m / 2), int(m / 2)), final)

    final = bg

    if args.preview:
        final.show()
    else:
        basedir = os.path.dirname(os.path.abspath(args.output))
        if not os.path.exists(basedir):
            os.makedirs(basedir)
        final.save(args.output)
