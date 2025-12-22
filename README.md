# 📊 Economic News Visual Bot

A production-ready Rust application that fetches high-impact economic news, filters them based on configurable rules, generates a clean visual summary, and publishes the result to Telegram.

---

## 🚀 Overview

**Economic News Visual Bot** is an automated pipeline designed for traders, analysts, and content creators who need **fast, reliable, and visual economic news delivery**.

The system:
1. Fetches economic events from a data source  
2. Filters them using configurable business rules  
3. Renders a shareable image  
4. Publishes it automatically to Telegram  

This project is built with **Rust**, focusing on **robustness, type safety, async I/O, and clean architecture**.

---

## 🎯 Problem

Economic calendars are:
- Text-heavy
- Not optimized for social media
- Time-consuming to manually summarize
- Hard to automate reliably

Traders and channels need **clean, visual, and filtered summaries**, delivered automatically.

---

## ✅ Solution

This project provides:

- ⚙️ Config-driven filtering (impact, currency, timezone, language)
- 🧠 Clear separation of business logic
- 🖼️ Deterministic image generation
- 📤 Automated Telegram publishing
- 🧵 Async networking without blocking
- 🛡️ Strong error handling (no panics)

---

## 🧩 Architecture

```
┌────────────┐
│   Config   │
└─────┬──────┘
      │
┌─────▼──────┐
│  Scraper   │
└─────┬──────┘
      │
┌─────▼──────┐
│   Filter   │
└─────┬──────┘
      │
┌─────▼──────┐
│  Renderer  │
└─────┬──────┘
      │
┌─────▼──────┐
│ Publisher  │
└────────────┘
```

---

## 🛠 Tech Stack

- **Language:** Rust
- **Async Runtime:** Tokio
- **Telegram API:** Teloxide
- **Image Rendering:** image, imageproc, ab_glyph
- **Config:** YAML
- **Error Handling:** Result-based, no panics

---

## 📂 Project Structure

```
src/
├── app.rs
├── main.rs
├── config/
├── scraper/
├── filter/
├── image/
├── publisher/
├── models/
└── error.rs
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

## ▶️ How to Run

### Requirements
- Rust (stable)
- Telegram Bot Token
- Telegram Chat ID or Channel Username

### Environment Variables

```
TELEGRAM_BOT_TOKEN=your_bot_token_here
TELEGRAM_CHAT_ID=@your_channel_or_numeric_id
```

### Run

```
cargo run
```

---

## 🖼 Output

The bot generates a visual summary image and sends it directly to Telegram.

---

## 🔐 Error Handling Philosophy

- No panic or unwrap in application logic
- Errors are propagated and logged cleanly
- Network failures handled gracefully

---

## 🧠 What This Project Demonstrates

- Async vs sync decision-making
- Clean architecture and separation of concerns
- Type-safe API integration
- Real-world external service integration
- Production-minded Rust design

---

## 🔮 Future Improvements

- Multiple data sources
- Instagram / Twitter publishing
- Scheduling support
- Multi-language rendering
- Retry and backoff strategies
