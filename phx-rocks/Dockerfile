# We use Ubuntu/Debian instead of Alpine to avoid DNS resolution issues in production.
ARG ELIXIR_VERSION=1.13.3
ARG OTP_VERSION=24.2
ARG DEBIAN_VERSION=bullseye-20210902-slim
ARG BUILDER_IMAGE="hexpm/elixir:${ELIXIR_VERSION}-erlang-${OTP_VERSION}-debian-${DEBIAN_VERSION}"
ARG RUNNER_IMAGE="debian:${DEBIAN_VERSION}"

FROM ${BUILDER_IMAGE} as builder
RUN apt-get update -y && apt-get install -y build-essential git \
    && apt-get clean && rm -f /var/lib/apt/lists/*_*
WORKDIR /app
RUN mix local.hex --force && mix local.rebar --force
ENV MIX_ENV="prod"
COPY mix.exs mix.lock ./
RUN mix deps.get --only $MIX_ENV
RUN mkdir config
COPY config/config.exs config/${MIX_ENV}.exs config/
RUN mix deps.compile
COPY priv priv
COPY lib lib
COPY assets assets
RUN mix assets.deploy
RUN mix compile
COPY config/runtime.exs config/
COPY rel rel
RUN mix release

FROM ${RUNNER_IMAGE}
RUN apt-get update -y && apt-get install -y libstdc++6 openssl libncurses5 locales \
    && apt-get clean && rm -f /var/lib/apt/lists/*_*
RUN sed -i '/en_US.UTF-8/s/^# //g' /etc/locale.gen && locale-gen
ENV LANG en_US.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL en_US.UTF-8
WORKDIR "/app"
RUN chown nobody /app
ENV MIX_ENV="prod"
COPY --from=builder --chown=nobody:root /app/_build/${MIX_ENV}/rel/app ./
EXPOSE 42069
USER nobody
CMD ["/app/bin/server"]