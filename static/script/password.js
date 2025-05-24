let pswrd = document.getElementById("pswrd")
let next = document.getElementById("next")
let error = document.getElementById("error")

next.addEventListener("submit", async (e) => {
    e.preventDefault()

    let fetch_req = "/password_check/" + pswrd.value
    let request = await fetch(fetch_req, { method: "POST" })

    if (request.redirected) {
        window.location.href = request.url
        return;
    }

    let response = await request.text()

    error.innerText = response
})