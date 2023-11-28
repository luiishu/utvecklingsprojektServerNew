//document.write("hello!");

function hello() {
    console.log("Hello!");
}

function fetch_test() {
    fetch("something")
    .then(res => {
        return res.json();
    })
    .then(data => {
        console.log(data);
    });

}

function fetch_test_no_json() {
    fetch("something")
    .then(res => {
        return res.text();
    })
    .then(data => {
        console.log(data);
    });

}

function fetch_latest_product() {
    fetch("latest-product")
    .then(res => {
        return res.text();
    })
    .then(data => {
        console.log(data);
    });
}

window.onload = function() {
    console.log("Hello!");
    fetch_test_no_json();
    //fetch_latest_product();
};