async function next_path() {
    let res = await fetch("/source_next", { method: "POST" })
    let path = await res.text();
    console.log(path)
}