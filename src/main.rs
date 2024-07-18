use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

use self::randr::Randr;

mod randr;
mod storage_enums;

use cassandra_cpp::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cas_url = env::var("CASSANDRA_URL").context("CASSANDRA_URL not found")?;
    let cas_username = env::var("CASSANDRA_USERNAME").context("CASSANDRA_USERNAME not found")?;
    let cas_password = env::var("CASSANDRA_PASSWORD").context("CASSANDRA_PASSWORD not found")?;

    let mut cluster = Cluster::default();

    cluster
        .set_contact_points(&cas_url)?
        .set_credentials(&cas_username, &cas_password)?
        .set_load_balance_round_robin();

    let session = cluster.connect().await?;

    create_table(&session).await?;

    let output = add_data(&session).await?;

    retrieve_data(output.payment_id, output.attempt_id, &session).await?;

    Ok(())
}

async fn create_table(session: &Session) -> Result<(), Box<dyn std::error::Error>> {
    session.execute(include_str!("keyspace.cql")).await?;
    session.execute(include_str!("schema.cql")).await?;

    Ok(())
}

async fn add_data(session: &Session) -> Result<PaymentAttempt, Box<dyn std::error::Error>> {
    let mut statement = session.statement(include_str!("insert_query.cql"));

    let payment_attempt = PaymentAttempt::randr(None, None);

    payment_attempt.populate_statement(&mut statement)?;

    statement.execute().await?;

    Ok(payment_attempt)
}

