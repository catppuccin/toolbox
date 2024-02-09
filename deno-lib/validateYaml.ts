import Ajv, { type Schema } from "https://esm.sh/ajv@8.12.0?pin=v135";
import { parse } from "https://deno.land/std@0.215.0/yaml/parse.ts";

/**
 * Validates a string of YAML content against a JSON schema with Ajv.
 * @param content A string containing YAML content
 * @param schema  A JSON schema
 * @returns A promise that resolves to the parsed YAML content, verified against the schema. Rejects if the content is invalid.
 */
export const validateYaml = <T>(
  content: string,
  schema: Schema,
): Promise<T> => {
  return new Promise((resolve, reject) => {
    const ajv = new Ajv();

    const validate = ajv.compile<T>(schema);
    const data = parse(content);

    if (!validate(data)) return reject(validate.errors);

    return resolve(data);
  });
};
