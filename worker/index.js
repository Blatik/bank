import { Router } from 'itty-router'

const router = Router()

// World Bank API base URL
const WB_API = 'https://api.worldbank.org/v2'

// CORS headers
const corsHeaders = {
  'Access-Control-Allow-Origin': '*',
  'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
  'Access-Control-Allow-Headers': 'Content-Type',
  'Content-Type': 'application/json',
}

// Handle CORS preflight
router.options('*', () => new Response(null, { headers: corsHeaders }))

// Get list of countries
router.get('/api/countries', async () => {
  try {
    const url = `${WB_API}/country?format=json&per_page=500`
    const response = await fetch(url)
    const data = await response.json()
    
    const countries = (data[1] || [])
      .filter(c => c.capitalCity)
      .map(c => ({
        id: c.id,
        name: c.name,
        region: c.region?.value,
        capital: c.capitalCity,
        code: c.id
      }))
    
    return new Response(JSON.stringify(countries), {
      headers: corsHeaders
    })
  } catch (error) {
    return new Response(JSON.stringify({ error: error.message }), {
      status: 500,
      headers: corsHeaders
    })
  }
})

// Get indicators
router.get('/api/indicators', async () => {
  const indicators = {
    economic: [
      { id: 'NY.GDP.MKTP.CD', name: 'ВВП (поточна США $)', category: 'economic' },
      { id: 'NY.GDP.PCAP.CD', name: 'ВВП на душу населення (США $)', category: 'economic' },
      { id: 'FP.CPI.TOTL.ZG', name: 'Інфляція, ціни на споживача (% річних)', category: 'economic' },
      { id: 'GC.DOD.TOTL.GD.ZS', name: 'Державний борг (% ВВП)', category: 'economic' },
    ],
    demographic: [
      { id: 'SP.POP.TOTL', name: 'Населення, всього', category: 'demographic' },
      { id: 'SP.URB.TOTL.IN.ZS', name: 'Міське населення (% від усього)', category: 'demographic' },
      { id: 'SP.DYN.CBRT.IN', name: 'Народжуваність (на 1000 чол.)', category: 'demographic' },
      { id: 'SP.DYN.CDRT.IN', name: 'Смертність (на 1000 чол.)', category: 'demographic' },
    ],
    social: [
      { id: 'SE.ADT.LITR.ZS', name: 'Грамотність дорослого населення (% від 15+)', category: 'social' },
      { id: 'NY.ADJ.AEDU.GN.PER', name: 'Середня тривалість навчання (років)', category: 'social' },
      { id: 'SL.UEM.TOTL.ZS', name: 'Безробіття (% від загальної робочої сили)', category: 'social' },
      { id: 'EG.ELC.ACCS.ZS', name: 'Доступ до електрики (% населення)', category: 'social' },
    ],
    environmental: [
      { id: 'EN.ATM.CO2E.KT', name: 'Викиди CO₂ (млн т)', category: 'environmental' },
      { id: 'EN.ATM.CO2E.PC', name: 'Викиди CO₂ на душу населення (т)', category: 'environmental' },
      { id: 'EG.USE.COMM.FO.ZS', name: 'Enергія від викопного палива (% від енергоспоживання)', category: 'environmental' },
      { id: 'AG.LND.FRST.ZS', name: 'Лісова площа (% від земельної площі)', category: 'environmental' },
    ]
  }
  
  return new Response(JSON.stringify(indicators), { headers: corsHeaders })
})

// Get country data for specific indicator
router.get('/api/data/:country/:indicator', async (req) => {
  try {
    const { country, indicator } = req.params
    const url = `${WB_API}/country/${country}/indicator/${indicator}?format=json&per_page=60&date=2000:2024`
    
    const response = await fetch(url)
    const data = await response.json()
    
    if (!data[1]) {
      return new Response(JSON.stringify([]), { headers: corsHeaders })
    }
    
    const values = data[1]
      .filter(d => d.value !== null)
      .map(d => ({
        year: parseInt(d.date),
        value: parseFloat(d.value),
        country: d.countryiso3code
      }))
      .sort((a, b) => a.year - b.year)
    
    return new Response(JSON.stringify(values), { headers: corsHeaders })
  } catch (error) {
    return new Response(JSON.stringify({ error: error.message }), {
      status: 500,
      headers: corsHeaders
    })
  }
})

// Get multiple countries data
router.post('/api/compare', async (req) => {
  try {
    const { countries, indicator } = await req.json()
    
    const promises = countries.map(country =>
      fetch(`${WB_API}/country/${country}/indicator/${indicator}?format=json&per_page=60&date=2000:2024`)
        .then(r => r.json())
    )
    
    const results = await Promise.all(promises)
    
    const data = {}
    countries.forEach((country, idx) => {
      const countryData = results[idx]
      if (countryData[1]) {
        data[country] = countryData[1]
          .filter(d => d.value !== null)
          .map(d => ({
            year: parseInt(d.date),
            value: parseFloat(d.value)
          }))
          .sort((a, b) => a.year - b.year)
      }
    })
    
    return new Response(JSON.stringify(data), { headers: corsHeaders })
  } catch (error) {
    return new Response(JSON.stringify({ error: error.message }), {
      status: 500,
      headers: corsHeaders
    })
  }
})

// Health check
router.get('/api/health', () => {
  return new Response(JSON.stringify({ status: 'ok' }), { headers: corsHeaders })
})

// 404
router.all('*', () => {
  return new Response('Not Found', { status: 404, headers: corsHeaders })
})

export default {
  fetch: router.handle
}
