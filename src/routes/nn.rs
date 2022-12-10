use actix_web::{HttpResponse, Responder};

use serde_derive::Deserialize;
use serde_derive::Serialize;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestData {
    pub user_id: u64,
    pub user_name: String,
    pub user_fax: bool
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestRet {
    pub cnt: usize,
    pub users: Vec<TestData>
}


pub async fn history_dates() -> impl Responder {

    let mut dates : Vec<TestData> = Vec::new();
    let data1 = TestData {
        user_id: 1,
        user_name: String::from("lzh1104"),
        user_fax: false
    };
    dates.push(data1);
    let data2 = TestData {
        user_id: 2,
        user_name: String::from("lzh1204"),
        user_fax: true
    };
    dates.push(data2);

    let ret = TestRet {
        cnt: dates.len(),
        users: dates
    };

    HttpResponse::Ok().json(ret)
}