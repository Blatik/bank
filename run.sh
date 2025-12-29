#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 Starting World Bank Rust Backend...${NC}"

# Start backend
cd backend
echo -e "${YELLOW}Starting API server on port 8080...${NC}"
cargo run --release &
BACKEND_PID=$!

sleep 2

if ! kill -0 $BACKEND_PID 2>/dev/null; then
    echo -e "${RED}❌ Backend failed to start${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Backend running (PID: $BACKEND_PID)${NC}"
echo ""
echo -e "${GREEN}🎉 Application is ready!${NC}"
echo -e "${YELLOW}Frontend: http://localhost:8080${NC}"
echo -e "${YELLOW}API: http://localhost:8080/api${NC}"
echo ""
echo "Press Ctrl+C to stop"

# Keep running
wait $BACKEND_PID
