

let cart = JSON.parse(localStorage.getItem('cart')) || [];

let personalInfo = [];
/*
document.addEventListener('DOMContentLoaded', function () {
    // Load cart from localStorage
    let cart = JSON.parse(localStorage.getItem('cart')) || [];
    console.log(cart)
    // Render the cart on page load
    //renderCart(cart);
});

*/

let allProducts =[];


async function fetchProductsForCart() {
    try {
        const response = await fetch('/products.json'); // change to "api/v1/products"
        const allProducts = await response.json();

        return allProducts;

    } catch (error) {
      console.error('Error fetching products for cart:', error);
      return [];
    }
}  

function saveCartToLocalStorage() {
    localStorage.setItem('cart', JSON.stringify(cart));
}
  
// Fetch products for cart and render the cart  
async function init() {

    // Fetch products for items in the cart

    //const cartProductIds = cart.map(item => item.productId);

    // Fetch products for the cart
    allProduct = await fetchProductsForCart();

    //const cartProducts = await fetchProductsForCart();
  
    // Render the cart
    console.log("Cart: ", cart)
    renderCart(allProduct);
}

init();

async function renderTotalCost(){
    const cartTotals = document.getElementsByClassName("cart-total");
    const totalAmount = retrieveTotalCost();
    for(const cartTotal of cartTotals)
    {cartTotal.textContent = totalAmount + ' kr';}
    console.log("Total cost: ", totalAmount);
}
renderTotalCost();

function renderCart(allProducts) {
    const cartBody = document.getElementById('cart-body');
    const cartTotals = document.getElementsByClassName("cart-total");
    let totalAmount = 0;
    // Clear existing cart items
    cartBody.innerHTML = '';

    // Loop through each