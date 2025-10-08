
const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event; 


function run_shiny_app() {
    // Esto inicia R.exe y el temporizador/emisor de eventos en el backend.
    invoke("run_shiny_app")
        .then(() => {
            // el evento 'shiny_ready' se reciba 
            console.log("Comando run_shiny_app iniciado en Rust. Esperando evento 'shiny_ready'...");
      })
      .catch((error) => {
        console.error(error);
        alert(error);
      });
}

// Configura el listener de eventos
listen('shiny_ready', (event) => {
    const shinyUrl = event.payload;
    console.log("Evento 'shiny_ready' recibido. Redirigiendo a:", shinyUrl);

    // realizamos la redirección para cargar la interfaz.
    window.location.assign(shinyUrl); 
});


window.addEventListener("DOMContentLoaded", () => {
    run_shiny_app();
});