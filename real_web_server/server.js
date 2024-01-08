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