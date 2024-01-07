import * as server from "../server.js"
import * as share from "./share.js"
console.log("hello?");
let error_array = [];
let storage_array = [];
let input_array = [];
let btn_array = [];
let price_array = [];

let lager_array = [0, 0, 0, 0];

let cart_total;
let cart_items;
let cart_btn;

const sync_time_in_millie = 10000;
alert(localStorage.getItem("username"));

document.addEventListener("DOMContentLoaded", async function () { // storage_txt_blue
    await share.init_price();
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
        price_array[i] = document.getElementById("price" + share.color_array[i]);
        price_array[i].innerHTML = "Pris: " + share.price_array[i] + " kr";
        console.log("hello??");

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
    await on_timer();
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
        lager[i] = data.rows[i].amount;
    }

    for (let i = 0; i < lager_array.length; i++) {
        lager_array[i] = lager[i];
        if (storage_array[i]) {
            storage_array[i].innerHTML = "Storage: " + lager_array[i]; // Correct the property name to innerHTM
        } else {
            console.error("Element with ID " + i + " not found.");
        }

    }


}
