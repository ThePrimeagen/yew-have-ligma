// TODO: I know i don't have to include them, but it seems to break on docker.
import React from 'react';

import { useEffect, useState } from 'react';
import './App.css'
import Item from './item/Item';

function App({
    size,
    depth,
    intervalMs
}: {depth: number, size: number, intervalMs: number}) {
    const [renderCount, render] = useState(0);

    useEffect(function() {
        const id = setTimeout(function() {
            render(renderCount + 1);
        }, intervalMs);
        return function() {
            clearTimeout(id);
        };
    }, [intervalMs, renderCount]);

    const items = new Array(32).fill(0).map((_, i) => <Item
        girth={size}
        key={i}
        bit={i}
        depth={depth}
        value={renderCount} />);

    return (
        <>
            {items}
        </>
    );
}

export default App
