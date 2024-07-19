mod models;

use reqwest::Error;
use models::ApiResponse;

async fn fetch_joblist_data(page: i32) ->Result<ApiResponse,Error> {
    // 构建请求的 URL
    let url = format!("https://www.dejob.top/api/worker/topics?page={}&limit=20", page);

    // 发送请求并解析 JSON 数据
    let response: ApiResponse = reqwest::get(&url).await?.json().await?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(),Error> {
    let page = 1;
    // 获取工作列表
    let data = fetch_joblist_data(page).await?;
    print!("{:#?}",data);
    Ok(())
}