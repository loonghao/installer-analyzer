# 自动更新指南

本指南说明如何使用 installer-analyzer 的自动更新功能。

## 概述

installer-analyzer 包含内置的自动更新功能，允许您：
- 自动检查新版本
- 通过单个命令下载和安装更新
- 保持工具与最新功能和安全修复同步

## 快速开始

### 检查更新

```bash
# 检查是否有可用更新（不安装）
installer-analyzer update --check-only
```

此命令将：
- 连接到 GitHub 检查最新发布版本
- 与您当前版本进行比较
- 显示版本信息和发布说明
- 不安装任何内容直接退出

### 安装更新

```bash
# 安装可用更新
installer-analyzer update
```

此命令将：
- 检查更新
- 提示确认（除非使用 `--yes`）
- 下载最新版本
- 替换当前可执行文件
- 使用新版本重启

### 强制更新

```bash
# 即使当前版本较新也强制更新
installer-analyzer update --force
```

使用此选项可以：
- 降级到特定版本
- 重新安装当前版本
- 覆盖版本检查

### 静默更新

```bash
# 无提示更新（用于自动化）
installer-analyzer update --yes
```

适用于：
- 自动化脚本
- CI/CD 流水线
- 计划更新

## 命令选项

| 选项 | 描述 | 示例 |
|------|------|------|
| `--check-only` | 仅检查更新，不安装 | `installer-analyzer update --check-only` |
| `--force` | 即使当前版本较新也强制更新 | `installer-analyzer update --force` |
| `--yes`, `-y` | 跳过确认提示 | `installer-analyzer update -y` |

## 更新过程

### 1. 版本检查
- 连接到 GitHub API
- 获取最新发布信息
- 使用语义版本控制比较版本
- 显示当前版本与最新版本

### 2. 下载
- 从 GitHub Releases 下载 Windows 可执行文件
- 使用 SHA256 校验和验证文件完整性
- 显示下载进度、速度和预计时间

### 3. 安装
- **Windows**: 使用平台特定的更新策略
  - 如果可能直接替换
  - 使用临时文件就地更新
  - 如需要提示提升权限

### 4. 验证
- 验证新可执行文件正常工作
- 显示新版本的成功消息

## 更新策略（Windows）

installer-analyzer 根据您的环境使用不同的更新策略：

### 直接更新
- **时机**: 可执行文件位于可写位置
- **过程**: 直接文件替换
- **要求**: 对可执行文件目录有写权限

### 就地更新
- **时机**: 写权限受限
- **过程**: 使用临时文件分阶段更新
- **要求**: 临时目录访问权限

### 需要提升权限
- **时机**: 可执行文件位于受保护位置（如 Program Files）
- **过程**: 提示管理员权限
- **要求**: 用户批准权限提升

## 故障排除

### 常见问题

**"权限被拒绝"错误**
```
解决方案：以管理员身份运行或将可执行文件移动到用户目录
命令：右键点击命令提示符 → "以管理员身份运行"
```

**"网络错误"或"连接失败"**
```
解决方案：检查网络连接和 GitHub 可访问性
测试：尝试访问 https://github.com/loonghao/installer-analyzer/releases
```

**下载后"更新失败"**
```
解决方案：关闭所有 installer-analyzer 实例并重试
命令：taskkill /f /im installer-analyzer.exe
```

**"版本检查失败"**
```
解决方案：验证 GitHub API 可访问
测试：curl https://api.github.com/repos/loonghao/installer-analyzer/releases/latest
```

### 调试信息

启用详细输出进行故障排除：

```bash
# 显示详细更新过程
installer-analyzer update --verbose

# 检查当前版本和构建信息
installer-analyzer --version

# 验证可执行文件位置和权限
where installer-analyzer
```

## 安全考虑

### 文件完整性
- 所有下载都使用 SHA256 校验和验证
- 文件通过 HTTPS 从 GitHub 下载
- 不使用第三方服务器或镜像

### 数字签名
- 更新保留任何现有的数字签名
- 可以使用 `--verify-signatures` 启用验证（未来功能）

### 网络安全
- 使用 GitHub 官方 API 和 CDN
- 不传输敏感信息
- 速率限制遵守 GitHub API 限制

## 自动化和 CI/CD

### 计划更新

**Windows 任务计划程序：**
```powershell
# 创建每周更新的计划任务
schtasks /create /tn "installer-analyzer-update" /tr "installer-analyzer update --yes" /sc weekly
```

**PowerShell 脚本：**
```powershell
# update-installer-analyzer.ps1
try {
    $result = & installer-analyzer update --check-only
    if ($LASTEXITCODE -eq 0 -and $result -match "update available") {
        Write-Host "正在安装更新..."
        & installer-analyzer update --yes
    } else {
        Write-Host "没有可用更新"
    }
} catch {
    Write-Error "更新检查失败: $_"
}
```

### CI/CD 集成

```yaml
# GitHub Actions 示例
- name: 更新 installer-analyzer
  run: |
    installer-analyzer update --check-only
    if [ $? -eq 0 ]; then
      installer-analyzer update --yes
    fi
```

## 替代更新方法

### Chocolatey（推荐）

如果您通过 Chocolatey 安装，请使用 Chocolatey 进行更新：

```powershell
# 通过 Chocolatey 更新
choco upgrade installer-analyzer

# 检查过时的包
choco outdated
```

### 手动更新

1. 从 [GitHub Releases](https://github.com/loonghao/installer-analyzer/releases) 下载
2. 替换现有可执行文件
3. 使用 `installer-analyzer --version` 验证

## 配置

### 环境变量

| 变量 | 描述 | 默认值 |
|------|------|--------|
| `INSTALLER_ANALYZER_UPDATE_CHECK` | 启用/禁用更新检查 | `true` |
| `INSTALLER_ANALYZER_UPDATE_URL` | 自定义更新服务器 URL | GitHub API |
| `INSTALLER_ANALYZER_TEMP_DIR` | 更新临时目录 | 系统临时目录 |

### 更新设置

目前更新设置是内置的。未来版本可能支持：
- 自定义更新通道（稳定版、测试版、每夜构建）
- 更新频率配置
- 代理服务器支持
- 自定义下载镜像

## 最佳实践

1. **定期更新**: 每周或每月检查更新
2. **备份**: 为关键环境保留工作版本的备份
3. **测试**: 在非生产环境中首先测试更新
4. **监控**: 在自动化环境中监控更新日志
5. **回滚**: 了解如何在更新导致问题时回滚

## 支持

对于更新相关问题：
- 首先查看本指南
- 查看 [GitHub Issues](https://github.com/loonghao/installer-analyzer/issues)
- 创建包含更新日志和系统信息的新问题
- 包含 `installer-analyzer --version` 和 `installer-analyzer update --check-only` 的输出
