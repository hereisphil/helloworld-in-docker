# Exploring Server Languages with Docker

This repository contains my work for the **Advanced Server Languages (ASL)** course at **Full Sail University**.  
The objective is to explore and run multiple server-side programming languages inside isolated Docker containers.

---

## 🎯 Assignment Objectives

For each programming language used in this project:

✔ Create a **dedicated Docker container**  
✔ Print the message **"Hello ASL!"**  
✔ Display the **current date/time**  
✔ Run the container from the terminal to demonstrate the output

---

## 🐳 Example: Running the Node.js Container

Change into the directory:

```bash
cd nodejs
```

Build the image:

```bash
docker build --no-cache -t node-js .
```

Run the container: <br />
_The `--rm` tag removes the container after it runs._

```bash
docker run --rm node-js
```

Example output:

```text
Hello ASL!
Process completed on: January 6, 2026 at 11:50:48 AM EST
```

---

## 🧪 Repository Structure

```tree
helloworld-in-docker/
 ├── Java/
 ├── Rust/
 ├── nodejs/
 └── python/
```

Each folder contains:

📄 Source code  
🐳 Dockerfile  
📁 Directories as needed

All work is committed on this main branch!

---

## 📹 Video Demo

A screen‑recorded walkthrough will:

✔ Show each Docker image being created
✔ Show each Docker container being created & ran
✔ Display both the **Hello ASL!** message  
✔ And the **current date output**
✔ The source files and dockerfile's themselves

### 👉 Watch it on Youtube: [youtube.com/watch?v=re5b_yMsaAc](https://www.youtube.com/watch?v=re5b_yMsaAc)

---

## 🌍 Learning Goals

This project demonstrates:

✔ Containerized development workflows  
✔ Running multiple language environments safely  
✔ Understanding differences between **compiled vs interpreted** languages  
✔ Improving backend language exposure

---

## 👋 Author

Hi! I’m Phillip Cantu, a current [Full Sail University](https://www.fullsail.edu/) web development student, _expected graduation February 2027_, and a [4Geeks Academy Full Stack](https://www.phillipcantu.com/certificate.pdf) bootcamp graduate.

-   **GitHub:** [hereisphil](https://github.com/hereisphil)
-   **LinkedIn:** [phillipcantu](https://www.linkedin.com/in/phillipcantu/)
-   **Email:** [thereisphil@gmail.com](mailto:thereisphil@gmail.com)
