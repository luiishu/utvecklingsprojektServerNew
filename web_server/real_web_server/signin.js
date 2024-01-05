const login_form_container = document.getElementById("login-form-container");
const register_form_container = document.getElementById("register-form-container");
const register_button = document.getElementById("registerButton");
const login_button = document.getElementById("loginButton");

register_button.addEventListener('click', ()=>{
    register_form_container.style.display = 'block';
    login_form_container.style.display = 'none';
})

login_button.addEventListener('click', ()=>{
    register_form_container.style.display = 'none';
    login_form_container.style.display = 'block';
})


api = "/web_server/api/v1/";
resources = ["addresses", "users", 
            "orders", "order-items", "order-positions", 
            "products", "product-images", "product-reviews", "product-types"]
  
resource = "product_pages"
  
let login_form = document.getElementById("login_form");
login_form.addEventListener("submit", (e) => {
    e.preventDefault();
    let username = document.getElementById("username_login");
    let password = document.getElementById("password_login");
          
    if (username.value == "" || password.value == "") {
        alert("Ensure you input a value in both fields!");
    } 
    else {
            console.log(
                `This form has a username of ${username.value} and password of ${password.value}`
            );
  
            post_form(username.value, password.value, 'login', 'users').then(res => {
                console.log("Inside then function for post and the response is: " + res.ok);
                // Check if login was successful
                if (res.ok) {
                    window.location.href = `/profile?user=${username.value}`;
                  return res.json();
                } else {
                  throw new Error('Login failed');
                }
              })
              .then(userData => {
                // Redirect to the profile page with user-specific data
                window.location.href = `/profile?user=${username.value}`;
              })
              .catch(error => {
                console.error('Error:', error);
                alert('Login failed. Please try again.');
              });
              
            username.value = "";
            password.value = "";
            
    }
    
});
  
let register_form = document.getElementById("register_form");
register_form.addEventListener("submit", (e) => {
    e.preventDefault();
    let username = document.getElementById("username_register");
    let password = document.getElementById("password_register");
          
    if (username.value == "" || password.value == "") {
        alert("Ensure you input a value in both fields!");
    } else {
        console.log(
          `This form has a username of ${username.value} and password of ${password.value}`
        );
  
        post_form(username.value, password.value, 'register', 'users');
              
        username.value = "";
        password.value = "";
        }
});


//Testing

/*
function post_form(username, password, request_type, resource) {
    return fetch(api + resource, {
      method: 'POST',
      body: JSON.stringify({ username, password, request_type }),
      headers: {
        "Content-Type": "application/json"
      }
    })
    .then(async response => {
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
  
      try {
        const data = await response.json();
        console.log("Inside then function for post and the response is:", data);
        return data;
      } catch (error) {
        // If parsing as JSON fails, treat the response as text
        console.log("Parsing as JSON failed. Trying to parse as text.");
        const clonedResponse = response.clone();
        return clonedResponse.text();
      }
    })
    .then(data => {
      // Handle the response data as needed (e.g., redirect to profile)
      console.log("Processed data:", data);
      return data;
    })
    .catch(error => {
      console.error('Error:', error);
      // Handle errors appropriately, e.g., show an alert
      alert('Login failed. Please try again.');
      throw error; // Rethrow the error for further handling if necessary
    });
  }
  
 */ 

function post_form(username, password, request_type, resource) {
    //alert("Inside post form");
    console.log(`Received username: ${username}, password: ${password}`);
    fetch(api + resource, {
        method: 'POST',
        body: JSON.stringify({username, password, request_type}),
        headers: {
          "Content-Type": "application/json"
        }
    }
    ).then(res => {
        if (!res.ok) {
            throw new Error(`HTTP error! Status: ${res.status}`);
        }
        console.log(res.headers.getSetCookie());
        console.log(res.headers);
        console.log(res);
        return res;
    }
    ).then(data => {
        if (data) {
        console.log(data);
        //var length = data.rows.length;
        //console.log("array length: " + length);
        //const value = `${document.cookie}`;
        console.log("cookie: " + document.cookie);
        //console.log(value);
        //document.cookie = "a_cookie=says_hello";
        console.log("cookie: " + document.cookie);
        console.log("From setCookie header: " + data.headers.getSetCookie());
        return data;
        }
    })
        .catch((error) => {
            console.error('Error:', error);
            console.log("server is down!!")   
        });
}

  
function fetch_data(uri) {
    fetch(api + uri)
    .then(res => {
          return res.json();
    })
    .then(data => {
        console.log(data);
        var length = data.rows.length;
        console.log("array length: " + length);
  
        for(var i = 0; i < length; i++) {
            //console.log(i + ". " + Object.values(data.rows[i]));
            //console.log('[');
            var j = 0;
            entry = "["
            for (const [key, value] of Object.entries(data.rows[i])) {
                //console.log(`${i}. ${key}: ${value}`);
                var entryLength = Object.entries(data.rows[i]).length
  
                entry += `${key}: ${value}`
                if (j < entryLength - 1) {entry += ", "}
                j++
            }
              
            entry += "]"
            console.log(entry);
        }
  
        document.querySelector("ul").innerHTML = "";
  
        data.rows.forEach(product => {
            const markup = `<li>${Object.entries(product).flat()}</li>`;
            document.querySelector("ul").insertAdjacentHTML("beforeend", markup);
        });
  
    });
}