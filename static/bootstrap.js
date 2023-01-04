function createAdministrator() {
    let form = document.forms["form-create-administrator"];
    let first_name = form["first_name"].value;
    let surname = form["surname"].value;
    let phone = form["phone"].value;
    let email = form["email"].value;
    let password = form["password"].value;

    let request = new XMLHttpRequest();
    let api_endpoint = "/api/users/add";
    let form_data = new FormData();
    form_data.append("first_name", first_name);
    form_data.append("surname", surname);
    form_data.append("phone", phone);
    form_data.append("email", email);
    form_data.append("password", password);
    form_data.append("is_admin", true);

    request.onreadystatechange = function() {
        if (this.readyState === 4) {
            if (this.status === 200) {
                window.location = "/"
            } else {
                alert("Invalid data.")
            }
        }
    };

    request.open("POST", api_endpoint);
    request.send(form_data);

    return false;
}