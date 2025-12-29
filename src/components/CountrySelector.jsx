import React, { useState, useEffect } from 'react'
import { apiClient } from '../utils/api'

export default function CountrySelector({ selectedCountries, onCountriesChange }) {
  const [countries, setCountries] = useState([])
  const [searchTerm, setSearchTerm] = useState('')
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    const loadCountries = async () => {
      try {
        const data = await apiClient.getCountries()
        setCountries(data)
      } catch (error) {
        console.error('Failed to load countries:', error)
      } finally {
        setLoading(false)
      }
    }
    loadCountries()
  }, [])

  const filteredCountries = countries.filter(c =>
    c.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
    c.code.toLowerCase().includes(searchTerm.toLowerCase())
  )

  const toggleCountry = (country) => {
    if (selectedCountries.find(c => c.code === country.code)) {
      onCountriesChange(selectedCountries.filter(c => c.code !== country.code))
    } else {
      if (selectedCountries.length < 3) {
        onCountriesChange([...selectedCountries, country])
      }
    }
  }

  const clearSelection = () => {
    onCountriesChange([])
  }

  return (
    <div className="selector-card">
      <h2>🌍 Вибір країни</h2>
      <p className="help-text">Оберіть до 3 країн для порівняння</p>

      <div className="search-box">
        <input
          type="text"
          placeholder="Пошук країни (назва або код)..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="search-input"
        />
      </div>

      {selectedCountries.length > 0 && (
        <div className="selected-items">
          <h3>Обрані країни:</h3>
          <div className="item-list">
            {selectedCountries.map(country => (
              <div key={country.code} className="selected-item">
                <span>{country.name}</span>
                <button
                  onClick={() => toggleCountry(country)}
                  className="btn-remove"
                  title="Видалити"
                >
                  ✕
                </button>
              </div>
            ))}
          </div>
          <button onClick={clearSelection} className="btn btn-secondary">
            Очистити все
          </button>
        </div>
      )}

      <div className="countries-list">
        {loading ? (
          <p className="loading">Завантаження...</p>
        ) : filteredCountries.length === 0 ? (
          <p className="no-results">Країни не знайдені</p>
        ) : (
          filteredCountries.map(country => (
            <button
              key={country.code}
              className={`country-item ${
                selectedCountries.find(c => c.code === country.code)
                  ? 'selected'
                  : ''
              } ${selectedCountries.length >= 3 && !selectedCountries.find(c => c.code === country.code) ? 'disabled' : ''}`}
              onClick={() => toggleCountry(country)}
              disabled={
                selectedCountries.length >= 3 &&
                !selectedCountries.find(c => c.code === country.code)
              }
            >
              <div className="country-info">
                <div className="country-name">{country.name}</div>
                <div className="country-details">
                  {country.region} • {country.code}
                </div>
              </div>
              {selectedCountries.find(c => c.code === country.code) && (
                <span className="checkmark">✓</span>
              )}
            </button>
          ))
        )}
      </div>
    </div>
  )
}
