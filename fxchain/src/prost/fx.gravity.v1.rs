/// Attestation is an aggregate of `claims` that eventually becomes `observed` by
/// all orchestrators
/// EVENT_NONCE:
/// EventNonce a nonce provided by the gravity contract that is unique per event fired
/// These event nonces must be relayed in order. This is a correctness issue,
/// if relaying out of order transaction replay attacks become possible
/// OBSERVED:
/// Observed indicates that >67% of validators have attested to the event,
/// and that the event should be executed by the gravity state machine
///
/// The actual content of the claims is passed in with the transaction making the claim
/// and then passed through the call stack alongside the attestation while it is processed
/// the key in which the attestation is stored is keyed on the exact details of the claim
/// but there is no reason to store those exact details becuause the next message sender
/// will kindly provide you with them.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attestation {
    #[prost(bool, tag = "1")]
    pub observed: bool,
    #[prost(string, repeated, tag = "2")]
    pub votes: ::std::vec::Vec<std::string::String>,
    #[prost(uint64, tag = "3")]
    pub height: u64,
    #[prost(message, optional, tag = "4")]
    pub claim: ::std::option::Option<::prost_types::Any>,
}

/// ERC20Token unique identifier for an Ethereum ERC20 token.
/// CONTRACT:
/// The contract address on ETH of the token, this could be a Cosmos
/// originated token, if so it will be the ERC20 address of the representation
/// (note: developers should look up the token symbol using the address on ETH to display for UI)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Token {
    #[prost(string, tag = "1")]
    pub contract: std::string::String,
    #[prost(string, tag = "2")]
    pub amount: std::string::String,
}

/// ClaimType is the cosmos type of an event from the counterpart chain that can
/// be handled
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClaimType {
    Unspecified = 0,
    Deposit = 1,
    Withdraw = 2,
    OriginatedToken = 3,
    ValsetUpdated = 4,
}

/// IDSet represents a set of IDs
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdSet {
    #[prost(uint64, repeated, tag = "1")]
    pub ids: ::std::vec::Vec<u64>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchFees {
    #[prost(string, tag = "1")]
    pub token_contract: std::string::String,
    #[prost(string, tag = "2")]
    pub total_fees: std::string::String,
    #[prost(uint64, tag = "3")]
    pub total_txs: u64,
}

/// OutgoingTxBatch represents a batch of transactions going from gravity to ETH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutgoingTxBatch {
    #[prost(uint64, tag = "1")]
    pub batch_nonce: u64,
    #[prost(uint64, tag = "2")]
    pub batch_timeout: u64,
    #[prost(message, repeated, tag = "3")]
    pub transactions: ::std::vec::Vec<OutgoingTransferTx>,
    #[prost(string, tag = "4")]
    pub token_contract: std::string::String,
    #[prost(uint64, tag = "5")]
    pub block: u64,
    #[prost(string, tag = "6")]
    pub fee_receive: std::string::String,
}

/// OutgoingTransferTx represents an individual send from gravity to ETH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutgoingTransferTx {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub sender: std::string::String,
    #[prost(string, tag = "3")]
    pub dest_address: std::string::String,
    #[prost(message, optional, tag = "4")]
    pub erc20_token: ::std::option::Option<Erc20Token>,
    #[prost(message, optional, tag = "5")]
    pub erc20_fee: ::std::option::Option<Erc20Token>,
}

/// BridgeValidator represents a validator's ETH address and its power
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BridgeValidator {
    #[prost(uint64, tag = "1")]
    pub power: u64,
    #[prost(string, tag = "2")]
    pub eth_address: std::string::String,
}

/// Valset is the Ethereum Bridge Multsig Set, each gravity validator also
/// maintains an ETH key to sign messages, these are used to check signatures on
/// ETH because of the significant gas savings
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Valset {
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
    #[prost(message, repeated, tag = "2")]
    pub members: ::std::vec::Vec<BridgeValidator>,
    #[prost(uint64, tag = "3")]
    pub height: u64,
}

/// LastObservedEthereumBlockHeight stores the last observed
/// Ethereum block height along with the fx block height that
/// it was observed at. These two numbers can be used to project
/// outward and always produce batches with timeouts in the future
/// even if no Ethereum block height has been relayed for a long time
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastObservedEthereumBlockHeight {
    #[prost(uint64, tag = "1")]
    pub fx_block_height: u64,
    #[prost(uint64, tag = "2")]
    pub eth_block_height: u64,
}

