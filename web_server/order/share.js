export function set_cart_items(cart_items) {

    localStorage.setItem('cartitems', JSON.stringify(cart_items));
}

export function get_cart_total() {
    let cart = JSON.parse(localStorage.getItem('cartitems'));
    let c = Array.isArray(cart) ? cart : Array.from([0, 0, 0, 0]);

    return c;
}

export const color_array = ["red", "green", "blue", "yellow"];
export const price_array = [10, 20, 30, 40];