async fn retrieve_data(
    payment_id: String,
    attempt_id: String,
    session: &Session,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut statement = session.statement(include_str!("select_query.cql"));

    statement.bind(0, payment_id.as_str())?;
    statement.bind(1, attempt_id.as_str())?;

    let rows = statement.execute().await?;

    let mut rows = rows.iter();

    let row = rows.next().context("No rows found")?;

    println!("{:#?}", row);

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct PaymentAttempt {
    pub payment_id: String,
    pub merchant_id: String,
    pub attempt_id: String,
    pub status: storage_enums::AttemptStatus,
    pub amount: i64,
    pub currency: Option<storage_enums::Currency>,
    pub save_to_locker: Option<bool>,
    pub connector: Option<String>,
    pub error_message: Option<String>,
    pub offer_amount: Option<i64>,
    pub surcharge_amount: Option<i64>,
    pub tax_amount: Option<i64>,
    pub payment_method_id: Option<String>,
    pub payment_method: Option<storage_enums::PaymentMethod>,
    pub connector_transaction_id: Option<String>,
    pub capture_method: Option<storage_enums::CaptureMethod>,
    pub capture_on: Option<time::PrimitiveDateTime>,
    pub confirm: bool,
    pub authentication_type: Option<storage_enums::AuthenticationType>,
    pub created_at: PrimitiveDateTime,
    pub modified_at: PrimitiveDateTime,
    pub last_synced: Option<PrimitiveDateTime>,
    pub cancellation_reason: Option<String>,
    pub amount_to_capture: Option<i64>,
    pub mandate_id: Option<String>,
    pub browser_info: Option<serde_json::Value>,
    pub error_code: Option<String>,
    pub payment_token: Option<String>,
    pub connector_metadata: Option<serde_json::Value>,
    pub payment_experience: Option<storage_enums::PaymentExperience>,
    pub payment_method_type: Option<storage_enums::PaymentMethodType>,
    pub payment_method_data: Option<serde_json::Value>,
    pub business_sub_label: Option<String>,
    pub straight_through_algorithm: Option<serde_json::Value>,
    pub preprocessing_step_id: Option<String>,
    // providing a location to store mandate details intermediately for transaction
    pub mandate_details: Option<storage_enums::MandateDataType>,
    pub error_reason: Option<String>,
    pub multiple_capture_count: Option<i16>,
    // reference to the payment at connector side
    pub connector_response_reference_id: Option<String>,
    pub amount_capturable: i64,
    pub updated_by: String,
    pub merchant_connector_id: Option<String>,
    pub authentication_data: Option<serde_json::Value>,
    pub encoded_data: Option<String>,
    pub unified_code: Option<String>,
    pub unified_message: Option<String>,
    pub net_amount: Option<i64>,
    pub external_three_ds_authentication_attempted: Option<bool>,
    pub authentication_connector: Option<String>,
    pub authentication_id: Option<String>,
    pub mandate_data: Option<storage_enums::MandateDetails>,
    pub fingerprint_id: Option<String>,
    pub payment_method_billing_address_id: Option<String>,
    pub charge_id: Option<String>,
    pub client_source: Option<String>,
    pub client_version: Option<String>,
}

impl Randr for PaymentAttempt {
    fn default() -> Self
    where
        Self: Sized,
    {
        Self {
            payment_id: Randr::randr(None, None),
            merchant_id: Randr::randr(None, None),
            attempt_id: Randr::randr(None, None),
            status: Randr::randr(None, None),
            amount: Randr::randr(None, None),
            currency: Randr::randr(None, None),
            save_to_locker: Randr::randr(None, None),
            connector: Randr::randr(None, None),
            error_message: Randr::randr(None, None),
            offer_amount: Randr::randr(None, None),
            surcharge_amount: Randr::randr(None, None),
            tax_amount: Randr::randr(None, None),
            payment_method_id: Randr::randr(None, None),
            payment_method: Randr::randr(None, None),
            connector_transaction_id: Randr::randr(None, None),
            capture_method: Randr::randr(None, None),
            capture_on: Randr::randr(None, None),
            confirm: Randr::randr(None, None),
            authentication_type: Randr::randr(None, None),
            created_at: Randr::randr(None, None),
            modified_at: Randr::randr(None, None),
            last_synced: Randr::randr(None, None),
            cancellation_reason: Randr::randr(None, None),
            amount_to_capture: Randr::randr(None, None),
            mandate_id: Randr::randr(None, None),
            browser_info: Randr::randr(None, None),
            error_code: Randr::randr(None, None),
            payment_token: Randr::randr(None, None),
            connector_metadata: Randr::randr(None, None),
            payment_experience: Randr::randr(None, None),
            payment_method_type: Randr::randr(None, None),
            payment_method_data: Randr::randr(None, None),
            business_sub_label: Randr::randr(None, None),
            straight_through_algorithm: Randr::randr(None, None),
            preprocessing_step_id: Randr::randr(None, None),
            mandate_details: Randr::randr(None, None),
            error_reason: Randr::randr(None, None),
            multiple_capture_count: Randr::randr(None, None),
            connector_response_reference_id: Randr::randr(None, None),
            amount_capturable: Randr::randr(None, None),
            updated_by: Randr::randr(None, None),
            merchant_connector_id: Randr::randr(None, None),
            authentication_data: Randr::randr(None, None),
            encoded_data: Randr::randr(None, None),
            unified_code: Randr::randr(None, None),
            unified_message: Randr::randr(None, None),
            net_amount: Randr::randr(None, None),
            external_three_ds_authentication_attempted: Randr::randr(None, None),
            authentication_connector: Randr::randr(None, None),
            authentication_id: Randr::randr(None, None),
            mandate_data: Randr::randr(None, None),
            fingerprint_id: Randr::randr(None, None),
            payment_method_billing_address_id: Randr::randr(None, None),
            charge_id: Randr::randr(None, None),
            client_source: Randr::randr(None, None),
            client_version: Randr::randr(None, None),
        }
    }
}

impl PaymentAttempt {
    fn populate_statement(&self, stmt: &mut Statement) -> Result<(), Box<dyn std::error::Error>> {
        stmt.bind(0, self.payment_id.as_str())?;
        stmt.bind(1, self.merchant_id.as_str())?;
        stmt.bind(2, self.attempt_id.as_str())?;
        stmt.bind(3, enum_parse(&self.status)?.as_str())?;
        stmt.bind(4, self.amount)?;
        for_opt(stmt, &self.currency, 5)?;
        e_for_opt(stmt, &self.save_to_locker, 6)?;

        opt_string(stmt, &self.connector, 7)?;
        opt_string(stmt, &self.error_message, 8)?;
        e_for_opt(stmt, &self.offer_amount, 9)?;
        e_for_opt(stmt, &self.surcharge_amount, 10)?;
        e_for_opt(stmt, &self.tax_amount, 11)?;
        opt_string(stmt, &self.payment_method_id, 12)?;
        for_opt(stmt, &self.payment_method, 13)?;
        opt_string(stmt, &self.connector_transaction_id, 14)?;
        for_opt(stmt, &self.capture_method, 15)?;
        for_opt(stmt, &self.capture_on, 16)?;
        stmt.bind(17, self.confirm)?;
        for_opt(stmt, &self.authentication_type, 18)?;
        stmt.bind(19, enum_parse(&self.created_at)?.as_str())?;
        stmt.bind(20, enum_parse(&self.modified_at)?.as_str())?;
        for_opt(stmt, &self.last_synced, 21)?;

        opt_string(stmt, &self.cancellation_reason, 22)?;
        e_for_opt(stmt, &self.amount_to_capture, 23)?;
        opt_string(stmt, &self.mandate_id, 24)?;
        for_opt(stmt, &self.browser_info, 25)?;
        opt_string(stmt, &self.error_code, 26)?;
        opt_string(stmt, &self.payment_token, 27)?;
        for_opt(stmt, &self.connector_metadata, 28)?;
        for_opt(stmt, &self.payment_experience, 29)?;
        for_opt(stmt, &self.payment_method_type, 30)?;
        for_opt(stmt, &self.payment_method_data, 31)?;
        opt_string(stmt, &self.business_sub_label, 32)?;

        for_opt(stmt, &self.straight_through_algorithm, 33)?;
        opt_string(stmt, &self.preprocessing_step_id, 34)?;
        for_opt(stmt, &self.mandate_details, 35)?;
        opt_string(stmt, &self.error_reason, 36)?;
        e_for_opt(stmt, &self.multiple_capture_count, 37)?;
        opt_string(stmt, &self.connector_response_reference_id, 38)?;
        stmt.bind(39, self.amount_capturable)?;
        stmt.bind(40, self.updated_by.as_str())?;
        opt_string(stmt, &self.merchant_connector_id, 41)?;
        for_opt(stmt, &self.authentication_data, 42)?;
        opt_string(stmt, &self.encoded_data, 43)?;
        opt_string(stmt, &self.unified_code, 44)?;
        opt_string(stmt, &self.unified_message, 45)?;
        e_for_opt(stmt, &self.net_amount, 46)?;
        e_for_opt(stmt, &self.external_three_ds_authentication_attempted, 47)?;
        opt_string(stmt, &self.authentication_connector, 48)?;
        opt_string(stmt, &self.authentication_id, 49)?;
        for_opt(stmt, &self.mandate_data, 50)?;
        opt_string(stmt, &self.fingerprint_id, 51)?;
        opt_string(stmt, &self.payment_method_billing_address_id, 52)?;
        opt_string(stmt, &self.charge_id, 53)?;

        opt_string(stmt, &self.client_source, 54)?;
        opt_string(stmt, &self.client_version, 55)?;

        Ok(())
    }
}

fn enum_parse<T: serde::Serialize>(em: &T) -> Result<String, Box<dyn std::error::Error>> {
    Ok(serde_json::to_string(em)?)
}

fn for_opt<T: serde::Serialize>(
    stat: &mut Statement,
    data: &Option<T>,
    loc: usize,
) -> Result<(), Box<dyn std::error::Error>>
where
{
    match data {
        Some(val) => stat.bind(loc, enum_parse(val)?.as_str())?,
        None => stat.bind_null(loc)?,
    };

    Ok(())
}

fn opt_string(
    stat: &mut Statement,
    data: &Option<String>,
    loc: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    match data {
        Some(val) => stat.bind(loc, val.as_str())?,
        None => stat.bind_null(loc)?,
    };

    Ok(())
}

fn e_for_opt<T: Clone>(
    stat: &mut Statement,
    data: &Option<T>,
    loc: usize,
) -> Result<(), Box<dyn std::error::Error>>
where
    cassandra_cpp::Statement: BindRustType<T>,
{
    match data {
        Some(val) => stat.bind(loc, val.clone())?,
        None => stat.bind_null(loc)?,
    };

    Ok(())
}
