document.getElementById("put-form").addEventListener("submit", function (event) {
    event.preventDefault(); // Impede o envio padrão do formulário

    // Obtém os dados do formulário
    const formData = new FormData(event.target.value);

    // Realiza uma solicitação PUT usando XMLHttpRequest ou Fetch API
    fetch("/add_book", {
        method: "PUT",
        body: formData,
    })
        .then((response) => {
            console.log("Ok ", formData)
        })
        .catch((error) => {
           console.log("aba")
        });
});