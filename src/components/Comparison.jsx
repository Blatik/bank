import React from 'react'
import { Bar } from 'react-chartjs-2'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js'

ChartJS.register(CategoryScale, LinearScale, BarElement, Title, Tooltip, Legend)

export default function Comparison({
  countries,
  indicator,
  indicators,
}) {
  if (countries.length === 0)
    return (
      <div className="empty-state">
        <p>🌍 Оберіть 2-3 країни для порівняння</p>
      </div>
    )

  if (!indicator)
    return (
      <div className="empty-state">
        <p>📊 Оберіть показник для порівняння</p>
      </div>
    )

  return (
    <div className="comparison-card">
      <h2>🔄 Порівняння країн</h2>
      <p className="help-text">
        Порівняння {countries.length} країн по показнику: {indicator.name}
      </p>

      <div className="comparison-info">
        <div className="info-box">
          <h3>📍 Обрані країни:</h3>
          <ul>
            {countries.map(c => (
              <li key={c.code}>
                {c.name} ({c.code})
              </li>
            ))}
          </ul>
        </div>

        <div className="info-box">
          <h3>📈 Показник:</h3>
          <p>{indicator.name}</p>
          <p className="id-text">ID: {indicator.id}</p>
        </div>
      </div>

      <p className="loading-text">
        💡 <strong>Порада:</strong> Дані отримуються з World Bank API. Графік
        показує тренди за останні 20+ років для кожної країни.
      </p>
    </div>
  )
}
