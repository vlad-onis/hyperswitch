pub mod authorize_flow;
pub mod cancel_flow;
pub mod capture_flow;
pub mod complete_authorize_flow;
pub mod psync_flow;
pub mod session_flow;
pub mod verfiy_flow;

use async_trait::async_trait;

use crate::{
    connector,
    core::{
        errors::{ConnectorError, CustomResult, RouterResult},
        payments,
    },
    routes::AppState,
    services,
    types::{self, api, storage},
};

#[async_trait]
pub trait ConstructFlowSpecificData<F, Req, Res> {
    async fn construct_router_data<'a>(
        &self,
        state: &AppState,
        connector_id: &str,
        merchant_account: &storage::MerchantAccount,
        customer: &Option<storage::Customer>,
    ) -> RouterResult<types::RouterData<F, Req, Res>>;
}

#[async_trait]
pub trait Feature<F, T> {
    async fn decide_flows<'a>(
        self,
        state: &AppState,
        connector: &api::ConnectorData,
        maybe_customer: &Option<storage::Customer>,
        call_connector_action: payments::CallConnectorAction,
        merchant_account: &storage::MerchantAccount,
    ) -> RouterResult<Self>
    where
        Self: Sized,
        F: Clone,
        dyn api::Connector: services::ConnectorIntegration<F, T, types::PaymentsResponseData>;

    async fn add_access_token<'a>(
        &self,
        state: &AppState,
        connector: &api::ConnectorData,
        merchant_account: &storage::MerchantAccount,
    ) -> RouterResult<types::AddAccessTokenResult>
    where
        F: Clone,
        Self: Sized,
        dyn api::Connector: services::ConnectorIntegration<F, T, types::PaymentsResponseData>;

    async fn add_payment_method_token<'a>(
        &self,
        _state: &AppState,
        _connector: &api::ConnectorData,
        _tokenization_action: &payments::TokenizationAction,
    ) -> RouterResult<Option<String>>
    where
        F: Clone,
        Self: Sized,
        dyn api::Connector: services::ConnectorIntegration<F, T, types::PaymentsResponseData>,
    {
        Ok(None)
    }

    async fn preprocessing_steps<'a>(
        self,
        _state: &AppState,
        _connector: &api::ConnectorData,
    ) -> RouterResult<Self>
    where
        F: Clone,
        Self: Sized,
        dyn api::Connector: services::ConnectorIntegration<F, T, types::PaymentsResponseData>,
    {
        Ok(self)
    }

    async fn create_connector_customer<'a>(
        &self,
        _state: &AppState,
        _connector: &api::ConnectorData,
        _connector_customer_map: Option<serde_json::Map<String, serde_json::Value>>,
    ) -> RouterResult<(Option<String>, Option<storage::CustomerUpdate>)>
    where
        F: Clone,
        Self: Sized,
        dyn api::Connector: services::ConnectorIntegration<F, T, types::PaymentsResponseData>,
    {
        Ok((None, None))
    }
}

macro_rules! default_imp_for_complete_authorize{
    ($($path:ident::$connector:ident),*)=> {
        $(
            impl api::PaymentsCompleteAuthorize for $path::$connector {}
            impl
            services::ConnectorIntegration<
            api::CompleteAuthorize,
            types::CompleteAuthorizeData,
            types::PaymentsResponseData,
        > for $path::$connector
        {}
    )*
    };
}

#[cfg(feature = "dummy_connector")]
impl<const T: u8> api::PaymentsCompleteAuthorize for connector::DummyConnector<T> {}
#[cfg(feature = "dummy_connector")]
impl<const T: u8>
    services::ConnectorIntegration<
        api::CompleteAuthorize,
        types::CompleteAuthorizeData,
        types::PaymentsResponseData,
    > for connector::DummyConnector<T>
{
}

default_imp_for_complete_authorize!(
    connector::Aci,
    connector::Adyen,
    connector::Authorizedotnet,
    connector::Bitpay,
    connector::Braintree,
    connector::Checkout,
    connector::Coinbase,
    connector::Cybersource,
    connector::Dlocal,
    connector::Fiserv,
    connector::Forte,
    connector::Iatapay,
    connector::Klarna,
    connector::Multisafepay,
    connector::Nexinets,
    connector::Nmi,
    connector::Noon,
    connector::Opennode,
    connector::Payeezy,
    connector::Payu,
    connector::Rapyd,
    connector::Stripe,
    connector::Trustpay,
    connector::Worldline,
    connector::Worldpay,
    connector::Zen
);

