# RustDesk Auto Config

一个用于自动安装和配置RustDesk的工具。

## 功能

- 自动安装RustDesk
- 自动配置服务器地址和密钥
- 通过TOML配置文件进行配置管理
- 自动重启RustDesk服务

## 配置

在运行程序之前，请编辑 `config.toml` 文件来设置您的配置：

```toml
# 安装文件名 (相对于当前目录或绝对路径)
installer_filename = "rustdesk.exe"

# 服务器地址 (包含端口)
server_address = "your-server.com:21116"

# 服务器主机 (不包含端口，用于custom-rendezvous-server选项)
server_host = "your-server.com"

# 密钥
key = "your-key-here"
```

## 使用方法

1. 将RustDesk安装文件放在项目目录中
2. 编辑 `config.toml` 文件，设置正确的配置参数
3. 运行程序：
   ```
   cargo run
   ```

## 注意事项

- 程序需要管理员权限运行
- 确保RustDesk安装文件存在且可访问
- 程序会自动停止现有的RustDesk进程并重新配置
