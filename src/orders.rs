use client::*;
use errors::*;
use serde_json::{from_str, Value};
use log::*;

#[derive(Serialize, Deserialize)]
pub struct Order { 
    pub id: i64,   
    pub group_id: Option<i32>,                   
    pub client_id: i64,
    pub symbol: String,
    pub creation_timestamp: i64,
    pub update_timestamp: i64,
    pub amount: f64,
    pub amount_original: f64,
    pub order_type: String,
    pub previous_order_type: Option<String>,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    
    pub flags: Option<i32>,                   
    pub order_status: Option<String>,

    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,

    pub price: f64,
    pub price_avg: f64,
    pub price_trailing: Option<f64>,
    pub price_aux_limit: Option<f64>,
    
    #[serde(skip_serializing)]
    __placeholder_5: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_6: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_7: Option<String>,    
    
    pub notify: i32,
    pub hidden: i32,
    pub placed_id: Option<i32>                      
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubmitOrder {
    #[serde(rename="type")]
    pub order_type: String,
    pub symbol: String,
    pub amount: String, 
    pub price: String
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubmitOrderResp {
    pub mts: i64, 
    pub notification_type: String, 
    pub message_id: Option<String>, 
    #[serde(skip_serializing)]
    pub _placeholder1: Option<String>,
    pub data: SubmitOrderRespDataWrap,
    pub code: Option<String>, 
    pub status: String,
    pub text: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubmitOrderRespDataWrap {
    pub data: SubmitOrderRespData
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubmitOrderRespData {
    pub id: i64, 
    pub gid: Option<i64>,
    pub cid: Option<i64>,
    pub symbol: Option<String>,
    pub mts_create: i64,
    pub mts_update: i64,
    pub amount: f64, 
    pub amount_orig: f64,
    pub order_type: String, 
    pub type_prev: Option<String>,
    pub mts_tif: Option<i64>,
    #[serde(skip_serializing)]
    pub _placeholder1: Option<String>,
    pub flags: i64, 
    pub status: String, 
    #[serde(skip_serializing)]
    pub _placeholder2: Option<String>,
    #[serde(skip_serializing)]
    pub _placeholder3: Option<String>,
    pub price: f64, 
    pub price_avg: f64, 
    pub price_trailing: f64, 
    pub price_aux_limit: f64, 

    #[serde(skip_serializing)]
    pub _placeholder4: Option<String>,
    #[serde(skip_serializing)]
    pub _placeholder5: Option<String>,
    #[serde(skip_serializing)]
    pub _placeholder6: Option<String>,

    pub notify: i64, 
    pub hidden: i64, 
    pub placed_id: Option<String>,

    #[serde(skip_serializing)]
    pub _placeholder7: Option<String>,
    #[serde(skip_serializing)]
    pub _placeholder8: Option<String>,

    pub routing: Option<String>,

    #[serde(skip_serializing)]
    pub _placeholder9: Option<String>,
    #[serde(skip_serializing)]
    pub _placeholder10: Option<String>,

    pub meta: Option<Value>,
}
  
#[derive(Clone)]
pub struct Orders {
    client: Client,
}

impl Orders {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Orders {
            client: Client::new(api_key, secret_key),
        }
    }

    pub fn active_orders(&self) -> Result<Vec<Order>> {
        let payload: String = format!("{}", "{}");

        self.orders("orders".to_owned(), payload)
    }

    pub fn history<T>(&self, symbol: T) -> Result<Vec<Order>>
        where T: Into<Option<String>>
    {    
        let value = symbol.into().unwrap_or("".into());
        let payload: String = format!("{}", "{}");

        if value.is_empty() {
            return self.orders("orders/hist".into(), payload);
        } else {
            let request: String = format!("orders/t{}/hist", value);
            return self.orders(request, payload);
        }
    }

    pub fn orders<S>(&self, request: S, payload: S) -> Result<Vec<Order>>
        where S: Into<String>
    {    
        let data = self.client.post_signed(request.into(), payload.into())?;

        let orders: Vec<Order> = from_str(data.as_str())?;

        Ok(orders)
    }

    pub fn submit(&self, req: SubmitOrder) -> Result<SubmitOrderResp> {
        let payload: String = serde_json::to_string(&req)?;
        let request: String = format!("order/submit");
        debug!("Payload: {payload}");

        let data = self.client.post_w_signed(request, payload)?;
        info!("Response: {data}");

        let info: SubmitOrderResp = from_str(data.as_str())?;

        Ok(info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_resp() {
        let data = "[1690988463,\"on-req\",null,null,[[123836039427,null,1690988463421,\"tBTCUST\",1690988463421,1690988463421,-0.00034232,-0.00034232,\"EXCHANGE LIMIT\",null,null,null,0,\"ACTIVE\",null,null,29290,0,0,0,null,null,null,0,0,null,null,null,\"API>BFX\",null,null,{}]],null,\"SUCCESS\",\"Submitting 1 orders.\"]";
        let _: SubmitOrderResp = from_str(data).expect("parsed"); 
    }
}