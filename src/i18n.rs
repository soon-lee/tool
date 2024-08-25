use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/**
 * 本地化语言枚举
 * - ZhCn: 简体中文（中国大陆）
 * - EnUs: 美式英文
 */
#[derive(Clone, Debug)]
pub(crate) enum Language {
    /* 简体中文（中国大陆） */
    ZhCn,
    /* 美式英文 */
    EnUs,
}

/**
 * 本地化语言管理器
 * - translations: 翻译的映射表
 * - change_language: 更改语言设置
 * - translate: 获取翻译，如果找不到则返回默认值
 */
pub(crate) struct I18n {
    /* 存储翻译的映射表 */
    translations: HashMap<String, String>,
}

impl I18n {
    /* 创建一个新的 `Locale` 实例 */
    pub(crate) fn new() -> Self {
        I18n {
            translations: HashMap::new(),
        }
    }

    /**
     * 更改语言设置
     * - language: 语言枚举
     * - 返回值：Result<(), String>
     */
    pub(crate) fn change_locale(&mut self, locale: &str) -> Result<(), String> {
        /* 根据语言选择文件路径 */
        let file_path = match locale {
            "zh-CN" => "assets/locales/zh-CN.txt",
            "en-US" => "assets/locales/en-US.txt",
            _ => ""
        };

        /* 打开文件错误消息 */
        let open_file_err = match locale {
            "zh-CN" => "打开文件失败！",
            "en-US" => "Failed to open file!",
            _ => ""
        };

        /* 打开文件 */
        let file = File::open(file_path).map_err(|_| format!("{} <= {}", open_file_err, file_path))?;
        let reader = BufReader::new(file);

        /* 读取行错误消息 */
        let read_line_err = match locale {
            "zh-CN" => "读取文件行失败！",
            "en-US" => "Failed to read line!",
            _ => ""
        };

        /* 逐行读取文件 */
        for line in reader.lines() {
            /* 读取每一行 */
            let line = line.map_err(|err| format!("{} <= {}", read_line_err, err))?;
            /* 检查行是否为空或注释 */
            if !line.is_empty() && !line.starts_with('#') {
                /* 分割键和值 */
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim().to_owned();
                    let value = parts[1].trim().to_owned();
                    /* 直接插入到 self.translations */
                    self.translations.insert(key, value);
                }
            }
        }
        Ok(())
    }

    /**
     * 获取翻译，如果找不到则返回默认值
     * - key: 键
     * - default: 默认值
     * - 返回值：&str
     */
    pub(crate) fn translate(&self, key: &str, default: &str) -> String {
        match self.translations.get(key){
            Some(value) => value.to_string(),
            None => default.to_string(),
        }
    }
}
