# 🦀 Rust Implementation Guide

Detailed implementation and reference guide for the Rust version of World Bank Analyzer.

## Table of Contents
1. [Project Overview](#project-overview)
2. [Architecture](#architecture)
3. [Code Structure](#code-structure)
4. [Build System](#build-system)
5. [API Reference](#api-reference)
6. [Component Guide](#component-guide)
7. [Testing](#testing)
8. [Deployment](#deployment)

## Project Overview

### What is This?
A complete rewrite of a Telegram Mini App from JavaScript/React to Rust, providing:
- **Backend API** for World Bank economic data
- **Frontend Web App** for data visualization
- **Type-safe** implementation across stack
- **WASM** frontend for native browser performance

### Why Rust?
- ✅ **Memory Safety**: No null pointers, buffer overflows, or data races
- ✅ **Performance**: Compiled to native code (backend) and WASM (frontend)
- ✅ **Type Safety**: Compile-time verification prevents entire classes of bugs
- ✅ **Concurrency**: Fearless parallelism with async/await
- ✅ **Zero Cost Abstractions**: High-level code compiles to optimal machine code

## Architecture

### System Design

```
┌─────────────────────────────────────────────────────┐
│ Client Browser                                      │
│  ┌───────────────────────────────────────────────┐ │
│  │ Frontend (Yew WASM)                           │ │
│  │  ├─ Components (UI rendering)                 │ │
│  │  ├─ Pages (routing)                           │ │
│  │  ├─ API Client (HTTP requests)                │ │
│  │  └─ Storage Manager (localStorage)            │ │
│  └────────────────┬────────────────────────────┘ │
└───────────────────┼──────────────────────────────┘
                    │ HTTP/REST
                    ▼
┌─────────────────────────────────────────────────────┐
│ Server (Rust Backend)                               │
│  ┌───────────────────────────────────────────────┐ │
│  │ Actix-web Framework                           │ │
│  │  ├─ Router (request routing)                  │ │
│  │  ├─ Middleware (CORS, logging)                │ │
│  │  ├─ Handlers (endpoint logic)                 │ │
│  │  └─ Error Handling                            │ │
│  ├─ API Client (Reqwest)                         │ │
│  ├─ Data Models (Serde)                          │ │
│  └─ Error Types (Custom)                         │ │
│  │                                                │ │
│  └─────────────────┬────────────────────────────┘ │
└────────────────────┼──────────────────────────────┘
                     │ HTTP (external)
                     ▼
              World Bank API
         https://api.worldbank.org/v2
```

### Data Flow

1. **User interacts** with UI (selects country, indicator)
2. **Frontend component** updates state
3. **Effect hook** triggers API call
4. **Gloo-net client** sends HTTP request to backend
5. **Backend receives** request at handler
6. **Handler validates** input and calls API client
7. **API client** fetches data from World Bank
8. **Response** is serialized to JSON
9. **Frontend** deserializes and updates UI
10. **Component** re-renders with new data

## Code Structure

### Backend Structure

#### Entry Point (`main.rs`)
```rust
// Sets up Actix-web server
// Initializes logger
// Creates app with routes
// Binds to 0.0.0.0:8080
```

#### Handlers (`handlers.rs`)
```rust
pub async fn get_countries() -> Result<HttpResponse, ApiError>
pub async fn get_indicators() -> HttpResponse
pub async fn get_data(path: web::Path<(String, String)>) -> Result<HttpResponse, ApiError>
pub async fn compare_countries(req: web::Json<ComparisonRequest>) -> Result<HttpResponse, ApiError>
```

#### API Client (`api.rs`)
```rust
pub async fn get_countries(&self) -> Result<Vec<Country>, ApiError>
pub async fn get_country_data(&self, country_id: &str, indicator_id: &str) -> Result<CountryData, ApiError>
```

#### Data Models (`models.rs`)
```rust
#[derive(Serialize, Deserialize)]
pub struct Country { ... }

#[derive(Serialize, Deserialize)]
pub struct Indicator { ... }

#[derive(Serialize, Deserialize)]
pub struct DataPoint { ... }
```

#### Error Handling (`errors.rs`)
```rust
#[derive(Error, Debug)]
pub enum ApiError {
    CountryNotFound,
    IndicatorNotFound,
    ExternalApiError,
    SerializationError,
    InternalError,
}

// Implements ResponseError for automatic HTTP response conversion
```

#### Indicator Data (`data.rs`)
```rust
pub fn get_indicators() -> IndicatorCategory
// Returns hardcoded indicator definitions
// Organized by category (economic, demographic, etc.)
```

### Frontend Structure

#### Entry Point (`main.rs`)
```rust
#[function_component(App)]
fn app() -> Html
// Initializes Yew app
// Sets up routing
// Renders header and main content
```

#### Data Models (`models.rs`)
```rust
// Mirrors backend models
#[derive(Serialize, Deserialize)]
pub struct Country { ... }

#[derive(Serialize, Deserialize)]
pub struct DataPoint { ... }
```

#### API Client (`api.rs`)
```rust
pub async fn get_countries(&self) -> Result<Vec<Country>, FetchError>
pub async fn get_indicators(&self) -> Result<IndicatorCategory, FetchError>
pub async fn get_data(&self, country_id: &str, indicator_id: &str) -> Result<CountryData, FetchError>
pub async fn compare_countries(&self, countries: Vec<String>, indicators: Vec<String>) -> Result<serde_json::Value, FetchError>
```

#### Storage Management (`storage.rs`)
```rust
pub fn get_favorites() -> Result<Vec<Value>, StorageError>
pub fn save_favorites(favorites: Vec<Value>) -> Result<(), StorageError>
pub fn add_favorite(favorite: Value) -> Result<(), StorageError>
pub fn remove_favorite(id: &str) -> Result<(), StorageError>
```

#### Components

**CountrySelector** (`components/country_selector.rs`)
- Props: countries list, callback on selection
- State: search filter
- Features: Search, multi-select (max 3)

**IndicatorSelector** (`components/indicator_selector.rs`)
- Props: indicator categories, callback on selection
- State: active category
- Features: Tab switching, category-based filtering

**ChartViewer** (`components/chart_viewer.rs`)
- Props: data points from countries
- No state (presentation component)
- Features: Bar chart visualization, multi-country comparison

#### Pages

**SearchPage** (`pages/search_page.rs`)
- Main application interface
- Orchestrates component interactions
- Handles data fetching and state management

**ComparisonPage** (`pages/comparison_page.rs`)
- Dedicated comparison view
- Foundation for advanced features

**FavoritesPage** (`pages/favorites_page.rs`)
- Display saved searches
- Manage saved queries

## Build System

### Cargo Workspace

```toml
[workspace]
members = ["backend", "frontend"]
```

Monorepo structure:
- Backend and frontend as separate members
- Shared `Cargo.toml` for workspace settings
- Each with its own dependencies

### Building

```bash
# Build backend
cd backend
cargo build --release

# Build frontend
cd frontend
wasm-pack build --release --target web

# Optimized outputs
# Backend: ./target/release/bank-api (~15MB)
# Frontend: ./pkg/ (~200KB)
```

### Dependencies

#### Backend
- `actix-web`: Web framework
- `tokio`: Async runtime
- `reqwest`: HTTP client
- `serde`: Serialization
- `log/env_logger`: Logging

#### Frontend
- `yew`: UI framework
- `yew-router`: Routing
- `gloo-net`: HTTP for WASM
- `gloo-storage`: Storage wrapper
- `wasm-bindgen`: WASM interop

## API Reference

### REST Endpoints

#### GET /api/countries
Returns list of all countries.

**Response:**
```json
[
  {
    "id": "USA",
    "name": "United States",
    "region": "North America",
    "capital": "Washington",
    "code": "USA"
  }
]
```

#### GET /api/indicators
Returns indicator categories.

**Response:**
```json
{
  "economic": [
    {
      "id": "NY.GDP.MKTP.CD",
      "name": "GDP (current US$)",
      "category": "economic",
      "unit": "USD"
    }
  ],
  "demographic": [...],
  "social": [...],
  "environmental": [...]
}
```

#### GET /api/data/{country_id}/{indicator_id}
Returns time-series data for a country and indicator.

**Response:**
```json
{
  "country_id": "USA",
  "country_name": "United States",
  "indicator_id": "NY.GDP.MKTP.CD",
  "indicator_name": "GDP (current US$)",
  "data": [
    { "year": "2023", "value": 27360718000000.0 },
    { "year": "2022", "value": 25744129000000.0 }
  ]
}
```

#### POST /api/compare
Compare multiple countries across multiple indicators.

**Request:**
```json
{
  "countries": ["USA", "GBR", "FRA"],
  "indicators": ["NY.GDP.MKTP.CD", "SP.POP.TOTL"]
}
```

**Response:**
```json
{
  "countries": [...],
  "data": {
    "USA": {
      "NY.GDP.MKTP.CD": [
        { "year": "2023", "value": 27360718000000.0 }
      ]
    }
  }
}
```

## Component Guide

### Creating a New Component

```rust
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MyComponentProps {
    pub title: String,
    pub on_click: Callback<()>,
}

#[function_component(MyComponent)]
pub fn my_component(props: &MyComponentProps) -> Html {
    let on_click = {
        let on_click = props.on_click.clone();
        Callback::from(move |_| {
            on_click.emit(());
        })
    };

    html! {
        <div onclick={on_click}>
            <h1>{&props.title}</h1>
        </div>
    }
}
```

### Using Effects

```rust
{
    let data = data.clone();
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let client = ApiClient::new();
                if let Ok(result) = client.get_countries().await {
                    data.set(result);
                }
            });
            || ()
        },
        (),  // Dependencies
    );
}
```

## Testing

### Backend Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test get_countries

# Run with output
cargo test -- --nocapture

# Run in release mode
cargo test --release
```

### Example Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_countries() {
        let client = ApiClient::new();
        let result = client.get_countries().await;
        assert!(result.is_ok());
    }
}
```

### Frontend Testing

```bash
# Add test dependency
cargo add --dev wasm-bindgen-test

# Create test
#[cfg(test)]
mod tests {
    #[wasm_bindgen_test]
    fn test_something() {
        assert_eq!(2 + 2, 4);
    }
}

# Run WASM tests
wasm-pack test --headless --firefox
```

## Deployment

### Option 1: Standalone Binary

```bash
# Build
cargo build --release

# Copy binary
scp backend/target/release/bank-api user@server:/opt/app/

# Run
/opt/app/bank-api
```

### Option 2: Docker

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/bank-api /usr/local/bin/
EXPOSE 8080
CMD ["bank-api"]
```

### Option 3: Systemd Service

```ini
[Unit]
Description=World Bank Analyzer API
After=network.target

[Service]
Type=simple
ExecStart=/opt/bank/bank-api
Restart=on-failure
User=bank
Group=bank

[Install]
WantedBy=multi-user.target
```

### Frontend Deployment

```bash
# Build WASM
cd frontend
wasm-pack build --release --target web

# Upload pkg/ to CDN
# Configure CORS headers:
# Access-Control-Allow-Origin: *
# Access-Control-Allow-Methods: GET, POST, OPTIONS

# Or serve with backend:
# cp -r frontend/pkg/* backend/static/
# Serve frontend from /static/ path
```

## Performance Optimization

### Backend
- Enable LTO in Cargo.toml: `lto = true`
- Use release profile: `cargo build --release`
- Enable SIMD: Add CPU-specific flags

### Frontend
- Minimize WASM size: `wasm-opt -O4`
- Tree-shake unused code: Rust compiler does this automatically
- Lazy load components: Use async routing

## Common Issues and Solutions

### Issue: Build fails with SSL error
**Solution:**
```bash
# macOS
brew install openssl
export LDFLAGS="-L$(brew --prefix openssl)/lib"
export CPPFLAGS="-I$(brew --prefix openssl)/include"

# Linux
sudo apt install libssl-dev
```

### Issue: WASM module not loading
**Solution:**
- Check MIME type: `application/wasm`
- Verify CORS headers
- Check browser console for errors
- Ensure wasm-pack built successfully

### Issue: API requests return CORS errors
**Solution:**
- Verify CORS middleware in `main.rs`
- Check `Access-Control-Allow-Origin` headers
- Test with curl: `curl -v http://localhost:8080/api/countries`

## Further Reading

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Yew Architecture](https://yew.rs/docs/architecture)
- [Actix-web Best Practices](https://actix.rs/actix-web/dev/)
- [WebAssembly MDN Docs](https://developer.mozilla.org/en-US/docs/WebAssembly)

---

**Last Updated**: December 2024
**Rust Edition**: 2021
**Maintainer**: Your Team
