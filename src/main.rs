mod models;

use reqwest::Error;
use models::ApiResponse;
use models::JobDetailApiResponse;

async fn fetch_joblist_data(page: i32) ->Result<ApiResponse,Error> {
    // 构建请求的 URL
    let url = format!("https://www.dejob.top/api/worker/topics?page={}&limit=20", page);

    // 发送请求并解析 JSON 数据
    let response: ApiResponse = reqwest::get(&url).await?.json().await?;

    Ok(response)
}

async fn fetch_jobdetail_data(job_id: i32) ->Result<JobDetailApiResponse,Error> {
    // 构建请求的 URL
    let url = format!("https://www.dejob.top/api/worker/{}", job_id);

    // 发送请求并解析 JSON 数据
    let response: JobDetailApiResponse = reqwest::get(&url).await?.json().await?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(),Error> {
    let page = 1;
    // 获取工作列表
    let data = fetch_joblist_data(page).await?;
    print!("{:#?}",data);

    let job_id = 3833;
    let detailData = fetch_jobdetail_data(job_id).await?;
    print!("{:#?}",detailData);
    Ok(())
}