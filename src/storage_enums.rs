use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

use crate::randr::Randr;

#[derive(Serialize, Deserialize)]
pub enum AttemptStatus {
    Started,
    AuthenticationFailed,
    RouterDeclined,
    AuthenticationPending,
    AuthenticationSuccessful,
    Authorized,
    AuthorizationFailed,
    Charged,
    Authorizing,
    CodInitiated,
    Voided,
    VoidInitiated,
    CaptureInitiated,
    CaptureFailed,
    VoidFailed,
    AutoRefunded,
    PartialCharged,
    PartialChargedAndChargeable,
    Unresolved,
    Pending,
    Failure,
    PaymentMethodAwaited,
    ConfirmationAwaited,
    DeviceDataCollectionPending,
}

#[derive(Serialize, Deserialize)]
pub enum Currency {
    AED,
    ALL,
    AMD,
    ANG,
    AOA,
    ARS,
    AUD,
    AWG,
    AZN,
    BAM,
    BBD,
    BDT,
    BGN,
    BHD,
    BIF,
    BMD,
    BND,
    BOB,
    BRL,
    BSD,
    BWP,
    BYN,
    BZD,
    CAD,
    CHF,
    CLP,
    CNY,
    COP,
    CRC,
    CUP,
    CVE,
    CZK,
    DJF,
    DKK,
    DOP,
    DZD,
    EGP,
    ETB,
    EUR,
    FJD,
    FKP,
    GBP,
    GEL,
    GHS,
    GIP,
    GMD,
    GNF,
    GTQ,
    GYD,
    HKD,
    HNL,
    HRK,
    HTG,
    HUF,
    IDR,
    ILS,
    INR,
    IQD,
    JMD,
    JOD,
    JPY,
    KES,
    KGS,
    KHR,
    KMF,
    KRW,
    KWD,
    KYD,
    KZT,
    LAK,
    LBP,
    LKR,
    LRD,
    LSL,
    LYD,
    MAD,
    MDL,
    MGA,
    MKD,
    MMK,
    MNT,
    MOP,
    MRU,
    MUR,
    MVR,
    MWK,
    MXN,
    MYR,
    MZN,
    NAD,
    NGN,
    NIO,
    NOK,
    NPR,
    NZD,
    OMR,
    PAB,
    PEN,
    PGK,
    PHP,
    PKR,
    PLN,
    PYG,
    QAR,
    RON,
    RSD,
    RUB,
    RWF,
    SAR,
    SBD,
    SCR,
    SEK,
    SGD,
    SHP,
    SLE,
    SLL,
    SOS,
    SRD,
    SSP,
    STN,
    SVC,
    SZL,
    THB,
    TND,
    TOP,
    TRY,
    TTD,
    TWD,
    TZS,
    UAH,
    UGX,
    USD,
    UYU,
    UZS,
    VES,
    VND,
    VUV,
    WST,
    XAF,
    XCD,
    XOF,
    XPF,
    YER,
    ZAR,
    ZMW,
}

#[derive(Serialize, Deserialize)]
pub enum PaymentMethod {
    Card,
    Token,
    PaymentProfile,
    Cash,
    Cheque,
    Interac,
    ApplePay,
    AndroidPay,
    #[serde(rename = "3d_secure")]
    ThreeDSecure,
    ProcessorToken,
}

#[derive(Serialize, Deserialize)]
pub enum CaptureMethod {
    /// Post the payment authorization, the capture will be executed on the full amount immediately
    Automatic,
    /// The capture will happen only if the merchant triggers a Capture API request
    Manual,
    /// The capture will happen only if the merchant triggers a Capture API request
    ManualMultiple,
    /// The capture can be scheduled to automatically get triggered at a specific date & time
    Scheduled,
}

#[derive(Serialize, Deserialize)]
pub enum AuthenticationType {
    /// If the card is enrolled for 3DS authentication, the 3DS based authentication will be activated. The liability of chargeback shift to the issuer
    ThreeDs,
    /// 3DS based authentication will not be activated. The liability of chargeback stays with the merchant.
    NoThreeDs,
}

#[derive(Serialize, Deserialize)]
pub enum PaymentExperience {
    /// The URL to which the customer needs to be redirected for completing the payment.
    RedirectToUrl,
    /// Contains the data for invoking the sdk client for completing the payment.
    InvokeSdkClient,
    /// The QR code data to be displayed to the customer.
    DisplayQrCode,
    /// Contains data to finish one click payment.
    OneClick,
    /// Redirect customer to link wallet
    LinkWallet,
    /// Contains the data for invoking the sdk client for completing the payment.
    InvokePaymentApp,
    /// Contains the data for displaying wait screen
    DisplayWaitScreen,
}

