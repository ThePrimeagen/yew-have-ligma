defmodule AppWeb.ItemController do
  use AppWeb, :controller

  def index(conn, %{"depth" => depth, "girth" => girth}) do
    render(conn, "index.html",
      girth: String.to_integer(girth),
      depth: String.to_integer(depth)
    )
  end
end
