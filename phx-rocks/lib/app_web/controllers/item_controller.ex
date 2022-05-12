defmodule AppWeb.ItemController do
  use AppWeb, :controller

  def index(conn, %{"depth" => depth, "girth" => girth}) do
    {usec, rendered} =
      :timer.tc(Phoenix.View, :render_to_string, [AppWeb.ItemView, "index.html", [
        girth: String.to_integer(girth),
        depth: String.to_integer(depth)
      ]])

    conn
    |> put_resp_header("time-taken", Integer.to_string(usec))
    |> html(rendered)
  end
end
