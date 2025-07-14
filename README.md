# Trade I/O

![Rust](https://img.shields.io/badge/Rust-006845?style=flat&logo=rust&logoColor=white&labelColor=333333)
![Build Status](https://github.com/xeodus/Trade-io/actions/workflows/ci.yml/badge.svg)

This is a high-performant trading system that integrates with Zerodha's KiteConnect API for automated stock trading with real-time market data. It enables us to talk to a centralised exchange like Zerodha directly through any LLM using MCP servers. So, this application was built on top of Zerodha's kite dev platform and the core idea is to harness the power of agent-based systems and MCP servers.

## Features

- [x] **Automated Trading:** Execute Buy/Sell/Cancel orders with market or limit pricing
- [x] **Smart Stock Selection:** Automatically identifies the best performing stocks over a certain time period and execute trades with stop losses
- [x] **Real-Time Data:** Live data feeds via WebSocket connections
- [x] **Risk Management:** Built in stop loss and target price configuration
- [x] **REST API:** Clean HTTP endpoints for trade execution and monitoring

## Prerequisites
- [x] Rust 1.70+
- [x] Zerodha's KiteConnect API
- [X] Active Zerodha account

## Installation

```bash
    git clone https://github.com/xeodus/Trade-io.git
    cd Trade-io
    cargo build --release
```
Once the server is up and running, we can perform our task through Claude AI via simple prompts.
## Environment Setup

```bash
    API_KEY=your_api_key
    API_SECRET=your_api_secret
    ACCESS_TOKEN=your_access_token
```
