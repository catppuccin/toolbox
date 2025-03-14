import { assertEquals, assertThrows } from "jsr:@std/assert@1.0.11";
import { updateReadme } from "./updateReadme.ts";

const readme_default = `
Text before the section.

<!-- AUTOGEN START -->
<!-- AUTOGEN END -->

Text after the section.
`;

Deno.test("updateReadme pass (defaults)", () => {
  const newReadme = updateReadme(readme_default, "<new content>");

  assertEquals(
    newReadme,
    `
Text before the section.

<!-- AUTOGEN START -->
<!-- the following section is auto-generated, do not edit -->
<new content>
<!-- AUTOGEN END -->

Text after the section.
`,
  );
});

Deno.test("updateReadme fail (defaults)", () => {
  assertThrows(() => {
    updateReadme(readme_default, "bar-section-title", {
      section: "foo-section-title",
    });
  });
});

const readme_custom = `
Text before the section.

# Custom start
Arbitrary end marker.

Text after the section.
`;

Deno.test("updateReadme pass (customized)", () => {
  const newReadme = updateReadme(readme_custom, "<new content>", {
    section: "custom",
    preamble: "Some custom preamble.",
    markers: {
      start: "# Custom start",
      end: "Arbitrary end marker",
    },
  });

  assertEquals(
    newReadme,
    `
Text before the section.

# Custom start
Some custom preamble.
<new content>
Arbitrary end marker.

Text after the section.
`,
  );
});
