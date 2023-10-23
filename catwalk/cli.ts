#!/usr/bin/env -S deno run -A
import { Catwalk, Layout } from "./pkg/deno/catwalk.js";
import {
  Command,
  EnumType,
} from "https://deno.land/x/cliffy@v1.0.0-rc.3/command/mod.ts";

const { args, options } = await new Command()
  .name("catwalk")
  .version("0.1.0")
  .description(
    "A sweet program that takes in four showcase images and displays them all at once. (JS port)",
  )
  .arguments("<latte:string> <frappe:string> <macchiato:string> <mocha:string>")
  .type(
    "layout",
    new EnumType(Object.keys(Layout).filter((k) => isNaN(Number(k)))),
  )
  .option("-g, --gap <size:number>", "Gap size", { default: 50 })
  .option("-l, --layout <layout:layout>", "Output file", { default: Layout[0] })
  .option("-o, --output <path:string>", "Output file", { default: "./out.png" })
  .option("-r, --radius <size:number>", "Number of images", { default: 3 })
  .parse(Deno.args);
const [latte, frappe, macchiato, mocha] = args.map((arg) =>
  Deno.readFileSync(arg)
);

// @ts-ignore: not dealing with this conversion mess manually
const layout = Layout[options.layout];
const catwalk = new Catwalk(latte, frappe, macchiato, mocha)
  .gap(options.gap)
  .layout(layout)
  .radius(options.radius)
  .build();

Deno.writeFile(options.output, catwalk);