macro_rules! default_imp_for_create_customer{
    ($($path:ident::$connector:ident),*)=> {
        $(
            impl api::ConnectorCustomer for $path::$connector {}
            impl
            services::ConnectorIntegration<
            api::CreateConnectorCustomer,
            types::ConnectorCustomerData,
            types::PaymentsResponseData,
        > for $path::$connector
        {}
    )*
    };
}

#[cfg(feature = "dummy_connector")]
impl<const T: u8> api::ConnectorCustomer for connector::DummyConnector<T> {}
#[cfg(feature = "dummy_connector")]
impl<const T: u8>
    services::ConnectorIntegration<
        api::CreateConnectorCustomer,
        types::ConnectorCustomerData,
        types::PaymentsResponseData,
    > for connector::DummyConnector<T>
{
}

default_imp_for_create_customer!(
    connector::Aci,
    connector::Adyen,
    connector::Airwallex,
    connector::Authorizedotnet,
    connector::Bambora,
    connector::Bitpay,
    connector::Braintree,
    connector::Checkout,
    connector::Coinbase,
    connector::Cybersource,
    connector::Dlocal,
    connector::Fiserv,
    connector::Forte,
    connector::Globalpay,
    connector::Iatapay,
    connector::Klarna,
    connector::Mollie,
    connector::Multisafepay,
    connector::Nexinets,
    connector::Nmi,
    connector::Noon,
    connector::Nuvei,
    connector::Opennode,
    connector::Payeezy,
    connector::Paypal,
    connector::Payu,
    connector::Rapyd,
    connector::Shift4,
    connector::Trustpay,
    connector::Worldline,
    connector::Worldpay,
    connector::Zen
);

macro_rules! default_imp_for_connector_redirect_response{
    ($($path:ident::$connector:ident),*)=> {
        $(
            impl services::ConnectorRedirectResponse for $path::$connector {
                fn get_flow_type(
                    &self,
                    _query_params: &str,
                    _json_payload: Option<serde_json::Value>,
                    _action: services::PaymentAction
                ) -> CustomResult<payments::CallConnectorAction, ConnectorError> {
                    Ok(payments::CallConnectorAction::Trigger)
                }
            }
    )*
    };
}

#[cfg(feature = "dummy_connector")]
impl<const T: u8> services::ConnectorRedirectResponse for connector::DummyConnector<T> {
    fn get_flow_type(
        &self,
        _query_params: &str,
        _json_payload: Option<serde_json::Value>,
        _action: services::PaymentAction,
    ) -> CustomResult<payments::CallConnectorAction, ConnectorError> {
        Ok(payments::CallConnectorAction::Trigger)
    }
}

default_imp_for_connector_redirect_response!(
    connector::Aci,
    connector::Adyen,
    connector::Authorizedotnet,
    connector::Bitpay,
    connector::Braintree,
    connector::Coinbase,
    connector::Cybersource,
    connector::Dlocal,
    connector::Fiserv,
    connector::Forte,
    connector::Iatapay,
    connector::Klarna,
    connector::Multisafepay,
    connector::Nexinets,
    connector::Nmi,
    connector::Opennode,
    connector::Payeezy,
    connector::Payu,
    connector::Rapyd,
    connector::Shift4,
    connector::Worldline,
    connector::Worldpay
);

macro_rules! default_imp_for_connector_request_id{
    ($($path:ident::$connector:ident),*)=> {
        $(
            impl api::ConnectorTransactionId for $path::$connector {}
    )*
    };
}

#[cfg(feature = "dummy_connector")]
impl<const T: u8> api::ConnectorTransactionId for connector::DummyConnector<T> {}

default_imp_for_connector_request_id!(
    connector::Aci,
    connector::Adyen,
    connector::Airwallex,
    connector::Authorizedotnet,
    connector::Bambora,
    connector::Bitpay,
    connector::Bluesnap,
    connector::Braintree,
    connector::Checkout,
    connector::Coinbase,
    connector::Cybersource,
    connector::Dlocal,
    connector::Fiserv,
    connector::Forte,
    connector::Globalpay,
    connector::Iatapay,
    connector::Klarna,
    connector::Mollie,
    connector::Multisafepay,
    connector::Nmi,
    connector::Noon,
    connector::Nuvei,
    connector::Opennode,
    connector::Payeezy,
    connector::Payu,
    connector::Rapyd,
    connector::Shift4,
    connector::Stripe,
    connector::Trustpay,
    connector::Worldline,
    connector::Worldpay,
    connector::Zen
);

