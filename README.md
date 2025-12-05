# BiliGet
简单的B站视频下载工具，可以免登录下载B站高清视频，灵感来自[share121/unidown](https://github.com/share121/unidown)

> [!WARNING]  
> 这个项目是个人学习Rust的练习项目，不适合在可靠性要求高的场景下使用，尽管它没有很大的问题

## 使用
在使用前，您需要安装[ffmpeg](https://ffmpeg.org/)命令行工具，并将其二进制程序添加至环境变量中。

```shell
./biliget [url]
```

## 待改进的功能

* 多线程下载
* 多视频平台下载
* 字幕、弹幕下载
* Github Action 自动构建与发布

由于本项目是我的第一个完整的Rust项目，因此代码质量可能不理想，欢迎提issue

## 致谢

* [share121/unidown](https://github.com/share121/unidown)

[share121](https://github.com/share121)是一个优秀的Rust开发者，让我领略到了Rust语言的魅力，并针对本项目提供了诸多建议。