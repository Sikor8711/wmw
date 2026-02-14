# Wildly Magnetic Website

![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)
![Leptos](https://img.shields.io/badge/frontend-Leptos-orange)
![Axum](https://img.shields.io/badge/backend-Axum-darkred)
![Status](https://img.shields.io/badge/status-Active-success)

## 📖 About

This is the official landing page and digital product platform for Wildly Magnetic.

Built entirely in Rust, this application leverages the Leptos framework for a reactive, high-performance frontend and Axum for a robust backend. It is designed to handle digital product sales and automated marketing workflows efficiently.

## 🛠️ Tech Stack

### Core

- Language: [Rust](https://www.rust-lang.org/)
- Frontend: [Leptos](https://leptos.dev/) (WebAssembly/SSR)
- Backend Server: [Axum](https://github.com/tokio-rs/axum)

### Integrations

- Payments: [Lemon Squeezy](https://www.lemonsqueezy.com/) (Checkout & Merchant of Record)
- Email Automation: [Mautic](https://www.mautic.org/) (Lead generation & Email flows)

## ✨ Features

- Server-Side Rendering (SSR): Optimized for SEO and fast initial load times using Leptos + Axum.
- Secure Checkout: Seamless integration with Lemon Squeezy for handling digital product transactions.
- Marketing Automation: User actions trigger specific email flows and tagging within Mautic.
- Type-Safety: Full-stack type safety from database to UI.

## 🚀 Getting Started

### Prerequisites

Ensure you have the following installed:

- [Rust toolchain](https://rustup.rs/) (latest stable)
- `cargo-leptos` (for running the project)

  ```bash
  cargo install cargo-leptos
  ```
