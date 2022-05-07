defmodule AppWeb.ItemView do
  use AppWeb, :view

  def render("index.html", assigns) do
    ~H"""
    <%= for i <- 0..31 do %>
      <%= render_item(Map.put(assigns, :bit, i)) %>
    <% end %>
    """
  end

  defp render_item(assigns) do
    ~H"""
    <div style={"border: 1px solid #DDD;width: #{@girth}px;height: 69px;background-color: #ff9500;"}>
      <%= render_inners(assigns) %>
    </div>
    """
  end

  # Recursively render the inner part of `item`, until `depth` is `0`.
  defp render_inners(%{depth: 0, bit: bit}), do: bit

  defp render_inners(assigns) do
    render_item(%{assigns | girth: assigns.girth - 3, depth: assigns.depth - 1})
  end
end
