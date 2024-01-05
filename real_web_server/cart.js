

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

    // Loop through each item in the cart and render it
    cart.forEach(cartItem => {
        const product = allProducts.data.find(p => p.id === cartItem.id);
        if(product){ 
            const row = document.createElement('tr');
            row.innerHTML = `
                <td><button class="remove-item" data-product-id="${product.id}">Remove</button></td>
                <td><img src="${product.image}" alt="${product.productName}" width="50"></td>
                <td>${product.productName}</td>
                <td>${product.price} kr</td>
                <td><input name="quantity" type="number" value="${cartItem.quantity}" data-product-id="${product.id}"></td>
                <td>${cartItem.quantity * product.price} kr</td>
                `;
          
            cartBody.appendChild(row);
            //Eventlistener for input quantity
            const quantityInput = row.querySelector('input[name="quantity"]');
            quantityInput.addEventListener('input', function () {
                updateCartQuantity(product.id, parseInt(this.value, 10));
            });
            totalAmount += cartItem.quantity * product.price;
        }
        else{
            console.error("Product not found for ID ${cartItem.id}");
        }
    });

    storeTotalCost(totalAmount);
    console.log("Total cost: ", totalAmount);
    for(const cartTotal of cartTotals)
    {cartTotal.textContent = totalAmount + ' kr';}
    
  
    // Add event listener for remove buttons
    const removeButtons = document.querySelectorAll('.remove-item');
    removeButtons.forEach(button => {
        button.addEventListener('click', function () {
        const id = this.getAttribute('data-product-id');
        removeFromCart(id);
        });
    });
}


function storeTotalCost(totalCost) {
    localStorage.setItem('totalCost', totalCost.toString());
}

function retrieveTotalCost() {
    const totalCost = localStorage.getItem('totalCost');
    // Ensure totalCost is a number
    return parseFloat(totalCost) || 0;
}
  
function removeFromCart(id) {
    console.log("id in remove: ", id)
    // Find the item in the cart
    const cartItemIndex = cart.findIndex(item => item.id === Number(id));
    console.log("CartItem index in remove: ", cartItemIndex);
    console.log("Cart in remove: ", cart);

    if (cartItemIndex !== -1) {
        // Decrease the quantity
        if (cart[cartItemIndex].quantity > 0) {
            cart[cartItemIndex].quantity--;
        }
        
        // Remove the item from the cart if the quantity is 0
        if (cart[cartItemIndex].quantity === 0) {
            cart.splice(cartItemIndex, 1);
        }
  
        // Update the local storage
        updateLocalStorage();
        console.log("Updated Cart: ", cart);
         // Render the updated cart
        init();
    }
    else{
        console.log("No item removed or decreased");
    }
  }
  
  // Function to update local storage
function updateLocalStorage() {
    localStorage.setItem('cart', JSON.stringify(cart)); 
}
  
function updateCartQuantity(productId, newQuantity) {
    const cartItem = cart.find(item => item.id === productId);
    if (cartItem) {
        cartItem.quantity = newQuantity;
        updateLocalStorage();
        init();
    }
}



/* The checkout function and order creation */



