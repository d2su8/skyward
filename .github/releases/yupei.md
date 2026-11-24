小米 Pad 8（`yupei`）适配预发布版，基于 [Kirklin 的 Skyward 0.5.1](https://github.com/kirklin/skyward/tree/43ec6da)。固件基线为 HyperOS OS4.0.12.0.XPZCNXM（Android 17）。

- 保留平板双栏界面、SSC 自动旋转、Android 17 QTEE Weaver 和 WCN7750 WLAN 适配。平板界面已接入上游的 15 种语言。
- 同步上游的内部存储备份与恢复、更多分区镜像安装格式以及安装进度改进。
- 本地实机镜像在 B 槽启动，已核验 Recovery 分区读回、WLAN 扫描和安全服务注册。[查看实机截图和适配说明](https://github.com/d2su8/skyward/blob/v0.5.1-yupei.r1/README.zh-CN.md)
- 此版本尚未重新验证输入锁屏凭据解密、实际转动、整包 ROM／OTA、备份恢复和双系统。

下载包含 `yupei` 的镜像 ZIP，用 `SHA256SUMS` 校验后解压。刷入前请备份当前 Recovery，并检查实际活动槽位。发布附件由 GitHub Actions 独立构建，确切的构建提交和镜像校验值见 `PROVENANCE.json`。

---

Xiaomi Pad 8 (`yupei`) prerelease based on [Skyward 0.5.1 by Kirklin](https://github.com/kirklin/skyward/tree/43ec6da) and HyperOS OS4.0.12.0.XPZCNXM (Android 17). This release brings the upstream 15-language UI, internal storage backup and restore, and expanded partition image installation to the Pad 8 fork.

A local build booted on slot B and passed recovery readback, WLAN scan and secure service registration checks. Credential unlock, physical rotation, full ROM/OTA writes, backup and restore, and dual boot were not retested on this version. The release assets are rebuilt in GitHub Actions. Verify `SHA256SUMS` and see `PROVENANCE.json` for the exact build source and image checksum.
