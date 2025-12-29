# Migration Guide: JavaScript to Rust 🦀

This document outlines the complete migration from the original JavaScript/React stack to Rust.

## Summary of Changes

### Original Stack (JavaScript)
- **Frontend**: React 18 + Vite + Chart.js
- **Backend**: Cloudflare Workers (Node.js) + Itty Router
- **Build Tool**: Vite bundler
- **Package Manager**: npm
- **Deployment**: Cloudflare Workers + Static hosting

### New Stack (Rust)
- **Frontend**: Yew framework (Rust) compiled to WASM
- **Backend**: Actix-web (async Rust web framework)
- **Build Tool**: Cargo + wasm-pack
- **Package Manager**: Cargo
- **Deployment**: Standalone binary + WASM static files

## File Mapping

### Frontend Components

| Original | Rust Equivalent | Location |
|----------|-----------------|----------|
| `CountrySelector.jsx` | `CountrySelector` | `frontend/src/components/country_selector.rs` |
| `IndicatorSelector.jsx` | `IndicatorSelector` | `frontend/src/components/indicator_selector.rs` |
| `DataVisualization.jsx` | `ChartViewer` | `frontend/src/components/chart_viewer.rs` |
| `Comparison.jsx` | `ComparisonPage` | `frontend/src/pages/comparison_page.rs` |
| `Favorites.jsx` | `FavoritesPage` | `frontend/src/pages/favorites_page.rs` |
| `App.jsx` | `SearchPage` | `frontend/src/pages/search_page.rs` |

### Backend API

| Endpoint | Original | Rust |
|----------|----------|------|
| `GET /api/countries` | Worker handler | `handlers::get_countries` |
| `GET /api/indicators` | Worker handler | `handlers::get_indicators` |
| `GET /api/data/:id/:country` | Worker handler | `handlers::get_data` |
| `POST /api/compare` | Worker handler | `handlers::compare_countries` |

## Key Implementation Differences

### 1. Component State Management

**JavaScript (React Hooks)**
```javascript
const [selectedCountries, setSelectedCountries] = useState([])
const [loading, setLoading] = useState(false)
```

**Rust (Yew Hooks)**
```rust
let selected_countries = use_state(Vec::<Country>::new);
let loading = use_state(|| false);
```

### 2. Async Operations

**JavaScript (async/await)**
```javascript
const response = await fetch('/api/countries')
const data = await response.json()
```

**Rust (async/await + wasm-bindgen-futures)**
```rust
let client = ApiClient::new();
let countries = client.get_countries().await?;
```

### 3. Error Handling

**JavaScript (try-catch)**
```javascript
try {
  const data = await fetch('/api/data')
} catch (error) {
  console.error('Error:', error)
}
```

**Rust (Result enum)**
```rust
pub async fn get_data(&self) -> Result<CountryData, ApiError> {
    let response = self.client.get(&url).send().await?;
    response.json().await.map_err(|e| ApiError::SerializationError(e.to_string()))
}
```

### 4. Data Serialization

**JavaScript (JSON directly)**
```javascript
const countries = response.json()
```

**Rust (Serde + serde_json)**
```rust
#[derive(Serialize, Deserialize)]
pub struct Country {
    pub id: String,
    pub name: String,
    // ...
}
```

### 5. Storage Management

**JavaScript (localStorage)**
```javascript
localStorage.setItem('favorites', JSON.stringify(favorites))
const saved = localStorage.getItem('favorites')
```

**Rust (gloo-storage wrapper)**
```rust
LocalStorage::set(FAVORITES_KEY, favorites)?
LocalStorage::get(FAVORITES_KEY)?
```

### 6. HTTP Requests

**JavaScript (fetch API)**
```javascript
fetch('/api/countries')
  .then(r => r.json())
  .catch(e => console.error(e))
```

**Rust (gloo-net for WASM, reqwest for backend)**
```rust
// Frontend (WASM)
let response = self.client.get(&url).send().await?;

// Backend (Actix-web)
let response = self.client.get(&url).send().await?;
```