macro_rules! default_imp_for_accept_dispute{
    ($($path:ident::$connector:ident),*)=> {
        $(
            impl api::Dispute for $path::$connector {}
            impl api::AcceptDispute for $path::$connector {}
            impl
                services::ConnectorIntegration<
                api::Accept,
                types::AcceptDisputeRequestData,
                types::AcceptDisputeResponse,
            > for $path::$connector
            {}
    )*
    };
}

#[cfg(feature = "dummy_connector")]
impl<const T: u8> api::Dispute for connector::DummyConnector<T> {}
#[cfg(feature = "dummy_connector")]
impl<const T: u8> api::AcceptDispute for connector::DummyConnector<T> {}
#[cfg(feature = "dummy_connector")]
impl<const T: u8>
    services::ConnectorIntegration<
        api::Accept,
        types::AcceptDisputeRequestData,
        types::AcceptDisputeResponse,
    > for connector::DummyConnector<T>
{
}

default_imp_for_accept_dispute!(
    connector::Aci,
    connector::Adyen,
    connector::Airwallex,
    connector::Authorizedotnet,
    connector::Bambora,
    connector::Bitpay,
    connector::Bluesnap,
    connector::Braintree,
    connector::Coinbase,
    connector::Cybersource,
    connector::Dlocal,
    connector::Fiserv,
    connector::Forte,
    connector::Globalpay,
    connector::Iatapay,
    connector::Klarna,
    connector::Mollie,
    connector::Multisafepay,
    connector::Nexinets,
    connector::Nmi,
    connector::Noon,
    connector::Nuvei,
    connector::Payeezy,
    connector::Paypal,
    connector::Payu,
    connector::Rapyd,
    connector::Shift4,
    connector::Stripe,
    connector::Trustpay,
    connector::Opennode,
    connector::Worldline,
    connector::Worldpay,
    connector::Zen
);

macro_rules! default_imp_for_file_upload{
    ($($path:ident::$connector:ident),*)=> {
        $(
            impl api::FileUpload for $path::$connector {}
            impl api::UploadFile for $path::$connector {}
            impl
                services::ConnectorIntegration<
                api::Upload,
                types::UploadFileRequestData,
                types::UploadFileResponse,
            > for $path::$connector
            {}
            impl api::RetrieveFile for $path::$connector {}
            impl
                services::ConnectorIntegration<
                api::Retrieve,
                types::RetrieveFileRequestData,
                types::RetrieveFileResponse,
            > for $path::$connector
            {}
    )*
    };
}

#[cfg(feature = "dummy_connector")]
impl<const T: u8> api::FileUpload for connector::DummyConnector<T> {}
#[cfg(feature = "dummy_connector")]
impl<const T: u8> api::UploadFile for connector::DummyConnector<T> {}
#[cfg(feature = "dummy_connector")]
impl<const T: u8>
    services::ConnectorIntegration<
        api::Upload,
        types::UploadFileRequestData,
        types::UploadFileResponse,
    > for connector::DummyConnector<T>
{
}
#[cfg(feature = "dummy_connector")]
impl<const T: u8> api::RetrieveFile for connector::DummyConnector<T> {}
#[cfg(feature = "dummy_connector")]
impl<const T: u8>
    services::ConnectorIntegration<
        api::Retrieve,
        types::RetrieveFileRequestData,
        types::RetrieveFileResponse,
    > for connector::DummyConnector<T>
{
}

default_imp_for_file_upload!(
    connector::Aci,
    connector::Adyen,
    connector::Airwallex,
    connector::Authorizedotnet,
    connector::Bambora,
    connector::Bitpay,
    connector::Bluesnap,
    connector::Braintree,
    connector::Coinbase,
    connector::Cybersource,
    connector::Dlocal,
    connector::Fiserv,
    connector::Forte,
    connector::Globalpay,
    connector::Iatapay,
    connector::Klarna,
    connector::Mollie,
    connector::Multisafepay,
    connector::Nexinets,
    connector::Nmi,
    connector::Noon,
    connector::Nuvei,
    connector::Payeezy,
    connector::Paypal,
    connector::Payu,
    connector::Rapyd,
    connector::Shift4,
    connector::Trustpay,
    connector::Opennode,
    connector::Worldline,
    connector::Worldpay,
    connector::Zen
);

macro_rules! default_imp_for_submit_evidence{
    ($($path:ident::$connector:ident),*)=> {
        $(
            impl api::SubmitEvidence for $path::$connector {}
            impl
                services::ConnectorIntegration<
                api::Evidence,
                types::SubmitEvidenceRequestData,
                types::SubmitEvidenceResponse,
            > for $path::$connector
            {}
    )*
    };
}

