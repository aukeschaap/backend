# K3s-Based Server Architecture with Gateway, Admin, and Multi-Environment Support 🚀

## Overview

This document describes the complete architecture for a secure, modular, and extensible backend system running on a single server using [k3s](https://k3s.io/). The system is composed of various pods for authentication, routing, administrative control, background jobs, and monitoring. It supports multiple environments (production, nightly testing, and development) for safe and flexible deployment.

---

## Core Goals 🎯

* Host microservices in separate pods using k3s.
* Securely expose only the authentication gateway.
* Provide an admin API to manage deployment operations like `kubectl apply`, rollouts, and rollbacks.
* Run background jobs (e.g. health checks) in a controlled and isolated way.
* Support distinct environments for production, nightly testing, and development.

---

## High-Level Architecture 🧱

### Core Pods

| Pod / Component       | Purpose                                                                    | Access Scope                  |
| --------------------- | -------------------------------------------------------------------------- | ----------------------------- |
| **Gateway Pod**       | Handles OAuth2 login, JWT issuance, and routes requests to services.       | Public via Ingress            |
| **Admin Pod**         | Provides internal `/admin` API for safe execution of `kubectl` operations. | via Gateway (authorized)      |
| **Job/Scheduler Pod** | Executes background jobs, health checks, and action proposal workflows.    | via Gateway                   |
| **Monitoring Pod**    | Exposes CPU, disk, and other metrics from the host (read-only).            | via Gateway                   |
| **Dashboard**         | Frontend interface for system and service monitoring.                      | Public (authenticated)        |
| **Microservices**     | Business logic services; separated by function.                            | via Gateway                   |

---

## Network Overview 🌐

```
[ Internet ]
     │
     ▼
[ Ingress Controller (Nginx) ]
     │
     ▼
[ Gateway Pod ]  ◄─────┐
     │                 │
     │                 │
     ├──► [ Dashboard Pod ]
     ├──► [ Microservices ]
     ├──► [ Monitoring Pod ]
     │
     └──► [ Admin Pod (internal only) ] ◄── [ Job/Scheduler Pod ]

[ SSH Access ] ──► [ Host System ]
```

---

## Exposed Endpoints 🔓

| Endpoint                                | Purpose                    | Auth Required?  | Access Scope  |
| --------------------------------------- | -------------------------- | --------------- | ------------- |
| `https://gateway.example.com`           | OAuth2 login, JWT issuance | Token based     | Exposed        |
| `https://gateway.example.com/api`       | Routed service calls       | ❌ No          | Internal only |
| `https://gateway.example.com/monitor`   | View host/pod metrics      | ❌ No          | Internal only |
| `https://gateway.example.com/dashboard` | Frontend dashboard         | ❌ No          | Internal only |
| `/admin/*` (Admin Pod)                  | Apply, restart, rollback   | ❌ No          | Internal only |
| SSH                                     | Manual host access         | Key based       | Exposed      |

---

## Routing and Exposure 🛰️

### Ingress Design

* Uses **Nginx Ingress Controller** to expose only the Gateway Pod.
* Host-based routing per environment:

  * `prod.gateway.example.com` → Gateway in `prod` namespace
  * `nightly.gateway.example.com` → Gateway in `nightly` namespace
  * `dev.gateway.example.com` → Gateway in `dev` namespace

### Gateway Pod

* Handles only:

  * OAuth2 login and token issuance
  * JWT verification
  * Routing to allowed internal services
* Does **not** include any `/admin` functionality.

### Admin Pod

* Exposes internal-only `/admin/...` endpoints
* Can execute:

  * `kubectl apply`
  * `kubectl rollout restart`
  * `kubectl rollout undo`
* Requires admin JWT scope
* Designed to handle administrative control safely and securely

### Job/Scheduler Pod

* Runs background jobs such as:

  * Checking pod health (e.g., CrashLoopBackOff detection)
  * Submitting action requests to Admin Pod
* Does not execute admin actions directly — instead, it proposes them for review and consent

### Monitoring Pod

* Mounted with read-only access to host paths such as `/proc`, `/sys`, etc.
* Reports system-level metrics like CPU, RAM, disk usage
* Called by Dashboard or Gateway to provide observability

---

## Recovery and Failover 🔁

### Pod Crash

* Kubernetes will restart pods on failure automatically
* If a rollout deploys faulty code, pods enter `CrashLoopBackOff`
* Admin Pod can be used to trigger `kubectl rollout undo`

### Gateway Crash

* Manual SSH access is used to rollback via `kubectl` if Gateway fails
* Gateway protected by Kubernetes liveness and readiness probes
* Future enhancement: a watchdog Job Pod that proposes rollback automatically if Gateway fails health checks

### Manual Recovery

* Always available via SSH
* `k3s` auto-starts on reboot via systemd
* Backup YAML manifests for critical components like Gateway can be stored on disk (e.g. `/opt/k8s-backups/...`)

---

## Multi-Environment Setup 🌍

### Environments

| Environment | Namespace | Purpose                  |
| ----------- | --------- | ------------------------ |
| Production  | `prod`    | Stable, customer-facing  |
| Nightly     | `nightly` | Automatic CI builds      |
| Development | `dev`     | Manual developer testing |

### Configuration Per Environment

* Resources deployed per namespace
* Ingress uses subdomains to isolate traffic:

  * `gateway.example.com` → prod
  * `nightly.gateway.example.com` → nightly
  * `dev.gateway.example.com` → dev
* ConfigMaps, Secrets, Services, Deployments are namespace-scoped
* RBAC permissions can be defined per namespace for added isolation

### Deployment Strategy

* Admin Pod can target specific environments via namespace input
* `kubectl -n <env> apply -f ...` used for CLI or API-based deployments
* GitOps compatible (Flux/Argo optional)

---

## Security Practices 🔒

* All exposed services go through the Gateway with OAuth2 and JWT
* Microservices and Admin APIs are **never exposed directly** to the public
* Monitoring pod is read-only
* SSH access is tightly restricted and key-auth only
* Admin Pod actions require proper JWT scopes and (optionally) human approval

---

## Summary ✅

This architecture enables:

* Secure public access via a JWT-authenticated Gateway
* Fine-grained control over deployments and rollouts via a dedicated Admin API
* Safe background automation through a scheduler Job Pod
* Full observability of host and pod health
* Clear separation of environments within a single-node k3s setup

This design is robust enough for production use, flexible enough for development and CI/CD, and minimal in terms of moving parts — perfect for a single-machine k3s deployment. 🔧
