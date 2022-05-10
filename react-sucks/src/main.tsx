import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'

ReactDOM.createRoot(document.body!).render(
  <React.StrictMode>
    <App
        depth={7}
        size={69}
        intervalMs={3}
    />
  </React.StrictMode>
)
