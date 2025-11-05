use crate::transactions::*;
use async_trait::async_trait;

pub type Result<T> = std::result::Result<T, PaymentsError>;

#[derive(Debug, thiserror::Error)]
pub enum PaymentsError {
    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Client error: {0}")]
    Client(String),
}

impl From<serde_json::Error> for PaymentsError {
    fn from(err: serde_json::Error) -> Self {
        PaymentsError::Serialization(err.to_string())
    }
}

#[async_trait]
pub trait PaymentsOperations: Send + Sync {
    async fn get_transaction(&self, transaction_id: &str) -> Result<Transaction>;
    async fn list_transactions(
        &self,
        params: ListTransactionsParams,
    ) -> Result<TransactionListResponse>;
    async fn update_transaction(
        &self,
        transaction_id: &str,
        request: UpdateTransactionRequest,
    ) -> Result<Transaction>;

    async fn capture_transaction(
        &self,
        transaction_id: &str,
        request: CaptureTransactionRequest,
    ) -> Result<Transaction>;
    async fn refund_transaction(
        &self,
        transaction_id: &str,
        request: RefundTransactionRequest,
    ) -> Result<Transaction>;
    async fn void_transaction(
        &self,
        transaction_id: &str,
        request: VoidTransactionRequest,
    ) -> Result<Transaction>;
    async fn extend_authorization(
        &self,
        transaction_id: &str,
        request: ExtendAuthorizationRequest,
    ) -> Result<Transaction>;
}

#[async_trait]
pub trait PaymentsAdapter: Send + Sync {
    async fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T>;
    async fn post_json<T: serde::de::DeserializeOwned, B: serde::Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T>;
    async fn put_json<T: serde::de::DeserializeOwned, B: serde::Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T>;
}

pub struct PaymentsClient<A: PaymentsAdapter> {
    adapter: A,
    account_id: String,
}

impl<A: PaymentsAdapter> PaymentsClient<A> {
    pub fn new(adapter: A, account_id: String) -> Self {
        Self {
            adapter,
            account_id,
        }
    }
}

#[async_trait]
impl<A: PaymentsAdapter> PaymentsOperations for PaymentsClient<A> {
    async fn get_transaction(&self, transaction_id: &str) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}",
            self.account_id, transaction_id
        );
        self.adapter.get_json(&path).await
    }

    async fn list_transactions(
        &self,
        params: ListTransactionsParams,
    ) -> Result<TransactionListResponse> {
        let mut path = format!("accounts/{}/transactions", self.account_id);

        let mut query_params = Vec::new();
        if let Some(limit) = params.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(token) = params.page_token {
            query_params.push(format!("page_token={}", token));
        }
        if let Some(status) = params.status {
            query_params.push(format!("status={:?}", status));
        }
        if let Some(reference) = params.merchant_reference {
            query_params.push(format!("merchant_reference={}", reference));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.adapter.get_json(&path).await
    }

    async fn update_transaction(
        &self,
        transaction_id: &str,
        request: UpdateTransactionRequest,
    ) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}",
            self.account_id, transaction_id
        );
        self.adapter.put_json(&path, &request).await
    }

    async fn capture_transaction(
        &self,
        transaction_id: &str,
        request: CaptureTransactionRequest,
    ) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}/capture",
            self.account_id, transaction_id
        );
        self.adapter.post_json(&path, &request).await
    }

    async fn refund_transaction(
        &self,
        transaction_id: &str,
        request: RefundTransactionRequest,
    ) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}/refund",
            self.account_id, transaction_id
        );
        self.adapter.post_json(&path, &request).await
    }

    async fn void_transaction(
        &self,
        transaction_id: &str,
        request: VoidTransactionRequest,
    ) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}/void",
            self.account_id, transaction_id
        );
        self.adapter.post_json(&path, &request).await
    }

    async fn extend_authorization(
        &self,
        transaction_id: &str,
        request: ExtendAuthorizationRequest,
    ) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}/extend_authorization",
            self.account_id, transaction_id
        );
        self.adapter.post_json(&path, &request).await
    }
}
