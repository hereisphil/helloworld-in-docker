# Exploring Rust with Docker

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

## 🐳 Running this Node.js Container

Build the image:

```bash
docker build --no-cache -t rust-hello .
```

Run the container: <br />
_The `--rm` tag removes the container after it runs._

```bash
docker run --rm rust-hello
```

Example expected output:

```text
Hello ASL!
Proccess completed: Tue Jan 06 18:14:55 UTC 2026
```

---

## 🌍 Learning Goals

This project demonstrates:

✔ Containerized development workflows  
✔ Running multiple language environments safely  
✔ Understanding differences between **compiled vs interpreted** languages

---

## 👋 Author

Hi! I’m Phillip Cantu, a current **Full Sail University** web development student, _expected graduation February 2027_, and a [4Geeks Academy Full Stack](https://www.phillipcantu.com/certificate.pdf) bootcamp graduate.

-   **GitHub:** [hereisphil](https://github.com/hereisphil)
-   **LinkedIn:** [phillipcantu](https://www.linkedin.com/in/phillipcantu/)
-   **Email:** [thereisphil@gmail.com](mailto:thereisphil@gmail.com)
