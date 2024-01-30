#[allow(unused_imports)]
use escrow_io::{EscrowAction, EscrowEvent, EscrowState, InitEscrow};
use gtest::{Log, Program, System};

const BUYER: u64 = 100;
const SELLER: u64 = 101;
const ONE_TOKEN: u128 = 1_000_000_000_000;
const PRICE: u128 = 15_000_000_000_000;
const MIN_MINT: u128 = 12_000_000_000_000;
const ESCROW_ID: u64 = 1;

#[test]
fn deposit() {
    let sys = System::new();
    init_escrow(&sys);

    let escrow = sys.get_program(ESCROW_ID);

    sys.mint_to(BUYER, PRICE + ONE_TOKEN);

    let res = escrow.send_with_value(BUYER, EscrowAction::Deposit(BUYER.into()), PRICE);
    let log = Log::builder()
        .dest(BUYER)
        .payload(EscrowEvent::FundsDeposited);
    assert!(res.contains(&log));

    let escrow_balance = sys.balance_of(ESCROW_ID);
    assert_eq!(escrow_balance, PRICE);
}

#[test]
fn deposit_failures() {
    let sys = System::new();
    init_escrow(&sys);

    let escrow = sys.get_program(ESCROW_ID);

    sys.mint_to(BUYER, PRICE);
    let res = escrow.send_with_value(
        BUYER,
        EscrowAction::Deposit(BUYER.into()),
        MIN_MINT - ONE_TOKEN,
    );
    assert!(res.main_failed());

    let res = escrow.send(SELLER, EscrowAction::Deposit(SELLER.into()));
    assert!(res.main_failed());

    sys.mint_to(BUYER, PRICE * 2 + ONE_TOKEN * 3);

    let res = escrow.send_with_value(BUYER, EscrowAction::Deposit(BUYER.into()), PRICE);
    assert!(!res.main_failed());
    let res = escrow.send_with_value(BUYER, EscrowAction::Deposit(BUYER.into()), PRICE);
    assert!(res.main_failed());
}

#[test]
fn confirm_delivery() {
    let sys = System::new();
    init_escrow(&sys);

    let escrow = sys.get_program(ESCROW_ID);

    sys.mint_to(BUYER, PRICE + ONE_TOKEN * 5);
    #[allow(unused_variables)]
    let res = escrow.send_with_value(BUYER, EscrowAction::Deposit(BUYER.into()), PRICE);
    #[allow(unused_variables)]
    let res = escrow.send(BUYER, EscrowAction::ConfirmDelivery(BUYER.into()));

    sys.claim_value_from_mailbox(SELLER);

    let seller_balance = sys.balance_of(SELLER);

    assert_eq!(seller_balance, PRICE);
}

#[test]
fn confirm_delivery_failures() {
    let sys = System::new();
    init_escrow(&sys);

    let escrow = sys.get_program(ESCROW_ID);

    sys.mint_to(BUYER, MIN_MINT + ONE_TOKEN * 8);
    let res = escrow.send_with_value(BUYER, EscrowAction::Deposit(BUYER.into()), ONE_TOKEN * 10);
    assert!(res.main_failed());

    let res = escrow.send(SELLER, EscrowAction::Deposit(SELLER.into()));
    assert!(res.main_failed());

    sys.mint_to(BUYER, ONE_TOKEN * 17);

    let res = escrow.send_with_value(BUYER, EscrowAction::Deposit(BUYER.into()), PRICE);

    assert!(!res.main_failed());
    let res = escrow.send_with_value(BUYER, EscrowAction::Deposit(BUYER.into()), PRICE);
    assert!(res.main_failed());

    let res = escrow.send(SELLER, EscrowAction::ConfirmDelivery(SELLER.into()));
    assert!(res.main_failed());

    let res = escrow.send(BUYER, EscrowAction::ConfirmDelivery(BUYER.into()));
    assert!(!res.main_failed());

    let res = escrow.send_with_value(BUYER, EscrowAction::Deposit(BUYER.into()), PRICE);
    assert!(res.main_failed());

    let res = escrow.send(BUYER, EscrowAction::ConfirmDelivery(BUYER.into()));
    assert!(res.main_failed());
}

fn init_escrow(sys: &System) {
    sys.init_logger();
    #[allow(clippy::needless_borrow)]
    let escrow = Program::current(&sys);
    let res = escrow.send(
        SELLER,
        InitEscrow {
            seller: SELLER.into(),
            buyer: BUYER.into(),
            price: PRICE,
        },
    );
    assert!(!res.main_failed());
}
