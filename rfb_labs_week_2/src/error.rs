use std::fmt;

/// Expected failures produced by transaction validation and coin selection.
#[derive(Debug, PartialEq, Eq)]
pub enum TransactionError {
    NoInputs,
    NoOutputs,
    ZeroValueOutput,
    OutputsExceedInputs {
        total_inputs: u64,
        total_outputs: u64,
    },
    CoinbaseMixedWithRegularInputs,
    MultipleCoinbaseInputs,
    InvalidTxid,
    InsufficientFunds {
        available: u64,
        required: u64,
    },
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionError::NoInputs => write!(f, "transaction must have at least one input"),
            TransactionError::NoOutputs => write!(f, "transaction must have at least one output"),
            TransactionError::ZeroValueOutput => {
                write!(
                    f,
                    "non-OpReturn outputs must have a value greater than zero"
                )
            }
            TransactionError::OutputsExceedInputs {
                total_inputs,
                total_outputs,
            } => write!(
                f,
                "total outputs ({total_outputs} sats) exceed total inputs ({total_inputs} sats)"
            ),
            TransactionError::CoinbaseMixedWithRegularInputs => {
                write!(f, "coinbase inputs cannot be mixed with regular inputs")
            }
            TransactionError::MultipleCoinbaseInputs => {
                write!(f, "a transaction may only contain one coinbase input")
            }
            TransactionError::InvalidTxid => {
                write!(f, "regular inputs must have a non-empty txid")
            }
            TransactionError::InsufficientFunds {
                available,
                required,
            } => write!(
                f,
                "insufficient funds: have {available} sats, need {required} sats"
            ),
        }
    }
}

impl std::error::Error for TransactionError {}
