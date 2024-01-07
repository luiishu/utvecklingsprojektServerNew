import * as server from "../server.js"

export function set_cart_items(cart_items) {

    localStorage.setItem('cartitems', JSON.stringify(cart_items));
}

export function get_cart_total() {
    let cart = JSON.parse(localStorage.getItem('cartitems'));
    let c = Array.isArray(cart) ? cart : Array.from([0, 0, 0, 0]);

    return c;
}

export function reset_cart_total() {

    localStorage.setItem('cartitems', JSON.stringify([0, 0, 0, 0]));
}

export const color_array = ["red", "yellow", "green", "blue"];


export const price_array = [];

export async function init_price() {
    let data = await server.get_product_pages();
    for (let i = 0; i < 4; i++) {
        price_array[i] = data.rows[i].price;
    }
}