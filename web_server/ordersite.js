const color_array = ["red", "green", "blue", "yellow"];
let error_array = [];
let storage_array = [];
let input_array = [];


<<<<<<< HEAD


document.addEventListener("DOMContentLoaded", function () {
    for (var i = 0; i < color_array.length; i++) {
        error_array[i] = document.getElementById("error_message_" + color_array[i])
        storage_array[i] = document.getElementById("storage_text_" + color_array[i]);
=======
const api = "api/v1/"
const resource = "product_pages"


console.log(document.cookie);
if (document.cookie == "") {
    // GO To login
    //http://127.0.0.1:7878/web_server/login/login.html
    window.location.assign("login/login.html");
}


document.addEventListener("DOMContentLoaded", function () { // storage_txt_blue
    for (var i = 0; i < color_array.length; i++) {

        let storage_id = "storagetxt" + color_array[i];



        error_array[i] = document.getElementById("error_message_" + color_array[i])
        storage_array[i] = document.getElementById(storage_id);
>>>>>>> c08daf20a958a8da8e6d9463743e27cd1dcf852c
        input_array[i] = document.getElementById("item_yellow_" + color_array[i]);
    }

    setTimeout(on_timer, 1000);

<<<<<<< HEAD
});

/// Periodically updating the lager status
function on_timer() {
    // TODO GET STORAGE FROM DATABASE
    sync_lager()
}

function sync_lager(storage_array_new) {

    for (var i = 0; i < storage_array.length; i++) {
        storage_array[i] = storage_array_new[i];
    }

}

function on_submit_pressed() {
    let input_val = [];

    let lager_max = [1, 1, 1, 1]; // TODO, get from database
=======
    console.log("loaded\n");

});

/// Periodically updating the lager status
async function on_timer() {
    // TODO GET STORAGE FROM DATABASE


    await sync_lager()
}

async function sync_lager() {

    let data = await fetch_data(resource);
    let lager_array = [0, 0, 0, 0];
    data.rows.forEach(element => {
        switch (element.color) {
            case "Red block":
                lager_array[0] += 1;
                break;
            case "Green block":

                lager_array[1] += 1;
                break;
            case "Blue block":
                lager_array[2] += 1;
                break;
            case "Yellow block":

                lager_array[3] += 1;
                break;
            default:
        }
    });

    for (let i = 0; i < lager_array.length; i++) {
        if (storage_array[i]) {
            storage_array[i].innerHTML = "Storage: " + lager_array[i]; // Correct the property name to innerHTML
        } else {
            console.error("Element with ID " + storage_id + " not found.");
        }

    }


}

function on_submit_pressed(lager_max) {
    let input_val = [];

>>>>>>> c08daf20a958a8da8e6d9463743e27cd1dcf852c

    for (var i = 0; i < input_array.length; i++) {
        let amount = 0;
        if (isNaN(input_array[i])) {
            if (lager_max[i] < input_array[i]) {
                error_array[i].Innerhtml = "Not enough items in storage";
                return;
            }
            amount = input_array[i];

        }
        input_val[i];
    }

    // input_val
    // FETCH from database
<<<<<<< HEAD
=======
}

async function fetch_data(uri) {
    try {
        const response = await fetch(api + uri);
        const data = await response.json();
        return data;
    } catch (error) {
        // Handle errors here
        console.error('Error fetching data:', error);
        throw error; // You might want to throw the error for the caller to handle or log it as needed
    }
>>>>>>> c08daf20a958a8da8e6d9463743e27cd1dcf852c
}