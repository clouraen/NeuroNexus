# Profile Page - Headers
profile-header-title = 个人资料
profile-header-subtitle = 管理您的账户和偏好设置

# Profile Page - Sections
profile-section-personal = 个人信息
profile-section-stats = 您的统计数据
profile-section-preferences = 偏好设置
profile-section-language = 语言设置

# Profile Page - Personal Info
profile-personal-name = 姓名
profile-personal-email = 电子邮件
profile-personal-joined = 加入时间
profile-personal-edit = 编辑资料
profile-personal-student = 学生
profile-personal-default-email = 学生@neuronexus.app

# Profile Page - Statistics
profile-stats-total-questions = 总题目数
profile-stats-answered = 已回答题目
profile-stats-correct = 正确答案
profile-stats-essays = 已写作文
profile-stats-study-time = 学习时间
profile-stats-streak = 当前连续
profile-stats-sequences = 最佳连续

# Profile Page - Preferences
profile-pref-notifications = 启用通知
profile-pref-dark-mode = 深色模式
profile-pref-auto-save = 自动保存作文
profile-pref-show-explanations = 显示解释

# Profile Page - Language
profile-lang-current = 当前语言
profile-lang-select = 选择语言
profile-lang-system-default = 系统默认
profile-lang-apply = 应用语言

# Profile Page - Actions
profile-action-save = 保存更改
profile-action-cancel = 取消
profile-action-logout = 退出
profile-action-delete-account = 删除账户

# Profile Page - Import/Export
profile-import-title = 导入数据
profile-import-questions = 导入题目
profile-import-trails = 导入学习路径
profile-import-select-file = 选择文件
profile-export-title = 导出数据
profile-export-all = 导出所有数据

# AI Configuration Section
profile-ai-title = AI模型配置
profile-ai-subtitle = 配置HuggingFace并下载AI模型
profile-ai-token-label = HuggingFace令牌
profile-ai-token-placeholder = 在此粘贴您的令牌(以hf_开头)
profile-ai-token-save = 保存令牌
profile-ai-token-validate = 验证令牌
profile-ai-token-clear = 清除令牌

# Status Messages
profile-ai-status-not-configured = 未配置
profile-ai-status-configured = 令牌已配置
profile-ai-status-connected = 已连接到HuggingFace
profile-ai-status-model-cached = 模型已下载
profile-ai-status-model-ready = 模型就绪
profile-ai-status-downloading = 正在下载模型
profile-ai-status-loading = 正在加载模型
profile-ai-status-error = 错误

# Actions
profile-ai-download-model = 下载模型
profile-ai-retry-download = 重试下载
profile-ai-cancel-download = 取消下载
profile-ai-test-model = 测试模型
profile-ai-clear-cache = 清除模型缓存
profile-ai-reload-model = 重新加载模型

# Guide
profile-ai-guide-title = 如何获取HuggingFace令牌
profile-ai-guide-intro = 按照以下步骤配置AI驱动的作文评估:
profile-ai-guide-step1 = 步骤1:创建HuggingFace账户
profile-ai-guide-step1-desc = 在HuggingFace免费注册账户
profile-ai-guide-step1-link = 创建账户
profile-ai-guide-step2 = 步骤2:转到令牌设置
profile-ai-guide-step2-desc = 导航到您的账户设置页面
profile-ai-guide-step2-link = 令牌设置
profile-ai-guide-step3 = 步骤3:创建新令牌
profile-ai-guide-step3-desc = 令牌类型:读取 | 所需权限:对模型的读取访问权限
profile-ai-guide-step3-name = 建议令牌名称:NeuroNexus-Desktop
profile-ai-guide-step4 = 步骤4:复制令牌
profile-ai-guide-step4-desc = 安全地复制并保存您的令牌(您将无法再次看到它)
profile-ai-guide-step5 = 步骤5:粘贴到NeuroNexus
profile-ai-guide-step5-desc = 在上面输入您的令牌并点击保存令牌

# Progress Messages
profile-ai-progress-validating = 正在验证令牌...
profile-ai-progress-connecting = 正在连接到存储库...
profile-ai-progress-config = 正在下载配置...
profile-ai-progress-tokenizer = 正在下载分词器...
profile-ai-progress-weights = 正在下载模型权重...
profile-ai-progress-loading = 正在将模型加载到内存...
profile-ai-progress-complete = 下载完成!

# Error Messages
profile-ai-error-invalid-format = 令牌格式无效。HuggingFace令牌以'hf_'开头
profile-ai-error-invalid-token = 令牌验证失败。请检查您的令牌并重试
profile-ai-error-network = 无法连接到HuggingFace。请检查您的互联网连接
profile-ai-error-disk-space = 磁盘空间不足。需要500MB可用空间
profile-ai-error-interrupted = 下载中断。您可以重试以恢复
profile-ai-error-corrupted = 模型文件损坏。请清除缓存并重试
profile-ai-error-access-denied = 访问被拒绝。请验证您的令牌具有'读取'权限
profile-ai-error-out-of-memory = 内存不足无法加载模型。请关闭其他应用程序并重试
profile-ai-error-incompatible = 模型版本不兼容。请更新应用程序
profile-ai-error-unknown = 发生未知错误。请重试

# Info Messages
profile-ai-info-cache-location = 缓存位置:
profile-ai-info-model-size = 模型大小:~420MB
profile-ai-info-first-download = 首次下载可能需要5-10分钟,具体取决于您的连接
profile-ai-info-offline-ready = 下载后,AI功能可100%离线工作
profile-ai-info-requires-token = 仅下载时需要令牌。缓存的模型无需令牌即可工作
profile-ai-info-token-permissions = 所需权限:对模型和存储库的读取访问权限
