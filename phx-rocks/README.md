# App

## Dev

Since it's very simple, all the work is done in this module:

 * [`phx-rocks/lib/app_web/live/items_live.ex`](https://github.com/ThePrimeagen/yew-have-ligma/blob/master/phx-rocks/lib/app_web/live/items_live.ex)

### To start your Phoenix server (with Elixir installed):

  * Install dependencies with `mix deps.get`
  * Start Phoenix endpoint with `mix phx.server` or inside IEx with `iex -S mix phx.server`

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.

### To start your Phoenix server (without Elixir installed):

  * Run `docker build -t e .`
  * Run `docker run -p 4000:4000 e`

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.

## Explanation

### Server-side rendering (SSR)

When the page is requested, the initial page render will be SSR.

That means it will first call `mount/3`, assign the initial values, and then
call `render/1`.

This will render the 32 `item` component stacks. It will work without JS
up until this point to render the initial HTML with the components in their
initial state.

### JavaScript DOM patching

After the initial SSR page load, a websocket connection is established.

The `interval` that is updating the `count` every 3ms will cause data
representing that change to be sent over the websocket to the browser/client
(at every interval).

This is when the DOM is then patched to match the updated data using a JS
library that is included with Phoenix Live View.