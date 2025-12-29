// API client for World Bank data
const API_BASE = '/api'
const CACHE_DURATION = 1000 * 60 * 60 // 1 hour

// Simple cache implementation
const cache = new Map()

function getCacheKey(endpoint) {
  return `${endpoint}_${Date.now()}`
}

function getFromCache(key) {
  const cached = cache.get(key)
  if (cached && cached.expires > Date.now()) {
    return cached.data
  }
  cache.delete(key)
  return null
}

function setInCache(key, data) {
  cache.set(key, {
    data,
    expires: Date.now() + CACHE_DURATION,
  })
}

export const apiClient = {
  async getCountries() {
    const key = 'countries'
    const cached = getFromCache(key)
    if (cached) return cached

    const response = await fetch(`${API_BASE}/countries`)
    if (!response.ok) throw new Error(`Failed to fetch countries: ${response.statusText}`)
    const data = await response.json()
    setInCache(key, data)
    return data
  },

  async getIndicators() {
    const key = 'indicators'
    const cached = getFromCache(key)
    if (cached) return cached

    const response = await fetch(`${API_BASE}/indicators`)
    if (!response.ok) throw new Error(`Failed to fetch indicators: ${response.statusText}`)
    const data = await response.json()
    setInCache(key, data)
    return data
  },

  async getCountryData(country, indicator) {
    const key = `country_${country}_${indicator}`
    const cached = getFromCache(key)
    if (cached) return cached

    const response = await fetch(
      `${API_BASE}/data/${country}/${indicator}`
    )
    if (!response.ok) throw new Error(`Failed to fetch country data: ${response.statusText}`)
    const data = await response.json()
    setInCache(key, data)
    return data
  },

  async compareCountries(countries, indicator) {
    const key = `compare_${countries.join(',')}_${indicator}`
    const cached = getFromCache(key)
    if (cached) return cached

    const response = await fetch(`${API_BASE}/compare`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ countries, indicator }),
    })
    if (!response.ok) throw new Error(`Failed to compare countries: ${response.statusText}`)
    const data = await response.json()
    setInCache(key, data)
    return data
  },

  clearCache() {
    cache.clear()
  },
}
