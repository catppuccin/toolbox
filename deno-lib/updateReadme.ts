/**
 * Update a section of a README.md file.
 *
 * Assumes that the section is marked by HTML comments like so:
 * ```md
 * <!-- AUTOGEN START -->
 * <!-- AUTOGEN END -->
 * ```
 *
 * When section names are used, the markers look like this:
 * ```md
 * <!-- AUTOGEN:SECTION START -->
 * <!-- AUTOGEN:SECTION END -->
 * ```
 *
 * The section name is case-insensitive, as section markers are converted to
 * uppercase.
 *
 * If there are multiple sections with the same name, only the first one will be
 * updated.
 *
 * @param {string} readme
 * The README.md file contents
 * @param {string} newContent
 * The new content to put in the section
 * @param {object} options
 * Options to customize the behavior
 * @param {string} options.section
 * The section name
 * @param {string} options.preamble
 * Text to insert after the start marker, usually a warning to the reader not to
 * edit the section by hand
 * @param {object} options.markers
 * The start and end markers
 * @param {string} [options.markers.start="<!-- AUTOGEN START -->"]
 * The start marker
 * @param {string} [options.markers.end="<!-- AUTOGEN END -->"]
 * The end marker
 *
 * @returns {string}
 * The updated README.md file contents
 * @throws if the section is not found
 */
export const updateReadme = (
  readme: string,
  newContent: string,
  options: {
    section?: string;
    preamble?: string;
    markers?: {
      start: string;
      end: string;
    };
  } = {},
): string => {
  const {
    section = "",
    preamble = "<!-- the following section is auto-generated, do not edit -->",
    markers = {
      start: `<!-- AUTOGEN${
        section !== "" ? `:${section.toUpperCase()}` : ""
      } START -->`,
      end: `<!-- AUTOGEN${
        section !== "" ? `:${section.toUpperCase()}` : ""
      } END -->`,
    },
  } = options;
  const wrapped = [markers.start, preamble, newContent, markers.end].join("\n");

  Object.values(markers).map((m) => {
    if (!readme.includes(m)) {
      throw new Error(`Marker ${m} not found in README.md`);
    }
  });

  const pre = readme.split(markers.start)[0];
  const end = readme.split(markers.end)[1];
  return pre + wrapped + end;
};
