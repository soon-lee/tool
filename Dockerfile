# 第一阶段：构建阶段
FROM rust:latest as builder

# 设置工作目录
WORKDIR /usr/src/novel

# 复制 Cargo.toml 和 Cargo.lock 文件
COPY . ./

# 创建目标目录
RUN mkdir /usr/src/novel/target

# 配置国内镜像源
RUN mkdir -p .cargo && \
    echo '[source.crates-io]' > .cargo/config.toml && \
    echo 'registry = "https://github.com/rust-lang/crates.io-index"' >> .cargo/config.toml && \
    echo 'replace-with = "tuna"' >> .cargo/config.toml && \
    echo '' >> .cargo/config.toml && \
    echo '[source.tuna]' >> .cargo/config.toml && \
    echo 'registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"' >> .cargo/config.toml

# 构建项目
RUN cargo build --release

# 第二阶段：运行阶段
FROM debian:stable-slim

# 设置工作目录
WORKDIR /app

# 从构建阶段复制可执行文件
COPY --from=builder /usr/src/novel/target/release/novel .

# 设置启动命令
CMD ["./novel"]