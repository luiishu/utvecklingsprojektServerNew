fetch_arg = "api/v1/products"
      api = "api/v1/"
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
          } else {
              console.log(
                  `This form has a username of ${username.value} and password of ${password.value}`
              );
  
              post_form(username.value, password.value, 'login', 'users');
              
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
              console.log(res.headers.getSetCookie());
              console.log(res.headers);
              console.log(res);
              return res;
          }
          ).then(data => {
            console.log(data);
            //var length = data.rows.length;
            //console.log("array length: " + length);
            //const value = `${document.cookie}`;
            console.log("cookie: " + document.cookie);
            //console.log(value);
            //document.cookie = "a_cookie=says_hello";
            console.log("cookie: " + document.cookie);
            console.log("From setCookie header: " + data.headers.getSetCookie());
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