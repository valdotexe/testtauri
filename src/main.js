const save_button = document.getElementById("save_button");
const name_list = document.getElementById("name_list");
const input_name = document.getElementById("input_name");
const namesDiv = document.getElementById("name");

var names = [];

save_button.addEventListener("click", () => {
    const name = input_name.value.trim();
    if (name){
        names.push(name);
        input_name.value = "";
        alert("name saved");
    } else {
        alert("enter a name");
    }
});

name_list.addEventListener("click", () =>{
    if (names.length > 0) {
        namesDiv.innerHTML = "<h2>saved names: </h2><ul>" + names.map(name => "<li>${name}</li>").join("") + "</ul>";
     } else {
        namesDiv.innerHTML = "<h2>no names saved yet</h2>"
    }
});

