# 📊 Economic News Visual Bot (Telegram)

A **production-ready Telegram bot** written in **Rust** that fetches economic calendar data for a given date, prioritizes events based on admin rules, collects admin feedback via poll, and finally **renders a clean visual summary image** and publishes it to Telegram.

---

## 🚀 Overview

**Economic News Visual Bot** automates the daily workflow of economic-news channels:

1. Fetch economic events for **today or any requested date**
2. Apply **business-level prioritization**
3. Ask the admin (via Telegram poll) which events should be rendered
4. Generate a **professional visual image**
5. Publish the result to Telegram automatically

The entire pipeline is **async, resilient, and designed for production use**.

---

## 🎯 What Problem Does It Solve?

Economic calendars are:
- Text-heavy
- Hard to customize
- Not suitable for visual content
- Time-consuming to summarize manually

This bot turns raw economic data into **curated, visual, and shareable content** with **admin control in the loop**.

---

## 🧠 Core Idea

> **Automation with human validation**

Instead of blindly publishing data:
- The bot prioritizes events
- The admin confirms selections via poll
- The final output is rendered exactly as desired

---

## 🔁 High-Level Pipeline

```
/get [date]
   ↓
Fetch economic calendar (ForexFactory)
   ↓
Filter & prioritize events
   ↓
Create Telegram poll (top 10)
   ↓
Admin votes
   ↓
Auto render selected events
   ↓
Send image to Telegram
```

---

## ✨ Features

- 📅 Fetch calendar for **today or any date**
- ⚙️ Config-driven filtering & prioritization
- 🗳 Admin selection via Telegram poll
- 🖼 Template-based image rendering
- 📤 Automatic Telegram publishing
- 🔁 Resilient startup (retry on Telegram failure)
- 🧠 In-memory daily state management
- 🧼 Clean error handling (no panics)

---

## 🛠 Tech Stack

- **Language:** Rust
- **Async Runtime:** Tokio
- **Telegram API:** Teloxide
- **Rendering:** image, imageproc, ab_glyph
- **Scraping:** Playwright (Node.js)
- **Config:** YAML
- **State:** In-memory (daily cache)

---

## 📂 Project Structure

```
src/
├── app.rs
├── main.rs
├── commands/
├── sources/
├── filter/
├── normalize/
├── renderer/
├── publisher/
├── state.rs
├── models/
└── error.rs
scripts/
└── forexfactory_fetch.js
assets/
└── templates & icons
```

---

## ▶️ Usage

### Telegram Commands

```
/get
/ get 12/2/2025
/poll
/render
```

---

## ⚙️ Configuration

Example `config/default.yaml`:

```yaml
impact: [high, medium]
currency: [USD, EUR, GBP]
timezone: NY
language: En
```

---

## 🔐 Environment Variables

```
TELEGRAM_BOT_TOKEN=your_bot_token
TELEGRAM_CHAT_ID=@your_channel_or_chat_id
```

---

## 🖼 Output

The final output is a **PNG image** generated from a fixed template:
- Header with date & timezone
- List of selected events
- Impact icons

---

## 📄 License

This project is proprietary software.

The source code is provided for review and evaluation purposes only.
Any use, reproduction, modification, distribution, or commercial use of this project,
in whole or in part, without explicit written permission from the author
is strictly prohibited.

See the LICENSE file for full details.