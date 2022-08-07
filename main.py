from PIL import Image, ImageOps, ImageDraw
import os
import sys

OUT_DIR = "out/"
show = False
backg = False
bg = None
m = 0


w, h = Image.open(sys.argv[1]).size  # Get size

while len(sys.argv) > 5:
    if sys.argv[5] == "--show":
        show = True
        sys.argv.pop(5)
    else:
        backg = True
        m = int(sys.argv.pop(9))
        bg = Image.new(
            "RGB",
            (w + m, h + m),
            (int(sys.argv[6]), int(sys.argv[7]), int(sys.argv[8])),
        )
        sys.argv.pop(8)
        sys.argv.pop(7)
        sys.argv.pop(6)
        sys.argv.pop(5)

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
    # im = Image.open('image.png')

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


final = Image.new("RGB", (w, h), (0, 0, 0))
for i, arg in enumerate(sys.argv[1:5]):
    masked = gen_masked(arg, masks[i])
    final.paste(masked, (0, 0), masked)


if backg:
    bg.paste(round_corners(final), (int(m / 2), int(m / 2)), round_corners(final))
    final = bg
if show:
    final.show()
else:
    if not os.path.exists(OUT_DIR):
        os.makedirs(OUT_DIR)
    final.save(OUT_DIR + "res.png")
