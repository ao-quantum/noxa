# Noxa

A full-stack spaced-repetition flashcard application powered by the modern **FSRS (Free Spaced Repetition Scheduler)** algorithm.

---

## Architecture

The project consists of two core services sharing a MySQL/MariaDB database:

- **`cadrs/`** — High-performance backend API written in Rust using [Rocket](https://rocket.rs/) and [sqlx](https://github.com/launchbadge/sqlx). Implements the FSRS algorithm for card scheduling, retrievability calculations, and study metrics.
- **`nafe/`** — Frontend application built with [SvelteKit](https://kit.svelte.dev/) (Svelte 5), TypeScript, and Tailwind CSS. Handles user authentication and the study/deck management interface.

---

## Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable toolchain)
- [Node.js](https://nodejs.org/) (v20+ recommended) and `npm`
- [MariaDB](https://mariadb.org/) or [MySQL](https://www.mysql.com/)

---

## Getting Started

### 1. Database Setup

Ensure your MySQL/MariaDB service is running:

```bash
# On systems with init.d / systemd:
sudo /etc/init.d/mysql start
# or
sudo systemctl start mariadb
```

Create a database and user (matching the default configs or your custom environment):

```sql
CREATE DATABASE noxa;
CREATE USER 'noxa'@'localhost' IDENTIFIED BY 'noxa';
GRANT ALL PRIVILEGES ON noxa.* TO 'noxa'@'localhost';
FLUSH PRIVILEGES;
```

### 2. Backend Setup (`cadrs`)

Navigate to the `cadrs` directory:

```bash
cd cadrs
cp .env.example .env
```

Ensure `DATABASE_URL` in `cadrs/.env` points to your MySQL database:

```env
DATABASE_URL=mysql://noxa:noxa@127.0.0.1:3306/noxa
```

Run the backend server:

```bash
cargo run
```

The Rocket server runs at `http://localhost:8000`.

### 3. Frontend Setup (`nafe`)

Navigate to the `nafe` directory:

```bash
cd nafe
cp .env.example .env
npm install
```

Verify your environment configuration in `nafe/.env`:

```env
DEBUG=false

MYSQL_USER=noxa
MYSQL_PASSWORD=noxa
MYSQL_HOST=127.0.0.1
MYSQL_PORT=3306
MYSQL_DATABASE=noxa

PUBLIC_CADRS_BASEURL=http://localhost:8000
```

Start the development server:

```bash
npm run dev
```

Visit `http://localhost:5173` in your browser.

### 4. Database Migrations

Once both services are running, visit:

```
http://localhost:5173/migrate
```

This will automatically create the required database tables (`users`, `sessions`, `cards`, `cardrecalls`).

---

## API Reference (`cadrs`)

All card endpoints require an `Authorization` header containing an active session ID:

- `GET /cards` — List all cards for the authenticated user (dynamically computes current retrievability).
- `POST /cards/create` — Create a new card (`{"front": "...", "back": "..."}`).
- `DELETE /cards/<id>` — Delete a card by ID.
- `GET /cards/next` — Fetch the next prioritized card to review based on FSRS scheduling.
- `PATCH /cards/review` — Submit review grade (`1` = Forgot, `2` = Hard, `3` = Good, `4` = Easy).
- `GET /cards/stats` — Summary statistics (total cards, due cards, mean stability, cards awaiting initial review).