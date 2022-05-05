import { useEffect, useState } from 'react';
import './App.css'
import createBuffer from './item/buffer';
import Item from './item/Item';

function App({
    size,
    depth,
    intervalMs
}: {depth: number, size: number, intervalMs: number}) {
    const buffer = createBuffer(size);
    const [renderCount, render] = useState(0);

    useEffect(function() {
        const id = setTimeout(function() {
            render(renderCount + 1);
        }, intervalMs);
        return function() {
            clearTimeout(id);
        };
    }, [intervalMs, renderCount]);

    const items = new Array(size).fill(0).map((_, i) => <Item
        girth={69}
        key={i}
        bit={i}
        depth={depth}
        buffer={buffer} />);

    return (
        <>
            {items}
        </>
    );
}

export default App
