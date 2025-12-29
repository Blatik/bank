#!/bin/bash

# World Bank Telegram Mini App - Deployment Guide

echo "🌍 Світовий Аналітик - Інструкція розгортання"
echo "================================================"
echo ""

# Step 1: Install dependencies
echo "📦 Крок 1: Встановлення залежностей..."
npm install

# Step 2: Build frontend
echo ""
echo "🔨 Крок 2: Побудова фронтенду..."
npm run build

# Step 3: Check if user is logged in to Cloudflare
echo ""
echo "☁️  Крок 3: Перевірка підключення до Cloudflare..."
ACCOUNT_ID=$(wrangler whoami 2>/dev/null | grep -oP '(?<=account_id: )\w+' || echo "")

if [ -z "$ACCOUNT_ID" ]; then
    echo "⚠️  Не знайдено облікового запису Cloudflare"
    echo "Будь ласка, виконайте: wrangler login"
    exit 1
fi

echo "✅ Знайдено Account ID: $ACCOUNT_ID"

# Step 4: Ask for confirmation
echo ""
echo "Налаштування для розгортання:"
echo "- Account ID: $ACCOUNT_ID"
echo "- Project Name: world-bank-miniapp"
echo ""
read -p "Готові розгорнути на Cloudflare? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    npm run deploy
    echo ""
    echo "✅ Розгортання завершено!"
    echo ""
    echo "Ваша аплікація доступна на:"
    echo "https://world-bank-miniapp.<your-subdomain>.workers.dev"
    echo ""
    echo "Для додавання в Telegram:"
    echo "1. Напишіть @BotFather"
    echo "2. Виконайте /newapp"
    echo "3. Вкажіть URL вище"
else
    echo "Розгортання скасовано"
    exit 1
fi
