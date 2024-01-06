import * as server from "../server.js"
import * as share from "./share.js"

let error_array = [];
let storage_array = [];
let input_array = [];
let btn_array = [];

let lager_array = [0, 0, 0, 0];
let price_array = [10, 20, 30, 40];

let cart_total;
let cart_items;
let cart_btn;

const sync_time_in_millie = 10000;






document.addEventListener("DOMContentLoaded", async function () { // storage_txt_blue


    for (let i = 0; i < share.color_array.length; i++) {

        let storage_id = "storagetxt" + share.color_array[i];

        error_array[i] = document.getElementById("errormessage" + share.color_array[i])
        storage_array[i] = document.getElementById(storage_id);
        input_array[i] = document.getElementById("item_yellow_" + share.color_array[i]);

        btn_array[i] = document.getElementById("btn" + share.color_array[i]);
        btn_array[i].addEventListener('click', (event) => {
            event.preventDefault()
            add_item(i);
        })
    }
    cart_total = document.getElementById("carttotal");
    cart_btn = document.getElementById("cartbtn").addEventListener('click', function () {
        window.location.href = "cart.html";
    });

    cart_items = share.get_cart_total();
    console.log(cart_items);
    let sum = 0;
    cart_items.forEach((element) => {
        sum += element;
    })
    cart_total.innerHTML = sum;
    setTimeout(on_timer, 500);
});




function add_item(index) {
    let storage = lager_array[index];
    let items = parseInt(cart_items[index]);
    if (storage > items) {
        cart_items[index] += 1;
        share.set_cart_items(cart_items);
        let sum = 0;
        cart_items.forEach((element) => {
            sum += element;
        })

        cart_total.innerHTML = sum;
    }
}

/// Periodically updating the lager status
async function on_timer() {

    await sync_lager()
    setTimeout(on_timer, sync_time_in_millie);
}

async function sync_lager() {

    let data = await server.get_product_pages();
    let lager = [0, 0, 0, 0];
    for (let i = 0; i < 4; i++) {
    }
    data.rows.forEach(element => {
        switch (element.color) {

            case "Red block":
                lager[0] += 1;
                break;

            case "Green block":
                lager[1] += 1;
                break;

            case "Blue block":
                lager[2] += 1;
                break;

            case "Yellow block":
                lager[3] += 1;
                break;

            default:
        }
    });
    for (let i = 0; i < lager_array.length; i++) {
        lager_array[i] = lager[i];
        if (storage_array[i]) {
            storage_array[i].innerHTML = "Storage: " + lager_array[i]; // Correct the property name to innerHTM
        } else {
            console.error("Element with ID " + i + " not found.");
        }

    }


}



async function create_order(user_id, product_amount, order_items) {

    const order_request = "POST /web_server/api/v1/orders HTTP/1.1"

    let total_cost;

    order_items.forEach(
        item => {
            total_cost += item.amount * price_array[item.product_id];
        }
    )

    const requestBody = {
        order: {
            user_id: user_id,
            product_amount: 0,
            total_cost: total_cost,
            order_date: get_year_month_day(),
            order_timestamp: get_year_month_day_hour_minute_second(),
            status: 'READY'
        },
        'order-items': [
            order_items
        ]
    };

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
