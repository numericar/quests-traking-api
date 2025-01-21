use std::fmt::{self};

use anyhow;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Stage {
    Not,
    Local,
    #[default]
    Development,
    Production
}

// ทำให้ enum stage ของเราสามารถใช้งาน .to_string() ได้
impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stage_str: &str = match self {
            Stage::Local => "Local",
            Stage::Development => "Development",
            Stage::Production => "Production",
            Stage::Not => "Not"
        };

        // ทำให้เราสามารถทำค่าที่แปลงเป็น string แล้ว นำไปใช้กับการจัด format ได้ เช่น println!("Hello {}", stage_str);
        write!(f, "{}", stage_str)
    }
}

impl Stage {
    pub fn try_from(stage_str: &str) -> anyhow::Result<Self> {
        let stage: Stage = match stage_str {
            "Local" => Stage::Local,
            "Development" => Stage::Development,
            "Production" => Stage::Production,
            _ => Stage::Not
        };

        if stage == Stage::Not {
            Err(anyhow::anyhow!("Invalid stage"))
        } else {
            Ok(stage)
        }
    }
}