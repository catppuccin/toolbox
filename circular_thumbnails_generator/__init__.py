from PIL import Image, ImageOps, ImageColor
import sys
import json

MASK = "circular_mask.png"
OUT_DIR = "/home/pocco81/dev/catppuccin/catppuccin/assets/palette/circles/"
show = False

if len(sys.argv) > 1:
    show = True

palettes = json.load(open('palettes.json'))

def gen_circular_png(color, name, width=100, height=100):
    if color.startswith("#"):
        color = ImageColor.getrgb(color)

    mask = Image.open(MASK).convert('L')
    img = Image.new('RGB', (width, height), color)
    # im = Image.open('image.png')

    output = ImageOps.fit(img, mask.size, centering=(0.5, 0.5))
    output.putalpha(mask)

    if show:
        output.show()
    else:
        output.save(OUT_DIR + name + '.png')


for palette in palettes:
    for label in palettes[palette]:
        gen_circular_png(palettes[palette][label]["hex"], palette + "_" + label)
