import React from 'react'

export default function Favorites({ favorites, onLoad, onDelete }) {
  if (favorites.length === 0)
    return (
      <div className="empty-state">
        <p>⭐ У вас ще немає улюблених запитів</p>
        <p className="help-text">
          Встав в улюблені ваші найчастіше використовувані комбінації країн та
          показників
        </p>
      </div>
    )

  return (
    <div className="favorites-container">
      <h2>⭐ Улюблені запити</h2>
      <div className="favorites-list">
        {favorites.map((fav, idx) => (
          <div key={fav.id} className="favorite-item">
            <div className="favorite-content">
              <span className="favorite-number">#{idx + 1}</span>
              <div className="favorite-details">
                <div className="favorite-countries">
                  {fav.countries.map(c => c.name).join(', ')}
                </div>
                <div className="favorite-indicator">
                  {fav.indicator.name}
                </div>
                <div className="favorite-time">{fav.timestamp}</div>
              </div>
            </div>
            <div className="favorite-actions">
              <button
                className="btn btn-small btn-primary"
                onClick={() => onLoad(fav)}
                title="Завантажити"
              >
                📊 Завантажити
              </button>
              <button
                className="btn btn-small btn-danger"
                onClick={() => onDelete(fav.id)}
                title="Видалити"
              >
                🗑️ Видалити
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}
