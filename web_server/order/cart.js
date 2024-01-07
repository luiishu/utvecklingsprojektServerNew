import * as share from "./share.js"
import * as server from "../server.js"

let checkout;
let checkout_btn;
let checkout_txt;
let order = [];

document.addEventListener("DOMContentLoaded", async function () {
    await share.init_price();
    checkout = document.getElementById("checkout");
    checkout_btn = document.getElementById("checkoutbtn");
    checkout_txt = document.getElementById("checkouttxt");


    let array_cart = share.get_cart_total();
    let i = 0;
    let price = 0;
    array_cart.forEach((element) => {
        if (element > 0)
            create_cart_item(share.color_array[i], element * share.price_array[i], element, i);
        price += share.price_array[i] * parseInt(element);
        i += 1;
    })
    if (price == 0) {
        checkout.style.visibility = "hidden";
    }
    else {
        checkout.style.visibility = "visible";
        checkout_txt.innerHTML = "Total: " + price + " sek";
        checkout_btn.visibility = "visible"
        checkout_btn.addEventListener('click', async function () {
            await on_checkout();
        })

    }
});

function create_cart_item(productName, price, quantity, itemId) {
    const cartItem = document.createElement('div');
    cartItem.classList.add('cart-item');


    const cartItemDetails = document.createElement('div');
    cartItemDetails.classList.add('cart-item-details');

    const heading = document.createElement('h3');
    heading.textContent = productName;

    const priceParagraph = document.createElement('p');
    priceParagraph.textContent = `Price: ${price.toFixed(2)}`; // Assuming price is a number

    const quantityParagraph = document.createElement('p');
    quantityParagraph.textContent = `Quantity: ${quantity}`;

    const removeButton = document.createElement('button');
    removeButton.textContent = 'Remove';
    removeButton.onclick = function () {
        cartItem.remove();
        let array = share.get_cart_total();
        array[itemId] = 0;
        share.set_cart_items(array);
    };

    cartItemDetails.appendChild(heading);
    cartItemDetails.appendChild(priceParagraph);
    cartItemDetails.appendChild(quantityParagraph);
    cartItem.appendChild(cartItemDetails);
    cartItem.appendChild(removeButton);

    document.getElementById('append_id').appendChild(cartItem);
}




function add_order_item(product_id, amount, cost) {
    console.log("product_id: " + product_id);
    console.log("amount: " + amount);
    console.log("cost: " + cost);
    product_id += 1;
    order.push({ product_id, amount, cost });
}

async function on_checkout() {
    let array_cart = share.get_cart_total();
    let i = 0;
    let items = 0;
    array_cart.forEach((element) => {
        if (element > 0) {
            add_order_item(i, element, share.price_array[i]);
            items += 1;
        }
        i += 1;
    })

    const order_data = {
        order: {
            user_id: 1, // TODO
            product_amount: items,
            total_cost: get_total_price(),
            order_date: get_year_month_day(),
            order_timestamp: get_year_month_day_hour_minute_second(),
            status: 'READY'
        },

        'order-items': order,
    };
    console.log(JSON.stringify(order_data));
    await console.log(server.send_order(order_data));

    share.reset_cart_total();
    document.getElementById('append_id').innerHTML = "";

}
function get_total_price() {
    let i = 0;
    let price = 0;
    let array_cart = share.get_cart_total();
    array_cart.forEach((element) => {
        if (element > 0)
            create_cart_item(share.color_array[i], element * share.price_array[i], element, i);
        price += share.price_array[i] * parseInt(element);
        i += 1;
    })
    return price;
}
// POST /web_server/api/v1/orders HTTP/1.1

// Body {
//     "order": {
//         "user_id": 1,
//         "product_amount": 0,
//         "total_cost": 0,
//         "order_date": "2023-12-19",
//         "order_timestamp": "2023-12-24 13:37:00",
//         "status": "READY"
//     },

//     "order-items": [
//     {"order_id": 2, "product_id": 1, "amount": 2, "cost": 0},
//     {"order_id": 2, "product_id": 2, "amount": 3, "cost": 0},
//     {"order_id": 2, "product_id": 3, "amount": 3, "cost": 0}
//     ]
// }


function get_year_month_day() {
    const currentDate = new Date();
    const year = currentDate.getFullYear();
    const month = String(currentDate.getMonth() + 1).padStart(2, '0');
    const day = String(currentDate.getDate()).padStart(2, '0');

    const yearMonthDay = `${year}-${month}-${day}`;
    return yearMonthDay;
}

function get_year_month_day_hour_minute_second() {
    const currentDate = new Date();
    const year = currentDate.getFullYear();
    const month = String(currentDate.getMonth() + 1).padStart(2, '0');
    const day = String(currentDate.getDate()).padStart(2, '0');
    const hours = String(currentDate.getHours()).padStart(2, '0');
    const minutes = String(currentDate.getMinutes()).padStart(2, '0');
    const seconds = String(currentDate.getSeconds()).padStart(2, '0');

    const dateTimeString = `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
    return dateTimeString;
}