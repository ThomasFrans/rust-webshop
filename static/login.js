function login() {
    let form = document.forms["form-login"];
    let email = form["email"].value;
    let password = form["password"].value;

    if (email === "" || password === "") {
        alert("Please fill out both the email and password.");
        return false;
    }

    let request = new XMLHttpRequest();
    let api_endpoint = "/api/login";
    let form_data = new FormData();
    form_data.append("email", email);
    form_data.append("password", password);

    request.onreadystatechange = function() {
        if (this.readyState === 4) {
            if (this.status === 200) {
                console.log("logged in.");
                window.location = "/";
            } else {
                alert("Error logging in!");
            }
        }
    };

    request.open("POST", api_endpoint);
    request.send(form_data);
    return false;
}

// TODO: Maybe don't send a request to the server just to delete a cookie...
function logout() {
    let request = new XMLHttpRequest();
    let api_endpoint = "/api/logout";

    request.onreadystatechange = function() {
        if (this.readyState === 4) {
            if (this.status === 200) {
                window.location = "/";
            } else {
                alert("Error logging out!");
            }
        }
    };

    request.open("POST", api_endpoint);
    request.send();
    return false;
}