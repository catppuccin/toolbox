/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Layout {
  Composite = 0,
  Stacked = 1,
  Grid = 2,
  Row = 3,
}
/**
*/
export class Catwalk {
  free(): void;
/**
* Create a new Catwalk from 4 `web_sys::ImageData` objects
* # Errors
* Returns an error if the images...
* - cannot be read.
* - are not the same size.
* @param {ImageData} latte
* @param {ImageData} frappe
* @param {ImageData} macchiato
* @param {ImageData} mocha
* @returns {Catwalk}
*/
  static new_from_imagedata(latte: ImageData, frappe: ImageData, macchiato: ImageData, mocha: ImageData): Catwalk;
/**
* Create a new Catwalk from 4 `Vec<u8>`, which are in practice `Vec<[u8; 4]>` (RGBA).
* # Errors
* Returns an error if the images...
* - cannot be read.
* - are not the same size.
* @param {Uint8Array} latte
* @param {Uint8Array} frappe
* @param {Uint8Array} macchiato
* @param {Uint8Array} mocha
* @param {number} width
* @returns {Catwalk}
*/
  static new_from_u8_array(latte: Uint8Array, frappe: Uint8Array, macchiato: Uint8Array, mocha: Uint8Array, width: number): Catwalk;
/**
* Calculate the Catwalk image & return an `ImageData` object.
* # Errors
* Returns an error if any of `self.images`, `self.height`, or `self.width` are not set.
* @returns {ImageData}
*/
  build_imagedata(): ImageData;
/**
* Calculate the Catwalk image & return a `CatwalkBuffer` object.
* # Errors
* Returns an error if any of `self.images`, `self.height`, or `self.width` are not set.
* @returns {CatwalkBuffer}
*/
  build_buffer(): CatwalkBuffer;
/**
* @param {number | undefined} aa_level
* @returns {Catwalk}
*/
  aa_level(aa_level?: number): Catwalk;
/**
* @param {number | undefined} gap
* @returns {Catwalk}
*/
  gap(gap?: number): Catwalk;
/**
* @param {number | undefined} layout
* @returns {Catwalk}
*/
  layout(layout?: number): Catwalk;
/**
* Sets the radius of the rounding mask.
* # Errors
* Returns an error if the height or width are not set (automatically inferred from the `new` method arguments)
* @param {number | undefined} radius
* @returns {Catwalk}
*/
  radius(radius?: number): Catwalk;
/**
* Returns the version of the Catwalk library.
*/
  static readonly version: string;
}
/**
*/
export class CatwalkBuffer {
  free(): void;
/**
*/
  readonly data: Uint8Array;
/**
*/
  height: number;
/**
*/
  width: number;
}
