import Config

# config/runtime.exs is executed for all environments, including
# during releases. It is executed after compilation and before the
# system starts, so it is typically used to load production configuration
# and secrets from environment variables or elsewhere. Do not define
# any compile-time configuration in here, as it won't be applied.
# The block below contains prod specific runtime configuration.

# Start the phoenix server if environment is set and running in a release
if System.get_env("PHX_SERVER") && System.get_env("RELEASE_NAME") do
  config :app, AppWeb.Endpoint, server: true
end

if config_env() == :prod do
  # The secret key base is used to sign/encrypt cookies and other secrets.
  # A default value is used in config/dev.exs and config/test.exs but you
  # want to use a different value for prod and you most likely don't want
  # to check this value into version control, so we use an environment
  # variable instead.
  # Normally this would not be hard-coded. However, this is a test application
  # and it doesn't matter.
  secret_key_base = "hQKMe1BbumSASasJfByEz7EYqZcvKgykgpeKkrEErg42lKICESBF3/06Afiy271x"

  host = System.get_env("PHX_HOST") || "0.0.0.0"
  port = String.to_integer(System.get_env("PORT") || "42069")

  config :app, AppWeb.Endpoint,
    check_origin: false,
    url: [host: host, port: port],
    http: [
      # Enable IPv6 and bind on all interfaces.
      # Set it to  {0, 0, 0, 0, 0, 0, 0, 1} for local network only access.
      # See the documentation on https://hexdocs.pm/plug_cowboy/Plug.Cowboy.html
      # for details about using IPv6 vs IPv4 and loopback vs public addresses.
      ip: {0, 0, 0, 0, 0, 0, 0, 0},
      port: port
    ],
    secret_key_base: secret_key_base

  # ## Using releases
  #
  # If you are doing OTP releases, you need to instruct Phoenix
  # to start each relevant endpoint:
  #
  #     config :app, AppWeb.Endpoint, server: true
  #
  # Then you can assemble a release by calling `mix release`.
  # See `mix help release` for more information.
end
