小米 Pad 8（yupei）适配预发布版，基于 Skyward 0.3.1。

适配基线：HyperOS OS4.0.12.0.XPZCNXM，Android 17。

已完成实机复核：触摸、锁屏凭据解密、WLAN、自动旋转、息屏唤醒，以及不写分区的脚本 ZIP 安装测试。安装测试前后 15 个关键分区的 SHA-256 一致。所用功能代码为 `ea31bb0049621129cec732dd6eb8834b0417863b`；本发布镜像由 GitHub Actions 重新构建，具体来源及镜像校验值见 `PROVENANCE.json`。

已知限制：

- 上游安装流程会卸载系统挂载；脚本安装后如需重新开启 WLAN，请先重启 Recovery。
- 尚未验证真实 ROM／OTA 写入及双系统，不能视为全部功能验收通过。
- Magisk／KernelSU 模块是否支持 Recovery 安装，由模块自身决定。

下载镜像 ZIP 和 `SHA256SUMS`，核对后解压。刷写前备份当前 Recovery，并核验实际运行槽位；本次实测只刷写 B 槽的 `recovery_b`，没有切换槽位。请勿因为此说明而盲目切槽。ZIP 是镜像压缩包，不是可直接安装的刷机脚本包。

---

Xiaomi Pad 8 (yupei) prerelease based on Skyward 0.3.1, tested on HyperOS OS4.0.12.0.XPZCNXM / Android 17.

Physical checks passed for touch, credential decryption, WLAN, rotation, screen-off/wake and a script ZIP that performs no partition writes. Real ROM/OTA writes and dual boot remain unverified. After a script installation, restart recovery before enabling WLAN again because the upstream installer detaches the system mounts.

The functional source was tested at `ea31bb0049621129cec732dd6eb8834b0417863b`; these assets are rebuilt by GitHub Actions. Verify `SHA256SUMS` and consult `PROVENANCE.json` for the exact build source and image checksum. Back up the current recovery and verify the actual active slot before flashing. The download ZIP contains a recovery image, not an installation script.
