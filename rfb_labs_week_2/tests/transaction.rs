use rfb_labs_week_2::{
    find_outputs_for_recipient, highest_value_output, InputKind, OutPoint, OutputType, Transaction,
    TransactionError, TxOutput,
};

fn regular_input(value: u64) -> InputKind {
    InputKind::Regular {
        previous_output: OutPoint {
            txid: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
            vout: 0,
        },
        value,
        sequence: u32::MAX,
    }
}

fn output(value: u64, recipient: &str) -> TxOutput {
    TxOutput {
        value,
        recipient: recipient.into(),
        output_type: OutputType::P2wpkh,
    }
}

// ── originally ignored starter tests ─────────────────────────────────────────

#[test]
fn valid_regular_transaction_passes_validation() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(regular_input(120_000));
    transaction.add_output(output(90_000, "bc1qreceiver"));
    transaction.add_output(output(28_000, "bc1qsender"));

    assert_eq!(transaction.validate(), Ok(()));
    assert_eq!(transaction.total_input_value(), 120_000);
    assert_eq!(transaction.total_output_value(), 118_000);
    assert_eq!(transaction.fee(), Ok(2_000));
}

#[test]
fn outputs_cannot_exceed_inputs() {
    let mut transaction = Transaction::new(2, 0);
    transaction.add_input(regular_input(50_000));
    transaction.add_output(output(60_000, "bc1qreceiver"));

    assert_eq!(
        transaction.validate(),
        Err(TransactionError::OutputsExceedInputs {
            total_inputs: 50_000,
            total_outputs: 60_000,
        })
    );
}

// ── fee ───────────────────────────────────────────────────────────────────────

#[test]
fn fee_returns_error_when_outputs_exceed_inputs() {
    let mut tx = Transaction::new(1, 0);
    tx.add_input(regular_input(10_000));
    tx.add_output(output(15_000, "bc1q"));
    assert_eq!(
        tx.fee(),
        Err(TransactionError::OutputsExceedInputs {
            total_inputs: 10_000,
            total_outputs: 15_000,
        })
    );
}

// ── validation errors ─────────────────────────────────────────────────────────

#[test]
fn no_inputs_is_invalid() {
    let mut tx = Transaction::new(1, 0);
    tx.add_output(output(1_000, "bc1q"));
    assert_eq!(tx.validate(), Err(TransactionError::NoInputs));
}

#[test]
fn no_outputs_is_invalid() {
    let mut tx = Transaction::new(1, 0);
    tx.add_input(regular_input(1_000));
    assert_eq!(tx.validate(), Err(TransactionError::NoOutputs));
}

#[test]
fn zero_value_non_opreturn_output_is_invalid() {
    let mut tx = Transaction::new(1, 0);
    tx.add_input(regular_input(1_000));
    tx.add_output(TxOutput {
        value: 0,
        recipient: "bc1q".into(),
        output_type: OutputType::P2wpkh,
    });
    assert_eq!(tx.validate(), Err(TransactionError::ZeroValueOutput));
}

#[test]
fn zero_value_opreturn_output_is_valid() {
    let mut tx = Transaction::new(1, 0);
    tx.add_input(regular_input(1_000));
    tx.add_output(TxOutput {
        value: 0,
        recipient: "".into(),
        output_type: OutputType::OpReturn,
    });
    assert_eq!(tx.validate(), Ok(()));
}

#[test]
fn coinbase_mixed_with_regular_inputs_is_invalid() {
    let mut tx = Transaction::new(1, 0);
    tx.add_input(InputKind::Coinbase {
        block_height: 800_000,
        reward: 312_500_000,
    });
    tx.add_input(regular_input(50_000));
    tx.add_output(output(100_000, "bc1q"));
    assert_eq!(
        tx.validate(),
        Err(TransactionError::CoinbaseMixedWithRegularInputs)
    );
}

#[test]
fn multiple_coinbase_inputs_is_invalid() {
    let mut tx = Transaction::new(1, 0);
    tx.add_input(InputKind::Coinbase {
        block_height: 800_000,
        reward: 312_500_000,
    });
    tx.add_input(InputKind::Coinbase {
        block_height: 800_001,
        reward: 312_500_000,
    });
    tx.add_output(output(600_000_000, "bc1q"));
    assert_eq!(tx.validate(), Err(TransactionError::MultipleCoinbaseInputs));
}

#[test]
fn empty_txid_is_invalid() {
    let mut tx = Transaction::new(1, 0);
    tx.add_input(InputKind::Regular {
        previous_output: OutPoint {
            txid: "".into(),
            vout: 0,
        },
        value: 10_000,
        sequence: u32::MAX,
    });
    tx.add_output(output(9_000, "bc1q"));
    assert_eq!(tx.validate(), Err(TransactionError::InvalidTxid));
}

// ── valid coinbase transaction ────────────────────────────────────────────────

#[test]
fn valid_coinbase_transaction_passes_validation() {
    let mut tx = Transaction::new(1, 0);
    tx.add_input(InputKind::Coinbase {
        block_height: 840_000,
        reward: 312_500_000,
    });
    tx.add_output(output(312_500_000, "bc1qminer"));
    assert_eq!(tx.validate(), Ok(()));
    assert_eq!(tx.total_input_value(), 312_500_000);
    assert_eq!(tx.total_output_value(), 312_500_000);
    assert_eq!(tx.fee(), Ok(0));
}

// ── borrowing helpers ─────────────────────────────────────────────────────────

#[test]
fn highest_value_output_returns_correct_output() {
    let mut tx = Transaction::new(2, 0);
    tx.add_input(regular_input(200_000));
    tx.add_output(output(90_000, "bc1qreceiver"));
    tx.add_output(output(100_000, "bc1qsender"));

    let highest = highest_value_output(&tx).unwrap();
    assert_eq!(highest.value, 100_000);
    assert_eq!(highest.recipient, "bc1qsender");
}

#[test]
fn highest_value_output_is_none_for_empty_outputs() {
    let tx = Transaction::new(2, 0);
    assert!(highest_value_output(&tx).is_none());
}

#[test]
fn find_outputs_for_recipient_returns_matching_outputs() {
    let mut tx = Transaction::new(2, 0);
    tx.add_input(regular_input(300_000));
    tx.add_output(output(90_000, "bc1qreceiver"));
    tx.add_output(output(100_000, "bc1qsender"));
    tx.add_output(output(50_000, "bc1qreceiver"));

    let found = find_outputs_for_recipient(&tx, "bc1qreceiver");
    assert_eq!(found.len(), 2);
    assert_eq!(found.iter().map(|o| o.value).sum::<u64>(), 140_000);
}

#[test]
fn find_outputs_for_recipient_returns_empty_when_no_match() {
    let mut tx = Transaction::new(2, 0);
    tx.add_input(regular_input(50_000));
    tx.add_output(output(49_000, "bc1qsender"));

    let found = find_outputs_for_recipient(&tx, "bc1qother");
    assert!(found.is_empty());
}
