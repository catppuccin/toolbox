from PIL import Image, ImageOps, ImageDraw
import sys

OUT_DIR = "out/"
show = False

if len(sys.argv) > 5:
    show = True
    sys.argv.pop(5)

w, h = Image.open(sys.argv[1]).size # Get size
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

res = [
    gen_masked(sys.argv[1], m1b),
    gen_masked(sys.argv[2], m2b),
    gen_masked(sys.argv[3], m3b), 
    gen_masked(sys.argv[4], m4b), 
]

final = res[0]
for result in res[1:]:
    final.paste(result, (0,0), result)

if show: final.show()
else: final.save(OUT_DIR + 'res.png')
