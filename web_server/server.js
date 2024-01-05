const api = "api/v1/"
const resource = "product_pages"
const root = window.location.origin + "/web_server/";

export async function get_product_pages() {
    return await fetch_data(resource);
}






async function fetch_data(uri) {
    try {
        console.log(root + api + resource);
        const response = await fetch(root);
        const data = response.json();
        return data;
    } catch (error) {
        // Handle errors here
        console.error('Error fetching data:', error);
        throw error; // You might want to throw the error for the caller to handle or log it as needed
    }
}