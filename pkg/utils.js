/* tslint:disable */
import * as wasm from './utils_bg';

/**
* @param {number} arg0
* @returns {number}
*/
export function add_one(arg0) {
    return wasm.add_one(arg0);
}

/**
* @returns {void}
*/
export function run() {
    return wasm.run();
}

