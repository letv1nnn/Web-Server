# ***Web Server***
(Not completed yet)

## *Drscription*
This project is a multithreaded web server that I built from scratch. I deployed it on an old laptop, effectively turning it into my own self-hosted server. Now I have a working, home-built web server running locally.

## *Current state*
I've built a server that accepts GET requests and configured the firewall so that other LAN devices can connect to it. I've also set up the build system for this project using Cargo and shell scripts.

## *Technologies*
I used Docker to containerize the server by creating a custom image and deploying it there. Additionally, I used Rust’s build system, Cargo, along with shell scripts to simplify the build and execution process.

## *Build and Run*
NOTE: If you are going to run this server on your PC, you need to configure the firewall to allow LAN devices to make requests to the server. Otherwise, the server will only work on localhost.
- *Cloning*
```bash
git clone https://github.com/letv1nnn/Web-Server.git
cd Web-Server/
```
- *Building*
```bash
sh build/prep.sh
```
- *Running*
```bash
sh build/run.sh
```
- *Cleaning*
```bash
sh build/clean.sh
```

## *Usage Scenarios*
This server can be used in several practical contexts:

- **Personal Website Hosting**: Host static HTML/CSS/JS websites directly from your home network.
- **File Server**: Share files locally or over your LAN using simple HTTP endpoints.
- **REST API**: Serve basic JSON-based APIs for apps or IoT devices.
- **Local Dev Server**: Use as a lightweight backend for testing frontend applications.
- **Learning Platform**: A great base to experiment with HTTP internals, async Rust, and server design.
- **Edge Device Service Host**: Deploy it on a low-power device (e.g., old laptop, Raspberry Pi) to run small services at the network edge.

## *Examples*
![image](https://github.com/user-attachments/assets/7d3edbc5-31dd-40ec-995a-8d137c51234f)


