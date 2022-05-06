defmodule AppWeb.ItemsLive do
  use AppWeb, :live_view

  import Bitwise

  @interval_ms 3

  @impl true
  def mount(_params, _session, socket) do
    # Send a `:tick` message to this process at the specified interval.
    :timer.send_interval(@interval_ms, self(), :tick)

    {:ok, assign(socket,
      girth: 69,
      depth: 7,
      count: 0
    )}
  end

  @impl true
  def handle_info(:tick, socket) do
    {:noreply, assign(socket, :count, socket.assigns.count + 1)}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div style="background-color: #555; width: 100%; height: 100%; position: absolute;">
      <%= for i <- 0..31 do %>
        <.item
          girth={@girth}
          depth={@depth}
          count={@count}
          bit={i}
        />
      <% end %>
    </div>
    """
  end

  defp item(assigns) do
    ~H"""
    <div
      style={"border: 1px solid #DDD;width: #{@girth}px;height: 69px;background-color: #{color(assigns)};"}
    >
      <%= render_inners(assigns) %>
    </div>
    """
  end

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
