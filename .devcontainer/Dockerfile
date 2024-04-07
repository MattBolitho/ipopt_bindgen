FROM ubuntu:22.04

RUN export DEBIAN_FRONTEND=noninteractive &&\
  apt update && \
  apt install --no-install-recommends -y \
  gfortran \
  build-essential \
  binutils \
  lsb-release \
  software-properties-common \
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
  apt-utils \
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
ARG LLVM_VER="18"
RUN wget https://apt.llvm.org/llvm.sh && \
  chmod +x llvm.sh && \
  sudo ./llvm.sh ${LLVM_VER} clang-${LLVM_VER} clang++-${LLVM_VER} && \
  apt autoremove -y && apt clean && rm -rf /var/lib/apt/lists/*

# Update alternatives with installed LLVM tools.
RUN update-alternatives --install /usr/bin/clang clang $(which clang-${LLVM_VER}) 100 && \
  update-alternatives --install /usr/bin/clang++ clang++ $(which clang++-${LLVM_VER}) 100

# Set complier environment.
ENV CC=clang
ENV CXX=clang++
ENV FC=gfortran
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
ARG IPOPT_VER="3.14.14"
RUN wget https://github.com/coin-or/Ipopt/archive/refs/tags/releases/${IPOPT_VER}.tar.gz && \
  tar -xvf ${IPOPT_VER}.tar.gz && \
  cd Ipopt-releases-${IPOPT_VER} && \
  mkdir build && \
  cd build && \
  ../configure && \
  make -j`nproc` && \
  make install && \
  cd ../.. && \
  rm -rf Ipopt*

# Install Rust for the container user.
ARG USERNAME=user
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Create the user.
# https://code.visualstudio.com/remote/advancedcontainers/add-nonroot-user
RUN groupadd --gid $USER_GID $USERNAME && \
  useradd --uid $USER_UID --gid $USER_GID -m $USERNAME && \
  echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME && \
  usermod -aG root $USERNAME && \
  chmod 0440 /etc/sudoers.d/$USERNAME

USER ${USERNAME}
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/${USERNAME}/.cargo/bin:${PATH}"
RUN rustup default stable
