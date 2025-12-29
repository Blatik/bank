#!/usr/bin/env bash

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
cat << "EOF"

╔═════════════════════════════════════════════════════════════════╗
║                                                                 ║
║           🌍 Світовий Аналітик - Telegram Mini App             ║
║               World Bank Economic Data Analysis                ║
║                                                                 ║
╚═════════════════════════════════════════════════════════════════╝

EOF
echo -e "${NC}"

# Check Node.js
echo -e "${YELLOW}[1/7]${NC} Перевірка Node.js..."
if ! command -v node &> /dev/null; then
    echo -e "${RED}✗ Node.js не встановлен${NC}"
    echo "Встановіть Node.js з https://nodejs.org/"
    exit 1
fi
echo -e "${GREEN}✓ Node.js $(node -v)${NC}"

# Check npm
echo -e "${YELLOW}[2/7]${NC} Перевірка npm..."
if ! command -v npm &> /dev/null; then
    echo -e "${RED}✗ npm не встановлен${NC}"
    exit 1
fi
echo -e "${GREEN}✓ npm $(npm -v)${NC}"

# Check wrangler
echo -e "${YELLOW}[3/7]${NC} Перевірка Wrangler..."
if ! command -v wrangler &> /dev/null; then
    echo -e "${YELLOW}! Wrangler не встановлен, встановлюю...${NC}"
    npm install -g wrangler
fi
echo -e "${GREEN}✓ Wrangler встановлено${NC}"

# Install dependencies
echo -e "${YELLOW}[4/7]${NC} Встановлення залежностей..."
npm install --silent
echo -e "${GREEN}✓ Залежності встановлені${NC}"

# Build frontend
echo -e "${YELLOW}[5/7]${NC} Побудова фронтенду..."
npm run build > /dev/null 2>&1
echo -e "${GREEN}✓ Фронтенд побудований${NC}"

# Check Cloudflare login
echo -e "${YELLOW}[6/7]${NC} Перевірка Cloudflare облікового запису..."
if ! wrangler whoami > /dev/null 2>&1; then
    echo -e "${YELLOW}! Потрібне підключення до Cloudflare${NC}"
    wrangler login
fi
ACCOUNT_ID=$(wrangler whoami 2>/dev/null | grep -oP '(?<=account_id: )\w+' || echo "")
if [ -z "$ACCOUNT_ID" ]; then
    echo -e "${RED}✗ Account ID не знайдено${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Account ID знайдено: $ACCOUNT_ID${NC}"

# Update wrangler.toml
echo -e "${YELLOW}[7/7]${NC} Оновлення конфігурації..."
sed -i "" "s/account_id = \"\"/account_id = \"$ACCOUNT_ID\"/" wrangler.toml
echo -e "${GREEN}✓ Конфігурація оновлена${NC}"

# Summary
echo -e "${BLUE}"
cat << "EOF"

╔═════════════════════════════════════════════════════════════════╗
║                     ✓ Готово до запуску!                       ║
╚═════════════════════════════════════════════════════════════════╝

EOF
echo -e "${NC}"

echo -e "${GREEN}Наступні кроки:${NC}"
echo ""
echo "  📖 Читайте документацію:"
echo "     - ${BLUE}README.md${NC} — Загальна інформація"
echo "     - ${BLUE}DEPLOYMENT.md${NC} — Розгортання на Cloudflare"
echo "     - ${BLUE}ARCHITECTURE.md${NC} — Деталі архітектури"
echo ""
echo "  🚀 Локальна розробка:"
echo "     ${BLUE}npm run dev:full${NC}"
echo "     Відкрийте: http://localhost:5173"
echo ""
echo "  📦 Розгортання на Cloudflare:"
echo "     ${BLUE}npm run deploy${NC}"
echo ""
echo "  📊 Перевірити API:"
echo "     ${BLUE}curl http://localhost:8787/api/health${NC}"
echo ""
echo "  💡 Вибір окремо:"
echo "     ${BLUE}npm run dev${NC}    — Запустити Worker"
echo "     ${BLUE}npm run build${NC}  — Побудувати фронтенд"
echo ""
echo -e "${YELLOW}Питання?${NC} Читайте документацію або відкрийте issue на GitHub!"
echo ""
