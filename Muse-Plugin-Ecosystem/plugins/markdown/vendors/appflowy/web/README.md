<h1 align="center" style="border-bottom: none">OpenMuse Web<br>
思构
</h1>
<p align="center"> Use OpenMuse in <a href="https://openmuseai.com">your browser</a><br>
</p>


<p align="center">
An AI-native office suite. Work locally, keep your data, and let AI help you write.
</p>
<p align="center">
    <a href="https://openmuseai.com"><b>Web</b></a> •
    <a href="https://openmuseai.com/download">Apps</a> •
    <a href="https://github.com/openmuseai/openmuse"><b>GitHub</b></a>
</p>


![appflowy_web](https://github.com/user-attachments/assets/beb79630-b134-4de0-b464-d164cd0f9adf)
![appflowy_web_property_types](https://github.com/user-attachments/assets/074f859d-2e7e-4bf0-ac00-ba0b36e808e5)
![appflowy_web_grid_view](https://github.com/user-attachments/assets/e1b8a950-5b57-4279-90aa-3917aae2f83b)
![appflowy_web_kanban_board_dark_mode](https://github.com/user-attachments/assets/3a6fda95-e57b-45d0-ae88-f2df362675ce)
![appflowy_web_open_row](https://github.com/user-attachments/assets/1c5a2891-90e0-4831-bfd5-89229235bbb4)
![appflowy_web_open_row_in_full_page](https://github.com/user-attachments/assets/011a9408-7ae4-4144-b114-3613e699f6b5)
![appflowy_web_quick_note_small_window](https://github.com/user-attachments/assets/8b72462c-00a2-4398-8d5d-6ee982f6a30e)
![appflowy_web_quick_note_large](https://github.com/user-attachments/assets/5c633e60-6f34-454c-91db-236a1b78966a)
![appflowy_web_share](https://github.com/user-attachments/assets/64b9955d-b11f-4aa1-b32b-ea79ea7f3566)

## Table of Contents

- [Use Cases](#use-cases)
- [Features](#features) 
- [Installation](#installation)
- [Development](#development)
- [Deployment](#deployment)
- [Documentation](#documentation)
- [License](#license)

## Use cases
- Build and maintain a knowledge base for your team
- Create and publish documentation for your customers and audience
- Write, publish, and manage content with AI
- Manage tasks and projects for yourself and your team

## Features
- Write beautiful documents with rich content types
- Create custom Grid & Kanban Board-view databases to manage and digest data
- Add a Quick Note to jot down lists, ideas, or to-dos
- Invite members to your workspace for seamless collaboration
- Create multiple public and private spaces to better organize your content

## Installation
### Cloud
Sign up at <a href="https://openmuseai.com">openmuseai.com</a>.

### Self-hosted
Take charge of your data; no vendor lock-in. Product names and legal URLs live in [`brand/config.yaml`](brand/config.yaml). After editing the YAML, run `python scripts/apply-brand.py`.

Backend env names remain `APPFLOWY_*` (see the deployment guide). This web app is derived from [AppFlowy-IO/AppFlowy-Web](https://github.com/AppFlowy-IO/AppFlowy-Web).

## Built with 🛠️
![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)
![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)
![Bun](https://img.shields.io/badge/Bun-000000?style=for-the-badge&logo=bun&logoColor=white)
![Nginx](https://img.shields.io/badge/Nginx-009639?style=for-the-badge&logo=nginx&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white)

## Development

Check out the [development guide](./doc/DEVELOPMENT_GUIDE.md) for more details

## Deployment
Once the cloud backend has been set up, follow the [deployment guide](doc/DEVELOPMENT_GUIDE.md)
to deploy OpenMuse Web.

## Documentation

- **[Development Guide](doc/DEVELOPMENT_GUIDE.md)** - Environment configuration
- **[Brand config](brand/README.md)** - Product name and public URLs
- Upstream: this app is derived from [AppFlowy-Web](https://github.com/AppFlowy-IO/AppFlowy-Web)

## License
Distributed under the AGPLv3 License. See [`LICENSE`](LICENSE) for more information. The NOTICE / LICENSE files keep the AppFlowy-IO attribution.