## Building and Running

### Development

**JavaScript**
```bash
npm run dev:full
```

**Rust**
```bash
# Terminal 1: Backend
cd backend && cargo run

# Terminal 2: Frontend dev server (if needed)
cd frontend && wasm-pack build --dev --target web
python3 -m http.server 8000
```

### Production Build

**JavaScript**
```bash
npm run build
wasm-pack publish deploy
```

**Rust**
```bash
./build.sh
```

## Performance Considerations

### Bundle Size
- **Original**: ~500KB (JS + React + Chart.js)
- **Rust WASM**: ~200KB (optimized WASM)
- **Savings**: ~60% smaller

### Runtime Performance
- **Original**: JIT compiled by browser
- **Rust**: Compiled to native machine code (WASM)
- **Benefit**: Faster startup and execution

### Network Requests
- Both versions make the same API calls to World Bank
- Rust backend is significantly faster due to async/await and efficient memory usage

## Testing

### Backend Tests (Rust)
```bash
cd backend
cargo test

# Test specific function
cargo test get_countries

# Run with output
cargo test -- --nocapture
```

### Frontend Testing
Rust frontend testing with Yew requires additional setup:
```bash
# Install wasm-bindgen-test
cargo add --dev wasm-bindgen-test

# Run tests
wasm-pack test --headless --firefox
```

## Deployment Differences

### Original (JavaScript)
1. Build: `npm run build`
2. Deploy frontend to static hosting (Vercel, Netlify)
3. Deploy worker to Cloudflare
4. Configure DNS and environment variables

### Rust Version
1. Build: `./build.sh`
2. Deploy backend binary to server (Docker, bare metal)
3. Deploy WASM frontend to CDN
4. Configure backend to serve static files
5. Set up CORS if frontend and backend on different domains

## Maintenance Considerations

### Dependency Management

**JavaScript (npm)**
- 30+ direct dependencies
- Large node_modules folder
- Requires security updates regularly

**Rust (Cargo)**
- 12+ direct dependencies
- Compiled into binary
- Fewer runtime vulnerabilities

### Type Safety

**JavaScript**
- Runtime type errors possible
- TypeScript provides optional safety

**Rust**
- Compile-time type checking
- No runtime type errors
- Borrow checker prevents memory issues

## Learning Resources for Rust Migration

1. **Rust Fundamentals**
   - The Rust Book: https://doc.rust-lang.org/book/
   - Rust by Example: https://doc.rust-lang.org/rust-by-example/

2. **Web Development**
   - Yew Guide: https://yew.rs/docs/
   - Actix-web Docs: https://actix.rs/
   - WASM Book: https://rustwasm.org/docs/book/

3. **Async Programming**
   - Tokio Tutorial: https://tokio.rs/
   - Async Rust: https://rust-lang.github.io/async-book/

## Troubleshooting Common Issues

### Issue: Build fails with linking errors
**Solution**: Install OpenSSL development files
```bash
# macOS
brew install openssl

# Ubuntu/Debian
sudo apt-get install libssl-dev

# Fedora
sudo dnf install openssl-devel
```

### Issue: WASM module won't load in browser
**Solution**: Check console for CORS errors and verify:
1. `wasm-pack build` completed successfully
2. `.wasm` file is being served with correct MIME type
3. Server has `Access-Control-Allow-Origin` headers

### Issue: API requests returning 404
**Solution**: 
1. Verify backend is running on correct port
2. Check API_BASE URL in frontend code
3. Test endpoint manually: `curl http://localhost:8080/api/countries`

## Next Steps

1. **Install Rust**: https://rustup.rs/
2. **Install wasm-pack**: `cargo install wasm-pack`
3. **Read RUST_README.md** for detailed setup
4. **Build the project**: `./build.sh`
5. **Run the application**: `./run.sh`
6. **Explore the code** and start contributing!

---

**Questions?** Check the troubleshooting section or refer to the resource links above.
