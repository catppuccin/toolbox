#!/usr/bin/env -S deno run -A --unstable

import catppuccin from "https://raw.githubusercontent.com/catppuccin/palette/v0.2.0/palette-porcelain.json" assert {
  type: "json",
};
import { Command, EnumType } from "cliffy/command";
import { Table } from "cliffy/table";
import { colors } from "cliffy/ansi";
import clipboard from "npm:clipboardy";

enum Format {
  hex = "hex",
  hsl = "hsl",
  rgb = "rgb",
}

const cKeys = Object.keys(catppuccin.mocha).map((k) => k.toLowerCase());
const fKeys = Object.keys(catppuccin).map((k) => k.toLowerCase());
type CtpFlavor = keyof typeof catppuccin;
type CtpColor = keyof typeof catppuccin[CtpFlavor];
type CtpColorInfo = typeof catppuccin[CtpFlavor][CtpColor];

const { options, args } = await new Command()
  .name("inkcat")
  .version("0.1.0")
  .description(
    "InkCat is a minimal and versatile solution for displaying colors from each one of Catppuccin's flavors in your terminal. This program also allows you to copy them to your clipboard.",
  )
  .type("format", new EnumType(Format))
  .type("flavor", new EnumType(fKeys))
  .type("color", new EnumType(cKeys))
  .option("-n, --no-copy", "Don't copy the colors to the clipboard")
  .option("-f, --format <format:format>", "Color format to use", {
    default: Format.hex,
  })
  .arguments("<palette:flavor> [color...]")
  .parse();

const argFlavor = args[0] as CtpFlavor;
const argColors = args[1] && args.slice(1, args.length);

const fgColor = (bgColor: { r: number; g: number; b: number }) => {
  const { r, g, b } = bgColor;
  const luma = 0.2126 * r + 0.7152 * g + 0.0722 * b;
  return luma > 128 ? colors.black : colors.white;
};

const fmtColor = (color: CtpColorInfo) => {
  const bg = { r: color.rgb[0], g: color.rgb[1], b: color.rgb[2] };
  const fg = fgColor(bg);
  return fg.bgRgb24(`${color.hex}`, bg);
};

let valuesToCopy: CtpColorInfo[] = [];
Table.from(
  Object.entries(catppuccin[argFlavor])
    .filter(([key, value]) => {
      if (!argColors) return true;
      const include = argColors.includes(key);
      if (include) valuesToCopy.push(value);
      return include;
    })
    .map(([key, value]) => {
      return [key, fmtColor(value)];
    }),
).render();

if (options.copy) {
  clipboard.writeSync(valuesToCopy.map((e) => e.hex).join("\n"));
}
