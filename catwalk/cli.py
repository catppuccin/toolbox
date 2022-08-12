#!/usr/bin/env python3
# vim:ft=python:fenc=utf-8:fdm=marker
from typing import List

from PIL import Image, ImageOps, ImageDraw, ImageFilter, __version__ as PIL_VERSION
import argparse
import os
import logging

logging.basicConfig(level=logging.INFO, format="%(message)s")

# check for Pillow-SIMD
if PIL_VERSION.startswith("9.0.0"):
    DS_METHOD = Image.ANTIALIAS
else:
    DS_METHOD = Image.Resampling.LANCZOS

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
    help="Draw on a solid background colour. Specify the hex code of the colour.",
    type=str,
)
parser.add_argument(
    "-m",
    "--margin",
    help="Margin around the screenshot (in pixels). Default: %(default)s",
    type=int,
    default=40,
)
parser.add_argument(
    "-r",
    "--radius",
    help="Radius of the rounded corners of the screenshot. Default: %(default)s",
    type=int,
    default=50,
)
parser.add_argument(
    "-u",
    "--outer",
    help="Radius of the background image. Default: Equal to `--radius`.",
    type=int,
    default=None,
)
parser.add_argument(
    "-a", "--rainbow", help="Generate a Rainbow background.", action="store_true"
)
parser.add_argument(
    "-s",
    "--shadow",
    help="""
    Make the screenshot cast a drop shadow.
    You can optionally specify the strength of the blur.
    Default: %(const)s
    """,
    type=int,
    const=12,
    nargs="?",
)
parser.add_argument(
    "-o",
    "--output",
    help="Output file. Default: %(default)s",
    type=str,
    default="out/res.webp",
)
parser.add_argument("latte", help="Latte screenshot")
parser.add_argument("frappe", help="Frappé screenshot")
parser.add_argument("macchiato", help="Macchiato screenshot")
parser.add_argument("mocha", help="Mocha screenshot")
args = parser.parse_args()
if args.outer is None:
    args.outer = args.radius
# }}}


# parse hex code into [r, g, b], stripping # if present
def parse_hex(code: str) -> (int, int, int):
    code = code.lstrip("#")
    return int(code[0:2], 16), int(code[2:4], 16), int(code[4:6], 16)


# generate anti-aliased slice masks
def gen_masks(size: (int, int)):
    # calculate the slices, 4x the original size for anti-aliasing
    w = size[0] * 4
    h = size[1] * 4
    slices = [
        [0, 0, 0, h, w / 8, h, (w / 8) * 3, 0],
        [w / 8, h, (w / 8) * 3, 0, (w / 8) * 5, 0, (w / 8) * 3, h],
        [(w / 8) * 3, h, (w / 8) * 5, 0, (w / 8) * 7, 0, (w / 8) * 5, h],
        [(w / 8) * 5, h, (w / 8) * 7, 0, w, 0, w, h],
    ]

    masks = []
    for s in slices:
        img = Image.new("L", (w, h), 0)
        draw = ImageDraw.Draw(img)
        draw.polygon(s, fill=255)
        masks.append(img)
    w = int(w / 4)
    h = int(h / 4)
    fmasks = [Image.new("L", (w, h), 255)]
    for index in range(1, 4):
        fmasks.append(gen_composite_mask(masks[0:index], (w, h)))
    return fmasks


def gen_rainbow(size: (int, int)) -> Image.Image:
    colors = ["#f38ba8", "#f9e2af", "#a6e3a1", "#89b4fa"]
    final = Image.new("RGBA", size, 0)
    masks = gen_masks(size)
    for i, color in enumerate(colors):
        new_img = Image.new("RGBA", size, parse_hex(color))
        final.paste(new_img, (0, 0), masks[i])
    return final


