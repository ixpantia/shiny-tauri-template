# Shiny Tauri

This project provides a template and tooling to package a Shiny application as a standalone desktop application for macOS using the [Tauri](https://tauri.app/) framework.

The goal is to create a distributable `.app` bundle that contains your Shiny app, a self-contained R environment, and all necessary R packages, so your users don't need to install R or any packages themselves.

**Platform Support:**
-   [x] macOS (Apple Silicon/ARM64)
-   [ ] Windows (Coming Soon)

## How It Works

This project automates the process of bundling a portable R environment with your Shiny application. Here's a high-level overview of the process:

1.  **Bundling R**: The build script downloads a specific version of R for macOS and extracts its core components into the `src-tauri/local-r` directory. This makes the R environment portable and self-contained within the final application.
2.  **Dependency Management**: We use [`rv`](https://github.com/a2-ai/rv), a simple and fast package manager for R, to install the R packages your Shiny app needs. The build script runs `rv sync` to install dependencies listed in your app into the bundled R environment.
3.  **Tauri Integration**: The Tauri application is configured to do the following at runtime:
    *   Dynamically locate the bundled R environment and Shiny app code.
    *   Start the R process in the background on a random, available port.
    *   Launch your Shiny application using `shiny::runApp()`.

## Prerequisites

Before you begin, ensure you have the following tools installed on your system:

-   **Rust and Cargo**: Follow the instructions at [rustup.rs](https://rustup.rs/).
-   **Tauri CLI**: Once Rust is installed, you can install the Tauri CLI with Cargo:
    ```sh
    cargo install tauri-cli
    ```
-   **macOS Command Line Tools**: Ensure you have tools like `curl`, `tar`, and `pkgutil`. These are typically installed with Xcode or by running `xcode-select --install`.

## Usage Guide

Follow these steps to package your own Shiny app.

### Step 1: Clone the Repository

Clone this repository to your local machine:

```sh
git clone https://github.com/ixpantia/shiny-tauri.git
cd shiny-tauri
```

### Step 2: Prepare Your Shiny App

Your Shiny application code (e.g., `app.R`, `ui.R`, `server.R`, and any related
files) must be placed inside the `shiny-app/` directory. If this directory does
not exist, create it. This app must use `rv` for dependencies. In order for
this setup to work all libraries must be statically linked. If you need dynamic
linking the bundling process may become more complicated and requires knowledge
on shipping dynamic libraries.

### Step 3: Build the Application

Run the macOS build script from the root of the project:

```sh
./build-macos.sh
```

This script will perform all the necessary steps:
-   Download a standalone R package and the `rv` tool (if not already present).
-   Set up the `src-tauri/local-r` directory.
-   Copy your app from `shiny-app/` to `src-tauri/app/`.
-   Install all your R package dependencies using `rv sync`.
-   Build the final Tauri application.

The first build may take a significant amount of time as it needs to download a multi-hundred MB R installer. Subsequent builds will be much faster.

### Step 4: Run Your Desktop App

Once the build is complete, you will find your packaged application in the `src-tauri/target/release/bundle/macos/` directory. The file will be named something like `shiny-tauri.app`.

You can double-click this file to run it, or move it to your `Applications` folder.

## Customization

You can customize the application's name, identifier, and icon by editing the Tauri configuration file.

### Changing the App Name

To change the name of your application, you need to modify the `src-tauri/tauri.conf.json` file.

1.  **Product Name**: Change the `productName` field. This is the main name of your application that will appear in the file system (e.g., `YourAppName.app`).

    ```json
    "productName": "your-app-name",
    ```

2.  **Bundle Identifier**: Update the `identifier` field. This is a unique reverse-domain-name style identifier for your app (e.g., `com.yourcompany.yourapp`).

    ```json
    "identifier": "com.yourcompany.your-app-name",
    ```

3.  **Window Title**: Change the `title` field inside the `windows` array. This is the title that appears at the top of the application window.

    ```json
    "windows": [
      {
        "title": "Your App Title"
      }
    ]
    ```

### Changing the Desktop Icon

The application icon is configured in `src-tauri/tauri.conf.json` under the `bundle.icon` section. To change the icon, you need to replace the default icon files located in the `src-tauri/icons/` directory with your own.

1.  **Prepare Your Icons**: Create your application icons in various sizes. The Tauri documentation provides guidance on the required formats for different platforms. At a minimum, you should have:
    *   `icon.icns` for macOS.
    *   `icon.ico` for Windows.
    *   Various PNG sizes (e.g., `32x32.png`, `128x128.png`) for Linux and other uses.

2.  **Replace the Files**: Overwrite the existing files in the `src-tauri/icons/` directory with your new icon files. Make sure the filenames match what is listed in `tauri.conf.json`.

3.  **Rebuild**: After making these changes, rebuild your application using the `./build-macos.sh` script for the changes to take effect.

## Project Structure

-   `shiny-app/`: **(User provided)** This is where you place your Shiny application source code and `dependencies.R` file.
-   `build-macos.sh`: The main build script for macOS. It orchestrates the entire process of downloading dependencies and building the app.
-   `src-tauri/`: The Tauri application source directory.
    -   `src/lib.rs`: The core Rust backend code that starts the Shiny R process at runtime.
    -   `tauri.conf.json`: The main Tauri configuration file.
    -   `app/`: An internal copy of your Shiny app, created by the build script.
    -   `local-r/`: An internal, self-contained R environment, created by the build script.
-   `macos-pkg/`: A directory created by the build script to cache downloaded files like the R installer and `rv`.
