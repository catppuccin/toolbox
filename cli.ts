#!/usr/bin/env -S deno run -A
import { Catwalk, Layout } from "./pkg/deno/catwalk.js";
import {
  Command,
  EnumType,
} from "https://deno.land/x/cliffy@v1.0.0-rc.3/command/mod.ts";
import {
  ImageMagick,
  initialize,
  MagickColor,
  MagickFormat,
  MagickImage,
} from "https://deno.land/x/imagemagick_deno@0.0.26/mod.ts";

const { args, options } = await new Command()
  .name("catwalk")
  .version("0.1.0")
  .description(
    "A sweet program that takes in four showcase images and displays them all at once. (JS port)",
  )
  .arguments("<latte:string> <frappe:string> <macchiato:string> <mocha:string>")
  .type(
    "layout",
    new EnumType(
      Object.keys(Layout)
        .filter((k) => isNaN(Number(k)))
        .map((k) => k.toLowerCase()),
    ),
  )
  .option("-g, --gap <size:number>", "Gap size for `grid` Layout")
  .option("-l, --layout <layout:layout>", "Layout to use")
  .option("-o, --output <path:string>", "Output file", {
    default: "./result.webp",
  })
  .option("-r, --radius <size:number>", "Radius of corners")
  .parse(Deno.args);

const capitalize = (s: string) => s.charAt(0).toUpperCase() + s.slice(1);

let fmt: MagickFormat;
switch (options.output.split(".").pop()) {
  case "png":
    fmt = MagickFormat.Png;
    break;
  case "webp":
    fmt = MagickFormat.Webp;
    break;
  default:
    console.error("Invalid output format");
    Deno.exit(1);
}

await initialize();

let [width, height] = [0, 0];
const [latte, frappe, macchiato, mocha] = args.map((path) => {
  const buffer = Deno.readFileSync(path);
  return ImageMagick.read(buffer, (data) => {
    height = data.height;
    width = data.width;
    const pixels = data.getPixels((pixel) =>
      pixel.getArea(0, 0, width, height)
    );
    return Uint8Array.from(pixels);
  });
});

const cw = Catwalk.new_from_u8_array(latte, frappe, macchiato, mocha, width)
  .gap(options.gap)
  //@ts-expect-error: i'm not dealing with this
  .layout(Layout[(capitalize(options.layout ?? "")) as keyof typeof Layout])
  .radius(options.radius)
  .build_buffer();

const img = MagickImage.create(
  new MagickColor(0, 0, 0, 0),
  cw.width,
  cw.height,
);
// needed for WASM, as it defaults to lossy
img.quality = 100;
img.getPixels((pixels) => pixels.setArea(0, 0, cw.width, cw.height, cw.data));
img.write(fmt, (data) => Deno.writeFile(options.output, data));
