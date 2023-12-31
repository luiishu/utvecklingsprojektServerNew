const color_array = ["red", "green", "blue", "yellow"];
let error_array = [];
let storage_array = [];
let input_array = [];




document.addEventListener("DOMContentLoaded", function () {
    for (var i = 0; i < color_array.length; i++) {
        error_array[i] = document.getElementById("error_message_" + color_array[i])
        storage_array[i] = document.getElementById("storage_text_" + color_array[i]);
        input_array[i] = document.getElementById("item_yellow_" + color_array[i]);
    }

    setTimeout(on_timer, 1000);

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
}