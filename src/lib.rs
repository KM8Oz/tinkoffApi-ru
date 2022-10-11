#![allow(non_snake_case)]
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
#[allow(dead_code)]
use std::collections::HashMap;
use hyper::{Client, Request, Method, Body};
use hyper_tls::HttpsConnector;
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    #[default]
    #[serde(rename_all = "lowercase")]
    RU,
    #[serde(rename_all = "lowercase")]
    EN,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum Taxation {
    #[default]
    #[serde(rename = "osn")]
    OSN,
    #[serde(rename = "usn_income")]
    UsnIncome,
    #[serde(rename = "usn_income_outcome")]
    UsnIncomeOutcome,
    #[serde(rename = "patent")]
    Patent,
    #[serde(rename = "envd")]
    Envd,
    #[serde(rename = "esn")]
    Esn,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum TaxNDK {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "vat0")]
    Vat0,
    #[serde(rename = "vat10")]
    Vat10,
    #[serde(rename = "vat20")]
    Vat20,
    #[serde(rename = "vat110")]
    Vat110,
    #[serde(rename = "vat120")]
    Vat120,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    #[default]
    #[serde(rename = "full_payment")]
    FullPayment,
    #[serde(rename = "full_prepayment")]
    FullPrepayment,
    #[serde(rename = "prepayment")]
    Prepayment,
    #[serde(rename = "advance")]
    Advance,
    #[serde(rename = "partial_payment")]
    PartialPayment,
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "credit_payment")]
    CreditPayment,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Payments {
    pub terminal_key: String,
    pub password: String,
    pub success_url: String,
    pub fail_url: String,
    pub notification_url: String,
    pub language: Language,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "Phone")]
    pub phone: String,
    #[serde(rename = "EmailCompany")]
    pub emailcompany: String,
    #[serde(rename = "Taxation")]
    pub taxation: Taxation,
    #[serde(rename = "Items")]
    pub items: Vec<Value>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub orderid: String,
    pub description: String,
    pub value: Value,
}

