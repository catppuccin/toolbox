from PIL import Image, ImageOps
import sys

OUT_DIR = "out/"
show = False

if len(sys.argv) > 5:
    show = True
    sys.argv.pop(5)



def gen_masked(source, mask):
    mask = Image.open(mask).convert('L')
    img = Image.open(source)
    # im = Image.open('image.png')

    output = ImageOps.fit(img, mask.size, centering=(0.5, 0.5))
    output.putalpha(mask)

    return output

res = []
for ind, source in enumerate(sys.argv[1:]):
   res.append(gen_masked(source, "mask" +str(ind+1)+ ".png")) 

final = res[0]
for result in res[1:]:
    final.paste(result, (0,0), result)

if show: final.show()
else: final.save(OUT_DIR + 'res.png')
