FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

RUN apt-get update && \
    # Install gnuplot for Criterion benchmarks ploting
    #apt-get install -y gnuplot \
    apt-get install -y python3-pip
    
RUN pip install ipykernel

# Install Rust Jupyter kernel
USER vscode

RUN rustup component add rust-src && \ 
    cargo install --locked evcxr_jupyter && \
    evcxr_jupyter --install

USER root