/// This records the relationship between an ERC20 token and the denom
/// of the corresponding fx originated asset
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20ToDenom {
    #[prost(string, tag = "1")]
    pub erc20: std::string::String,
    #[prost(string, tag = "2")]
    pub denom: std::string::String,
}

/// MsgSetOrchestratorAddress
/// this message allows validators to delegate their voting responsibilities
/// to a given key. This key is then used as an optional authentication method
/// for sigining oracle claims
/// VALIDATOR
/// The validator field is a cosmosvaloper1... string (i.e. sdk.ValAddress)
/// that references a validator in the active set
/// ORCHESTRATOR
/// The orchestrator field is a cosmos1... string  (i.e. sdk.AccAddress) that
/// references the key that is being delegated to
/// ETH_ADDRESS
/// This is a hex encoded 0x Ethereum public key that will be used by this validator
/// on Ethereum
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetOrchestratorAddress {
    #[prost(string, tag = "1")]
    pub validator: std::string::String,
    #[prost(string, tag = "2")]
    pub orchestrator: std::string::String,
    #[prost(string, tag = "3")]
    pub eth_address: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetOrchestratorAddressResponse {}

/// MsgValsetConfirm
/// this is the message sent by the validators when they wish to submit their
/// signatures over the validator set at a given block height. A validator must
/// first call MsgSetEthAddress to set their Ethereum address to be used for
/// signing. Then someone (anyone) must make a ValsetRequest, the request is
/// essentially a messaging mechanism to determine which block all validators
/// should submit signatures over. Finally validators sign the validator set,
/// powers, and Ethereum addresses of the entire validator set at the height of a
/// ValsetRequest and submit that signature with this message.
///
/// If a sufficient number of validators (66% of voting power) (A) have set
/// Ethereum addresses and (B) submit ValsetConfirm messages with their
/// signatures it is then possible for anyone to view these signatures in the
/// chain store and submit them to Ethereum to update the validator set
/// -------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValsetConfirm {
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
    #[prost(string, tag = "2")]
    pub orchestrator: std::string::String,
    #[prost(string, tag = "3")]
    pub eth_address: std::string::String,
    #[prost(string, tag = "4")]
    pub signature: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValsetConfirmResponse {}

/// MsgSendToEth
/// This is the message that a user calls when they want to bridge an asset
/// it will later be removed when it is included in a batch and successfully
/// submitted tokens are removed from the users balance immediately
/// -------------
/// AMOUNT:
/// the coin to send across the bridge, note the restriction that this is a
/// single coin not a set of coins that is normal in other Cosmos messages
/// FEE:
/// the fee paid for the bridge, distinct from the fee paid to the chain to
/// actually send this message in the first place. So a successful send has
/// two layers of fees for the user
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendToEth {
    #[prost(string, tag = "1")]
    pub sender: std::string::String,
    #[prost(string, tag = "2")]
    pub eth_dest: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::std::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "4")]
    pub bridge_fee: ::std::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendToEthResponse {}

