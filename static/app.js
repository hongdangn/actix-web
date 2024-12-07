const API_URL = "http://127.0.0.1:8080/users";

async function fetchUsers() {
    const response = await fetch(API_URL);
    const users = await response.json();
    const tableBody = document.querySelector("#user-table tbody");
    tableBody.innerHTML = "";
    users.forEach((user) => {
        const row = document.createElement("tr");
        row.innerHTML = `
            <td>${user.id}</td>
            <td>${user.name}</td>
            <td>${user.age}</td>
            <td>${user.email}</td>
            <td>
                <button onclick="deleteUser(${user.id})">Delete</button>
                <button onclick="editUser(${user.id})">Edit</button>
            </td>
        `;
        tableBody.appendChild(row);
    });
}

async function addUser() {
    const id = prompt("Enter id:")
    const name = prompt("Enter name:");
    const age = prompt("Enter age:");
    const email = prompt("Enter email:");
    if (id && name && age && email) {
        await fetch(API_URL, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ id, name, age, email }),
        });
        fetchUsers();
    }
}

async function deleteUser(id) {
    await fetch(`${API_URL}/${id}`, { method: "DELETE" });
    fetchUsers();
}

async function editUser(id) {
    const name = prompt("Enter new name:");
    const age = prompt("Enter new age:");
    const email = prompt("Enter new email:");
    if (name && age && email) {
        await fetch(API_URL, {
            method: "PUT",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ id, name, age, email }),
        });
        fetchUsers();
    }
}

fetchUsers();
