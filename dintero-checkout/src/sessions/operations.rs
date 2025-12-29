//! Module implementation.

use super::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct CreateSessionRequestPayload {
    pub url: SessionUrl,
    pub order: Order,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_terms_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<SessionConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
}

impl From<CreateSessionRequest> for CreateSessionRequestPayload {
    fn from(req: CreateSessionRequest) -> Self {
        Self {
            url: req.url,
            order: req.order,
            profile_id: req.profile_id,
            return_url: req.return_url,
            merchant_terms_url: req.merchant_terms_url,
            configuration: req.configuration,
            customer: req.customer,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionListResponse {
    pub sessions: Vec<CheckoutSession>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ListSessionsParams {
    pub limit: Option<u32>,
    pub page_token: Option<String>,
}

impl ListSessionsParams {
    pub fn builder() -> ListSessionsParamsBuilder {
        ListSessionsParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ListSessionsParamsBuilder {
    limit: Option<u32>,
    page_token: Option<String>,
}

impl ListSessionsParamsBuilder {
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn page_token(mut self, token: impl Into<String>) -> Self {
        self.page_token = Some(token.into());
        self
    }

    pub fn build(self) -> ListSessionsParams {
        ListSessionsParams {
            limit: self.limit,
            page_token: self.page_token,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentOperation {
    Purchase,
    RecurringPurchase,
    UnscheduledPurchase,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaymentConfiguration {
    pub payment_product_type: String,
    pub operation: PaymentOperation,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateMerchantInitiatedSessionRequest {
    pub url: SessionUrl,
    pub order: Order,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<SessionConfiguration>,

    pub customer: Customer,

    pub payment: PaymentConfiguration,
}

impl CreateMerchantInitiatedSessionRequest {
    pub fn builder() -> CreateMerchantInitiatedSessionRequestBuilder {
        CreateMerchantInitiatedSessionRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateMerchantInitiatedSessionRequestBuilder {
    url: SessionUrl,
    order: Option<Order>,
    profile_id: Option<String>,
    configuration: Option<SessionConfiguration>,
    customer: Option<Customer>,
    payment_product_type: Option<String>,
    operation: Option<PaymentOperation>,
}

impl CreateMerchantInitiatedSessionRequestBuilder {
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    pub fn callback_url(mut self, url: impl Into<String>) -> Self {
        self.url.callback_url = Some(url.into());
        self
    }

    pub fn profile_id(mut self, id: impl Into<String>) -> Self {
        self.profile_id = Some(id.into());
        self
    }

    pub fn configuration(mut self, config: SessionConfiguration) -> Self {
        self.configuration = Some(config);
        self
    }

    pub fn auto_capture(mut self, auto_capture: bool) -> Self {
        let config = self.configuration.get_or_insert_with(SessionConfiguration::default);
        config.auto_capture = Some(auto_capture);
        self
    }

    pub fn customer(mut self, customer: Customer) -> Self {
        self.customer = Some(customer);
        self
    }

    pub fn with_payex_recurrence_token(mut self, token: impl Into<String>) -> Self {
        let customer = self.customer.get_or_insert_with(Customer::default);
        let tokens = customer.tokens.get_or_insert_with(CustomerTokens::default);
        let payex = tokens.payex.get_or_insert_with(PayexTokens::default);
        let creditcard = payex.creditcard.get_or_insert_with(PayexCreditcardTokens::default);
        creditcard.recurrence_token = Some(token.into());
        self.payment_product_type = Some("payex.creditcard".to_string());
        self
    }

    pub fn with_payex_payment_token(mut self, token: impl Into<String>) -> Self {
        let customer = self.customer.get_or_insert_with(Customer::default);
        let tokens = customer.tokens.get_or_insert_with(CustomerTokens::default);
        let payex = tokens.payex.get_or_insert_with(PayexTokens::default);
        let creditcard = payex.creditcard.get_or_insert_with(PayexCreditcardTokens::default);
        creditcard.payment_token = Some(token.into());
        self.payment_product_type = Some("payex.creditcard".to_string());
        self
    }

    pub fn with_bambora_payment_token(mut self, token: impl Into<String>) -> Self {
        let customer = self.customer.get_or_insert_with(Customer::default);
        let tokens = customer.tokens.get_or_insert_with(CustomerTokens::default);
        let bambora = tokens.bambora.get_or_insert_with(BamboraTokens::default);
        let creditcard = bambora.creditcard.get_or_insert_with(BamboraCreditcardTokens::default);
        creditcard.payment_token = Some(token.into());
        self.payment_product_type = Some("bambora.creditcard".to_string());
        self
    }

    pub fn with_dintero_psp_payment_token(mut self, token: impl Into<String>) -> Self {
        let customer = self.customer.get_or_insert_with(Customer::default);
        let tokens = customer.tokens.get_or_insert_with(CustomerTokens::default);
        let dintero_psp = tokens.dintero_psp.get_or_insert_with(DinteroPspTokens::default);
        let creditcard =
            dintero_psp.creditcard.get_or_insert_with(DinteroPspCreditcardTokens::default);
        creditcard.payment_token = Some(token.into());
        self.payment_product_type = Some("dintero_psp.creditcard".to_string());
        self
    }

    pub fn recurring_purchase(mut self) -> Self {
        self.operation = Some(PaymentOperation::RecurringPurchase);
        self
    }

    pub fn unscheduled_purchase(mut self) -> Self {
        self.operation = Some(PaymentOperation::UnscheduledPurchase);
        self
    }

    pub fn build(self) -> Result<CreateMerchantInitiatedSessionRequest, String> {
        let order = self.order.ok_or("order is required")?;
        let customer = self.customer.ok_or("customer with tokens is required")?;
        let payment_product_type = self
            .payment_product_type
            .ok_or("payment_product_type is required (set a token first)")?;
        let operation = self
            .operation
            .ok_or("operation is required (call recurring_purchase() or unscheduled_purchase())")?;

        Ok(CreateMerchantInitiatedSessionRequest {
            url: self.url,
            order,
            profile_id: self.profile_id,
            configuration: self.configuration,
            customer,
            payment: PaymentConfiguration { payment_product_type, operation },
        })
    }
}
