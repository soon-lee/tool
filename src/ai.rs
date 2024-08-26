// 导入必要的库
use candle_core::{DType, Device, IndexOp, Result, Tensor};
use candle_nn::{batch_norm, conv2d, conv2d_no_bias, Func, Module, VarBuilder};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// 表示 Darknet 配置文件中的一个配置块。
#[derive(Debug)]
struct Block {
    /// 配置块的类型。
    block_type: String,

    /// 与配置块关联的参数。
    parameters: BTreeMap<String, String>,
}

impl Block {
    /// 通过键获取参数。
    fn get(&self, key: &str) -> Result<&str> {
        match self.parameters.get(key) {
            None => candle_core::bail!("无法找到 {} 在 {}", key, self.block_type),
            Some(value) => Ok(value),
        }
    }
}

/// 表示带有配置块和参数的 Darknet 模型。
#[derive(Debug)]
pub struct Darknet {
    /// 配置块列表。
    blocks: Vec<Block>,

    /// 与网络关联的参数。
    parameters: BTreeMap<String, String>,
}

impl Darknet {
    /// 通过键获取参数。
    fn get(&self, key: &str) -> Result<&str> {
        match self.parameters.get(key) {
            None => candle_core::bail!("无法找到 {} 在网络参数中", key),
            Some(value) => Ok(value),
        }
    }
}

/// 在解析 Darknet 配置文件时积累配置块信息。
struct Accumulator {
    /// 当前正在积累的配置块类型。
    block_type: Option<String>,

    /// 当前配置块的参数。
    parameters: BTreeMap<String, String>,

    /// 正在构建的 Darknet 模型。
    net: Darknet,
}

impl Accumulator {
    /// 创建一个新的积累器。
    fn new() -> Accumulator {
        Accumulator {
            block_type: None,
            parameters: BTreeMap::new(),
            net: Darknet {
                blocks: vec![],
                parameters: BTreeMap::new(),
            },
        }
    }

    /// 完成当前配置块并将其添加到 Darknet 模型中。
    fn finish_block(&mut self) {
        match &self.block_type {
            None => (),
            Some(block_type) => {
                if block_type == "net" {
                    self.net.parameters = self.parameters.clone();
                } else {
                    let block = Block {
                        block_type: block_type.to_string(),
                        parameters: self.parameters.clone(),
                    };
                    self.net.blocks.push(block);
                }
                self.parameters.clear();
            }
        }
        self.block_type = None;
    }
}

/// 解析 Darknet 配置文件并将其转换为 Darknet 模型。
pub fn parse_config<T: AsRef<Path>>(path: T) -> Result<Darknet> {
    let file = File::open(path.as_ref())?;
    let mut acc = Accumulator::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        if line.is_empty() || line.starts_with('#') {
            continue; // 跳过空行或注释行
        }
        let line = line.trim();
        if line.starts_with('[') {
            if !line.ends_with(']') {
                candle_core::bail!("行未以 ']' 结束: {line}");
            }
            let line = &line[1..line.len() - 1]; // 去除方括号
            acc.finish_block();
            acc.block_type = Some(line.to_string()); // 设置新的配置块类型
        } else {
            let key_value: Vec<&str> = line.splitn(2, '=').collect();
            if key_value.len() != 2 {
                candle_core::bail!("缺少等号: {line}");
            }
            let prev = acc.parameters.insert(
                key_value[0].trim().to_owned(),
                key_value[1].trim().to_owned(),
            );
            if prev.is_some() {
                candle_core::bail!("多个值对应同一个键: {}", line);
            }
        }
    }
    acc.finish_block(); // 处理最后一个配置块
    Ok(acc.net)
}

/// 表示 Darknet 模型中的不同类型的层或连接。
enum Bl {
    /// 神经网络层。
    Layer(Box<dyn Module + Send + Sync>),

    /// 路由层，用于连接多个层。
    Route(Vec<usize>),

    /// 快捷连接层，用于连接两个层。
    Shortcut(usize),

