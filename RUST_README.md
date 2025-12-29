# World Bank Analyzer - Rust Edition 🦀

A complete rewrite of the Telegram Mini App in **Rust**, featuring a high-performance backend and WASM-based frontend.

## 🎯 Tech Stack

### Backend
- **Framework**: Actix-web (async/await Rust web framework)
- **Runtime**: Tokio (async runtime)
- **API Client**: Reqwest (HTTP client)
- **Serialization**: Serde + serde_json
- **Data Source**: World Bank Open Data API

### Frontend
- **Framework**: Yew (Rust UI framework)
- **Target**: WebAssembly (WASM)
- **Routing**: Yew Router
- **Storage**: Gloo Storage (LocalStorage wrapper)
- **Networking**: Gloo Net (HTTP client for WASM)

## 📁 Project Structure

```
bank/
├── backend/                    # Rust backend (Actix-web)
│   ├── src/
│   │   ├── main.rs            # Server entry point
│   │   ├── api.rs             # API client for World Bank
│   │   ├── handlers.rs        # Request handlers
│   │   ├── models.rs          # Data models
│   │   ├── errors.rs          # Error types
│   │   └── data.rs            # Indicator definitions
│   └── Cargo.toml
│
├── frontend/                   # Rust frontend (Yew + WASM)
│   ├── src/
│   │   ├── main.rs            # App entry point
│   │   ├── models.rs          # Shared models
│   │   ├── api.rs             # API client
│   │   ├── storage.rs         # LocalStorage management
│   │   ├── components/        # Reusable components
│   │   │   ├── country_selector.rs
│   │   │   ├── indicator_selector.rs
│   │   │   └── chart_viewer.rs
│   │   └── pages/             # Page components
│   │       ├── search_page.rs
│   │       ├── comparison_page.rs
│   │       └── favorites_page.rs
│   ├── index.html
│   ├── style.css
│   └── Cargo.toml
│
├── Cargo.toml                 # Workspace configuration
├── build.sh                   # Build script
├── run.sh                     # Run script
└── README.md
```

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ ([Install](https://rustup.rs/))
- Cargo (comes with Rust)
- wasm-pack (for frontend compilation)
  ```bash
  cargo install wasm-pack
  ```

### Installation

```bash
# Clone the repository
git clone <repo-url>
cd bank

# Build the project
chmod +x build.sh
./build.sh

# Run the backend
chmod +x run.sh
./run.sh
```

The application will be available at:
- **Frontend**: http://localhost:8080
- **API**: http://localhost:8080/api

## 📝 API Endpoints

### Countries
```
GET /api/countries
```
Returns list of all countries with metadata.

### Indicators
```
GET /api/indicators
```
Returns available indicators organized by category (economic, demographic, social, environmental).

### Country Data
```
GET /api/data/{country_id}/{indicator_id}
```
Returns historical data for a specific country and indicator.

### Compare
```
POST /api/compare
Content-Type: application/json

{
  "countries": ["USA", "GBR", "FRA"],
  "indicators": ["NY.GDP.MKTP.CD", "SP.POP.TOTL"]
}
```
Returns comparison data for multiple countries.

## 🛠️ Development

### Backend Development

```bash
cd backend

# Development mode with auto-reload
cargo watch -x run

# Run tests
cargo test

# Build release binary
cargo build --release
```

### Frontend Development

```bash
cd frontend

# Build for development
wasm-pack build --dev --target web

# Serve locally (requires python or similar)
python3 -m http.server 8000
```

## 🏗️ Architecture

### Data Flow

```
Browser (Yew/WASM)
        ↓
Frontend API Client (gloo-net)
        ↓
Actix-web Backend
        ↓
World Bank API (https://api.worldbank.org/v2)
```

### Components

**Backend (Actix-web):**
- `api.rs`: Handles HTTP requests to World Bank API with error handling
- `handlers.rs`: Route handlers for REST endpoints
- `models.rs`: Serde-compatible data structures
- `errors.rs`: Custom error types implementing ResponseError

**Frontend (Yew):**
- `CountrySelector`: Multi-select country picker with search
- `IndicatorSelector`: Categorized indicator selection
- `ChartViewer`: Data visualization component
- `SearchPage`: Main search and analysis interface
- `ComparisonPage`: Side-by-side country comparison
- `FavoritesPage`: Saved searches management

## 📊 Features

✅ **Country Selection**: Choose up to 3 countries for analysis
✅ **Multiple Indicators**: 16+ economic, demographic, social, environmental indicators
✅ **Data Visualization**: Bar charts showing trends over time
✅ **Comparison**: Analyze multiple countries simultaneously
✅ **Favorites**: Save frequently used searches
✅ **Responsive Design**: Mobile-friendly interface
✅ **Type Safe**: Full Rust type safety from backend to frontend
✅ **Performance**: WASM frontend for near-native performance

## 🔄 Advantages Over Original

| Feature | Original (JS) | Rust Version |
|---------|---------------|--------------|
| Type Safety | Runtime errors | Compile-time safety |
| Performance | Cloudflare Workers | Native performance |
| Bundle Size | ~500KB | ~200KB (WASM) |
| Error Handling | Basic try-catch | Rust Result/Option |
| Build Times | Fast | ~30-60s (first build) |
| Learning Curve | JavaScript | Rust |

## 🧪 Testing

```bash
# Backend tests
cd backend
cargo test

# Run specific test
cargo test get_countries
```

## 📦 Deployment

### Deploy Backend to Production

```bash
cd backend

# Build release binary
cargo build --release

# Binary location: ./target/release/bank-api

# Run on production server
./target/release/bank-api
```

### Deploy Frontend

```bash
cd frontend

# Build for production
wasm-pack build --release --target web

# Deploy the `pkg/` directory to your web server
```

## 🐛 Troubleshooting

### Build Errors

**Error: "cannot find -lssl"**
- Install OpenSSL: `brew install openssl` (macOS) or `apt-get install libssl-dev` (Linux)

**Error: "wasm-pack not found"**
- Install: `cargo install wasm-pack`

### Runtime Errors

**API Connection Failed**
- Check backend is running: `curl http://localhost:8080/api/countries`
- Verify CORS headers in `handlers.rs`

## 📚 Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Yew Documentation](https://yew.rs/)
- [Actix-web Guide](https://actix.rs/)
- [World Bank API Docs](https://data.worldbank.org/developers)

## 📝 License

MIT

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

**Built with ❤️ in Rust 🦀**
