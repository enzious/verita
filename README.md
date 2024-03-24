![image](https://github.com/enzious/fuzion-verita/assets/4645608/39697778-a4a4-43e3-93da-4050c63e9e6c)

[![Contributors](https://img.shields.io/github/contributors/enzious/fuzion-verita)](https://github.com/enzious/actix-web-thiserror/graphs/contributors)
[![GitHub Repo stars](https://img.shields.io/github/stars/enzious/fuzion-verita?style=social)](https://github.com/enzious/actix-web-thiserror)

# Open Source Identity, Role, and Permission Management

Manages login, registration, identity, roles, and permissions outside your application.

### Work-in-progress

## Requirements for development

 - cargo
   - Recommended method: https://rustup.rs
 - Docker
 - Also currently requires `fuzion-commons` to be cloned in same directory.
   - https://github.com/enzious/fuzion-commons

## Getting started with development

Clone the repo, then run the following command to start the development environment:

    ./dev.sh start

Once the environment has initialized, you can run the application with cargo:

    ADMIN=admin ADMIN_PASSWORD=password cargo run -- --migrate

This will initialize the database with the parameters set in `./dev/.env`
