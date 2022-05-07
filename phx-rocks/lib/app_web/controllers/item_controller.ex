defmodule AppWeb.ItemController do
  use AppWeb, :controller

  def index(conn, %{"girth" => girth, "depth" => depth}) do
    render(conn, "index.html",
      girth: String.to_integer(girth),
      depth: String.to_integer(depth)
    )
  end

  def index(conn, _params) do
    render(conn, "index.html", girth: 69, depth: 7)
  end
end
