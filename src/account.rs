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


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WithdrawReq {
    pub wallet: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_deduct: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_rule_tos: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WithdrawResp {
    pub mts: i64,
    pub notification_type: String, 
    pub message_id: Option<i64>,
    pub _placeholder: Option<Value>,
    pub data: WithdrawData,
    pub code: Option<i64>,
    pub status: String,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WithdrawData {
    pub withdrawal_id: i64,
    pub _placehodler1: Option<Value>,
    pub method: String, 
    pub payment_id: Option<String>, 
    pub wallet: String,
    pub amount: f32, 
    pub _placeholder2: Option<Value>,
    pub _placeholder3: Option<Value>,
    pub withdraw_fee: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MovementReq {
    pub id: i64, 
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MovementResp {
    pub id: i64, 
    pub currency: String, 
    pub method: String, 
    pub _placehodler1: Option<Value>, 
    pub remark: Option<String>,
    pub mts_started: i64, 
    pub mts_updated: i64, 
    pub _placeholder2: Option<Value>,
    pub _placeholder3: Option<Value>,
    pub status: String, 
    pub _placeholder4: Option<Value>,
    pub _placeholder5: Option<Value>,
    pub amount: f64, 
    pub fees: f64, 
    pub _placeholder6: Option<Value>,
    pub _placeholder7: Option<Value>,
    pub destination_address: Option<String>, 
    pub memo: Option<String>, 
    pub _placeholder8: Option<Value>,
    pub _placeholder9: Option<Value>,
    pub transaction_id: Option<String>, 
    pub movement_note: Option<String>, 
    pub _placeholder10: Option<Value>,
    pub _placeholder11: Option<Value>,
    pub bank_fees: Option<f64>, 
    pub bank_router_id: Option<i64>,
    pub _placeholder12: Option<Value>,
    pub _placeholder13: Option<Value>,
    pub external_bank_mov_id: Option<String>,
    pub external_bank_mov_status: Option<String>, 
    pub external_bank_mov_description: Option<String>,
    pub external_bank_mov_acc_info: Option<Value>, 
}

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
        info!("Response: {}", data.as_str());
        let info: TransferResp = from_str(data.as_str())?;

        Ok(info)
    }

    pub fn withdraw(&self, req: WithdrawReq) -> Result<WithdrawResp> {
        let payload: String = serde_json::to_string(&req)?;
        let request: String = format!("withdraw");
        debug!("Payload: {payload}");

        let data = self.client.post_w_signed(request, payload)?;
        info!("Response: {}", data.as_str());
        let info: WithdrawResp = from_str(data.as_str())?;

        Ok(info)
    }

    pub fn movement_info(&self, req: MovementReq) -> Result<MovementResp> {
        let payload: String = serde_json::to_string(&req)?;
        let request: String = format!("movements/info");
        debug!("Payload: {payload}");

        let data = self.client.post_signed(request, payload)?;
        info!("Response: {}", data.as_str());
        let info: MovementResp = from_str(data.as_str())?;

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

    #[test]
    fn test_withdrawal_resp() {
        let data = "[1568742390999,\"acc_wd-req\",null,null,[13080092,null,\"ethereum\",null,\"exchange\",0.01,null,null,0.00135],null,\"SUCCESS\",\"Your withdrawal request has been successfully submitted.\"]";
        let _: WithdrawResp = from_str(data).expect("parsed"); 
    }

    #[test]
    fn test_movement_resp() {
        let data = "[24,\"EUR\",\"WIRE\",null,\"remark related to bank details\",1677086074000,1677086210000,null,null,\"COMPLETED\",null,null,-29.5,-0.5,null,null,null,null,null,null,null,\"testing note\",null,null,0,123,null,null,\"abcd-1234\",\"COMPLETED\",\"finished withdrawal in platform\",{\"router\":\"my-router\",\"meta\":{\"foo\":\"bar\"}}]";
        let _: MovementResp = from_str(data).expect("parsed"); 
    }
}
