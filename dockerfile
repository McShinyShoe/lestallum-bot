FROM brainboxdotcc/dpp:latest

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y software-properties-common && rm -rf /var/lib/apt/lists/*
RUN add-apt-repository -y ppa:ubuntu-toolchain-r/test

RUN apt-get update && apt-get install --no-install-recommends -y g++-11

WORKDIR /app 

RUN echo a

COPY . .

RUN make all

CMD ./lestallum-bot