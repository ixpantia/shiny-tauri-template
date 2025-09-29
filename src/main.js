const { invoke } = window.__TAURI__.core;


function run_shiny_app() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  invoke("run_shiny_app")
    .then((shiny_app_url) => {
      console.log(shiny_app_url);
      (async () => {
        while (true) {
          window.location.assign(shiny_app_url);
          // Wait for a short period before attempting again,
          // allowing the event loop to remain non-blocked.
          await new Promise((resolve) => setTimeout(resolve, 1000)); // Waits for 0.2 second
        }
      })();
    })
    .catch((error) => {
      console.error(error);
      alert(error);
    });
}

window.addEventListener("DOMContentLoaded", () => {
  run_shiny_app();
});
