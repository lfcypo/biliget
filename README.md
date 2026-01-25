# BiliGet

简单的 B 站视频下载工具 可以免登录下载 B 站高清视频

灵感来自[share121/unidown](https://github.com/share121/unidown).

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

在 Linux 系统上，您也可以使用您喜爱的方式进行安装。

### 下载

默认模式：自动合并音视频为一个`.mp4`视频文件

```shell
./biliget [url]
```

仅下载音频：保存为`.wav`音频文件

```shell
./biliget [url] -a
```

## 编译与构建

GitHub Action 会自动编译构建并发布至[Release页面](https://github.com/lfcypo/biliget/releases)

如您需要自行编译，请参考以下指南：

请确保您安装并配置了[Rust](https://rust-lang.org/)语言开发工具链。

```shell
git clone https://github.com/lfcypo/biliget.git
cd biliget
cargo update
cargo build --release
```

然后 您可以在`target/release/`目录下找到您编译的可执行文件 `biliget`

## 待实现的功能

- :white_check_mark: GitHub Action 自动构建与发布
- 多线程下载
- 多视频平台下载
- 字幕、弹幕下载

## 常见问题

### 命令执行出错

```text
zsh: parse error near `&'
```

请把`[url]`的部分使用英文双引号包裹

### 找不到`ffmpeg`命令

请参照[使用前准备](https://github.com/lfcypo/biliget?tab=readme-ov-file#%E4%BD%BF%E7%94%A8%E5%89%8D)正确安装`ffmpeg`

## 致谢

- [share121/unidown](https://github.com/share121/unidown)

[share121](https://github.com/share121)是一个优秀的 Rust 开发者，让我领略到了 Rust 语言的魅力，并针对本项目提供了诸多建议。
