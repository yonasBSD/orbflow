<div align="center">

<img src="apps/web/public/favicon.svg" alt="Orbflow" width="80" />

# Orbflow

Build workflows visually, run them reliably, and manage AI, plugins, approvals, and costs in one place.

[![Rust](https://img.shields.io/badge/Rust-Edition_2024-f74c00?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Next.js](https://img.shields.io/badge/Next.js-16-000?style=flat-square&logo=next.js)](https://nextjs.org)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-16-336791?style=flat-square&logo=postgresql&logoColor=white)](https://www.postgresql.org)
[![NATS](https://img.shields.io/badge/NATS-JetStream-27AAE1?style=flat-square&logo=nats.io&logoColor=white)](https://nats.io)
[![License](https://img.shields.io/badge/License-AGPL--3.0--or--later-blue?style=flat-square)](LICENSE)
[![CI](https://img.shields.io/github/actions/workflow/status/orbflow-dev/orbflow/ci.yml?branch=main&style=flat-square&label=CI)](https://github.com/orbflow-dev/orbflow/actions/workflows/ci.yml)
[![codecov](https://img.shields.io/codecov/c/github/orbflow-dev/orbflow?style=flat-square&logo=codecov&logoColor=white)](https://codecov.io/gh/orbflow-dev/orbflow)

[Start Here](#start-here) · [Product Tour](#product-tour) · [Quick Start](#quick-start) · [Technical Highlights](#technical-highlights) · [Development](#development)

</div>

---

Orbflow is a visual workflow platform for teams that want to connect APIs, AI steps, schedules, webhooks, and human approvals without stitching everything together by hand.

You can use it to:

- build automations in a canvas instead of starting with glue code
- watch workflow runs, failures, and costs in one place
- manage credentials, plugins, reviews, budgets, alerts, and access from the same app

If you are new to the project, read the next three sections first. The later sections move into runtime architecture, configuration, APIs, and development details.

## Start Here

Most people use Orbflow in this order:

1. Start in `Templates` if you want a ready-made workflow, or `Builder` if you want to create one from scratch.
2. Run the workflow and watch progress in `Activity`.
3. Check `Analytics`, `Budgets`, and `Alerts` when you need visibility into performance and cost.
4. Add API keys in `Credentials` and install extensions from `Marketplace`.
5. Use `Reviews` and `Access` when multiple people work in the same environment.

## Product Tour

These screenshots come from [`docs/user-guide/screenshots/`](docs/user-guide/screenshots/).

| Page | What it is for | Image file | Preview |
|------|----------------|------------|---------|
| Builder | Create automations from scratch in the visual canvas | [`01-builder.png`](docs/user-guide/screenshots/01-builder.png) | <img src="docs/user-guide/screenshots/01-builder.png" alt="Builder page" width="260" /> |
| Activity | Monitor workflow runs and inspect execution progress | [`02-activity.png`](docs/user-guide/screenshots/02-activity.png) | <img src="docs/user-guide/screenshots/02-activity.png" alt="Activity page" width="260" /> |
| Analytics | View execution metrics, latency, and SLA trends | [`03-analytics.png`](docs/user-guide/screenshots/03-analytics.png) | <img src="docs/user-guide/screenshots/03-analytics.png" alt="Analytics page" width="260" /> |
| Templates | Start from prebuilt workflows instead of building from zero | [`04-templates.png`](docs/user-guide/screenshots/04-templates.png) | <img src="docs/user-guide/screenshots/04-templates.png" alt="Templates page" width="260" /> |
| Credentials | Store and manage secrets, tokens, and API keys | [`05-credentials.png`](docs/user-guide/screenshots/05-credentials.png) | <img src="docs/user-guide/screenshots/05-credentials.png" alt="Credentials page" width="260" /> |
| Marketplace | Browse, install, and manage plugins and integrations | [`06-marketplace.png`](docs/user-guide/screenshots/06-marketplace.png) | <img src="docs/user-guide/screenshots/06-marketplace.png" alt="Marketplace page" width="260" /> |
| Reviews | Create and review change requests before merging workflow updates | [`07-reviews.png`](docs/user-guide/screenshots/07-reviews.png) | <img src="docs/user-guide/screenshots/07-reviews.png" alt="Reviews page" width="260" /> |
| Budgets | Track spend and set cost limits for workflows | [`08-budgets.png`](docs/user-guide/screenshots/08-budgets.png) | <img src="docs/user-guide/screenshots/08-budgets.png" alt="Budgets page" width="260" /> |
| Alerts | Define monitoring rules and notification triggers | [`09-alerts.png`](docs/user-guide/screenshots/09-alerts.png) | <img src="docs/user-guide/screenshots/09-alerts.png" alt="Alerts page" width="260" /> |
| Access | Manage roles and permissions for your team | [`10-access.png`](docs/user-guide/screenshots/10-access.png) | <img src="docs/user-guide/screenshots/10-access.png" alt="Access page" width="260" /> |

## Quick Start

The fastest way to try Orbflow locally is:

1. Install the tools below.
2. Run `just setup`.
3. Run `just dev`.
4. Open `http://localhost:3000`.

If this is your first time inside the app, start with `Templates` for a guided path or `Builder` for a blank canvas.

### What You Need

| Tool | Version | Install |
|------|---------|---------|
| Rust | Edition 2024 | [rustup.rs](https://rustup.rs) |
| Node.js | 20+ | [nodejs.org](https://nodejs.org) |
| pnpm | 10+ | `corepack enable && corepack prepare pnpm@latest --activate` |
| Docker | Latest | [docker.com](https://www.docker.com/get-started) |
| just | Latest | `cargo install just` or `winget install Casey.Just` |

### Run It

```bash
# Clone the repository
git clone https://github.com/orbflow-dev/orbflow.git
cd orbflow

# First-time setup: checks tools, installs deps, starts Postgres + NATS
just setup

# Start the app stack (server + worker + frontend) with live reload
just dev
```

Open [http://localhost:3000](http://localhost:3000) to access the visual workflow builder.

> [!TIP]
> Start components individually with `just dev-server`, `just dev-worker`, or `just dev-web`. Run `just dev-backend` for server + worker without the frontend.
> **Note:** Unlike `just dev`, these individual commands do not start infrastructure automatically. Run `just infra` first if starting components individually.

> [!NOTE]
> **Windows users:** The Next.js dev server (Turbopack) can consume excessive memory during long sessions. The justfile caps the Node.js heap at 4 GB (`--max-old-space-size=4096`). If running outside of `just`, set it manually:
> ```bash
> set NODE_OPTIONS=--max-old-space-size=4096
> pnpm dev
> ```

<details>
<summary><strong>Running without Docker</strong></summary>

You can run Orbflow entirely without Docker by installing PostgreSQL and NATS natively.

#### PostgreSQL

**macOS:**
```bash
brew install postgresql@16
brew services start postgresql@16
createdb orbflow_dev
```

**Ubuntu/Debian:**
```bash
sudo apt install postgresql-16
sudo -u postgres createdb orbflow_dev
sudo -u postgres psql -c "CREATE USER orbflow WITH PASSWORD 'orbflow'; GRANT ALL ON DATABASE orbflow_dev TO orbflow;"
```

**Windows:**
```powershell
winget install PostgreSQL.PostgreSQL.16
createdb -U postgres orbflow_dev
psql -U postgres -c "CREATE USER orbflow WITH PASSWORD 'orbflow'; GRANT ALL ON DATABASE orbflow_dev TO orbflow;"
```

#### NATS

**macOS:** `brew install nats-server && nats-server --jetstream --store_dir /tmp/nats-data &`

**Ubuntu/Debian:** `curl -sf https://binaries.nats.dev/nats-io/nats-server/v2@latest | sh && nats-server --jetstream --store_dir /tmp/nats-data &`

**Windows:** `winget install NATS.Server` then in a separate terminal: `nats-server --jetstream --store_dir C:\temp\nats-data`

#### Configure and run

Set the credential encryption key and start:

```bash
export CREDENTIAL_ENCRYPTION_KEY=$(openssl rand -hex 32)
just dev
```

Verify everything is up:

```bash
curl http://localhost:8080/health
curl http://localhost:8080/api/v1/node-types | jq '.data | length'
```

</details>

---

## Technical Highlights

Orbflow uses a Rust backend workspace, a Next.js frontend, PostgreSQL for persistence, and NATS JetStream for task routing. The points below are the implementation-oriented capabilities behind the user-facing product tour above.

### Workflow Runtime

- **DAG-based orchestration** -- conditional branching, parallel execution, and saga compensation for rollbacks
- **CEL expression evaluation** -- dynamic values using [Common Expression Language](https://github.com/google/cel-spec), evaluated at runtime
- **Event sourcing** -- all state changes persisted as domain events with periodic snapshots for crash recovery
- **Per-instance locking** -- concurrent result handling with optimistic retry (up to 3 attempts)

### AI

- **6 specialized AI nodes** -- chat, extract, classify, summarize, sentiment, translate
- **Multi-provider** -- OpenAI, Anthropic, and Google AI via a unified interface
- **Real-time streaming** -- Server-Sent Events for live token-by-token AI responses
- **MCP integration** -- [Model Context Protocol](https://modelcontextprotocol.io) support for connecting AI models to external tools
- **Cost tracking** -- per-execution token usage and budget enforcement

### Team Controls

- **RBAC** -- fine-grained, node-level permission enforcement with configurable policies
- **PR-style collaboration** -- change requests with visual diffs, inline comments, and submit/approve/reject/merge workflow
- **Audit trails** -- SHA-256 hash chain, Ed25519 signatures, Merkle proofs for tamper detection
- **Compliance exports** -- SOC2, HIPAA, and PCI-ready audit trail exports
- **Workflow versioning** -- automatic snapshots with diff comparison between any two versions

### Visibility

- **OpenTelemetry** -- OTLP export to Jaeger, Grafana Tempo, or any OTel-compatible collector
- **Analytics** -- p50/p95/p99 latency breakdowns, failure pattern detection, cost tracking
- **Alert rules** -- configurable conditions with webhook and log notification channels

### Extensibility

- **21 built-in nodes** -- AI, data processing, integration, control flow, and triggers
- **Plugin system** -- subprocess JSON-RPC and gRPC protocols for custom nodes in any language
- **Trigger system** -- cron schedules, webhooks, and event-driven execution
- **Visual builder** -- drag-and-drop canvas with node configuration, live execution overlay, and a plugin marketplace

---

## Architecture

Orbflow follows **Ports & Adapters** (hexagonal) architecture. `orbflow-core` defines all domain types and port traits. Every other crate implements one adapter. Dependencies point inward.

```mermaid
graph TD
    subgraph Frontend["Frontend (pnpm monorepo)"]
        Web["apps/web\nNext.js 16 + React 19"]
        SDK["packages/orbflow-core\nHeadless SDK (Zustand)"]
    end

    subgraph Server["orbflow-server (coordinator)"]
        HTTP["orbflow-httpapi\nAxum REST API"]
        GRPC["orbflow-grpcapi\ngRPC API"]
        Engine["orbflow-engine\nDAG coordinator"]
    end

    subgraph Core["orbflow-core (domain)"]
        Ports["Port Traits\nEngine · Store · Bus\nNodeExecutor · CredentialStore"]
        Domain["Domain Types\nWorkflow · Instance · Event\nCEL · RBAC · Audit · Crypto"]
    end

    subgraph Adapters["Adapters"]
        Postgres["orbflow-postgres\nPostgreSQL persistence"]
        NatsBus["orbflow-natsbus\nNATS JetStream"]
        Builtins["orbflow-builtins\n21 built-in nodes"]
        Trigger["orbflow-trigger\nCron · Webhook · Event"]
        Plugin["orbflow-plugin\nJSON-RPC + gRPC"]
    end

    subgraph Worker["orbflow-worker (executor)"]
        Executor["Task Router\nNodeExecutor dispatch"]
    end

    subgraph Infra["Infrastructure"]
        PG[("PostgreSQL 16")]
        NATS[("NATS 2.11\nJetStream")]
    end

    Web --> SDK
    SDK -- "REST API" --> HTTP
    HTTP --> Engine
    GRPC --> Engine
    Engine --> Ports
    Executor --> Ports
    Ports --> Postgres
    Ports --> NatsBus
    Executor --> Builtins
    Executor --> Plugin
    Engine --> Trigger
    Postgres --> PG
    NatsBus --> NATS
    Engine -- "TaskMessage" --> NATS
    NATS -- "TaskMessage" --> Executor
    Executor -- "ResultMessage" --> NATS
    NATS -- "ResultMessage" --> Engine

    style Frontend fill:#1a1a2e,stroke:#7C5CFC,stroke-width:2px,color:#fff
    style Server fill:#1a1a2e,stroke:#7C5CFC,stroke-width:2px,color:#fff
    style Core fill:#7C5CFC,stroke:#5a3fd4,stroke-width:2px,color:#fff
    style Adapters fill:#1a1a2e,stroke:#7C5CFC,stroke-width:2px,color:#fff
    style Worker fill:#1a1a2e,stroke:#7C5CFC,stroke-width:2px,color:#fff
    style Infra fill:#1a1a2e,stroke:#555,stroke-width:1px,color:#aaa
    style Web fill:#2d2d44,stroke:#7C5CFC,color:#fff
    style SDK fill:#2d2d44,stroke:#7C5CFC,color:#fff
    style HTTP fill:#2d2d44,stroke:#7C5CFC,color:#fff
    style GRPC fill:#2d2d44,stroke:#7C5CFC,color:#fff
    style Engine fill:#2d2d44,stroke:#7C5CFC,color:#fff
    style Ports fill:#5a3fd4,stroke:#7C5CFC,color:#fff
    style Domain fill:#5a3fd4,stroke:#7C5CFC,color:#fff
    style Postgres fill:#2d2d44,stroke:#7C5CFC,color:#fff
    style NatsBus fill:#2d2d44,stroke:#7C5CFC,color:#fff
    style Builtins fill:#2d2d44,stroke:#7C5CFC,color:#fff
    style Trigger fill:#2d2d44,stroke:#7C5CFC,color:#fff
    style Plugin fill:#2d2d44,stroke:#7C5CFC,color:#fff
    style Executor fill:#2d2d44,stroke:#7C5CFC,color:#fff
    style PG fill:#336791,stroke:#336791,color:#fff
    style NATS fill:#27AAE1,stroke:#27AAE1,color:#fff
```

**Data flow:**

1. **Server** receives a start request, creates an instance, evaluates the DAG
2. Ready node tasks are published to **NATS** as `TaskMessage`
3. **Worker** receives tasks, routes to the matching `NodeExecutor`, executes
4. Results published back as `ResultMessage`
5. Engine processes results, advances the DAG, dispatches next ready nodes

---

## Built-in Nodes

| Category | Node | Description |
|----------|------|-------------|
| **AI** | `ai-chat` | Multi-turn conversation with streaming |
| | `ai-extract` | Structured data extraction from text |
| | `ai-classify` | Text classification into categories |
| | `ai-summarize` | Text summarization |
| | `ai-sentiment` | Sentiment analysis |
| | `ai-translate` | Language translation |
| **Data** | `transform` | JSON transformation via CEL expressions |
| | `filter` | Conditional data filtering |
| | `sort` | Data sorting by field |
| | `encode` | Base64, hex, and URL encoding/decoding |
| | `template` | Tera template rendering |
| | `log` | Structured logging output |
| | `capability-postgres` | Direct PostgreSQL queries |
| **Integration** | `http` | HTTP requests with SSRF protection |
| | `email` | SMTP email sending |
| | `mcp_tool` | MCP tool invocation |
| **Control** | `delay` | Timed delay between nodes |
| **Triggers** | `trigger-manual` | Manual workflow start |
| | `trigger-cron` | Cron-scheduled execution |
| | `trigger-webhook` | HTTP webhook trigger |
| | `trigger-event` | Event-driven trigger |

All AI nodes support OpenAI, Anthropic, and Google AI providers. Configure credentials via the credential store.

---

## Configuration

Orbflow is configured via YAML. Both `orbflow-server` and `orbflow-worker` accept a config path as their first argument. Environment variables are expanded with `${VAR}` syntax.

| File | Purpose |
|------|---------|
| `configs/orbflow.yaml` | Production defaults |
| `configs/orbflow.dev.yaml` | Local development |
| `configs/orbflow.docker.yaml` | Docker Compose |

<details>
<summary><strong>Full configuration reference</strong></summary>

```yaml
# ── HTTP API Server ─────────────────────────────────────
server:
  host: "0.0.0.0"              # Bind address
  port: 8080                   # HTTP port
  cors_origins: ["*"]          # Allowed CORS origins
  auth_token: "${AUTH_TOKEN}"   # Bearer token for API auth (omit to disable)

# ── gRPC API ────────────────────────────────────────────
grpc:
  enabled: false
  port: 9090

# ── Worker ──────────────────────────────────────────────
worker:
  pool: "default"              # Worker pool name (for task routing)
  concurrency: 4               # Max concurrent task executions

# ── Database ────────────────────────────────────────────
database:
  dsn: "postgres://orbflow:orbflow@localhost:5432/orbflow?sslmode=disable"

# ── NATS ────────────────────────────────────────────────
nats:
  url: "nats://127.0.0.1:4222"
  embedded: true               # Run embedded NATS (no external server needed)
  data_dir: "/tmp/orbflow-nats"

# ── Credentials ────────────────────────────────────────
credentials:
  encryption_key: "${CREDENTIAL_ENCRYPTION_KEY}"  # AES-256-GCM key (hex)

# ── MCP ─────────────────────────────────────────────────
mcp:
  enabled: false
  transport: "http"
  port: 3001

# ── Logging ─────────────────────────────────────────────
log:
  level: "info"                # trace, debug, info, warn, error
  format: "json"               # "json" or "console"

# ── OpenTelemetry ───────────────────────────────────────
otel:
  enabled: false
  endpoint: "http://localhost:4317"
  service_name: "orbflow"
  sample_rate: 1.0

# ── Plugins ─────────────────────────────────────────────
plugins:
  dir: "./plugins"
  grpc:
    - name: "my-plugin"
      address: "http://localhost:50051"
      timeout_secs: 30
```

</details>

---

## API Reference

All endpoints are under `/api/v1`. Responses use a consistent envelope:

```json
{
  "data": { "id": "wf_abc123", "name": "My Workflow" },
  "error": null,
  "meta": { "total": 42, "offset": 0, "limit": 20 }
}
```

### Authentication

Set `server.auth_token` in your config. All requests (except `/health`, `/node-types`, `/credential-types`, and webhook paths) require:

```
Authorization: Bearer <your-token>
```

### Endpoints

<details>
<summary><strong>Workflows</strong></summary>

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/v1/workflows` | List workflows |
| `POST` | `/api/v1/workflows` | Create a workflow |
| `GET` | `/api/v1/workflows/{id}` | Get workflow |
| `PUT` | `/api/v1/workflows/{id}` | Update workflow |
| `DELETE` | `/api/v1/workflows/{id}` | Delete workflow |
| `POST` | `/api/v1/workflows/{id}/start` | Start execution |
| `GET` | `/api/v1/workflows/{id}/versions` | List versions |
| `GET` | `/api/v1/workflows/{id}/diff` | Compare versions |

</details>

<details>
<summary><strong>Instances</strong></summary>

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/v1/instances` | List instances |
| `GET` | `/api/v1/instances/{id}` | Get instance |
| `POST` | `/api/v1/instances/{id}/cancel` | Cancel instance |
| `GET` | `/api/v1/instances/{id}/nodes/{node_id}/stream` | SSE stream |
| `POST` | `/api/v1/instances/{id}/nodes/{node_id}/approve` | Approve node |
| `POST` | `/api/v1/instances/{id}/nodes/{node_id}/reject` | Reject node |

</details>

<details>
<summary><strong>Credentials, Change Requests, RBAC, Analytics, Alerts, Budgets</strong></summary>

| Method | Path | Description |
|--------|------|-------------|
| `GET/POST` | `/api/v1/credentials` | List / create credentials |
| `GET/PUT/DELETE` | `/api/v1/credentials/{id}` | CRUD single credential |
| `GET/POST` | `/api/v1/workflows/{id}/change-requests` | List / create CRs |
| `POST` | `/api/v1/workflows/{id}/change-requests/{cr}/submit` | Submit CR |
| `POST` | `/api/v1/workflows/{id}/change-requests/{cr}/approve` | Approve CR |
| `POST` | `/api/v1/workflows/{id}/change-requests/{cr}/merge` | Merge CR |
| `GET/PUT` | `/api/v1/rbac/policy` | Get / update RBAC policy |
| `GET` | `/api/v1/rbac/subjects` | List distinct RBAC subjects |
| `GET` | `/api/v1/analytics/executions` | Execution latency percentiles |
| `GET` | `/api/v1/analytics/nodes` | Per-node performance |
| `GET` | `/api/v1/analytics/failures` | Failure patterns |
| `GET` | `/api/v1/analytics/costs` | Cost tracking |
| `GET/POST` | `/api/v1/alerts` | List / create alert rules |
| `PUT/DELETE` | `/api/v1/alerts/{id}` | Update / delete alert |
| `GET/POST` | `/api/v1/budgets` | List / create budgets |
| `PUT/DELETE` | `/api/v1/budgets/{id}` | Update / delete budget |
| `GET` | `/api/v1/instances/{id}/audit/trail` | Audit trail |
| `GET` | `/api/v1/instances/{id}/audit/verify` | Verify integrity |
| `GET` | `/api/v1/instances/{id}/audit/export` | Compliance export |
| `GET` | `/api/v1/node-types` | List available node types |
| `GET` | `/api/v1/marketplace/plugins` | Browse marketplace |
| `POST` | `/api/v1/marketplace/plugins/{name}/install` | Install plugin |
| `DELETE` | `/api/v1/marketplace/plugins/{name}` | Uninstall plugin |
| `POST` | `/api/v1/marketplace/validate-manifest` | Validate plugin manifest |
| `GET` | `/api/v1/plugins/status` | List plugin statuses |
| `POST` | `/api/v1/plugins/{name}/start` | Start plugin |
| `POST` | `/api/v1/plugins/{name}/stop` | Stop plugin |
| `POST` | `/api/v1/plugins/{name}/restart` | Restart plugin |
| `POST` | `/api/v1/plugins/reload` | Reload all plugins |

</details>

---

## RBAC

Role-Based Access Control is enforced per-request at the HTTP API layer.

```bash
# 1. Set a bootstrap admin
export ORBFLOW_BOOTSTRAP_ADMIN=admin@example.com

# 2. Create a policy
curl -X PUT http://localhost:8080/api/v1/rbac/policy \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "statements": [
      { "effect": "allow", "principals": ["role:engineer"],
        "actions": ["workflow:read", "workflow:execute"], "resources": ["*"] },
      { "effect": "deny", "principals": ["role:viewer"],
        "actions": ["workflow:write", "workflow:delete"], "resources": ["*"] }
    ]
  }'
```

Available actions: `workflow:read`, `workflow:write`, `workflow:delete`, `workflow:execute`, `node:execute`, `credential:read`, `credential:write`.

> [!NOTE]
> Configure your API gateway to inject `X-User-Id` headers and enable `trust_x_user_id` in server options. Only enable this behind a trusted reverse proxy.

---

## Observability

Enable OpenTelemetry tracing in your config:

```yaml
otel:
  enabled: true
  endpoint: "http://localhost:4317"
  service_name: "orbflow"
  sample_rate: 1.0
```

Start a local Jaeger instance for trace visualization:

```bash
docker run -d --name jaeger -p 16686:16686 -p 4317:4317 jaegertracing/all-in-one:latest
```

Query execution analytics:

```bash
curl http://localhost:8080/api/v1/analytics/executions   # Latency percentiles
curl http://localhost:8080/api/v1/analytics/failures     # Failure patterns
curl http://localhost:8080/api/v1/analytics/costs        # Cost tracking
```

---

## Development

> [!NOTE]
> The primary task runner is [`just`](https://github.com/casey/just). Run `just` to see all available recipes.

### Project Structure

```
orbflow/
├── crates/                    Rust workspace (20 crates)
│   ├── orbflow-core/          Domain types, port traits, events
│   ├── orbflow-engine/        DAG coordinator, CEL evaluation
│   ├── orbflow-postgres/      PostgreSQL persistence
│   ├── orbflow-natsbus/       NATS JetStream transport
│   ├── orbflow-httpapi/       Axum REST API
│   ├── orbflow-grpcapi/       gRPC API surface
│   ├── orbflow-builtins/      21 built-in node executors
│   ├── orbflow-worker/        Task executor library
│   ├── orbflow-trigger/       Cron, webhook, event triggers
│   ├── orbflow-plugin/        Plugin loader (JSON-RPC + gRPC)
│   ├── orbflow-cel/           CEL expression evaluator with cache
│   ├── orbflow-config/        YAML config with env var expansion
│   ├── orbflow-mcp/           MCP client
│   ├── orbflow-registry/      Plugin marketplace index
│   ├── orbflow-memstore/      In-memory store (testing)
│   ├── orbflow-testutil/      Mock implementations
│   ├── orbflow-test/          Integration test utilities
│   ├── orbflow-server/        Server binary
│   └── orbflow-worker-bin/    Worker binary
├── apps/web/                  Next.js 16 frontend
├── packages/orbflow-core/     Frontend SDK (stores, types, hooks)
├── configs/                   YAML configuration files
├── proto/                     gRPC / plugin protocol definitions
├── plugins/                   Plugin directory
├── docker-compose.yml         Infrastructure (Postgres + NATS)
├── Dockerfile                 Frontend production image
├── justfile                   Task runner recipes
└── Cargo.toml                 Rust workspace definition
```

### Common Commands

```bash
# Build
just build                  # Release binaries
just build-web              # Frontend production build
just check                  # Type-check Rust workspace (fast)

# Test
just test                   # All Rust tests
just test-crate orbflow-core   # Single crate
just test-web               # Frontend tests
just test-all               # Everything

# Lint & Format
just lint                   # Clippy with -D warnings
just fmt                    # Format Rust code

# Quality Gate
just ci                     # Full pipeline: format + lint + test + build
just pre-commit             # Quick pre-commit check

# Infrastructure
just infra                  # Start Postgres + NATS
just infra-down             # Stop infrastructure
just infra-reset            # Wipe all data and restart
just db-shell               # Open psql shell
just db-status              # Check connection + table count

# Debugging
just debug-server           # Server with RUST_LOG=debug
just debug-worker           # Worker with RUST_LOG=debug
just env-check              # Verify all required tools
just tree                   # Crate dependency graph
```

### Docker Deployment

```bash
just docker-up              # Start all services
just docker-down            # Stop all services
just docker-logs            # Tail service logs
```

| Service | Port | Description |
|---------|------|-------------|
| PostgreSQL 16 | `5432` | Persistent storage |
| NATS 2.11 | `4222` | JetStream message transport |
| NATS Monitor | `8222` | NATS monitoring dashboard |
