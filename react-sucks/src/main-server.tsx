import { renderToString } from "react-dom/server";

import App from "./App"
import React from "react"

export default function render(size: number, depth: number) {
    return renderToString(
        <React.StrictMode>
            <App
                depth={depth}
                size={size}
                intervalMs={3}
            />
        </React.StrictMode>);
}
