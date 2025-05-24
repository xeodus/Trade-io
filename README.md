# Trade GPT

<p align="center">
  ![Claude AI](./claude-logo.svg)
  &nbsp;&nbsp;&nbsp;&nbsp;
  <a href="https://zerodha.com" target="_blank">
    <img src="https://zerodha.com/static/images/logo.svg" alt="Zerodha" height="50"/>
  </a>
</p>

## Introduction

![Rust](https://img.shields.io/badge/Rust-006845?style=flat&logo=rust&logoColor=white&labelColor=333333)

This is a high-performant trading system that integrates with Zerodha's KiteConnect API for automated stock trading with real-time market data. So, this application enables us to talk to a centralised exchange like Zerodha directly through any LLM using MCP servers.

## Features

- [x] **Automated Trading:** Execute Buy/Sell/Cancel orders with market or limit pricing
- [x] **Smart Stock Selection:** Automatically identifies the best performing stocks over a certain time period and execute trades with stop losses
- [x] **Real-Time Data:** Live data feeds via WebSocket connections
- [x] **Risk Management:** Built in stop loss and target price configuration
- [x] **RESTful API:** Clean HTTP endpoints for trade execution and monitoring

## Prerequisites
- [x] Rust 1.70+
- [x] Zerodha's KiteConnect API
- [X] Active Zerodha account

## Installation

```bash
    git clone https://github.com/xeodus/Trade-GPT.git
    cd Trade-GPT
    cargo build --release
```
## Environment Setup

```bash
    API_KEY=your_api_key
    API_SECRET=your_api_secret
    ACCESS_TOKEN=your_access_token
```
