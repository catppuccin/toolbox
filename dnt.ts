#!/usr/bin/env -S deno run -A
import { build, emptyDir } from "dnt";

await emptyDir("./npm");

const postBuild = () => {
  Deno.copyFileSync("LICENSE", "npm/LICENSE");
  Deno.copyFileSync("README.md", "npm/README.md");
};

const common = {
  importMap: "deno.json",
  shims: {
    deno: true,
  },
  package: {
    license: "MIT",
    repository: {
      type: "git",
      url: "git+https://github.com/catppuccin/toolbox.git",
    },
    bugs: {
      url: "https://github.com/catppuccin/toolbox/issues",
    },
  },
  publishConfig: {
    access: "public",
  },
  test: false,
};

await build({
  ...common,
  entryPoints: [{
    kind: "bin",
    path: "./inkcat/index.ts",
    name: "inkcat",
  }],
  outDir: "./npm/inkcat",
  package: {
    ...common.package,
    name: "@catppuccin/inkcat",
    version: "v0.0.1",
    description: "Display Catppuccin flavors and colors in your terminal.",
  },
  // cliffy doesn't support this yet
  typeCheck: false,
  scriptModule: false,
});

await build({
  ...common,
  entryPoints: [{
    kind: "bin",
    path: "./contrast_test/index.js",
    name: "contrast_test",
  }],
  outDir: "./npm/contrast_test",
  package: {
    ...common.package,
    name: "@catppuccin/contrast_test",
    version: "v0.0.1",
    description: "",
  },
  // cliffy doesn't support this yet
  typeCheck: false,
  scriptModule: false,
});

await build({
  ...common,
  entryPoints: [{
    kind: "bin",
    path: "./docpuccin/index.js",
    name: "docpuccin",
  }],
  outDir: "./npm/docppuccin",
  package: {
    ...common.package,
    name: "@catppuccin/docppuccin",
    version: "v0.0.1",
    description: "",
  },
  // cliffy doesn't support this yet
  typeCheck: false,
  scriptModule: false,
});
