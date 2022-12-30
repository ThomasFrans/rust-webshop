function addUser() {
    let form = document.forms["form-add-user"];
    let first_name = form["first_name"].value;
    let surname = form["surname"].value;
    let phone = form["phone"].value;
    let email = form["email"].value;
    let password = form["password"].value;
    let is_admin = form["is_admin"].checked;

    let request = new XMLHttpRequest();
    let api_endpoint = "/api/users/add";
    let form_data = new FormData();
    form_data.append("first_name", first_name);
    form_data.append("surname", surname);
    form_data.append("phone", phone);
    form_data.append("email", email);
    form_data.append("password", password);
    form_data.append("is_admin", is_admin);

    request.onreadystatechange = function() {
        if (this.readyState === 4) {
            if (this.status === 200) {
                let user_table = document.getElementById("table-users");
                let response = JSON.parse(this.responseText);
                form.reset();

                user_table.innerHTML += "<tr id=\"user-table-row-"+response["user_id"]+"\"><td>"+response["user_id"]+"</td><td>"+response["first_name"]+"</td><td>"+response["surname"]+"</td><td>"+response["phone"]+"</td><td>"+response["email"]+"</td><td>"+response["password"]+"</td><td>"+response["is_active"]+"</td><td>"+response["is_admin"]+"</td></tr>";
            } else {
                alert("Email already in use. Use a different email.")
            }
        }
    };

    request.open("POST", api_endpoint);
    request.send(form_data);

    return false;
}

function removeUser() {
    let form = document.forms["form-remove-user"];
    let user_id = form["user_id"].value;

    let request = new XMLHttpRequest();
    let api_endpoint = "/api/users/remove";
    let form_data = new FormData();
    form_data.append("user_id", user_id);

    request.onreadystatechange = function() {
        if (this.readyState === 4) {
            if (this.status === 200) {
                let user_row = document.getElementById("table-users-row-"+user_id);
                user_row.outerHTML = "";
            } else {
                console.log("Error removing user.");
            }
        }
    };

    request.open("DELETE", api_endpoint);
    request.send(form_data);
    return false;
}

function addProduct() {
    let form = document.forms["form-add-product"];
    let name = form["name"].value;
    let description = form["description"].value;
    let image_uri = form["image_uri"].value;
    let price = form["price"].value;

    let request = new XMLHttpRequest();
    let api_endpoint = "/api/products/add";
    let form_data = new FormData();
    form_data.append("name", name);
    form_data.append("description", description);
    form_data.append("image_uri", image_uri);
    form_data.append("price", price);

    request.onreadystatechange = function() {
        if (this.readyState === 4) {
            if (this.status === 200) {
                let product_table = document.getElementById("table-products");
                let response = JSON.parse(this.responseText);
                form.reset();

                product_table.innerHTML += "<tr id=\"table-products-row-"+response["product_id"]+"\"><td>"+response["product_id"]+"</td><td>"+response["name"]+"</td><td>"+response["description"]+"</td><td>"+response["price"]+"</td><td>"+response["image_uri"]+"</td><td>"+response["is_active"]+"</td></tr>";
            } else {
                alert("Email already in use. Use a different email.")
            }
        }
    };

    request.open("POST", api_endpoint);
    request.send(form_data);

    return false;
}