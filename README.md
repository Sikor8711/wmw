# Wildly Magnetic Website

![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)
![Leptos](https://img.shields.io/badge/frontend-Leptos-orange)
![Actix](https://img.shields.io/badge/backend-Actix-green)
![Status](https://img.shields.io/badge/status-Active-success)

## 📖 About

This is the official landing page and digital product platform for **Wildly Magnetic**.

Built entirely in **Rust**, this application leverages the **Leptos** framework for a reactive, high-performance frontend and **Actix Web** for a robust backend. It is designed to handle digital product sales and automated marketing workflows efficiently.

## 🛠️ Tech Stack

### Core

- **Language:** [Rust](https://www.rust-lang.org/)
- **Frontend:** [Leptos](https://leptos.dev/) (WebAssembly/SSR)
- **Backend Server:** [Actix Web](https://actix.rs/)

### Integrations

- **Payments:** [Lemon Squeezy](https://www.lemonsqueezy.com/) (Checkout & Merchant of Record)
- **Email Automation:** [Mautic](https://www.mautic.org/) (Lead generation & Email flows)

## ✨ Features

- **Server-Side Rendering (SSR):** Optimized for SEO and fast initial load times using Leptos + Actix.
- **Secure Checkout:** Seamless integration with Lemon Squeezy for handling digital product transactions.
- **Marketing Automation:** User actions trigger specific email flows and tagging within Mautic.
- **Type-Safety:** Full-stack type safety from database to UI.

## 🚀 Getting Started

### Prerequisites

Ensure you have the following installed:

- [Rust toolchain](https://rustup.rs/) (latest stable)
- `cargo-leptos` (for running the project)
  ```bash
  cargo install cargo-leptos
  ```

### Installation

1.  **Clone the repository**

    ```bash
    git clone [https://github.com/yourusername/wildly-magnetic-website.git](https://github.com/yourusername/wildly-magnetic-website.git)
    cd wildly-magnetic-website
    ```

2.  **Environment Configuration**
    Create a `.env` file in the root directory and add your API keys:

    ```env
    LEMON_SQUEEZY_API_KEY=your_key_here
    MAUTIC_BASE_URL=[https://your-mautic-instance.com](https://your-mautic-instance.com)
    MAUTIC_PUBLIC_KEY=your_public_key
    MAUTIC_SECRET_KEY=your_secret_key
    RUST_LOG=info
    ```

3.  **Run the Application**
    To start the dev server with hot-reloading:

    ```bash
    cargo leptos watch
    ```

4.  **Build for Production**
    ```bash
    cargo leptos build --release
    ```

## 📂 Project Structure

- `src/app.rs`: Main frontend UI logic (Leptos).
- `src/main.rs`: Backend entry point (Actix).
- `src/api/`: Payment and Mautic integration logic.

## © Copyright & License

**All Rights Reserved.**

Copyright © 2025 **Wildly Magnetic**.

This source code is the proprietary property of Wildly Magnetic. Any unauthorized copying, alteration, distribution, transmission, performance, display, or other use of this material is strictly prohibited.
