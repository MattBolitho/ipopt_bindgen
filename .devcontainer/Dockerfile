FROM ubuntu:24.04

RUN export DEBIAN_FRONTEND=noninteractive &&\
  apt update && \
  apt install --no-install-recommends -y \
  gfortran-14 \
  binutils \
  zip \
  unzip \
  wget \
  curl \
  ninja-build \
  git \
  pkg-config \
  patch \
  valgrind \
  sudo \
  file \
  openssh-client \
  gnupg \
  gpg-agent \
  socat \
  rsync \
  ninja-build \
  libzstd-dev \
  libedit-dev \
  linux-libc-dev \
  liblapack-dev \
  libmetis-dev \
  libfontconfig \
  libfontconfig1-dev \
  python3 \
  python3-pip && \
  apt autoremove -y && apt clean && rm -rf /var/lib/apt/lists/*

# Install LLVM.
ARG LLVM_VER="19"
RUN apt update && \
  apt install -y --no-install-recommends \
  software-properties-common \
  apt-utils \
  build-essential \
  lsb-release && \
  wget https://apt.llvm.org/llvm.sh && \
  chmod +x llvm.sh && \
  ./llvm.sh ${LLVM_VER} && \
  export DEBIAN_FRONTEND=noninteractive && \
  apt install -y --no-install-recommends \
  clang-${LLVM_VER} \
  libc++-${LLVM_VER}-dev \
  libc++abi-${LLVM_VER}-dev \
  flang-${LLVM_VER} && \
  apt autoremove -y && apt clean && rm -rf /var/lib/apt/lists/*

# Update alternatives with installed LLVM tools.
RUN update-alternatives --install /usr/bin/clang clang $(which clang-${LLVM_VER}) 100 && \
  update-alternatives --install /usr/bin/clang++ clang++ $(which clang++-${LLVM_VER}) 100

# Set complier environment.
ENV CC=clang
ENV CXX=clang++
ENV FC=gfortran-14
ENV LIBRARY_PATH=/usr/local/lib
ENV LD_LIBRARY_PATH=/usr/local/lib

# Install the MUMPS library for IPOPT linear solver support.
RUN git clone https://github.com/coin-or-tools/ThirdParty-Mumps.git && \
  cd ThirdParty-Mumps && \
  ./get.Mumps && \
  ./configure && \
  make -j`nproc` && \
  make install && \
  cd ../ && \
  rm -rf ThirdParty-Mumps

# Install Ipopt.
ARG IPOPT_VER="3.14.17"
RUN wget https://github.com/coin-or/Ipopt/archive/refs/tags/releases/${IPOPT_VER}.tar.gz && \
  tar -xzf ${IPOPT_VER}.tar.gz && \
  cd Ipopt-releases-${IPOPT_VER} && \
  mkdir build && \
  cd build && \
  ../configure \
  CXXFLAGS="-O3 -stdlib=libc++ -flto=thin" \
  --disable-sipopt && \
  make -j$(nproc) && \
  make install && \
  cd ../.. && \
  rm -rf Ipopt* ${IPOPT_VER}*

# Install Rust for the container user.
ARG USERNAME=user
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Create the user.
# https://code.visualstudio.com/remote/advancedcontainers/add-nonroot-user
ARG USERNAME=user
ARG USER_UID=1234
ARG USER_GID=$USER_UID
RUN groupadd --gid ${USER_GID} ${USERNAME} && \
  apt update && \
  apt install -y sudo && \
  useradd --uid ${USER_UID} --gid ${USER_GID} --create-home --home-dir /home/${USERNAME} --shell /bin/bash ${USERNAME} && \
  echo ${USERNAME} ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/${USERNAME} && \
  chmod 0440 /etc/sudoers.d/${USERNAME}

USER ${USERNAME}
ENV HOME=/home/${USERNAME}
WORKDIR ${HOME}

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
  . "$HOME/.cargo/env" && \
  rustup component add rustfmt
