let form = document.getElementById("form")
let discordIn = document.getElementById("discord")
let keyIn = document.getElementById("key")
let error = document.getElementById("error")

form.addEventListener("submit", async (e) => {
    e.preventDefault()
    let fetch_req = "/key/" + discordIn.value + "/" + keyIn.value
    
    let request = await fetch(fetch_req, { method: "POST" })
    let response = await request.json()

    error.innerText = response.answer
    error.style = "color: #" + response.color + ";"
})