#[cfg(feature = "dummy_connector")]
impl<const T: u8> api::SubmitEvidence for connector::DummyConnector<T> {}
#[cfg(feature = "dummy_connector")]
impl<const T: u8>
    services::ConnectorIntegration<
        api::Evidence,
        types::SubmitEvidenceRequestData,
        types::SubmitEvidenceResponse,
    > for connector::DummyConnector<T>
{
}

default_imp_for_submit_evidence!(
    connector::Aci,
    connector::Adyen,
    connector::Airwallex,
    connector::Authorizedotnet,
    connector::Bambora,
    connector::Bitpay,
    connector::Bluesnap,
    connector::Braintree,
    connector::Cybersource,
    connector::Coinbase,
    connector::Dlocal,
    connector::Fiserv,
    connector::Forte,
    connector::Globalpay,
    connector::Iatapay,
    connector::Klarna,
    connector::Mollie,
    connector::Multisafepay,
    connector::Nexinets,
    connector::Nmi,
    connector::Noon,
    connector::Nuvei,
    connector::Payeezy,
    connector::Paypal,
    connector::Payu,
    connector::Rapyd,
    connector::Shift4,
    connector::Trustpay,
    connector::Opennode,
    connector::Worldline,
    connector::Worldpay,
    connector::Zen
);

macro_rules! default_imp_for_defend_dispute{
    ($($path:ident::$connector:ident),*)=> {
        $(
            impl api::DefendDispute for $path::$connector {}
            impl
                services::ConnectorIntegration<
                api::Defend,
                types::DefendDisputeRequestData,
                types::DefendDisputeResponse,
            > for $path::$connector
            {}
    )*
    };
}

#[cfg(feature = "dummy_connector")]
impl<const T: u8> api::DefendDispute for connector::DummyConnector<T> {}
#[cfg(feature = "dummy_connector")]
impl<const T: u8>
    services::ConnectorIntegration<
        api::Defend,
        types::DefendDisputeRequestData,
        types::DefendDisputeResponse,
    > for connector::DummyConnector<T>
{
}

default_imp_for_defend_dispute!(
    connector::Aci,
    connector::Adyen,
    connector::Airwallex,
    connector::Authorizedotnet,
    connector::Bambora,
    connector::Bitpay,
    connector::Bluesnap,
    connector::Braintree,
    connector::Cybersource,
    connector::Coinbase,
    connector::Dlocal,
    connector::Fiserv,
    connector::Forte,
    connector::Globalpay,
    connector::Iatapay,
    connector::Klarna,
    connector::Mollie,
    connector::Multisafepay,
    connector::Nexinets,
    connector::Nmi,
    connector::Noon,
    connector::Nuvei,
    connector::Payeezy,
    connector::Paypal,
    connector::Payu,
    connector::Rapyd,
    connector::Stripe,
    connector::Shift4,
    connector::Trustpay,
    connector::Opennode,
    connector::Worldline,
    connector::Worldpay,
    connector::Zen
);

macro_rules! default_imp_for_pre_processing_steps{
    ($($path:ident::$connector:ident),*)=> {
        $(
            impl api::PaymentsPreProcessing for $path::$connector {}
            impl
            services::ConnectorIntegration<
            api::PreProcessing,
            types::PaymentsPreProcessingData,
            types::PaymentsResponseData,
        > for $path::$connector
        {}
    )*
    };
}

#[cfg(feature = "dummy_connector")]
impl<const T: u8> api::PaymentsPreProcessing for connector::DummyConnector<T> {}
#[cfg(feature = "dummy_connector")]
impl<const T: u8>
    services::ConnectorIntegration<
        api::PreProcessing,
        types::PaymentsPreProcessingData,
        types::PaymentsResponseData,
    > for connector::DummyConnector<T>
{
}

default_imp_for_pre_processing_steps!(
    connector::Aci,
    connector::Adyen,
    connector::Airwallex,
    connector::Authorizedotnet,
    connector::Bambora,
    connector::Bitpay,
    connector::Bluesnap,
    connector::Braintree,
    connector::Checkout,
    connector::Coinbase,
    connector::Cybersource,
    connector::Dlocal,
    connector::Iatapay,
    connector::Fiserv,
    connector::Forte,
    connector::Globalpay,
    connector::Klarna,
    connector::Mollie,
    connector::Multisafepay,
    connector::Nexinets,
    connector::Nmi,
    connector::Noon,
    connector::Nuvei,
    connector::Opennode,
    connector::Payeezy,
    connector::Paypal,
    connector::Payu,
    connector::Rapyd,
    connector::Shift4,
    connector::Trustpay,
    connector::Worldline,
    connector::Worldpay,
    connector::Zen
);
