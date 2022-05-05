import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App
        depth={7}
        size={69}
        intervalMs={250}
    />
  </React.StrictMode>
)
