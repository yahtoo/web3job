// src/models.rs

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub errorCode: i32,
    pub message: String,
    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub page: Page,
    pub results: Vec<Result>,
}

#[derive(Deserialize, Debug)]
pub struct Page {
    pub page: i32,
    pub limit: i32,
    pub total: i32,
}

#[derive(Deserialize, Debug)]
pub struct Result {
    pub topicId: i32,
    pub user: User,
    pub content: String,
    pub content2: String,
    pub content3: String,
    pub content5: String,
    pub email: String,
    pub phone: String,
    pub wechat: String,
    pub telegram: String,
    pub positionName: String,
    pub positionId: i32,
    pub viewCount: i32,
    pub applied: bool,
    pub applyCount: i32,
    pub createTime: i64,
    pub url: String,
    pub workTypeId: i32,
    pub workTypeName: String,
    pub officeModeId: i32,
    pub officeModeName: String,
    pub company: String,
    pub companyIntroduction: String,
    pub companySizeName: String,
    pub companyLogo: String,
    pub companyWebsite: String,
    pub companyId: i32,
    pub minSalary: i32,
    pub maxSalary: i32,
    pub leverId: i32,
    pub leverName: String,
    pub location: String,
    pub tags: Vec<Tag>,
    pub base: String,
    pub status: i32,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub nickname: String,
    pub avatar: String,
    pub smallAvatar: String,
    pub topicCount: i32,
    pub commentCount: i32,
    pub fansCount: i32,
    pub followCount: i32,
    pub score: i32,
    pub description: String,
    pub createTime: i64,
    pub walletAddress: String,
    pub followed: bool,
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub tagId: i32,
    pub tagName: String,
}

#[derive(Deserialize, Debug)]
pub struct JobDetailApiResponse {
    errorCode: i32,
    message: String,
    data: JobDetail,
    success: bool,
}

#[derive(Deserialize, Debug)]
pub struct JobDetail {
    topicId: i32,
    content: String,
    content2: String,
    content3: String,
    content5: String,
    email: Option<String>,
    phone: Option<String>,
    wechat: Option<String>,
    telegram: Option<String>,
    positionName: String,
    positionId: i32,
    viewCount: i32,
    applied: bool,
    applyCount: i32,
    createTime: i64,
    url: String,
    workTypeId: i32,
    workTypeName: String,
    officeModeId: i32,
    officeModeName: String,
    company: String,
    companyIntroduction: String,
    companySizeName: String,
    companyLogo: String,
    companyWebsite: String,
    companyId: i32,
    minSalary: f64,
    maxSalary: f64,
    leverId: i32,
    leverName: String,
    location: Option<String>,
    base: String,
    status: i32,
}