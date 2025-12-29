import React, { useState } from 'react'

export default function IndicatorSelector({
  indicators,
  selectedIndicator,
  onIndicatorChange
}) {
  const [activeCategory, setActiveCategory] = useState('economic')

  const categories = {
    economic: { name: '📊 Економіка', color: '#3498db' },
    demographic: { name: '👥 Демографія', color: '#e74c3c' },
    social: { name: '🏫 Соціальні', color: '#2ecc71' },
    environmental: { name: '🌱 Екологія', color: '#27ae60' },
  }

  return (
    <div className="selector-card">
      <h2>📈 Вибір показника</h2>

      <div className="category-tabs">
        {Object.entries(categories).map(([key, category]) => (
          <button
            key={key}
            className={`category-tab ${activeCategory === key ? 'active' : ''}`}
            onClick={() => setActiveCategory(key)}
            style={{
              borderBottomColor:
                activeCategory === key ? category.color : 'transparent',
            }}
          >
            {category.name}
          </button>
        ))}
      </div>

      <div className="indicators-list">
        {indicators[activeCategory]?.map(indicator => (
          <button
            key={indicator.id}
            className={`indicator-item ${
              selectedIndicator?.id === indicator.id ? 'selected' : ''
            }`}
            onClick={() => onIndicatorChange(indicator)}
          >
            <div className="indicator-content">
              <div className="indicator-name">{indicator.name}</div>
              <div className="indicator-id">ID: {indicator.id}</div>
            </div>
            {selectedIndicator?.id === indicator.id && (
              <span className="checkmark">✓</span>
            )}
          </button>
        ))}
      </div>

      {selectedIndicator && (
        <div className="selected-indicator">
          <h3>Вибраний показник:</h3>
          <div className="indicator-preview">
            <div className="indicator-name">{selectedIndicator.name}</div>
            <div className="indicator-category">
              {categories[selectedIndicator.category].name}
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
