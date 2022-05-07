defmodule AppWeb.ItemsLive do
  @moduledoc """
  All of the "work" is handled in this module.

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
  """
  use AppWeb, :live_view

  import Bitwise

  @interval_ms 3

  @impl true
  def mount(_params, _session, socket) do
    # Send this process (`self`), a `:throb` message at the specified interval.
    :timer.send_interval(@interval_ms, self(), :throb)
    {:ok, socket}
  end

  @impl true
  def handle_params(%{"girth" => girth, "depth" => depth}, _uri, socket) do
    {:noreply, assign(socket,
      girth: String.to_integer(girth),
      depth: String.to_integer(depth),
      count: 0
    )}
  end

  def handle_params(_params, _uri, socket) do
    {:noreply, assign(socket,
      girth: 69,
      depth: 7,
      count: 0
    )}
  end

  # This is handling of an asynchronous message sent to this process from
  # `:timer.send_interval/3`.
  # When we update socket assigns, the client will get a message with changes in
  # order to patch the DOM.
  @impl true
  def handle_info(:throb, socket) do
    {:noreply, assign(socket, :count, socket.assigns.count + 1)}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <%= for i <- 0..31 do %>
      <.item
        girth={@girth}
        depth={@depth}
        count={@count}
        bit={i}
      />
    <% end %>
    """
  end

  # This is the `item` function component.
  defp item(assigns) do
    ~H"""
    <div style={"border: 1px solid #DDD;width: #{@girth}px;height: 69px;background-color: #{color(assigns)};"}>
      <%= render_inners(assigns) %>
    </div>
    """
  end

  # Recursively render the inner part of `item`, until `depth` is `0`.
  defp render_inners(%{depth: 0, bit: bit}), do: bit

  defp render_inners(assigns) do
    ~H"""
    <.item
      girth={@girth - 3}
      depth={@depth - 1}
      count={@count}
      bit={@bit}
    />
    """
  end

  defp color(%{bit: bit, depth: depth, count: count}) do
    if is_even?(depth) == has_background?(count, bit), do: "#0385ff", else: "#ff9500"
  end

  defp has_background?(count, bit) do
    mask = 1 <<< max(0, bit - 1)
    (count &&& mask) > 0
  end

  defp is_even?(depth) do
    (depth &&& 1) != 1
  end
end
