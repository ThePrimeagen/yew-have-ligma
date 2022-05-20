import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'

const params = window.location.toString().split("render/")[1];
const parts = params.split("/");
const depth = +parts[0];
const girth = +parts[1];
ReactDOM.createRoot(document.body!).render(
  <React.StrictMode>
    <App
        depth={depth}
        size={girth}
        intervalMs={100}
    />
  </React.StrictMode>
)
