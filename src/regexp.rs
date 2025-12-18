use regex::Regex;

#[derive(Clone, Copy, Debug)]
pub struct TelegramRegexp {
    pub expression: &'static str,
    /// -1 表示匹配所有结果
    pub index: isize,
    /// -1 表示无限次匹配
    pub max_matches: isize,
}

pub fn has_newline(s: &str) -> bool {
    if s.contains('\n') {
        let trimmed = s.trim();
        if trimmed.contains('\n') {
            return true;
        }
    }
    false
}

pub fn telegram_parser(data: &str, rules: &[TelegramRegexp]) -> String {
    let mut formatted = String::new();

    for rule in rules {
        let re = Regex::new(rule.expression)
            .unwrap_or_else(|e| panic!("invalid regex {}: {e}", rule.expression));

        let results: Vec<String> = if rule.max_matches < 0 {
            re.find_iter(data).map(|m| m.as_str().to_string()).collect()
        } else {
            re.find_iter(data)
                .take(rule.max_matches as usize)
                .map(|m| m.as_str().to_string())
                .collect()
        };

        if results.is_empty() {
            continue;
        }

        if rule.index == -1 {
            for s in results {
                formatted.push_str(&s);
                formatted.push('\n');
            }
        } else {
            let idx = rule.index as usize;
            if idx < results.len() {
                formatted.push_str(&results[idx]);
                formatted.push('\n');
            }
        }
    }

    formatted
}

pub const MATCH_RULES: [TelegramRegexp; 6] = [
    TelegramRegexp {
        expression: r"(\d{2}\s){3}\d{12}\s\w\d{2}",
        index: 0,
        max_matches: 1,
    }, // 匹配地震报文头部
    TelegramRegexp {
        expression: r"\d{12}",
        index: 1,
        max_matches: -1,
    }, // 匹配地震发生时刻
    TelegramRegexp {
        expression: r"ND\d{14}\sNCN.+\sJD.+\sJN.{3}",
        index: 0,
        max_matches: 1,
    }, // 匹配地震 ID 和编号
    TelegramRegexp {
        expression: r"\d{3}\s(N|S)\d.+\s(W|E)\d.+RC.{5}",
        index: 0,
        max_matches: 1,
    }, // 匹配地震震源和深度
    TelegramRegexp {
        expression: r"E\wI\s.{59}",
        index: 0,
        max_matches: 1,
    }, // 匹配最大震度预测值
    TelegramRegexp {
        expression: r"9999=",
        index: 0,
        max_matches: 1,
    }, // 匹配报文尾部
];