/// MsgRequestBatch
/// this is a message anyone can send that requests a batch of transactions to
/// send across the bridge be created for whatever block height this message is
/// included in. This acts as a coordination point, the handler for this message
/// looks at the AddToOutgoingPool tx's in the store and generates a batch, also
/// available in the store tied to this message. The validators then grab this
/// batch, sign it, submit the signatures with a MsgConfirmBatch before a relayer
/// can finally submit the batch
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestBatch {
    #[prost(string, tag = "1")]
    pub sender: std::string::String,
    #[prost(string, tag = "2")]
    pub denom: std::string::String,
    #[prost(string, tag = "3")]
    pub minimum_fee: std::string::String,
    #[prost(string, tag = "4")]
    pub fee_receive: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestBatchResponse {}

/// MsgConfirmBatch
/// When validators observe a MsgRequestBatch they form a batch by ordering
/// transactions currently in the txqueue in order of highest to lowest fee,
/// cutting off when the batch either reaches a hardcoded maximum size (to be
/// decided, probably around 100) or when transactions stop being profitable
/// (determine this without nondeterminism) This message includes the batch
/// as well as an Ethereum signature over this batch by the validator
/// -------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConfirmBatch {
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
    #[prost(string, tag = "2")]
    pub token_contract: std::string::String,
    #[prost(string, tag = "3")]
    pub eth_signer: std::string::String,
    #[prost(string, tag = "4")]
    pub orchestrator: std::string::String,
    #[prost(string, tag = "5")]
    pub signature: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConfirmBatchResponse {}

/// EthereumBridgeDepositClaim
/// When more than 66% of the active validator set has
/// claimed to have seen the deposit enter the ethereum blockchain coins are
/// issued to the Cosmos address in question
/// -------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositClaim {
    #[prost(uint64, tag = "1")]
    pub event_nonce: u64,
    #[prost(uint64, tag = "2")]
    pub block_height: u64,
    #[prost(string, tag = "3")]
    pub token_contract: std::string::String,
    #[prost(string, tag = "4")]
    pub amount: std::string::String,
    #[prost(string, tag = "5")]
    pub eth_sender: std::string::String,
    #[prost(string, tag = "6")]
    pub fx_receiver: std::string::String,
    #[prost(string, tag = "7")]
    pub target_ibc: std::string::String,
    #[prost(string, tag = "8")]
    pub orchestrator: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositClaimResponse {}

/// WithdrawClaim claims that a batch of withdrawal
/// operations on the bridge contract was executed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawClaim {
    #[prost(uint64, tag = "1")]
    pub event_nonce: u64,
    #[prost(uint64, tag = "2")]
    pub block_height: u64,
    #[prost(uint64, tag = "3")]
    pub batch_nonce: u64,
    #[prost(string, tag = "4")]
    pub token_contract: std::string::String,
    #[prost(string, tag = "5")]
    pub orchestrator: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawClaimResponse {}

/// This call allows the sender (and only the sender)
/// to cancel a given MsgSendToEth and recieve a refund
/// of the tokens
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelSendToEth {
    #[prost(uint64, tag = "1")]
    pub transaction_id: u64,
    #[prost(string, tag = "2")]
    pub sender: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelSendToEthResponse {}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFxOriginatedTokenClaim {
    #[prost(uint64, tag = "1")]
    pub event_nonce: u64,
    #[prost(uint64, tag = "2")]
    pub block_height: u64,
    #[prost(string, tag = "3")]
    pub token_contract: std::string::String,
    #[prost(string, tag = "4")]
    pub name: std::string::String,
    #[prost(string, tag = "5")]
    pub symbol: std::string::String,
    #[prost(uint64, tag = "6")]
    pub decimals: u64,
    #[prost(string, tag = "7")]
    pub orchestrator: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFxOriginatedTokenClaimResponse {}

/// This informs the Cosmos module that a validator
/// set has been updated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValsetUpdatedClaim {
    #[prost(uint64, tag = "1")]
    pub event_nonce: u64,
    #[prost(uint64, tag = "2")]
    pub block_height: u64,
    #[prost(uint64, tag = "3")]
    pub valset_nonce: u64,
    #[prost(message, repeated, tag = "4")]
    pub members: ::std::vec::Vec<BridgeValidator>,
    #[prost(string, tag = "6")]
    pub orchestrator: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValsetUpdatedClaimResponse {}

#[doc = r" Generated client implementations."]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs)]

    use tonic::codegen::*;

    #[doc = " Msg defines the state transitions possible within gravity"]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }

    impl MsgClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }

    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn valset_confirm(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgValsetConfirm>,
        ) -> Result<tonic::Response<super::MsgValsetConfirmResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fx.gravity.v1.Msg/ValsetConfirm");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_to_eth(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSendToEth>,
        ) -> Result<tonic::Response<super::MsgSendToEthResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fx.gravity.v1.Msg/SendToEth");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn request_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRequestBatch>,
        ) -> Result<tonic::Response<super::MsgRequestBatchResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fx.gravity.v1.Msg/RequestBatch");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn confirm_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgConfirmBatch>,
        ) -> Result<tonic::Response<super::MsgConfirmBatchResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fx.gravity.v1.Msg/ConfirmBatch");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn deposit_claim(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDepositClaim>,
        ) -> Result<tonic::Response<super::MsgDepositClaimResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fx.gravity.v1.Msg/DepositClaim");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn withdraw_claim(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawClaim>,
        ) -> Result<tonic::Response<super::MsgWithdrawClaimResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fx.gravity.v1.Msg/WithdrawClaim");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn valset_update_claim(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgValsetUpdatedClaim>,
        ) -> Result<tonic::Response<super::MsgValsetUpdatedClaimResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fx.gravity.v1.Msg/ValsetUpdateClaim");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_orchestrator_address(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetOrchestratorAddress>,
        ) -> Result<tonic::Response<super::MsgSetOrchestratorAddressResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/fx.gravity.v1.Msg/SetOrchestratorAddress");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn cancel_send_to_eth(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCancelSendToEth>,
        ) -> Result<tonic::Response<super::MsgCancelSendToEthResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fx.gravity.v1.Msg/CancelSendToEth");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn fx_originated_token_claim(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgFxOriginatedTokenClaim>,
        ) -> Result<tonic::Response<super::MsgFxOriginatedTokenClaimResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/fx.gravity.v1.Msg/FxOriginatedTokenClaim");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }

    impl<T: Clone> Clone for MsgClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }

    impl<T> std::fmt::Debug for MsgClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MsgClient {{ ... }}")
        }
    }
} // Params represent the Gravity genesis and store parameters
  // gravity_id:
  // a random 32 byte value to prevent signature reuse, for example if the
  // cosmos validators decided to use the same Ethereum keys for another chain
  // also running Gravity we would not want it to be possible to play a deposit
  // from chain A back on chain B's Gravity. This value IS USED ON ETHEREUM so
  // it must be set in your genesis.json before launch and not changed after
  // deploying Gravity
  //
  // contract_hash:
  // the code hash of a known good version of the Gravity contract
  // solidity code. This can be used to verify the correct version
  // of the contract has been deployed. This is a reference value for
  // goernance action only it is never read by any Gravity code
  //
  // bridge_eth_address:
  // is address of the bridge contract on the Ethereum side, this is a
  // reference value for governance only and is not actually used by any
  // Gravity code
  //
  // bridge_chain_id:
  // the unique identifier of the Ethereum chain, this is a reference value
  // only and is not actually used by any Gravity code
  //
  // These reference values may be used by future Gravity client implemetnations
  // to allow for saftey features or convenience features like the Gravity address
  // in your relayer. A relayer would require a configured Gravity address if
  // governance had not set the address on the chain it was relaying for.
  //
  // signed_valsets_window
  // signed_batches_window
  // signed_claims_window
  //
  // These values represent the time in blocks that a validator has to submit
  // a signature for a batch or valset, or to submit a claim for a particular
  // attestation nonce. In the case of attestations this clock starts when the
  // attestation is created, but only allows for slashing once the event has passed
  //
  // target_batch_timeout:
  //
  // This is the 'target' value for when batches time out, this is a target becuase
  // Ethereum is a probabalistic chain and you can't say for sure what the block
  // frequency is ahead of time.
  //
  // average_block_time
  // average_eth_block_time
  //
  // These values are the average Cosmos block time and Ethereum block time repsectively
  // and they are used to copute what the target batch timeout is. It is important that
  // governance updates these in case of any major, prolonged change in the time it takes
  // to produce a block
  //
  // slash_fraction_valset
  // slash_fraction_batch
  // slash_fraction_claim
  // slash_fraction_conflicting_claim
  //
  // The slashing fractions for the various gravity related slashing conditions. The first three
  // refer to not submitting a particular message, the third for submitting a different claim
  // for the same Ethereum event

