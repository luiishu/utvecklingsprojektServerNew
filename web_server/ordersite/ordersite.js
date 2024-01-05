import {product_pages} from "../server.js"


const color_array = ["red", "green", "blue", "yellow"];
let error_array = [];
let storage_array = [];
let input_array = [];
let price_array = [10, 20, 30, 40];


const user_id = 0; // TODO FIX

const sync_time_in_millie = 1000;


if (document.cookie == "") {
    // GO To login
    //http://127.0.0.1:7878/web_server/login/login.html
    window.location.assign("login/login.html");
}



document.addEventListener("DOMContentLoaded", function () { // storage_txt_blue
    for (var i = 0; i < color_array.length; i++) {

        let storage_id = "storagetxt" + color_array[i];



        error_array[i] = document.getElementById("error_message_" + color_array[i])
        storage_array[i] = document.getElementById(storage_id);
        input_array[i] = document.getElementById("item_yellow_" + color_array[i]);
    }

    setInterval(on_timer, sync_time_in_millie);
});

/// Periodically updating the lager status
async function on_timer() {
    // TODO GET STORAGE FROM DATABASE
    await sync_lager()
}

async function sync_lager() {

    let data = await product_pages();
    let lager_array = [0, 0, 0, 0];
    data.rows.forEach(element => {
        switch (element.color) {

            case "Red block":
                lager_array[0] += 1;
                break;

            case "Green block":
                lager_array[1] += 1;
                break;

            case "Blue block":
                lager_array[2] += 1;
                break;

            case "Yellow block":
                lager_array[3] += 1;
                break;

            default:
        }
    });

    for (let i = 0; i < lager_array.length; i++) {
        if (storage_array[i]) {
            storage_array[i].innerHTML = "Storage: " + lager_array[i]; // Correct the property name to innerHTML
        } else {
            console.error("Element with ID " + i + " not found.");
        }

    }


}

function on_submit_pressed(lager_max) {
    let input_val = [];


    for (var i = 0; i < input_array.length; i++) {
        let amount = 0;
        if (isNaN(input_array[i])) {
            if (lager_max[i] < input_array[i]) {
                error_array[i].Innerhtml = "Not enough items in storage";
                return;
            }
            amount = input_array[i];

        }
        input_val[i];
    }

    // input_val
    // FETCH from database
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
