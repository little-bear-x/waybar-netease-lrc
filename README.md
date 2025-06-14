# 项目介绍

可在waybar上显示网易云音乐歌词
效果图片：

![](./docs/images/Docs.png)

支持的网易云音乐客户端

- ElectronNCM
- musicfox
- feeluown
- yesplaymusic
- Qcm
- NeteaseCloudMusicGtk4

完成测试的网易云音乐客户端

- NeteaseCloudMusicGtk4

# 项目参考

本项目参考 [waybar-netease-music-lyrics](https://github.com/kangxiaoju/waybar-netease-music-lyrics) 完成，使用rust重写了代码，降低系统内存占用的同时，将歌词获取间隔降低至400ms，提升歌词获取的实时性。

# 快速开始

0. 前置工作：安装并配置rust cargo工具链，git工具

1. 克隆项目：
   执行
   
   ```
   git clone https://github.com/little-bear-x/waybar-netease-lrc.git
   ```

2. 编译项目：
   执行
   
   ```
   cd /path.to.project/
   cargo build --release
   ```

3. 将可执行文件复制到一个你喜欢的位置
   执行
   
   ```
   cp /path.to.project/target/release/waybar-netease-lrc /path.to.waybar-netease-lrc
   ```

4. 配置waybar
   将
   
   ```
   "custom/song":{
   "exec": "~/.config/waybar/scripts/waybar-netease-lrc",
   "max-length": 40,
   "separate-outputs": true,
   },
   ```
   
   加入config中，并将`"custom/song"`加入到一个你希望歌词被展示的位置
