import React, { useEffect, useState } from 'react'
import { Line } from 'react-chartjs-2'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler,
} from 'chart.js'
import { apiClient } from '../utils/api'

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler
)

const colors = [
  '#3498db',
  '#e74c3c',
  '#2ecc71',
  '#f39c12',
  '#9b59b6',
  '#1abc9c',
]

export default function DataVisualization({ countries, indicator }) {
  const [data, setData] = useState(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState(null)

  useEffect(() => {
    const loadData = async () => {
      setLoading(true)
      setError(null)
      try {
        if (countries.length === 1) {
          const chartData = await apiClient.getCountryData(
            countries[0].code,
            indicator.id
          )
          setData({ [countries[0].code]: chartData })
        } else {
          const chartData = await apiClient.compareCountries(
            countries.map(c => c.code),
            indicator.id
          )
          setData(chartData)
        }
      } catch (err) {
        setError(err.message)
        console.error('Failed to load data:', err)
      } finally {
        setLoading(false)
      }
    }

    loadData()
  }, [countries, indicator])

  if (loading)
    return <div className="loading-container">📊 Завантаження даних...</div>
  if (error)
    return <div className="error-container">❌ Помилка: {error}</div>
  if (!data)
    return <div className="error-container">📭 Дані не знайдені</div>

  const years = Array.from(
    new Set(
      Object.values(data)
        .flat()
        .map(d => d.year)
    )
  ).sort()

  const datasets = Object.entries(data).map(([countryCode, countryData], idx) => {
    const countryName =
      countries.find(c => c.code === countryCode)?.name || countryCode
    return {
      label: countryName,
      data: years.map(year => {
        const point = countryData.find(d => d.year === year)
        return point ? point.value : null
      }),
      borderColor: colors[idx % colors.length],
      backgroundColor: colors[idx % colors.length] + '22',
      tension: 0.4,
      fill: false,
      pointRadius: 3,
      pointHoverRadius: 6,
    }
  })

  const chartData = {
    labels: years,
    datasets,
  }

  const options = {
    responsive: true,
    maintainAspectRatio: true,
    plugins: {
      legend: {
        display: true,
        position: 'top',
        labels: {
          font: { size: 12 },
          padding: 15,
          usePointStyle: true,
        },
      },
      title: {
        display: true,
        text: indicator.name,
        font: { size: 16, weight: 'bold' },
        padding: 20,
      },
      tooltip: {
        backgroundColor: 'rgba(0,0,0,0.8)',
        padding: 12,
        titleFont: { size: 13 },
        bodyFont: { size: 12 },
        borderColor: '#ddd',
        borderWidth: 1,
        callbacks: {
          label: function (context) {
            const value = context.parsed.y
            if (value === null) return 'Немає даних'
            return context.dataset.label + ': ' + value.toFixed(2)
          },
        },
      },
    },
    scales: {
      x: {
        grid: { display: false },
        title: { display: true, text: 'Роки' },
      },
      y: {
        title: { display: true, text: 'Значення' },
        beginAtZero: true,
      },
    },
  }

  return (
    <div className="visualization-card">
      <div className="chart-container">
        <Line data={chartData} options={options} />
      </div>

      <div className="data-stats">
        <h3>📊 Статистика</h3>
        <div className="stats-grid">
          {Object.entries(data).map(([countryCode, countryData]) => {
            const countryName =
              countries.find(c => c.code === countryCode)?.name || countryCode
            const values = countryData.map(d => d.value).filter(v => v !== null)
            if (values.length === 0) return null

            const latest = countryData[countryData.length - 1]?.value || 'N/A'
            const earliest = countryData[0]?.value || 'N/A'
            const change =
              typeof latest === 'number' && typeof earliest === 'number'
                ? ((latest - earliest) / earliest * 100).toFixed(1)
                : 'N/A'

            return (
              <div key={countryCode} className="stat-box">
                <h4>{countryName}</h4>
                <div className="stat-row">
                  <span>Поточне:</span>
                  <strong>{parseFloat(latest).toFixed(2)}</strong>
                </div>
                <div className="stat-row">
                  <span>Початкове:</span>
                  <strong>{parseFloat(earliest).toFixed(2)}</strong>
                </div>
                <div className={`stat-row ${change > 0 ? 'positive' : 'negative'}`}>
                  <span>Зміна:</span>
                  <strong>
                    {change > 0 ? '+' : ''}
                    {change}%
                  </strong>
                </div>
              </div>
            )
          })}
        </div>
      </div>
    </div>
  )
}
