#!/usr/bin/env python3
# vim:ft=python:fenc=utf-8:fdm=marker

import argparse
import logging
import os
import re

from PIL import Image

from catwalk.utils import (
    alpha_fit,
    gen_composite_image,
    gen_stacked_image,
    gen_grid_image,
    gen_rainbow,
    gen_shadow,
    round_mask,
)

logging.basicConfig(level=logging.INFO, format="%(message)s")

# argparse {{{
parser = argparse.ArgumentParser()
parser.add_argument(
    "-l",
    "--layout",
    help="Layout style for the output file",
    default="composite",
    type=str,
    choices=["composite", "grid", "stacked"],
)
parser.add_argument(
    "-g",
    "--gap",
    help="Gap between images for the grid layout. Default: %(default)s",
    default=20,
    type=int,
)
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


def main():
    # parse the 4 screenshots into an array
    imgs = [args.latte, args.frappe, args.macchiato, args.mocha]
    style = args.layout

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

    # put it on a coloured background, if `--background` is passed
    m = args.margin if args.margin else 0
    if args.rainbow:
        bg = gen_rainbow(((w + m), (h + m)))
    else:
        bg_colour = args.background or "#00000000"
        if not re.match(r"^#[0-9a-fA-F]{6,8}$", bg_colour):
            logging.error("Invalid background colour. Expected: #RRGGBB or #RRGGBBAA.")
            exit(1)
        bg = Image.new("RGBA", (w + m, h + m), bg_colour)

    # round the outer corners
    bg = round_mask(bg, args.outer)

    if style == "composite":
        final = gen_composite_image(imgs, args.radius)
    elif style == "grid":
        final = gen_grid_image(imgs, args.radius, args.gap)
    elif style == "stacked":
        final = gen_stacked_image(imgs, args.radius)

    # create a drop shadow if `--shadow` is passed
    if args.shadow:
        drop_shadow = gen_shadow(final, strength=args.shadow)
        bg = alpha_fit(bg, drop_shadow, (20, 20))

    output = alpha_fit(bg, final)

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