#[derive(Serialize, Deserialize)]
pub enum PaymentMethodType {
    Ach,
    Affirm,
    AfterpayClearpay,
    Alfamart,
    AliPay,
    AliPayHk,
    Alma,
    ApplePay,
    Atome,
    Bacs,
    BancontactCard,
    Becs,
    Benefit,
    Bizum,
    Blik,
    Boleto,
    BcaBankTransfer,
    BniVa,
    BriVa,
    CardRedirect,
    CimbVa,
    #[serde(rename = "classic")]
    ClassicReward,
    Credit,
    CryptoCurrency,
    Cashapp,
    Dana,
    DanamonVa,
    Debit,
    DuitNow,
    Efecty,
    Eps,
    Fps,
    Evoucher,
    Giropay,
    Givex,
    GooglePay,
    GoPay,
    Gcash,
    Ideal,
    Interac,
    Indomaret,
    Klarna,
    KakaoPay,
    LocalBankRedirect,
    MandiriVa,
    Knet,
    MbWay,
    MobilePay,
    Momo,
    MomoAtm,
    Multibanco,
    OnlineBankingThailand,
    OnlineBankingCzechRepublic,
    OnlineBankingFinland,
    OnlineBankingFpx,
    OnlineBankingPoland,
    OnlineBankingSlovakia,
    Oxxo,
    PagoEfectivo,
    PermataBankTransfer,
    OpenBankingUk,
    PayBright,
    Paypal,
    Pix,
    PaySafeCard,
    Przelewy24,
    PromptPay,
    Pse,
    RedCompra,
    RedPagos,
    SamsungPay,
    Sepa,
    Sofort,
    Swish,
    TouchNGo,
    Trustly,
    Twint,
    UpiCollect,
    UpiIntent,
    Vipps,
    VietQr,
    Venmo,
    Walley,
    WeChatPay,
    SevenEleven,
    Lawson,
    MiniStop,
    FamilyMart,
    Seicomart,
    PayEasy,
    LocalBankTransfer,
    Mifinity,
}

#[derive(Serialize, Deserialize)]
pub enum MandateDataType {
    SingleUse(MandateAmountData),
    MultiUse(Option<MandateAmountData>),
}

#[derive(Serialize, Deserialize)]
pub struct MandateAmountData {
    pub amount: i64,
    pub currency: Currency,
    pub start_date: Option<PrimitiveDateTime>,
    pub end_date: Option<PrimitiveDateTime>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct MandateDetails {
    pub update_mandate_id: Option<String>,
}

impl Randr for MandateDetails {
    fn default() -> Self
    where
        Self: Sized,
    {
        Self {
            update_mandate_id: Option::<String>::randr(None, None),
        }
    }
}

impl Randr for MandateAmountData {
    fn default() -> Self
    where
        Self: Sized,
    {
        Self {
            amount: i64::randr(None, None),
            currency: Currency::randr(None, None),
            start_date: Option::<PrimitiveDateTime>::randr(None, None),
            end_date: Option::<PrimitiveDateTime>::randr(None, None),
            metadata: Option::<serde_json::Value>::randr(None, None),
        }
    }
}

impl Randr for Currency {
    fn default() -> Self {
        Currency::USD
    }
}

impl Randr for AttemptStatus {
    fn default() -> Self {
        AttemptStatus::Started
    }
}

impl Randr for PaymentMethod {
    fn default() -> Self {
        PaymentMethod::Card
    }
}

impl Randr for CaptureMethod {
    fn default() -> Self {
        CaptureMethod::Automatic
    }
}

impl Randr for AuthenticationType {
    fn default() -> Self {
        AuthenticationType::ThreeDs
    }
}

impl Randr for PaymentExperience {
    fn default() -> Self {
        PaymentExperience::RedirectToUrl
    }
}

impl Randr for PaymentMethodType {
    fn default() -> Self {
        PaymentMethodType::CardRedirect
    }
}

impl Randr for MandateDataType {
    fn default() -> Self {
        match bool::randr(None, None) {
            true => Self::MultiUse(Randr::randr(None, None)),
            false => Self::SingleUse(Randr::randr(None, None)),
        }
    }
}
