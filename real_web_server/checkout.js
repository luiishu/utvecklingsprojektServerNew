import * as share from "./share.js"
import * as server from "./server.js"
import * as cart from "./cart.js"


let order = [];
const orderButton = document.getElementById("orderbutton");
//orderButton.addEventListener("click", createOrder());

async function init_checkout(){
    console.log("Ready to Create order");
    orderButton.addEventListener("click", createOrder);
}

init_checkout();
function createOrder() {
    console.log("Create Oreder function is running");
    const contact_details = cart.getContactDetails();
    let array_cart = share.get_cart_total();
    console.log("Checkout array_cart: ", array_cart);
    let i = 0;
    let items = 0;
    array_cart.forEach(element => {    
        if (element) {
            add_order_item(i, element, share.price_array[i]);
            items += 1;
        }
        i += 1;
    })

    console.log("Checkout items value: ", items);
    // add_order_item(2, 1, 2, 0);
    // add_order_item(2, 1, 2, 0);
    // add_order_item(2, 1, 2, 0);

    //     {"order_id": 2, "product_id": 3, "amount": 3, "cost": 0}

    const order_data = {
        order: {
            user_id: cart.createUserId(contact_details.email, contact_details.phoneNumber), // TODO
            product_amount: items,
            total_cost: cart.retrieveTotalCost(),
            order_date: cart.getTodayDate(),
            order_timestamp: cart.getTimeStamp(),
            status: 'READY'
        },

        'order-items': order,
    };
    console.log(JSON.stringify(order_data));
    console.log(server.send_order(order_data));

    share.reset_cart_total();
}