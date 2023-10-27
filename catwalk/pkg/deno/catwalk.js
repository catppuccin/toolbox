

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

let WASM_VECTOR_LEN = 0;

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachedUint8ClampedMemory0 = null;

function getUint8ClampedMemory0() {
    if (cachedUint8ClampedMemory0 === null || cachedUint8ClampedMemory0.byteLength === 0) {
        cachedUint8ClampedMemory0 = new Uint8ClampedArray(wasm.memory.buffer);
    }
    return cachedUint8ClampedMemory0;
}

function getClampedArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ClampedMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}
/**
*/
export const Layout = Object.freeze({ Composite:0,"0":"Composite",Stacked:1,"1":"Stacked",Grid:2,"2":"Grid", });
/**
*/
export class Catwalk {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Catwalk.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_catwalk_free(ptr);
    }
    /**
    * @param {number | undefined} aa_level
    * @returns {Catwalk}
    */
    aa_level(aa_level) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.catwalk_aa_level(ptr, !isLikeNone(aa_level), isLikeNone(aa_level) ? 0 : aa_level);
        return Catwalk.__wrap(ret);
    }
    /**
    * @param {number | undefined} gap
    * @returns {Catwalk}
    */
    gap(gap) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.catwalk_gap(ptr, !isLikeNone(gap), isLikeNone(gap) ? 0 : gap);
        return Catwalk.__wrap(ret);
    }
    /**
    * @param {number | undefined} layout
    * @returns {Catwalk}
    */
    layout(layout) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.catwalk_layout(ptr, isLikeNone(layout) ? 3 : layout);
        return Catwalk.__wrap(ret);
    }
    /**
    * Sets the radius of the rounding mask.
    * # Errors
    * Returns an error if the height or width are not set (automatically inferred from the `new` method arguments)
    * @param {number | undefined} radius
    * @returns {Catwalk}
    */
    radius(radius) {
        try {
            const ptr = this.__destroy_into_raw();
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.catwalk_radius(retptr, ptr, !isLikeNone(radius), isLikeNone(radius) ? 0 : radius);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Catwalk.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
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
    static new_from_imagedata(latte, frappe, macchiato, mocha) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.catwalk_new_from_imagedata(retptr, addHeapObject(latte), addHeapObject(frappe), addHeapObject(macchiato), addHeapObject(mocha));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Catwalk.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
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
    static new_from_u8_array(latte, frappe, macchiato, mocha, width) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(latte, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passArray8ToWasm0(frappe, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passArray8ToWasm0(macchiato, wasm.__wbindgen_malloc);
            const len2 = WASM_VECTOR_LEN;
            const ptr3 = passArray8ToWasm0(mocha, wasm.__wbindgen_malloc);
            const len3 = WASM_VECTOR_LEN;
            wasm.catwalk_new_from_u8_array(retptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, width);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Catwalk.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Calculate the Catwalk image & return an `ImageData` object.
    * # Errors
    * Returns an error if any of `self.images`, `self.height`, or `self.width` are not set.
    * @returns {ImageData}
    */
    build_imagedata() {
        try {
            const ptr = this.__destroy_into_raw();
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.catwalk_build_imagedata(retptr, ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Calculate the Catwalk image & return a `CatwalkBuffer` object.
    * # Errors
    * Returns an error if any of `self.images`, `self.height`, or `self.width` are not set.
    * @returns {CatwalkBuffer}
    */
    build_buffer() {
        try {
            const ptr = this.__destroy_into_raw();
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.catwalk_build_buffer(retptr, ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return CatwalkBuffer.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
/**
*/
export class CatwalkBuffer {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(CatwalkBuffer.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_catwalkbuffer_free(ptr);
    }
    /**
    * @returns {number}
    */
    get width() {
        const ret = wasm.__wbg_get_catwalkbuffer_width(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set width(arg0) {
        wasm.__wbg_set_catwalkbuffer_width(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get height() {
        const ret = wasm.__wbg_get_catwalkbuffer_height(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set height(arg0) {
        wasm.__wbg_set_catwalkbuffer_height(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {Uint8Array}
    */
    get data() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.catwalkbuffer_data(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const imports = {
    __wbindgen_placeholder__: {
        __wbindgen_object_drop_ref: function(arg0) {
            takeObject(arg0);
        },
        __wbindgen_string_new: function(arg0, arg1) {
            const ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        },
        __wbg_width_c97f89a38a3c1da7: function(arg0) {
            const ret = getObject(arg0).width;
            return ret;
        },
        __wbg_height_c8424a3757db7869: function(arg0) {
            const ret = getObject(arg0).height;
            return ret;
        },
        __wbg_data_eaf4962120932fdc: function(arg0, arg1) {
            const ret = getObject(arg1).data;
            const ptr1 = passArray8ToWasm0(ret, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        },
        __wbg_newwithu8clampedarray_fb90064f569c16ec: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = new ImageData(getClampedArrayU8FromWasm0(arg0, arg1), arg2 >>> 0);
            return addHeapObject(ret);
        }, arguments) },
        __wbindgen_throw: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
    },

};

const wasm_url = new URL('catwalk_bg.wasm', import.meta.url);
let wasmCode = '';
switch (wasm_url.protocol) {
    case 'file:':
    wasmCode = await Deno.readFile(wasm_url);
    break
    case 'https:':
    case 'http:':
    wasmCode = await (await fetch(wasm_url)).arrayBuffer();
    break
    default:
    throw new Error(`Unsupported protocol: ${wasm_url.protocol}`);
}

const wasmInstance = (await WebAssembly.instantiate(wasmCode, imports)).instance;
const wasm = wasmInstance.exports;