pub fn merge(v: &Value, fields: &HashMap<String, String>) -> Value {
    match v {
        Value::Object(m) => {
            let mut m = m.clone();
            for (k, v) in fields {
                m.insert(k.clone(), Value::String(v.clone()));
            }
            Value::Object(m)
        }
        v => v.clone(),
    }
}
pub fn merge_value(v: &Value, fields: &HashMap<String, Value>) -> Value {
    match v {
        Value::Object(m) => {
            let mut m = m.clone();
            for (k, v) in fields {
                m.insert(k.clone(), v.clone());
            }
            Value::Object(m)
        }
        v => v.clone(),
    }
}
impl Receipt {
    pub fn build(&self, email: &str, phone: &str, emailcompany: &str, taxation: Taxation) -> Receipt {
        Receipt {
            email: email.to_string(),
            phone: phone.to_string(),
            emailcompany: emailcompany.to_string(),
            taxation,
            ..Receipt::default()
        }
    }
    pub fn add_item(&self, name: &str, quantity: u32, price: u32, tax: TaxNDK) -> Receipt {
        let mut _items = self.items.clone();
        _items.push(json!({
            "Name":name.to_string(),
            "Quantity":quantity.to_string(),
            "Amount":(price*quantity*100).to_string(),
            "Price":(price*100).to_string(),
            "Tax":tax,
        }));
        Receipt {
            email: self.email.clone(),
            phone: self.phone.clone(),
            emailcompany: self.emailcompany.clone(),
            taxation: self.taxation.clone(),
            items: _items,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PaymentUrl {
    #[serde(rename = "Success")]
    pub success: bool,
    #[serde(rename = "TerminalKey")]
    pub terminalkey: String,
    #[serde(rename = "ErrorCode")]
    pub error_code: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "PaymentId")]
    pub payment_id: String,
    #[serde(rename = "OrderId")]
    pub order_id: String,
    #[serde(rename = "Amount")]
    pub amount: u32,
    #[serde(rename = "PaymentURL")]
    pub payment_url: String
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PaymentUrlError {
    #[serde(rename = "Success")]
    pub success: bool,
    #[serde(rename = "ErrorCode")]
    pub error_code: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Details")]
    pub details: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum PayUrlError {
        String(String),
        PaymentUrlError(PaymentUrlError)
}
impl Payment {
    pub fn set_amount(&self, amount: u32) -> Payment {
        Payment {
            orderid: self.orderid.clone(),
            description: self.description.clone(),
            value: merge(
                &self.value,
                &HashMap::from([("Amount".into(), (amount*100).to_string())]),
            ),
        }
    }
    pub fn set_customer(&self, phone: &str, email: &str) -> Payment {
        Payment {
            orderid: self.orderid.clone(),
            description: self.description.clone(),
            value: merge_value(
                &self.value,
                &HashMap::from([(
                    "DATA".into(),
                    json!({
                        "Phone": phone,
                        "Email": email,
                    }),
                )]),
            ),
        }
    }
    pub fn set_receipt(&self, receipt: Receipt) -> Payment {
        Payment {
            orderid: self.orderid.clone(),
            description: self.description.clone(),
            value: merge_value(
                &self.value,
                &HashMap::from([(
                    "Receipt".into(),
                    serde_json::value::to_value(receipt).unwrap(),
                )]),
            ),
        }
    }
    pub async fn to_url(&self) -> Result<PaymentUrl, PayUrlError> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let req = Request::builder()
            .header("Content-Type", "application/json")
            .method(Method::POST)
            .uri("https://securepay.tinkoff.ru/v2/Init")
            .body(Body::from(self.value.to_string()))
            .expect("request builder");
        let future = client.request(req);
        let body = future.await;
        match body {
         Ok(m)=> {
            let v = hyper::body::to_bytes(m.into_body()).await.expect("body format error:");
            let _str = String::from_utf8(v.to_vec()).unwrap().as_str().replace("\\","");
            let vl:Value =  serde_json::from_str(&_str).expect("body format error:");
            let res = serde_json::value::from_value::<PaymentUrl>(vl.clone());
            match res {
                Ok(f)=> Ok(f),
                Err(err) => {
                    let res = serde_json::value::from_value::<PaymentUrlError>(vl.clone());
                    match res {
                        Ok(f)=> Err(PayUrlError::PaymentUrlError(f)),
                        Err(_err) => Err(PayUrlError::String(format!("Error formating the responce to PaymentUrl: [1]> {:?}, [2]> {:?}", err, _err))),
                    }
                }
            }
         },
         Err(err)=> Err(PayUrlError::String(format!("Error getting respone: {:?}", err)))
        }
        
    }
}
impl Payments {
    pub fn build(&self, terminal_key: &str, callback_url: &str,  orderid: &str, description: &str) -> Payment {
        let SuccessFailURL =  format!("{}?order_id=", callback_url.to_string());
        Payment {
            orderid: orderid.to_string(),
            description: description.to_string(),
            value: json!({
                "TerminalKey": terminal_key.to_string(),
                "Language": self.language,
                "OrderId": orderid.to_string(),
                "Description": description.to_string(),
                "SuccessURL": SuccessFailURL,
                "FailURL":SuccessFailURL,
                "NotificationURL":callback_url.to_string()
            }),
        }
    }
}
#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{Payments, Receipt, Taxation, TaxNDK};
    #[test]
    fn parser() {
        let _str =  "{\\\\\\\"Success\\\\\\\":true,\\\\\\\"ErrorCode\\\\\\\":\\\\\\\"0\\\\\\\",\\\\\\\"TerminalKey\\\\\\\":\\\\\\\"TinkoffBankTest\\\\\\\",\\\\\\\"Status\\\\\\\":\\\\\\\"NEW\\\\\\\",\\\\\\\"PaymentId\\\\\\\":\\\\\\\"1852009375\\\\\\\",\\\\\\\"OrderId\\\\\\\":\\\\\\\"210950\\\\\\\",\\\\\\\"Amount\\\\\\\":50000,\\\\\\\"PaymentURL\\\\\\\":\\\\\\\"https://securepay.tinkoff.ru/new/lrhj1Llb\\\\\\\"}".replace("\\", "");
        println!("value: {}", _str.clone());
        let data:Value = serde_json::from_str(&_str).unwrap();
        println!("value: {:?}", data);
        // let res =  serde_json::value::from_value()
    }
    #[tokio::test]
    async fn request_demo_url() {
        let payment_url = Payments::default()
        .build(
            "TinkoffBankTest",
            "https://bulkus.ru/pay_back",
            "210950", 
            "Подарочная карта на 1400.00 рублей")
        .set_amount(500) // in ruble
        .set_customer("+71234567890", "a@test.com")
        .set_receipt(
            Receipt::default()
            .build(
                "a@test.com", 
                "+71234567890", 
                "a@test.com", 
                Taxation::OSN
            )
            .add_item(
                "Наименование товара 1", 
                1, 
                100, // in ruble
                TaxNDK::None
            )
            .add_item(
                "Наименование товара 2", 
                2, 
                200, // in ruble
                TaxNDK::None
            )
        )
        .to_url().await;
        match payment_url.clone() {
            Ok(m) => {
                println!("payment url: {:?}", m);
            },
            Err(err) => {
                println!("payment error: {:?}", err);
            }
        }
    }
}
