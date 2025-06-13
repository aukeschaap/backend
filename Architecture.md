# K3s-Based Server Architecture with Gateway, Admin, and Multi-Environment Support 🚀

## Overview

This document describes the complete architecture for a secure, modular, and extensible backend system running on a single server using [k3s](https://k3s.io/). The system is composed of various pods for authentication, routing, administrative control, background jobs, persistent logging, and monitoring. It supports multiple environments (production, nightly testing, and development) for safe and flexible deployment.

---

## Core Goals 🎯

* Host microservices in separate pods using k3s.
* Securely expose only the authentication gateway.
* Provide an admin API to manage deployment operations like `kubectl apply`, rollouts, and rollbacks.
* Run background jobs (e.g. health checks) in a controlled and isolated way.
* Log job metadata persistently to survive reboots and crashes.
* Support distinct environments for production, nightly testing, and development.

---

## High-Level Architecture 🧱

### Core Pods (Per Environment)


| Pod / Component       | Purpose                                                                    | Exposure             | Persistent?  |
| --------------------- | -------------------------------------------------------------------------- | -------------------- | ------------ |
| **Gateway Pod**       | Handles OAuth2 login, JWT issuance, and routes requests to services.       | Public via Ingress   | ❌            |
| **Gateway DB Pod**    | Local database storing user roles/rights per environment.                  | Internal only        | ✅ PVC        |
| **Admin Pod**         | Provides internal `/admin` API for safe execution of `kubectl` operations. | Internal only        | ❌            |
| **Job/Scheduler Pod** | Executes background jobs, health checks, and action proposal workflows.    | Internal only        | ❌            |
| **Logging DB Pod**    | Central database storing job metadata and log entries.                     | Internal only        | ✅ PVC        |
| **Monitoring Pod**    | Exposes CPU, disk, and other metrics from the host (read-only).            | Internal only        | ❌            |
| **Data Platform Pod** | Shared persistent database or data lake for all services.                  | Internal only        | ✅ PVC        |
| **Secret Store Pod**  | Optional: Centralized secret management (e.g. Vault).                      | Internal only        | ✅ (optional) |
| **Microservices**     | Business logic services; separated by function.                            | Internal only        | Varies       |



---

## Network Overview 🌐

```
[ Internet ]
     │
     ▼
[ Ingress Controller (Nginx) ]
     │
     ▼
[ Gateway Pod ] ──► [ Gateway DB Pod ]
     │
     │
     ├──► [ Monitoring Pod ]   ──► [ Logging DB Pod ]
     ├──► [ Data Platform Pod ]            ▲
     ├──► [ Job/Scheduler Pod ]          ──┤
     ├──► [ Admin Pod]                   ──┘
     ├──► [ Secret Store Pod ]
     |
     └──► [ Microservices ]

[ SSH Access ] ──► [ Host System ]
```

---

## Exposed Endpoints 🔓

| Endpoint                                | Purpose                    | Access Scope  | Auth method    |
| --------------------------------------- | -------------------------- | ------------- | -------------- |
| `https://gateway.example.com`           | OAuth2 login, JWT issuance | ⚠️ Direct     | Token based   |
| `/monitor/*` (Monitoring Pod)           | System metrics             | via Gateway   | RBAC           |
| `/admin/*` (Admin Pod)                  | Apply, restart, rollback   | via Gateway   | RBAC           |
| `/jobs/*` (Job/Scheduler Pod)           | Background job management  | via Gateway   | RBAC           |
| `/data/*` (Data Platform Pod)           | Access data lake/warehouse | via Gateway   | RBAC           |
| SSH                                     | Manual host access         | ⚠️ Direct     | Key based     |

---

## Routing and Exposure 🚀

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
* Enforces **custom application-level RBAC**

### Admin Pod

* Exposes `/admin/...` endpoints
* Executes:

  * `kubectl apply`
  * `kubectl rollout restart`
  * `kubectl rollout undo`
* Requires **admin JWT scope**
* Designed to handle administrative control safely and securely

### Job/Scheduler Pod

* Runs background jobs such as:

  * Checking pod health (e.g., CrashLoopBackOff detection)
  * Submitting job records to Logging DB
  * Proposing actions to Admin Pod

### Logging DB Pod

* Central persistent database (e.g., Postgres or Mongo)
* Stores job submissions, statuses, and results
* Survives crashes and system reboots

### Monitoring Pod

* Mounted with read-only access to host paths such as `/proc`, `/sys`, etc.
* Reports system-level metrics like CPU, RAM, disk usage
* Called by Dashboard or Gateway to provide observability

### Data Platform Pod

* Central data lake or warehouse for business or analytic data
* Used by Microservices or external tools
* Persistent with automated backups (optional)

---

## Recovery and Failover ♻️

### Pod Crash

* Kubernetes automatically restarts failed pods
* Faulty rollouts enter `CrashLoopBackOff`
* Admin Pod supports `kubectl rollout undo`

### Gateway Crash

* Liveness/readiness probes provide self-healing
* SSH fallback to rollback Gateway manually
* Optionally monitored by Scheduler Pod

### Persistent Logging

* Job/scheduler logs are written to the **Logging DB**, not ephemeral logs
* Ensures traceability of job status after restarts or outages

### Manual Recovery

* Always available via SSH
* `k3s` auto-starts on reboot via systemd
* Backup YAML manifests stored on disk (`/opt/k8s-backups/...`)

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
* ConfigMaps, Secrets, PVCs, Services, Deployments are namespace-scoped
* RBAC permissions are defined per namespace for security and isolation

### Deployment Strategy

* Admin Pod targets specific namespaces via API input
* `kubectl -n <env> apply -f ...` used for CLI or API-based deployments
* GitOps compatible (Flux/Argo optional)

---

## Security Practices 🔒

* All exposed services go through the Gateway with OAuth2 and JWT
* Application-level RBAC enforced by Gateway via user roles
* Microservices, Admin APIs, and internal DBs are **never exposed directly**
* Logging DB is protected with Secrets and Kubernetes RBAC
* Monitoring pod is read-only
* SSH access is restricted and key-auth only
* Admin actions require elevated JWT scopes
* Optional: NetworkPolicies restrict cross-namespace traffic

---

## Summary ✅

This architecture enables:

* Secure public access via a JWT-authenticated Gateway
* Application-level RBAC for custom access control
* Persistent job logging and recovery
* Safe administrative control via a dedicated Admin API
* Self-healing and observable system components
* Clear namespace isolation across environments

This design is robust enough for production use, flexible enough for CI/CD and dev testing, and lightweight enough to run on a single-node k3s cluster. 💠