    /// YOLO 检测层。
    Yolo(usize, Vec<(usize, usize)>),
}

/// 根据配置块构建卷积层。
fn conv(vb: VarBuilder, index: usize, p: usize, b: &Block) -> Result<(usize, Bl)> {
    // ... (省略了部分细节，见原始代码)
}

/// 构建上采样层。
fn upsample(prev_channels: usize) -> Result<(usize, Bl)> {
    // ... (省略了部分细节，见原始代码)
}

/// 从字符串解析整数列表。
fn int_list_of_string(s: &str) -> Result<Vec<i64>> {
    // ... (省略了部分细节，见原始代码)
}

/// 将索引转换为正整数。
fn usize_of_index(index: usize, i: i64) -> usize {
    // ... (省略了部分细节，见原始代码)
}

/// 构建路由层，用于连接多个层。
fn route(index: usize, p: &[(usize, Bl)], block: &Block) -> Result<(usize, Bl)> {
    // ... (省略了部分细节，见原始代码)
}

/// 构建快捷连接层，用于连接两个层。
fn shortcut(index: usize, p: usize, block: &Block) -> Result<(usize, Bl)> {
    // ... (省略了部分细节，见原始代码)
}

/// 构建 YOLO 层。
fn yolo(p: usize, block: &Block) -> Result<(usize, Bl)> {
    // ... (省略了部分细节，见原始代码)
}

/// 实现 YOLO 检测逻辑。
fn detect(
    xs: &Tensor,
    image_height: usize,
    classes: usize,
    anchors: &[(usize, usize)],
) -> Result<Tensor> {
    // ... (省略了部分细节，见原始代码)
}

impl Darknet {
    /// 获取图像高度。
    pub fn height(&self) -> Result<usize> {
        let image_height = self.get("height")?.parse::<usize>()?;
        Ok(image_height)
    }

    /// 获取图像宽度。
    pub fn width(&self) -> Result<usize> {
        let image_width = self.get("width")?.parse::<usize>()?;
        Ok(image_width)
    }

    /// 构建 Darknet 模型。
    pub fn build_model(&self, vb: VarBuilder) -> Result<Func> {
        let mut blocks: Vec<(usize, Bl)> = vec![];
        let mut prev_channels: usize = 3;
        for (index, block) in self.blocks.iter().enumerate() {
            let channels_and_bl = match block.block_type.as_str() {
                "convolutional" => conv(vb.pp(index.to_string()), index, prev_channels, block)?,
                "upsample" => upsample(prev_channels)?,
                "shortcut" => shortcut(index, prev_channels, block)?,
                "route" => route(index, &blocks, block)?,
                "yolo" => yolo(prev_channels, block)?,
                otherwise => candle_core::bail!("不支持的配置块类型 {}", otherwise),
            };
            prev_channels = channels_and_bl.0;
            blocks.push(channels_and_bl);
        }
        let image_height = self.height()?;
        let func = candle_nn::func(move |xs| {
            let mut prev_ys: Vec<Tensor> = vec![];
            let mut detections: Vec<Tensor> = vec![];
            for (_, b) in blocks.iter() {
                let ys = match b {
                    Bl::Layer(l) => {
                        let xs = prev_ys.last().unwrap_or(xs);
                        l.forward(xs)?
                    }
                    Bl::Route(layers) => {
                        let layers: Vec<_> = layers.iter().map(|&i| &prev_ys[i]).collect();
                        Tensor::cat(&layers, 1)?
                    }
                    Bl::Shortcut(from) => (prev_ys.last().unwrap() + prev_ys.get(*from).unwrap())?,
                    Bl::Yolo(classes, anchors) => {
                        let xs = prev_ys.last().unwrap_or(xs);
                        detections.push(detect(xs, image_height, *classes, anchors)?);
                        Tensor::new(&[0u32], &Device::Cpu)?
                    }
                };
                prev_ys.push(ys);
            }
            Tensor::cat(&detections, 1)
        });
        Ok(func)
    }
}
