# vim:ft=python:fenc=utf-8:fdm=marker

from typing import List, Tuple

from PIL import Image, ImageDraw, ImageFilter, ImageOps

DS_METHOD = Image.LANCZOS


# generate anti-aliased slice masks
def gen_masks(size: Tuple[int, int]):
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
        img = Image.new("L", (w, h))
        draw = ImageDraw.Draw(img)
        draw.polygon(s, fill=255)
        masks.append(img)
    w = int(w / 4)
    h = int(h / 4)
    fmasks = [Image.new("L", (w, h), "white")]
    for index in range(1, 4):
        fmasks.append(gen_composite_mask(masks[0:index], (w, h)))

    return fmasks


def gen_rainbow(size: Tuple[int, int]) -> Image.Image:
    colors = ["#f38ba8", "#f9e2af", "#a6e3a1", "#89b4fa"]
    final = Image.new("RGBA", size)
    masks = gen_masks(size)
    for i, color in enumerate(colors):
        new_img = Image.new("RGBA", size, color)
        final.paste(new_img, (0, 0), masks[i])
    return final


def alpha_fit(
    img1: Image.Image, img2: Image.Image, offset: tuple[int, int] = (0, 0)
) -> Image.Image:
    dest = ((img1.width // 2 - img2.width // 2), (img1.height // 2 - img2.height // 2))
    dest = (dest[0] + offset[0], dest[1] + offset[1])
    img1.alpha_composite(img2, dest)
    return img1


def gen_masked(
    source: Image.Image, mask: Image.Image, final: Image.Image
) -> Image.Image:
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
    final = Image.new("RGBA", (max_w, max_h))
    for i, img in enumerate(imgs):
        masked = gen_masked(img, masks[i], final)
        final.paste(masked, (0, 0), masked)

    if radius:
        final = round_mask(final, radius)

    return final


def gen_grid_image(imgs: List[Image.Image], radius: int, gap: int) -> Image.Image:
    """Generate a grid layout of 4 images"""
    # find the largest image
    max_w = max([img.width for img in imgs])
    max_h = max([img.height for img in imgs])

    final = Image.new("RGBA", (max_w, max_h))
    # gap = 20

    for i, img in enumerate(imgs):
        img = round_mask(
            img.resize(
                (int(round(max_w / 2) - gap * 2), int(round(max_h / 2) - gap * 2))
            ),
            radius,
        )
        final.paste(
            img,
            (
                int((i % 2) * round(max_w / 2) + gap),
                int((i // 2) * round(max_h / 2) + gap),
            ),
        )

    if radius:
        final = round_mask(final, radius)

    return final


def gen_stacked_image(imgs: List[Image.Image], radius: int) -> Image.Image:
    """Stack images on top of each other"""
    max_w = max([img.width for img in imgs])
    max_h = max([img.height for img in imgs])

    final = Image.new("RGBA", (max_w, max_h))
    gap = int((max_h / 2) // (len(imgs) - 1))

    padding_x = int((max_w / 2 - 3 * gap) / 2)

    for i, img in enumerate(imgs):
        img = round_mask(
            img.resize((int(max_w / 2), int(max_h / 2))),
            radius,
        )
        final.alpha_composite(img, (padding_x + (gap * i), (gap * i)))

    if radius:
        final = round_mask(final, radius)

    return final


def anti_alias(img: Image.Image, output_size: Tuple[int, int]) -> Image.Image:
    """Cheap anti-aliasing."""
    return img.resize(output_size, DS_METHOD)


def gen_composite_mask(
    masks: List[Image.Image], size: Tuple[int, int], aa_factor: int = 4
) -> Image.Image:
    w = size[0] * aa_factor
    h = size[1] * aa_factor
    img = Image.new("L", (w, h))

    for mask in masks:
        img.paste(mask, (0, 0), mask)

    img = ImageOps.invert(img)
    return img.resize(size, DS_METHOD)


def gen_shadow(img, strength: int, opacity: float = 0.3):
    """Generate a shadow effect."""
    caster = Image.new("RGB", img.size)
    caster.putalpha(img.getchannel("A"))

    padded_size = (round(img.width * 1.2), round(img.height * 1.2))
    bg = Image.new("RGBA", img.size)
    bg.alpha_composite(caster)

    # create an image that is a bit larger than the original, to fit the shadow
    padded = Image.new("RGBA", padded_size)
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
    rounded = Image.new("RGBA", size)
    draw = ImageDraw.Draw(rounded)
    draw.rounded_rectangle(((0, 0), size), radius, fill="white")
    # scale down for the output, cheap anti-aliasing
    rounded = rounded.resize(image.size, DS_METHOD)

    img = Image.new("RGBA", image.size)
    img.paste(image, (0, 0), rounded)
    return img
