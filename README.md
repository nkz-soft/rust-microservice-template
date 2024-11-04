# rust-microservice-template

![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/nkz-soft/rust-microservice-template?style=flat-square)
![license](https://img.shields.io/github/license/nkz-soft/rust-microservice-template?style=flat-square)
![GitHub Workflow Status (with branch)](https://img.shields.io/github/actions/workflow/status/nkz-soft/rust-microservice-template/build-by-tag.yaml)

Template for microservice based on Domain Driven Design principles with Rust

The purpose of this project is to provide a means for building microservices with the last version of Rust that follows basic Domain Driven Design principles

### ⭐ Give a star

If you're using this repository for your learning, samples or your project, please give a star. Thanks :+1:

## Table of Contents

- [Installation](#installation)
- [Architecture](#architecture )
- [Deployment](#deployment)
- [Plan](#plan)
- [Technologies - Libraries](#technologies-used)

## Usage
### Prerequisites

Before you can install and configure the microservice, you will need to ensure that you have the following prerequisites installed:
- Docker and Docker Compose

### Installation
Once you have all the prerequisites installed, you can follow these steps to install and configure your microservice:

1. Clone the repository for your microservice using Git:
```bash
git clone https://github.com/nkz-soft/rust-microservice-template.git
```
2. Change into the directory for your microservice:
```bash
cd rust-microservice-template/deployment/docker
```

3. You can use the included Dockerfile and docker-compose.yml files to build and deploy the microservice.
   Simply run the following command in the root directory of your project:
```bash
./docker-compose.sh up --build -d
```
4. Verify that the microservice is running correctly by visiting the endpoint in your web browser or using a tool like curl:
```bash
curl -v  http://localhost:8181/to-do-items
```

### Configuration
To configure the microservice, you will need to modify the configuration file: config.app.toml.

## Architecture
The microservice is divided into three layers: the Domain Layer, the Application Layer, and the Infrastructure Layer.

### Domain Layer
The Domain Layer is the heart of the Domain Driven Design (DDD) approach. It contains the business logic and rules that drive the application.
In Rust, the Domain Layer consists of structs, traits, enums, and functions that model the problem domain in a way that is easy to understand and maintain.

### Application Layer
The Application Layer is responsible for coordinating the Domain Layer and the Infrastructure Layer.
It translates user requests and external events into actions that the Domain Layer can understand, and communicates the results back to the user or external systems.

### Infrastructure Layer
The Infrastructure Layer is responsible for providing the necessary infrastructure to run the application.
This can include things like databases, message queues, and external APIs.

### Presentation Layer
The presentation layer is responsible for handling user interactions and presenting information to users.
This layer typically includes user interfaces such as web applications, desktop applications, mobile apps, or APIs.



### CQRS
CQRS (Command Query Responsibility Segregation) is a pattern that separates the read and write responsibilities of an application into separate models.

## Deployment

## Plan

- [x] New blank microservice solution
- [x] Basic Github actions
- [x] Docker compose
- [x] REST API
- [x] CQRS (Command Query Responsibility Segregation)
- [x] PostgreSQL storage
    - [X] Add support for migrations
    - [X] CRUD Operation
- [X] Integration tests
- [ ] Advanced error handling
- [ ] Coming soon :)

## Technologies used

- [Rust](https://github.com/rust-lang/rust): The Rust Programming Language
- [Actix](https://github.com/actix/actix-web): Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust
- [Refinery](https://github.com/rust-db/refinery): Refinery strives to make running migrations for different databases as easy as possible.
- [rust-postgres](https://github.com/sfackler/rust-postgres): PostgreSQL support for Rust.
- [testcontainers-rs](https://github.com/testcontainers/testcontainers-rs): Testcontainers-rs is the official Rust language fork of http://testcontainers.org.
