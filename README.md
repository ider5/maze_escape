# 迷宫逃脱 (Maze Escape)

这是一个使用 Rust 编写的简单的命令行迷宫逃脱游戏。

## 游戏玩法

-   程序会随机生成一个迷宫。
-   玩家 `👨` 的目标是找到出口 `🚪`。
-   在寻找出口的路上，可以收集金币 `💰`。
-   游戏会记录你的移动步数和收集到的金币数量。

## 如何运行

1.  **确保你已经安装了 Rust**
    如果你还没有安装，可以访问 [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) 进行安装。

2.  **克隆或下载本项目**

3.  **运行游戏**
    在项目根目录下，执行以下命令：
    ```bash
    cargo run
    ```

4.  **开始游戏**
    -   使用 `W`, `A`, `S`, `D` 键进行移动。
    -   输入 `Q` 键退出游戏。

## 如何构建

如果你想构建一个可执行文件，可以运行：

```bash
cargo build --release
```

可执行文件将会出现在 `target/release/` 目录下。

## 如何测试

运行以下命令来执行单元测试：

```bash
cargo test
```
