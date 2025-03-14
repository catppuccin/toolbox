import Ajv, { type Schema } from "npm:ajv@8.17.1";
import { parse } from "jsr:@std/yaml@1.0.5";

/**
 * Validates a string of YAML content against a JSON schema with Ajv.
 * @param content A string containing YAML content
 * @param schema  A JSON schema
 * @param options Optional Ajv options
 * @returns A promise that resolves to the parsed YAML content, verified against the schema. Rejects if the content is invalid.
 */
export const validateYaml = <T>(
  content: string,
  schema: Schema,
  options?: Ajv.Options,
): Promise<T> => {
  return new Promise((resolve, reject) => {
    const ajv = new Ajv.default(options);

    const validate = ajv.compile<T>(schema);
    const data = parse(content);

    if (!validate(data)) return reject(validate.errors);

    return resolve(data);
  });
};
