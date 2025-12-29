import React, { useEffect, useState } from 'react'
import CountrySelector from './components/CountrySelector'
import IndicatorSelector from './components/IndicatorSelector'
import DataVisualization from './components/DataVisualization'
import Comparison from './components/Comparison'
import Favorites from './components/Favorites'

export default function App() {
  const [selectedCountries, setSelectedCountries] = useState([])
  const [selectedIndicator, setSelectedIndicator] = useState(null)
  const [indicators, setIndicators] = useState({})
  const [activeTab, setActiveTab] = useState('search') // search, comparison, favorites
  const [favorites, setFavorites] = useState(() => {
    const saved = localStorage.getItem('favorites')
    return saved ? JSON.parse(saved) : []
  })

  // Load indicators on mount
  useEffect(() => {
    const loadIndicators = async () => {
      try {
        const response = await fetch('/api/indicators')
        const data = await response.json()
        setIndicators(data)
      } catch (error) {
        console.error('Failed to load indicators:', error)
      }
    }
    loadIndicators()
  }, [])

  // Save favorites to localStorage
  useEffect(() => {
    localStorage.setItem('favorites', JSON.stringify(favorites))
  }, [favorites])

  const addFavorite = () => {
    if (selectedCountries.length > 0 && selectedIndicator) {
      const favorite = {
        id: Date.now(),
        countries: selectedCountries,
        indicator: selectedIndicator,
        timestamp: new Date().toLocaleString('uk-UA')
      }
      setFavorites([favorite, ...favorites.slice(0, 9)])
    }
  }

  const removeFavorite = (id) => {
    setFavorites(favorites.filter(f => f.id !== id))
  }

  const loadFavorite = (favorite) => {
    setSelectedCountries(favorite.countries)
    setSelectedIndicator(favorite.indicator)
    setActiveTab('search')
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>🌍 Світовий Аналітик</h1>
        <p>Економічні дані країн світу</p>
      </header>

      <nav className="tabs">
        <button
          className={`tab ${activeTab === 'search' ? 'active' : ''}`}
          onClick={() => setActiveTab('search')}
        >
          📊 Пошук
        </button>
        <button
          className={`tab ${activeTab === 'comparison' ? 'active' : ''}`}
          onClick={() => setActiveTab('comparison')}
        >
          🔄 Порівняння
        </button>
        <button
          className={`tab ${activeTab === 'favorites' ? 'active' : ''}`}
          onClick={() => setActiveTab('favorites')}
        >
          ⭐ Улюблені ({favorites.length})
        </button>
      </nav>

      <main className="app-main">
        {activeTab === 'search' && (
          <div className="search-tab">
            <CountrySelector
              selectedCountries={selectedCountries}
              onCountriesChange={setSelectedCountries}
            />

            <IndicatorSelector
              indicators={indicators}
              selectedIndicator={selectedIndicator}
              onIndicatorChange={setSelectedIndicator}
            />

            {selectedCountries.length > 0 && selectedIndicator && (
              <>
                <button className="btn btn-primary" onClick={addFavorite}>
                  ⭐ Додати в улюблені
                </button>
                <DataVisualization
                  countries={selectedCountries}
                  indicator={selectedIndicator}
                />
              </>
            )}
          </div>
        )}

        {activeTab === 'comparison' && (
          <Comparison
            countries={selectedCountries}
            indicator={selectedIndicator}
            indicators={indicators}
          />
        )}

        {activeTab === 'favorites' && (
          <Favorites
            favorites={favorites}
            onLoad={loadFavorite}
            onDelete={removeFavorite}
          />
        )}
      </main>
    </div>
  )
}
