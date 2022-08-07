from PIL import Image, ImageOps, ImageDraw
import sys

OUT_DIR = "out/"
show = False
backg = False
bg = None
m = 0



w, h = Image.open(sys.argv[1]).size # Get size

while len(sys.argv) > 5:
    if (sys.argv[5] == "--show"):
        show = True
        sys.argv.pop(5)
    else:
        backg = True
        m = int(sys.argv.pop(9))
        bg = Image.new('RGB', (w+m, h+m), (int(sys.argv[6]), int(sys.argv[7]), int(sys.argv[8])))
        sys.argv.pop(8)
        sys.argv.pop(7)
        sys.argv.pop(6)
        sys.argv.pop(5)

m1b = Image.new('RGB', (w, h), (0, 0, 0))
m2b = Image.new('RGB', (w, h), (0, 0, 0))
m3b = Image.new('RGB', (w, h), (0, 0, 0))
m4b = Image.new('RGB', (w, h), (0, 0, 0))
m1p = [0, 0, 0, h, w/3, 0]
m2p = [0, h, w/3, 0, (w/3)*2, 0, w/3, h]
m3p = [w/3, h, (w/3)*2, 0, w, 0, (w/3)*2, h]
m4p = [(w/3)*2, h, w, 0, w, h]
m1f = ImageDraw.Draw(m1b)
m2f = ImageDraw.Draw(m2b)
m3f = ImageDraw.Draw(m3b)
m4f = ImageDraw.Draw(m4b)
m1f.polygon(m1p, fill = (255, 255, 255))
m2f.polygon(m2p, fill = (255, 255, 255))
m3f.polygon(m3p, fill = (255, 255, 255))
m4f.polygon(m4p, fill = (255, 255, 255))




def gen_masked(source, mask):
    mask = mask.convert('L')
    img = Image.open(source)
    # im = Image.open('image.png')

    output = ImageOps.fit(img, mask.size, centering=(0.5, 0.5))
    output.putalpha(mask)

    return output
def round_corners(im, rad=15):
    circle = Image.new('L', (rad * 2, rad * 2), 0)
    draw = ImageDraw.Draw(circle)
    draw.ellipse((0, 0, rad * 2, rad * 2), fill=255)
    alpha = Image.new('L', im.size, "white")
    w, h = im.size
    alpha.paste(circle.crop((0, 0, rad, rad)), (0, 0))
    alpha.paste(circle.crop((0, rad, rad, rad * 2)), (0, h - rad))
    alpha.paste(circle.crop((rad, 0, rad * 2, rad)), (w - rad, 0))
    alpha.paste(circle.crop((rad, rad, rad * 2, rad * 2)), (w - rad, h - rad))
    im.putalpha(alpha)
    return im

res = [
    gen_masked(sys.argv[1], m1b),
    gen_masked(sys.argv[2], m2b),
    gen_masked(sys.argv[3], m3b), 
    gen_masked(sys.argv[4], m4b), 
]

final = res[0]
for result in res[1:]:
    final.paste(result, (0,0), result)

if backg:
    bg.paste(round_corners(final), (int(m/2), int(m/2)), round_corners(final))
    final = bg
if show: final.show()
else: final.save(OUT_DIR + 'res.png')
