

<h1 id="top" align="center">TinkoffPay-rs</h1>
<p id="top" color="#343434" align="center">⭐️ <font color="#F7C815">Leave me a start please</font> ⭐️</p>
<p id="top" align="center">
<font color="#F7C815">it will motivate me to continue maintaining and adding futures</font></p>
<div align="center" style="display:flex;flex-direction:row;gap:5px; width:100%;justify-content:center;">
  <img alt="Github top language" href="https://crates.io/crates/tinkoffpay" src="https://img.shields.io/github/languages/top/KM8Oz/TinkoffPay-rs?color=56BEB8">

  <img alt="Github language count" href="https://crates.io/crates/tinkoffpay" src="https://img.shields.io/github/languages/count/KM8Oz/tinkoffpay-rs?color=56BEB8">

  <img alt="Repository size" href="https://crates.io/crates/tinkoffpay" src="https://img.shields.io/github/repo-size/KM8Oz/tinkoffpay-rs?color=56BEB8">

  <img alt="License" href="https://crates.io/crates/tinkoffpay" src="https://img.shields.io/github/license/KM8Oz/tinkoffpay-rs?color=56BEB8">
  <img alt="Crates.io" href="https://crates.io/crates/tinkoffpay" src="https://img.shields.io/crates/v/tinkoffpay?color=56BEB8&label=tinkoffpay">
  <!-- <img alt="Github issues" src="https://img.shields.io/github/issues/KM8Oz/tinkoffpay-rs?color=56BEB8" /> -->

  <!-- <img alt="Github forks" src="https://img.shields.io/github/forks/KM8Oz/tinkoffpay-rs?color=56BEB8" /> -->

  <!-- <img alt="Github stars" src="https://img.shields.io/github/stars/KM8Oz/tinkoffpay-rs?color=56BEB8" /> -->
</div>

<!-- Status -->

<!-- <h4 align="center"> 
	🚧  TinkoffPay 🚀 Under construction...  🚧
</h4> 

<hr> -->

<p align="center" >
  <a href="#-about">About</a> &#xa0; | &#xa0; 
  <!-- <a href="#sparkles-features">Features</a> &#xa0; | &#xa0; -->
  <a href="#-technologies">Technologies</a> &#xa0; | &#xa0;
  <a href="#-requirements">Requirements</a> &#xa0; | &#xa0;
  <a href="#-starting">Starting</a> &#xa0; | &#xa0;
  <a href="#-usage">Usage</a> &#xa0; | &#xa0;
  <a href="#-license">License & Screen</a> &#xa0; | &#xa0;
  <a href="https://github.com/KM8Oz" target="_blank">Author</a>
</p>

<br>

## 🎯 About ##

Simple tinkoff integration (the seller receives a link to the payment form and redirect the buyer to it); NB: with callback method

## 🚀 Technologies ##

The following tools were used in this project:

- [rust](https://www.rust-lang.org/)
- [crago](https://crates.io/)

## ✅ Requirements ##

Before starting :checkered_flag:, you need to have [Git](https://git-scm.com) and [Rust](https://www.rust-lang.org/) installed.

## 🏁 Starting ##

```bash
# Clone this project
$ git clone https://github.com/KM8Oz/tinkoffpay-rs

# Access
$ cd tinkoffpay-rs

# Install dependencies
$ cargo install --path=.

# Run the project
$ cargo test request_demo_url -- --nocapture 

# The server will initialize in the <http://localhost:3000>
```

## ✅ Usage ##

```rust
    use crate::{Payments, Receipt, Taxation, TaxNDK};
// should be an async fn 
    let payment_url = Payments::default()
        .build(
            "TinkoffBankTest",
            "https://bulkus.ru/pay_back", 
            //  Success or Fail URL (GET) =  [your callback url]?order_id=[orderid]
            //  payment catch url (POST) has those params:
            //           Json(Object {
            //          "Amount": Number(10000),
            //          "CardId": Number(130550983),
            //          "ErrorCode": String("0"),
            //          "ExpDate": String("1122"),
            //          "OrderId": String("278600"),
            //          "Pan": String("430000******0777"),
            //          "PaymentId": Number(1866123460),
            //          "Status": String("CONFIRMED"),
            //          "Success": Bool(true),
            //          "TerminalKey": String("1639044907391DEMO"),
            //          "Token": String("711cd4fa0df2afa8a69a56884e9d902eb882c9a01af56498fc6bfdefaf9eef8d")
            //          })
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
                "Наименование товара 1", // name
                1, // quantity
                100, // in ruble
                TaxNDK::None
            )
            .add_item(
                "Наименование товара 2", // name
                2, // quantity
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

```

## 📝 License ##

This project is under license from MIT. For more details, see the [LICENSE](LICENCE.md) file.


Made with :heart: by <a href="https://github.com/KM8Oz" target="_blank">@KM8Oz</a>
&#xa0;
<div align="center" > 
  <img src="./Screen-Shot.png" alt="TinkoffPay-rs" />

  &#xa0;

  <!-- <a href="https://tinkoffpay-rs.netlify.app">Demo</a> -->
</div>

&#xa0;
<a href="#top">Back to top</a>
