# API Client Tauri App (API Client for REST APIs)

This project is a Tauri-based desktop application that acts as an API client. It allows users to interact with REST APIs by sending HTTP requests (GET, POST, PUT, DELETE, PATCH) and viewing responses. The main goal of this project is to help developers test and debug REST APIs efficiently and intuitively.

---

## 🚀 Table of Contents

1. [Introduction](#-introduction)
2. [Features](#-features)
3. [Getting Started](#-getting-started)
   - [Prerequisites](#prerequisites)
   - [Installation](#installation)
   - [Running the App](#running-the-app)
4. [Using the App](#-using-the-app)
5. [Configuration](#️-configuration)
6. [Troubleshooting](#-troubleshooting)
7. [License](#-license)
8. [Contact](#-contact)

---

## 📖 Introduction

This API Client enables you to test and interact with REST APIs effortlessly. You can easily:

- Send **GET, POST, PUT, DELETE, PATCH** requests
- Set custom HTTP headers
- Send JSON or form-encoded data as request bodies
- View API responses (status codes, headers, and body)

---

## 📝 Features

- **Support for All HTTP Methods**: GET, POST, PUT, DELETE, PATCH
- **Custom Headers**: Easily add custom headers for your requests.
- **Request Body Support**: Send JSON or form data as request bodies.
- **API Response Inspection**: View status codes, headers, and response bodies.
- **Cross-Platform**: Works on **Windows, macOS**, and **Linux**.

---

## 🔧 Getting Started

### Prerequisites

Make sure you have the following tools installed before running the project:

- Rust
- Node.js & npm
- Tauri CLI

---

### Installation

- Clone the repository.

```bash
git clone https://github.com/your-username/api-client-tauri.git
cd api-client-tauri
```

- Install dependencies.

```bash
npm install
```

- Build the project.

```bash
cargo tauri dev
```

---

### Running the App

Start your application. The app will open, and you'll be ready to interact with REST APIs.

```bash
npm run tauri dev
```

---

## 🔍 Using the App

### **1. Select HTTP Method**

- Choose the desired HTTP method (GET, POST, PUT, DELETE, PATCH).

### **2. Enter API URL**

- In the input field, type the URL of the API endpoint.

### **3. Set Headers**

- Provide custom HTTP headers in key-value pairs.

### **4. Define Request Body**

- For methods like **POST/PUT/DELETE/PATCH**, you can provide JSON or form-encoded data.

### **5. Send Request**

- Use the **"Send"** button to make the API call.

### **6. View Response**

- The response panel includes:
  - **Status Code**
  - **Headers**
  - **Response Body**

### **7. Local Data Storage**

- When you run the application, a file named api.json is automatically created in the dist folder.
  This file stores local configuration and API request information, ensuring that your data is saved for future reference and quick access.
  You can find it at:

```bash
~/dist/api.json
```

---

## ⚙️ Configuration

Customize the application configuration in the provided configuration files. These files contain paths, environment variables, and other project-specific setup information.

---

## 🛠 Troubleshooting

---

## 📜 License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).

---

## 📧 Contact

- **GitHub**: [https://github.com/chhun9](https://github.com/chhun9)
- **Email**: chhun3830@naver.com

---

Happy API testing! 🎉
