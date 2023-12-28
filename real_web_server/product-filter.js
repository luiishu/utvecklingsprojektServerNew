/*
let product = {
    data:[{
        id:1,
        productName: "Cute Cat",
        color: "Yellow",
        category: "Hoodie",
        brand: "Grupp4",
        review: 1,
        price:"199",
        image:"/img/clothes/hoodie/hoodie-yellow-front.png",
        hoverImage:"/img/clothes/hoodie/hoodie-yellow-back.png"

    },
    {
        id:2,
        productName: "Cute Cat",
        color: "Blue",
        brand: "Grupp4",
        review: 2,
        category: "Hoodie",
        price:"299",
        image:"/img/clothes/hoodie/hoodie-blue-front.png",
        hoverImage:"/img/clothes/hoodie/hoodie-blue-back.png",
    },
    {
        id:3,
        productName: "Cute Cat",
        color: "Red",
        category: "Hoodie",
        brand: "Grupp4",
        review: 3,
        price:"159",
        image:"/img/clothes/hoodie/hoodie-red-front.png",
        hoverImage:"/img/clothes/hoodie/hoodie-red-back.png"
    },
    {
        id:4,
        productName: "Cute Cat",
        color: "Green",
        brand: "Grupp4",
        review: 4,
        category: "Hoodie",
        price:"399",
        image:"/img/clothes/hoodie/hoodie-green-front.png",
        hoverImage:"/img/clothes/hoodie/hoodie-green-back.png"
    },
    {
        id:5,
        productName: "Cute Cat",
        color: "Green",
        brand: "Grupp4",
        review: 5,
        category: "T-shirt",
        price:"599",
        image:"/img/clothes/tshirt/tshirt-green-front.png",
        hoverImage:"/img/clothes/tshirt/tshirt-green-back.png"
    },
    {
        id:6,
        productName: "Cute Cat",
        color: "Blue",
        category: "T-shirt",
        brand: "Grupp4",
        review: 3,
        price:"699",
        image:"/img/clothes/tshirt/tshirt-blue-front.png",
        hoverImage:"/img/clothes/tshirt/tshirt-blue-back.png"
    },
    {
        id:7,
        productName: "Cute Cat",
        color: "Yellow",
        brand: "Grupp4",
        review: 2,
        category: "T-shirt",
        price:"799",
        image:"/img/clothes/tshirt/tshirt-yellow-front.png",
        hoverImage:"/img/clothes/tshirt/tshirt-yellow-back.png"
    },
    {
        id:8,
        productName: "Cute Cat",
        color: "Red",
        category: "T-shirt",
        brand: "Grupp4",
        review: 1,
        price:"899",
        image:"/img/clothes/tshirt/tshirt-red-front.png",
        hoverImage:"/img/clothes/tshirt/tshirt-red-back.png"
    },
    {
        id:9,
        productName: "Cute Cat",
        color: "Yellow",
        category: "Pants",
        brand: "Grupp4",
        review: 1,
        price:"799",
        image:"/img/clothes/pants/pants-yellow-front.png",
        hoverImage:"/img/clothes/pants/pants-yellow-back.png"
    },
    {
        id:10,
        productName: "Cute Cat",
        color: "Red",
        category: "Pants",
        brand: "Grupp4",
        review: 5,
        price:"899",
        image:"/img/clothes/pants/pants-red-front.png",
        hoverImage:"/img/clothes/pants/pants-red-back.png"
    },
    {
        id:11,
        productName: "Cute Cat",
        color: "Blue",
        category: "Pants",
        brand: "Grupp4",
        review: 3,
        price:"599",
        image:"/img/clothes/pants/pants-blue-front.png",
        hoverImage:"/img/clothes/pants/pants-blue-back.png"
    },
    {
        id:12,
        productName: "Cute Cat",
        color: "Green",
        category: "Pants",
        brand: "Grupp4",
        review: 4,
        price:"699",
        image:"/img/clothes/pants/pants-green-front.png",
        hoverImage:"/img/clothes/pants/pants-green-back.png"
    }]
};
*/
/*

for(let i of products.data){
    //Create product template
    let card = document.createElement("div");
    //Category
    card.classList.add("card","i.category");
    //Img
    let imgContainer = document.createElement("div");
    imgContainer.classList.add("image-container");
    //Img Tag
    let image = document.createElement("img");
    image.setAttribute("src", i.image);
    imageContainer.appendChild(image);
    card.appendChild(imageContainer);


    document.getElementById("product-container").appendChild(card);
}
*/



