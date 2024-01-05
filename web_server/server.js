const api = "api/v1/"
const root = window.location.origin + "/web_server/";

const orders = "orders"
const pages = "product_pages"
export async function get_product_pages() {
    try {
        const response = await fetch(root + api + pages);
        const data = await response.json();
        return data;
    } catch (error) {
        // Handle errors here
        console.error('Error fetching data:', error);
        throw error; // You might want to throw the error for the caller to handle or log it as needed
    }
}

export async function send_order(body) {
    try {
        const response = await fetch(root + api + orders, {
            method: "POST",
            body: JSON.stringify(body),
        });
        const data = await response.json();
        return await data;
    } catch (error) {
        // Handle errors here
        console.log("does it come here insted? " + JSON.stringify(error));
        return error;
    }
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



async function fetch_data(uri) {
    try {
        console.log(root + api + resource);
        const response = await fetch("");
        const data = await response.json();
        return data;
    } catch (error) {
        // Handle errors here
        console.error('Error fetching data:', error);
        throw error; // You might want to throw the error for the caller to handle or log it as needed
    }
}