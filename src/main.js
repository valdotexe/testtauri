const { invoke } = window.__TAURI__.tauri;

async function insert() {
    console.log("testresrees")
    await invoke("insert", { name: input_name.value.trim() });
}

async function select() {
    let names = await invoke("select", {});
    let namesdiv = document.getElementById("names_div")

    namesdiv.innerHTML = "<h1>names:</h1>" +
        "<ul>" + names.map(name => `<li>${name}</li>`) + "</ul>"
}

window.addEventListener("DOMContentLoaded", () => {
    let save_button = document.getElementById("save_button");
    let name_list = document.getElementById("name_list");

    save_button.addEventListener("click", (e) => {
        e.preventDefault();
        insert();
    });

    name_list.addEventListener("click", (e) => {
        e.preventDefault();
        select();
    });
});