/// valset_update_power_change_percent
///
/// If power change between validators of CurrentValset and latest valset request is > 10%
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag = "1")]
    pub gravity_id: std::string::String,
    #[prost(string, tag = "2")]
    pub contract_source_hash: std::string::String,
    #[prost(string, tag = "4")]
    pub bridge_eth_address: std::string::String,
    #[prost(uint64, tag = "5")]
    pub bridge_chain_id: u64,
    #[prost(uint64, tag = "6")]
    pub signed_valsets_window: u64,
    #[prost(uint64, tag = "7")]
    pub signed_batches_window: u64,
    #[prost(uint64, tag = "8")]
    pub signed_claims_window: u64,
    #[prost(uint64, tag = "10")]
    pub target_batch_timeout: u64,
    #[prost(uint64, tag = "11")]
    pub average_block_time: u64,
    #[prost(uint64, tag = "12")]
    pub average_eth_block_time: u64,
    #[prost(bytes, tag = "13")]
    pub slash_fraction_valset: std::vec::Vec<u8>,
    #[prost(bytes, tag = "14")]
    pub slash_fraction_batch: std::vec::Vec<u8>,
    #[prost(bytes, tag = "15")]
    pub slash_fraction_claim: std::vec::Vec<u8>,
    #[prost(bytes, tag = "16")]
    pub slash_fraction_conflicting_claim: std::vec::Vec<u8>,
    #[prost(uint64, tag = "17")]
    pub unbond_slashing_valsets_window: u64,
    #[prost(uint64, tag = "18")]
    pub ibc_transfer_timeout_height: u64,
    #[prost(bytes, tag = "19")]
    pub valset_update_power_change_percent: std::vec::Vec<u8>,
}

