/*
 * Rigetti QCS API
 *
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://gitlab.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://gitlab.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation.
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */

/// BillingInvoiceLine : A line item on an invoice representing a particular purchase (such as a reservation) or aggregate usage for the invoice period.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BillingInvoiceLine {
    #[serde(rename = "amount")]
    pub amount: i32,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "invoiceItem", skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<String>,
    #[serde(rename = "lineItemType")]
    pub line_item_type: LineItemType,
    #[serde(rename = "metadata")]
    pub metadata: ::std::collections::HashMap<String, String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<crate::models::BillingPrice>>,
    #[serde(rename = "quantity")]
    pub quantity: i32,
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    #[serde(rename = "subscriptionItem", skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<String>,
}

impl BillingInvoiceLine {
    /// A line item on an invoice representing a particular purchase (such as a reservation) or aggregate usage for the invoice period.
    pub fn new(
        amount: i32,
        description: String,
        id: String,
        line_item_type: LineItemType,
        metadata: ::std::collections::HashMap<String, String>,
        quantity: i32,
    ) -> BillingInvoiceLine {
        BillingInvoiceLine {
            amount,
            description,
            id,
            invoice_item: None,
            line_item_type,
            metadata,
            price: None,
            quantity,
            subscription: None,
            subscription_item: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LineItemType {
    #[serde(rename = "invoiceitem")]
    Invoiceitem,
    #[serde(rename = "subscription")]
    Subscription,
}

impl Default for LineItemType {
    fn default() -> LineItemType {
        Self::Invoiceitem
    }
}