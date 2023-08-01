use client::*;
use errors::*;
use log::*;
use serde_json::{from_str, Value};

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub wallet_type: String,
    pub currency: String,
    pub balance: f64,
    pub unsettled_interest: f64,
    pub balance_available: Option<f64>,
    pub last_change: Option<String>,
    pub trade_details: Option<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct MarginBase {
    key: String,
    pub margin: Base,
}

#[derive(Serialize, Deserialize)]
pub struct Base {
    pub user_profit_loss: f64,
    pub user_swaps: f64,
    pub margin_balance: f64,
    pub margin_net: f64,
}

#[derive(Serialize, Deserialize)]
pub struct MarginSymbol {
    key: String,
    symbol: String,
    pub margin: Symbol,
}

#[derive(Serialize, Deserialize)]
pub struct Symbol {
    pub tradable_balance: f64,
    pub gross_balance: f64,
    pub buy: f64,
    pub sell: f64,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FundingInfo {
    key: String,
    symbol: String,
    pub funding: Funding,
}

#[derive(Serialize, Deserialize)]
pub struct Funding {
    pub yield_loan: f64,
    pub yield_lend: f64,
    pub duration_loan: f64,
    pub duration_lend: f64,
}

#[derive(Clone)]
pub struct Account {
    client: Client,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InvoiceReq {
    pub wallet: String,
    pub currency: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InvoiceInfo {
    pub invoice_hash: String,
    pub invoice: String,
    pub _placeholder1: Option<String>,
    pub _placehodler2: Option<String>,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LnAddressReq {
    pub method: String, 
    pub wallet: String,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransferReq {
    pub from: String, 
    pub to: String, 
    pub currency: String, 
    pub currency_to: Option<String>,
    pub amount: String, 
    // pub email_dst: Option<String>, 
    // pub user_id_dst: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransferResp {
    pub mts: i64, 
    pub notification_type: String,
    #[serde(skip_serializing)]
    pub _unknown_field1: Option<String>,
    pub message_id: Option<String>, 
    pub info: TransferRespInfo,
    pub code: Option<String>,
    pub status: String,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransferRespInfo {
    pub mts_updated: Option<i64>,
    pub wallet_from: String, 
    pub wallet_to: String, 
    #[serde(skip_serializing)]
    pub _unknown_field1: Option<String>,
    pub currency: String, 
    pub currency_to: String, 
    #[serde(skip_serializing)]
    pub _unknown_field2: Option<String>,
    pub amount: f64,
}

// [1690900849181,"acc_tf",null,null,[1690900849181,"exchange","exchange",null,"LNX","BTC",null,0.00034754],null,"SUCCESS","0.00034754 Bitcoin (Lightning Network) transfered from Exchange to Exchange"]
impl Account {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Account {
            client: Client::new(api_key, secret_key),
        }
    }

    pub fn get_wallets(&self) -> Result<Vec<Wallet>> {
        let payload: String = format!("{}", "{}");
        let data = self.client.post_signed("wallets".into(), payload)?;
        debug!("Wallets response: {}", data.as_str());
        let wallets: Vec<Wallet> = from_str(data.as_str())?;

        Ok(wallets)
    }

    pub fn margin_base(&self) -> Result<MarginBase> {
        let payload: String = format!("{}", "{}");

        let data = self
            .client
            .post_signed("info/margin/base".into(), payload)?;

        let margin: MarginBase = from_str(data.as_str())?;

        Ok(margin)
    }

    pub fn margin_symbol<S>(&self, key: S) -> Result<MarginSymbol>
    where
        S: Into<String>,
    {
        let payload: String = format!("{}", "{}");
        let request: String = format!("info/margin/t{}", key.into());

        let data = self.client.post_signed(request, payload)?;

        let margin: MarginSymbol = from_str(data.as_str())?;

        Ok(margin)
    }

    pub fn funding_info<S>(&self, key: S) -> Result<FundingInfo>
    where
        S: Into<String>,
    {
        let payload: String = format!("{}", "{}");
        let request: String = format!("info/funding/f{}", key.into());

        let data = self.client.post_signed(request, payload)?;

        let info: FundingInfo = from_str(data.as_str())?;

        Ok(info)
    }

    // If this is the first time you are generating an LNX invoice on your account, you will first need to create a deposit address. To do this, call w/deposit/address with { method: 'LNX', wallet: 'exchange' }
    pub fn generate_invoice_address(&self)  -> Result<()> {
        let req = LnAddressReq {
            method: "LNX".to_owned(),
            wallet: "exchange".to_owned(),
        };
        let payload: String = serde_json::to_string(&req)?;
        let request: String = format!("deposit/address");

        debug!("Payload: {payload}");
        self.client.post_w_signed(request, payload)?;

        Ok(())
    }

    pub fn generate_invoice(&self, req: InvoiceReq) -> Result<InvoiceInfo> {
        let payload: String = serde_json::to_string(&req)?;
        let request: String = format!("deposit/invoice");
        debug!("Payload: {payload}");

        let data = self.client.post_w_signed(request, payload)?;

        let info: InvoiceInfo = from_str(data.as_str())?;

        Ok(info)
    }

    pub fn transfer(&self, req: TransferReq) -> Result<TransferResp> {
        let payload: String = serde_json::to_string(&req)?;
        let request: String = format!("transfer");
        debug!("Payload: {payload}");

        let data = self.client.post_w_signed(request, payload)?;

        let info: TransferResp = from_str(data.as_str())?;

        Ok(info)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_resp() {
        let data = "[1690901416558,\"acc_tf\",null,null,[1690901416558,\"exchange\",\"exchange\",null,\"LNX\",\"BTC\",null,0.00034774],null,\"SUCCESS\",\"0.00034774 Bitcoin (Lightning Network) transfered from Exchange to Exchange\"]";
        let _: TransferResp = from_str(data).expect("parsed"); 
    }
}