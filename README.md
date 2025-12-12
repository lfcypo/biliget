# BiliGet

简单的 B 站视频下载工具 可以免登录下载 B 站高清视频

灵感来自[share121/unidown](https://github.com/share121/unidown).

> [!WARNING]  
> 这个项目是个人学习 Rust 的练习项目，不适合在可靠性要求高的场景下使用，尽管它没有很大的问题。

## 编译

请确保您安装并配置了[Rust](https://rust-lang.org/)语言开发工具链。

```shell
git clone https://github.com/lfcypo/biliget.git
cd biliget
cargo update
cargo build --release
```

然后 您可以在`target/release/`目录下找到您编译的可执行文件 `biliget`

## 使用

### 使用前

在使用前，您需要安装[FFmpeg](https://ffmpeg.org/)命令行工具，并将其二进制程序添加至环境变量中。

对于 Windows 系统，您可以使用[Scoop](https://scoop.sh/)。

```shell
scoop install ffmpeg
```

对于 macOS 系统，您可以使用[HomeBrew](https://brew.sh/)。

```shell
brew install ffmpeg
```

### 下载

默认模式：自动合并音视频为一个`.mp4`视频文件
```shell
./biliget [url]
```

仅下载音频：保存为`.wav`音频文件
```shell
./biliget [url] -a
```

## 待实现的功能

- 多线程下载
- 多视频平台下载
- 字幕、弹幕下载
- Github Action 自动构建与发布

由于本项目是我的第一个完整的 Rust 项目，因此代码质量可能不理想，欢迎提 issue。


## 常见问题

命令执行出错
```text
zsh: parse error near `&'
```
请把`[url]`的部分使用英文双引号包裹

## 致谢

- [share121/unidown](https://github.com/share121/unidown)

[share121](https://github.com/share121)是一个优秀的 Rust 开发者，让我领略到了 Rust 语言的魅力，并针对本项目提供了诸多建议。