let productCards = [];
let products;


async function fetchProducts() {
    try {
        const response = await fetch('/products.json');
        const data = await response.json();
        return data;
    } catch (error) {
        console.error('Error fetching products:', error);
        return null;
    }
}

async function init() {
    const products = await fetchProducts();

    if (products) {
        //Add here what you want to be rendered
        renderProducts(products, "product-container");
        renderProducts(products, "product-container-homepage");
    }
    else {
        console.error('Failed to fetch products. RenderProducts will not be called.');
      }
}

init();


let cart = JSON.parse(localStorage.getItem('cart')) || [];
function saveCartToLocalStorage() {
    // Save cart to localStorage
    localStorage.setItem('cart', JSON.stringify(cart));
  }

// Function to add a product to the cart
function addToCart(products, id) {
  // Find the product in the 'products' array based on the productId
 // const product = products.find(p => p.id === productId);

  //if (product) {
    // Check if the product is already in the cart
    const existingItemIndex = cart.findIndex(item => item.id === id);

    if (existingItemIndex !== -1) {
      // If the product is already in the cart, increase the quantity
      cart[existingItemIndex].quantity++;
    } else {
      // If the product is not in the cart, add it with quantity 1
      cart.push({id, quantity: 1 });
    }


    saveCartToLocalStorage();
    // Update the cart display
   // console.log("Product added to cart:", product);
    console.log("Updated cart:", cart);
    //renderCart();
  //}
}

function renderProducts(product, targetContainerId) {
    const productContainer = document.getElementById(targetContainerId);

    if (!productContainer) {
        console.error(`Container with ID ${targetContainerId} not found.`);
        return;
    }

    product.data.forEach((product, index) => {
        const productCard = document.createElement("div");
        productCard.classList.add("pro");  // Add the 'pro' class



        productCard.addEventListener("mouseover", () => changeImage(productCard, product.hoverImage));
        productCard.addEventListener("mouseout", () => restoreImage(productCard, product.image));

        // Add a unique class based on the index
        productCard.classList.add(`product-${index + 1}`);

        productCard.dataset.category = product.category;
        productCard.dataset.color = product.color;

        // Rest of the code remains the same
        const productImage = document.createElement("img");
        productImage.src = product.image;
        productImage.alt = product.productName;

        const productDescription = document.createElement("div");
        productDescription.classList.add("des");

        const productBrand = document.createElement("span");
        productBrand.textContent = product.brand;

        const productTitle = document.createElement("h5");
        productTitle.textContent = `${product.productName} ${product.category} - ${product.color}`;

        const starContainer = document.createElement("div");
        starContainer.classList.add("star");

        for (let i = 0; i < product.review; i++) {
            const star = document.createElement("i");
            star.classList.add("fas", "fa-star");
            starContainer.appendChild(star);
        }

        const productPrice = document.createElement("h4");
        productPrice.textContent = `${product.price} kr`;

        // Cart icon
        const cartIcon = document.createElement("button");
        cartIcon.addEventListener("click", () => addToCart(product, product.id));
        const cartIconElement = document.createElement("i");
        cartIconElement.classList.add("fa-solid", "fa-shopping-cart", "cart");

        cartIcon.appendChild(cartIconElement);

        // Append elements to the product card
        productCard.appendChild(productImage);
        productCard.appendChild(productDescription);
        productCard.appendChild(cartIcon);

        productDescription.appendChild(productBrand);
        pro