async function sendOrderToServer(data, table) {

        const response = await fetch(`http://localhost:3000/${table}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(data),
        });
        /*
        if (response.ok) {
            console.log('Order sent to server successfully.');

        } else {
            console.error('Failed to send order to server:', response.statusText);
        }
        */
        if (!response.ok) {
            throw new Error(`Failed to send data to server. Status: ${response.status}`);
        }
    
        return response.json();
}


function clearAllLocalStorage (){
    localStorage.clear();
    console.log("Local Storage CLeared");
}

async function createOrder() {
    try { 
        const product_order_ids = generateOrderId();
        const order_ids = Math.floor(Number(generateOrderId())/1000);

        const contact_details = getContactDetails();
        contact_details.user_id = createUserId(contact_details.email, contact_details.phoneNumber);

        tempCart = cart;
        //The content for an order
        const orderItems = tempCart.map((item, index) =>({
            id: index + 1,
            products_order_id: product_order_ids,
            product_id: item.id,
            amount: item.quantity,
            cost: 0,
        }));

        const order = {
            order_id: order_ids,
            products_order_id: product_order_ids,
            user_id:contact_details.user_id,
            total_cost: retrieveTotalCost(),
            order_date: getTodayDate(),
            time_stamp: getTimeStamp(),
            status: "READY",
        };
        
       
        //storeOrderId(order_ids);
        console.log("Total Cost: ", retrieveTotalCost());
        console.log("Order for json created:", order);
        console.log("Order_items for json created:", orderItems);
        console.log("Contact details for json created:", contact_details);
        await sendOrderToServer(order, "orders");
        await sendOrderToServer(orderItems, "order_items");
        await sendOrderToServer(contact_details, "contact_info");

        clearAllLocalStorage();

    }
    catch(error){
        console.error('Error creating order:', error);
    }

}

function getTimeStamp(){
    const today = new Date();
    const time = today.toTimeString().split(' ')[0];;
    return time;
}

function getTodayDate(){
    const today = new Date();
    const date = today.toISOString().split('T')[0];
    return date;
}

function generateOrderId(){
    const orderId = Date.now();
    return orderId;
}

function handleCheckout() {
    const orderData = createOrder(cart);
    writeToJsonFile(orderData, 'order.json');
    consol.log("we are in the handleCheckout");
    window.location.href = 'checkout.html';
}

function writeToJsonFile(data, filename) {
    fs.writeFileSync(filename, JSON.stringify(data, null, 2));
}

/*
const checkoutButton = document.getElementById("proceedWithCheckout");
checkoutButton.addEventListener('click', handleCheckout);
*/

function storeOrderId(orderId) {
    localStorage.setItem('tempOrderId', orderId);
}

function getStoredOrderId() {
    return localStorage.getItem('tempOrderId');
}

function clearStoredOrderId() {
    localStorage.removeItem('tempOrderId');
}


//Contact info
function storeContactDetails() {
    const contactDetails = {
        user_id:0,
        email: document.querySelector('#personal-info input[placeholder="Email"]').value,
        firstName: document.querySelector('#personal-info input[placeholder="First Name"]').value,
        lastName: document.querySelector('#personal-info input[placeholder="Last Name"]').value,
        phoneNumber: document.querySelector('#personal-info input[placeholder="Phone Number"]').value,
        address1: document.querySelector('.shipping-details input[placeholder="Adress 1"]').value,
        address2: document.querySelector('.shipping-details input[placeholder="Adress 2 (Optinal)"]').value,
        postalCode: document.querySelector('.shipping-details input[placeholder="Postal Code"]').value,
        city: document.querySelector('.shipping-details input[placeholder="City"]').value,
        country: document.querySelector('.shipping-details input[placeholder="Country"]').value,
    };

    // Store the contact details in local storage
    localStorage.setItem('contactDetails', JSON.stringify(contactDetails));
    console.log(contactDetails);
    window.location.href =''
}

document.getElementById('order-details-container').addEventListener('submit', function (event) {
    event.preventDefault();
    storeContactDetails();
    window.location.href = 'checkout.html';
});


// Function to retrieve contact details from local storage
function getContactDetails() {
    const storedDetails = localStorage.getItem('contactDetails');
    return storedDetails ? JSON.parse(storedDetails) : null;
}


//Order creation




//Temp Functions to try this

function createUserId(email, phoneNumber) {
    const uniqueString = email + phoneNumber;
    // You can use a hashing algorithm here, for example:
    const hashedUserId = hashFunction(uniqueString);
    return hashedUserId;
  }
  function hashFunction(input) {
    let hash = 0;
    for (let i = 0; i < input.length; i++) {
      const char = input.charCodeAt(i);
      hash = (hash << 5) - hash + char;
    }
    if(hash<0){
        hash = hash*-1;
    }
    return hash;
  }