def alpha_fit(
    img1: Image.Image, img2: Image.Image, offset: tuple[int, int] = (0, 0)
) -> Image.Image:
    dest = ((img1.width // 2 - img2.width // 2), (img1.height // 2 - img2.height // 2))
    dest = (dest[0] + offset[0], dest[1] + offset[1])
    img1.alpha_composite(img2, dest)
    return img1


def gen_masked(source: Image.Image, mask: Image.Image, final: Image.Image) -> Image:
    output = ImageOps.fit(source, mask.size, centering=(0.5, 0.5))
    final.paste(output, (0, 0), mask)
    return final


def gen_composite_image(imgs: List[Image.Image], radius: int) -> Image.Image:
    """Generate a composite image."""
    # find the largest image
    max_w = max([img.width for img in imgs])
    max_h = max([img.height for img in imgs])
    # create the diagonal masks
    masks = gen_masks((max_w, max_h))

    # make the composite image
    final = Image.new("RGBA", (max_w, max_h), (0, 0, 0, 0))
    for i, img in enumerate(imgs):
        masked = gen_masked(img, masks[i], final)
        final.paste(masked, (0, 0), masked)

    if radius:
        final = round_mask(final, radius)

    return final


def anti_alias(img: Image.Image, output_size: (int, int)) -> Image.Image:
    """Cheap anti-aliasing."""
    return img.resize(output_size, DS_METHOD)


def gen_composite_mask(
    masks: List[Image.Image], size: (int, int), aa_factor: int = 4
) -> Image.Image:
    w = size[0] * aa_factor
    h = size[1] * aa_factor
    img = Image.new("L", (w, h), 0)

    for mask in masks:
        img.paste(mask, (0, 0), mask)

    img = ImageOps.invert(img)
    return img.resize(size, DS_METHOD)


def gen_shadow(img, strength: int, opacity: float = 0.3):
    """Generate a shadow effect."""
    caster = Image.new("RGB", img.size, color="black")
    caster.putalpha(img.getchannel("A"))

    padded_size = (round(img.width * 1.2), round(img.height * 1.2))
    bg = Image.new("RGBA", img.size, (0, 0, 0, 0))
    bg.alpha_composite(caster)

    # create an image that is a bit larger than the original, to fit the shadow
    padded = Image.new("RGBA", padded_size, (0, 0, 0, 0))
    center_offset = (
        int(padded_size[0] / 2 - img.width / 2),
        int(padded_size[1] / 2 - img.height / 2),
    )
    padded.alpha_composite(bg, center_offset)
    bg = padded.filter(ImageFilter.GaussianBlur(strength))

    # set the opacity
    bg.putalpha(Image.eval(bg.split()[3], lambda x: x * opacity))
    return bg


def round_mask(image: Image.Image, radius: int) -> Image.Image:
    w, h = image.size
    size = (w * 4, h * 4)
    rounded = Image.new("RGBA", size, 0)
    draw = ImageDraw.Draw(rounded)
    draw.rounded_rectangle(((0, 0), size), radius, fill=(255, 255, 255, 255))
    # scale down for the output, cheap anti-aliasing
    rounded = rounded.resize(image.size, DS_METHOD)

    img = Image.new("RGBA", image.size, (0, 0, 0, 0))
    img.paste(image, (0, 0), rounded)
    return img


def main():
    # parse the 4 screenshots into an array
    imgs = [args.latte, args.frappe, args.macchiato, args.mocha]

    try:
        imgs = [Image.open(img).convert("RGBA") for img in imgs]
    except IOError as e:
        logging.error(e)
        exit(1)

    # check if all images are the same size
    w, h = imgs[0].size
    for img in imgs:
        if img.size != (w, h):
            logging.warning("Images are not the same size")
            break

    composite = gen_composite_image(imgs, args.radius)

    # put it on a coloured background, if `--background` is passed
    m = args.margin if args.margin else 0
    if args.rainbow:
        bg = gen_rainbow(((w + m), (h + m)))
    else:
        bg_colour = parse_hex(args.background) if args.background else (0, 0, 0, 0)
        bg = Image.new("RGBA", (w + m, h + m), bg_colour)

    # round the outer corners
    bg = round_mask(bg, args.outer)

    # create a drop shadow if `--shadow` is passed
    if args.shadow:
        drop_shadow = gen_shadow(composite, strength=args.shadow)
        bg = alpha_fit(bg, drop_shadow, (20, 20))

    output = alpha_fit(bg, composite)

    if args.preview:
        output.show()
    else:
        basedir = os.path.dirname(os.path.abspath(args.output))
        if not os.path.exists(basedir):
            try:
                os.makedirs(basedir)
            except OSError as e:
                logging.error(e)
                exit(1)
        try:
            output.save(args.output, None, compress_level=9, lossless=True)
            logging.info("Saved to %s" % args.output)
            exit(0)
        except IOError as e:
            logging.error(e)
            exit(1)


if __name__ == "__main__":
    main()