/// GenesisState struct
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::std::option::Option<Params>,
    #[prost(uint64, tag = "2")]
    pub last_observed_nonce: u64,
    #[prost(message, repeated, tag = "3")]
    pub valsets: ::std::vec::Vec<Valset>,
    #[prost(message, repeated, tag = "4")]
    pub valset_confirms: ::std::vec::Vec<MsgValsetConfirm>,
    #[prost(message, repeated, tag = "5")]
    pub batches: ::std::vec::Vec<OutgoingTxBatch>,
    #[prost(message, repeated, tag = "6")]
    pub batch_confirms: ::std::vec::Vec<MsgConfirmBatch>,
    #[prost(message, repeated, tag = "7")]
    pub attestations: ::std::vec::Vec<Attestation>,
    #[prost(message, repeated, tag = "8")]
    pub delegate_keys: ::std::vec::Vec<MsgSetOrchestratorAddress>,
    #[prost(message, repeated, tag = "9")]
    pub erc20_to_denoms: ::std::vec::Vec<Erc20ToDenom>,
    #[prost(message, repeated, tag = "10")]
    pub unbatched_transfers: ::std::vec::Vec<OutgoingTransferTx>,
    #[prost(message, repeated, tag = "11")]
    pub module_coins: ::std::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::std::option::Option<Params>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentValsetRequest {}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentValsetResponse {
    #[prost(message, optional, tag = "1")]
    pub valset: ::std::option::Option<Valset>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetRequestRequest {
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetRequestResponse {
    #[prost(message, optional, tag = "1")]
    pub valset: ::std::option::Option<Valset>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetConfirmRequest {
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
    #[prost(string, tag = "2")]
    pub address: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetConfirmResponse {
    #[prost(message, optional, tag = "1")]
    pub confirm: ::std::option::Option<MsgValsetConfirm>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetConfirmsByNonceRequest {
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetConfirmsByNonceResponse {
    #[prost(message, repeated, tag = "1")]
    pub confirms: ::std::vec::Vec<MsgValsetConfirm>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastValsetRequestsRequest {}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastValsetRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub valsets: ::std::vec::Vec<Valset>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastPendingValsetRequestByAddrRequest {
    #[prost(string, tag = "1")]
    pub address: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastPendingValsetRequestByAddrResponse {
    #[prost(message, repeated, tag = "1")]
    pub valsets: ::std::vec::Vec<Valset>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchFeeRequest {}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchFeeResponse {
    #[prost(message, repeated, tag = "1")]
    pub batch_fees: ::std::vec::Vec<BatchFees>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastPendingBatchRequestByAddrRequest {
    #[prost(string, tag = "1")]
    pub address: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastPendingBatchRequestByAddrResponse {
    #[prost(message, optional, tag = "1")]
    pub batch: ::std::option::Option<OutgoingTxBatch>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOutgoingTxBatchesRequest {}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOutgoingTxBatchesResponse {
    #[prost(message, repeated, tag = "1")]
    pub batches: ::std::vec::Vec<OutgoingTxBatch>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchRequestByNonceRequest {
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
    #[prost(string, tag = "2")]
    pub contract_address: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchRequestByNonceResponse {
    #[prost(message, optional, tag = "1")]
    pub batch: ::std::option::Option<OutgoingTxBatch>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchConfirmsRequest {
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
    #[prost(string, tag = "2")]
    pub contract_address: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchConfirmsResponse {
    #[prost(message, repeated, tag = "1")]
    pub confirms: ::std::vec::Vec<MsgConfirmBatch>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastEventNonceByAddrRequest {
    #[prost(string, tag = "1")]
    pub address: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastEventNonceByAddrResponse {
    #[prost(uint64, tag = "1")]
    pub event_nonce: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryErc20ToDenomRequest {
    #[prost(string, tag = "1")]
    pub erc20: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryErc20ToDenomResponse {
    #[prost(string, tag = "1")]
    pub denom: std::string::String,
    #[prost(bool, tag = "2")]
    pub fx_originated: bool,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomToErc20Request {
    #[prost(string, tag = "1")]
    pub denom: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomToErc20Response {
    #[prost(string, tag = "1")]
    pub erc20: std::string::String,
    #[prost(bool, tag = "2")]
    pub fx_originated: bool,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegateKeysByValidatorAddress {
    #[prost(string, tag = "1")]
    pub validator_address: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegateKeysByValidatorAddressResponse {
    #[prost(string, tag = "1")]
    pub eth_address: std::string::String,
    #[prost(string, tag = "2")]
    pub orchestrator_address: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegateKeysByEthAddress {
    #[prost(string, tag = "1")]
    pub eth_address: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegateKeysByEthAddressResponse {
    #[prost(string, tag = "1")]
    pub validator_address: std::string::String,
    #[prost(string, tag = "2")]
    pub orchestrator_address: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegateKeysByOrchestratorAddress {
    #[prost(string, tag = "1")]
    pub orchestrator_address: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegateKeysByOrchestratorAddressResponse {
    #[prost(string, tag = "1")]
    pub validator_address: std::string::String,
    #[prost(string, tag = "2")]
    pub eth_address: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingSendToEth {
    #[prost(string, tag = "1")]
    pub sender_address: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingSendToEthResponse {
    #[prost(message, repeated, tag = "1")]
    pub transfers_in_batches: ::std::vec::Vec<OutgoingTransferTx>,
    #[prost(message, repeated, tag = "2")]
    pub unbatched_transfers: ::std::vec::Vec<OutgoingTransferTx>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIbcSequenceHeightRequest {
    #[prost(string, tag = "1")]
    pub source_port: std::string::String,
    #[prost(string, tag = "2")]
    pub source_channel: std::string::String,
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIbcSequenceHeightResponse {
    #[prost(bool, tag = "1")]
    pub found: bool,
    #[prost(uint64, tag = "2")]
    pub height: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastObservedEthBlockHeightRequest {}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastObservedEthBlockHeightResponse {
    #[prost(uint64, tag = "1")]
    pub block_height: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastEventBlockHeightByAddrRequest {
    #[prost(string, tag = "1")]
    pub address: std::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastEventBlockHeightByAddrResponse {
    #[prost(uint64, tag = "1")]
    pub block_height: u64,
}

#[doc = r" Generated client implementations."]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs)]

    use tonic::codegen::*;

    #[doc = " Query defines the gRPC querier service"]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }

    impl QueryClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }

    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Deployments queries deployments"]
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fx.gravity.v1.Query/Params");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn current_valset(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCurrentValsetRequest>,
        ) -> Result<tonic::Response<super::QueryCurrentValsetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fx.gravity.v1.Query/CurrentValset");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn valset_request(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValsetRequestRequest>,
        ) -> Result<tonic::Response<super::QueryValsetRequestResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fx.gravity.v1.Query/ValsetRequest");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn valset_confirm(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValsetConfirmRequest>,
        ) -> Result<tonic::Response<super::QueryValsetConfirmResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fx.gravity.v1.Query/ValsetConfirm");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn valset_confirms_by_nonce(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValsetConfirmsByNonceRequest>,
        ) -> Result<tonic::Response<super::QueryValsetConfirmsByNonceResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/fx.gravity.v1.Query/ValsetConfirmsByNonce");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn last_valset_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLastValsetRequestsRequest>,
        ) -> Result<tonic::Response<super::QueryLastValsetRequestsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/fx.gravity.v1.Query/LastValsetRequests");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn last_pending_valset_request_by_addr(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLastPendingValsetRequestByAddrRequest>,
        ) -> Result<
            tonic::Response<super::QueryLastPendingValsetRequestByAddrResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fx.gravity.v1.Query/LastPendingValsetRequestByAddr",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn last_pending_batch_request_by_addr(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLastPendingBatchRequestByAddrRequest>,
        ) -> Result<tonic::Response<super::QueryLastPendingBatchRequestByAddrResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fx.gravity.v1.Query/LastPendingBatchRequestByAddr",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn last_event_nonce_by_addr(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLastEventNonceByAddrRequest>,
        ) -> Result<tonic::Response<super::QueryLastEventNonceByAddrResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/fx.gravity.v1.Query/LastEventNonceByAddr");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn last_event_block_height_by_addr(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLastEventBlockHeightByAddrRequest>,
        ) -> Result<tonic::Response<super::QueryLastEventBlockHeightByAddrResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fx.gravity.v1.Query/LastEventBlockHeightByAddr",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn batch_fees(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBatchFeeRequest>,
        ) -> Result<tonic::Response<super::QueryBatchFeeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fx.gravity.v1.Query/BatchFees");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn last_observed_eth_block_height(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLastObservedEthBlockHeightRequest>,
        ) -> Result<tonic::Response<super::QueryLastObservedEthBlockHeightResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fx.gravity.v1.Query/LastObservedEthBlockHeight",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn outgoing_tx_batches(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryOutgoingTxBatchesRequest>,
        ) -> Result<tonic::Response<super::QueryOutgoingTxBatchesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/fx.gravity.v1.Query/OutgoingTxBatches");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn batch_request_by_nonce(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBatchRequestByNonceRequest>,
        ) -> Result<tonic::Response<super::QueryBatchRequestByNonceResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/fx.gravity.v1.Query/BatchRequestByNonce");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn batch_confirms(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBatchConfirmsRequest>,
        ) -> Result<tonic::Response<super::QueryBatchConfirmsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fx.gravity.v1.Query/BatchConfirms");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn erc20_to_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryErc20ToDenomRequest>,
        ) -> Result<tonic::Response<super::QueryErc20ToDenomResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fx.gravity.v1.Query/ERC20ToDenom");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn denom_to_erc20(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDenomToErc20Request>,
        ) -> Result<tonic::Response<super::QueryDenomToErc20Response>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fx.gravity.v1.Query/DenomToERC20");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_delegate_key_by_validator(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegateKeysByValidatorAddress>,
        ) -> Result<
            tonic::Response<super::QueryDelegateKeysByValidatorAddressResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fx.gravity.v1.Query/GetDelegateKeyByValidator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_delegate_key_by_eth(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegateKeysByEthAddress>,
        ) -> Result<tonic::Response<super::QueryDelegateKeysByEthAddressResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/fx.gravity.v1.Query/GetDelegateKeyByEth");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_delegate_key_by_orchestrator(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegateKeysByOrchestratorAddress>,
        ) -> Result<
            tonic::Response<super::QueryDelegateKeysByOrchestratorAddressResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fx.gravity.v1.Query/GetDelegateKeyByOrchestrator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_pending_send_to_eth(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPendingSendToEth>,
        ) -> Result<tonic::Response<super::QueryPendingSendToEthResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/fx.gravity.v1.Query/GetPendingSendToEth");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_ibc_sequence_height_by_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryIbcSequenceHeightRequest>,
        ) -> Result<tonic::Response<super::QueryIbcSequenceHeightResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fx.gravity.v1.Query/GetIbcSequenceHeightByChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }

    impl<T: Clone> Clone for QueryClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }

    impl<T> std::fmt::Debug for QueryClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "QueryClient {{ ... }}")
        }
    }
}

/// SignType defines messages that have been signed by an orchestrator
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SignType {
    Unspecified = 0,
    OrchestratorSignedMultiSigUpdate = 1,
    OrchestratorSignedWithdrawBatch = 2,
}