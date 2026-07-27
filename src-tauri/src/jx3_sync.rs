use crate::entity::jx3_server::Jx3Server;
use serde::Deserialize;

const JX3_API_URL: &str = "https://www.jx3api.com/server/status/check";

pub(crate) const FALLBACK_JX3_SERVERS: &[(&str, &str, &str)] = &[
    ("无界区", "眉间雪", "拥挤"),
    ("无界区", "山海相逢", "正常"),
    ("电信区", "龙争虎斗", "拥挤"),
    ("电信区", "剑胆琴心", "拥挤"),
    ("电信区", "斗转星移", "拥挤"),
    ("电信区", "乾坤一掷", "爆满"),
    ("电信区", "绝代天骄", "爆满"),
    ("电信区", "梦江南", "爆满"),
    ("电信区", "幽月轮", "拥挤"),
    ("电信区", "长安城", "拥挤"),
    ("电信区", "唯我独尊", "爆满"),
    ("电信区", "蝶恋花", "拥挤"),
    ("双线区", "天鹅坪", "爆满"),
    ("双线区", "破阵子", "拥挤"),
    ("双线区", "飞龙在天", "拥挤"),
];

#[derive(Debug, Deserialize)]
struct ApiResponse {
    code: i32,
    data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct ServerItem {
    zone: String,
    server: String,
    status: String,
}

pub(crate) fn fallback_servers() -> Vec<Jx3Server> {
    FALLBACK_JX3_SERVERS
        .iter()
        .map(|(zone, server, status)| Jx3Server {
            id: None,
            zone: zone.to_string(),
            server: server.to_string(),
            status: status.to_string(),
            last_update_time: None,
        })
        .collect()
}

fn parse_server_list(data: &serde_json::Value) -> Option<Vec<Jx3Server>> {
    let items = if data.is_array() {
        data.as_array()?.clone()
    } else if data.is_object() && data.get("server").is_some() {
        vec![data.clone()]
    } else {
        return None;
    };

    let mut servers = Vec::new();
    for item in items {
        if let Ok(parsed) = serde_json::from_value::<ServerItem>(item) {
            servers.push(Jx3Server {
                id: None,
                zone: parsed.zone,
                server: parsed.server,
                status: parsed.status,
                last_update_time: None,
            });
        }
    }

    if servers.is_empty() {
        None
    } else {
        Some(servers)
    }
}

pub(crate) fn fetch_servers_from_api() -> Result<Vec<Jx3Server>, String> {
    let response = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?
        .get(JX3_API_URL)
        .send()
        .map_err(|e| e.to_string())?;

    let body: ApiResponse = response.json().map_err(|e| e.to_string())?;
    if body.code != 200 {
        return Err(format!("JX3API 返回错误: code={}", body.code));
    }

    parse_server_list(&body.data).ok_or_else(|| "无法解析区服数据".to_string())
}
