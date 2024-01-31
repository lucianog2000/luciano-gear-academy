#![no_std]

#[allow(unused_imports)]
use gstd::{async_main, debug, exec, fmt, msg, prelude::*, ActorId, ReservationId};
use sharded_fungible_token_io::{FTokenAction, FTokenEvent, LogicAction};
use store_io::{StoreAction, StoreEvent};
use tamagotchi_utils_io::{GasReservationHandler, Tamagotchi, TmgAction, TmgEvent};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;
static mut GAS_RESERVATION_HANDLER: Option<GasReservationHandler> = None;

const HUNGER_PER_BLOCK: u32 = 1;
const HUNGER_THRESHOLD: u32 = 2000;
const FILL_PER_FEED: u32 = 1000;

const BOREDOM_PER_BLOCK: u32 = 2;
const BOREDOM_THRESHOLD: u32 = 2000;
const FILL_PER_ENTERTAINMENT: u32 = 1000;

const ENERGY_PER_BLOCK: u32 = 2;
const ENERGY_THRESHOLD: u32 = 2000;
const FILL_PER_SLEEP: u32 = 1000;

const DELAY_OF_ONE_MINUTE: u32 = 20;
const MAX_STAT_VALUE: u32 = 10000;

#[no_mangle]
extern fn init() {
    let init_message: String = msg::load().expect("Can't decode tamagotchi's name");
    let current_block = exec::block_height();
    let tamagotchi = Tamagotchi {
        name: init_message,
        date_of_birth: exec::block_timestamp(),
        owner: msg::source(),
        fed: MAX_STAT_VALUE,
        fed_block: current_block,
        entertained: MAX_STAT_VALUE,
        entertained_block: current_block,
        slept: MAX_STAT_VALUE,
        slept_block: current_block,
        approved_account: None,
        ft_contract_id: Default::default(),
        transaction_id: Default::default(),
        approve_transaction: None,
        reservations: Vec::new(),
    };
    let payload = format!(
        "The Tamagotchi Program was initialized with name {:?} and birth date {:?}",
        tamagotchi.name, tamagotchi.date_of_birth
    );

    unsafe {
        TAMAGOTCHI = Some(tamagotchi);
        GAS_RESERVATION_HANDLER = Some(GasReservationHandler {
            contract_send_a_delayed_message: false,
            can_send_delayed_message: false,
        });
    };
    msg::reply(payload, 0).expect("error in reply");
}

#[async_main]
async fn main() {
    let tamagotchi = unsafe {
        TAMAGOTCHI
            .as_mut()
            .expect("The contract is not initialized")
    };
    let GasReservationHandler {
        can_send_delayed_message,
        contract_send_a_delayed_message,
    } = gas_reservation_handler_state_mut();

    let tmg_action: TmgAction = msg::load().expect("Error loading TmgAction");
    match tmg_action {
        TmgAction::Name => reply_with(&tamagotchi.name),
        TmgAction::Age => {
            reply_with((exec::block_timestamp() - tamagotchi.date_of_birth).to_string())
        }
        TmgAction::Feed => {
            fill_stat_and_reply(
                &mut tamagotchi.fed,
                &mut tamagotchi.fed_block,
                HUNGER_PER_BLOCK,
                FILL_PER_FEED,
            );
            debug!("Feed action {:?}", &tamagotchi.fed.to_string());
        }
        TmgAction::Entertain => {
            fill_stat_and_reply(
                &mut tamagotchi.entertained,
                &mut tamagotchi.entertained_block,
                BOREDOM_PER_BLOCK,
                FILL_PER_ENTERTAINMENT,
            );
            debug!("Entertain action {:?}", &tamagotchi.entertained.to_string());
        }
        TmgAction::Sleep => {
            fill_stat_and_reply(
                &mut tamagotchi.slept,
                &mut tamagotchi.slept_block,
                ENERGY_PER_BLOCK,
                FILL_PER_SLEEP,
            );
            debug!("Sleep action {:?}", &tamagotchi.slept.to_string());
        }
        TmgAction::Transfer(new_owner) => {
            if msg::source() == tamagotchi.owner
                || Some(msg::source()) == tamagotchi.approved_account
            {
                tamagotchi.owner = new_owner;
                msg::reply(TmgEvent::Transferred(new_owner), 0).expect("Error in sending reply");
            } else {
                panic!("You don't have permission to do this action")
            }
        }
        TmgAction::Approve(account) => {
            if msg::source() == tamagotchi.owner {
                tamagotchi.approved_account = Some(account);
                msg::reply(TmgEvent::Approved(account), 0).expect("Error in sending reply");
            } else {
                panic!("You don't have permission to do this action")
            }
        }
        TmgAction::RevokeApproval => {
            if msg::source() == tamagotchi.owner {
                tamagotchi.approved_account = None;
                msg::reply(TmgEvent::ApprovalRevoked, 0).expect("Error in sending reply");
            } else {
                panic!("You don't have permission to do this action")
            }
        }
        TmgAction::SetFTokenContract(contract) => {
            tamagotchi.ft_contract_id = Some(contract);
            msg::reply(TmgEvent::FTokenContractSet, 0)
                .expect("Error in a reply `TmgEvent::FTokenContractSet`");
        }
        TmgAction::ApproveTokens { account, amount } => {
            approve_tokens(tamagotchi, &account, amount).await;
            msg::reply(TmgEvent::TokensApproved { account, amount }, 0)
                .expect("Error in a reply `TmgEvent::TokensApproved`");
        }
        TmgAction::BuyAttribute {
            store_id,
            attribute_id,
        } => {
            buy_attribute(&store_id, attribute_id).await;
            msg::reply(TmgEvent::AttributeBought(attribute_id), 0)
                .expect("Error in a reply `TmgEvent::AttributeBought`");
        }
        TmgAction::CheckState => {
            let payload;

            update_stats(
                tamagotchi,
                tamagotchi.fed_block,
                tamagotchi.entertained_block,
                tamagotchi.slept_block,
            );

            if tamagotchi.fed <= HUNGER_THRESHOLD {
                payload = TmgEvent::FeedMe;
            } else if tamagotchi.entertained <= BOREDOM_THRESHOLD {
                payload = TmgEvent::PlayWithMe;
            } else if tamagotchi.slept <= ENERGY_THRESHOLD {
                payload = TmgEvent::WantToSleep;
            } else {
                payload = TmgEvent::NothingToDo;
            }

            if exec::program_id() != msg::source() {
                msg::reply(payload, 0).expect("Error in reply");
                return;
            }

            if tamagotchi.reservations.len() == 1 {
                *can_send_delayed_message = false;
                *contract_send_a_delayed_message = false;
                let Some(reservation_id) = tamagotchi.reservations.pop() else {
                    panic!("Error getting reservation id");
                };

                msg::send_delayed_from_reservation(
                    reservation_id,
                    tamagotchi.owner,
                    TmgEvent::MakeReservation,
                    0,
                    DELAY_OF_ONE_MINUTE,
                )
                .expect("Error sending message from reservation");
            } else {
                let Some(reservation_id) = tamagotchi.reservations.pop() else {
                    panic!("Error getting reservation id");
                };

                msg::send_delayed_from_reservation(
                    reservation_id,
                    exec::program_id(),
                    TmgAction::CheckState,
                    0,
                    DELAY_OF_ONE_MINUTE,
                )
                .expect("Error sending message from reservation");
            }
        }
        TmgAction::ReserveGas {
            reservation_amount,
            duration,
        } => {
            let reservation_id = ReservationId::reserve(reservation_amount, duration)
                .expect("Error in reserving gas");
            tamagotchi.reservations.push(reservation_id);

            if tamagotchi.reservations.len() >= 3 {
                *can_send_delayed_message = true;
            }

            if *can_send_delayed_message && !(*contract_send_a_delayed_message) {
                *contract_send_a_delayed_message = true;
                let Some(reservation_id) = tamagotchi.reservations.pop() else {
                    panic!("Error getting reservation id");
                };

                msg::send_delayed_from_reservation(
                    reservation_id,
                    exec::program_id(),
                    TmgAction::CheckState,
                    0,
                    DELAY_OF_ONE_MINUTE,
                )
                .expect("Error sending message from reservation");
            }

            msg::reply(TmgEvent::GasReserved, 0)
                .expect("Error in replying GasReserved event payload");
        }
    }
}

#[no_mangle]
extern fn state() {
    let tamagotchi = unsafe {
        TAMAGOTCHI
            .as_mut()
            .expect("The contract is not initialized")
    };
    update_stats(
        tamagotchi,
        tamagotchi.fed_block,
        tamagotchi.entertained_block,
        tamagotchi.slept_block,
    );
    msg::reply(tamagotchi, 0).expect("Failed to share state");
}

fn reply_with<T: fmt::Display>(value: T) {
    msg::reply(&value.to_string(), 0).expect("Error in sending reply");
}

fn fill_stat_and_reply(
    stat: &mut u32,
    stat_block: &mut u32,
    stat_wasted_per_block: u32,
    fill_per_action: u32,
) {
    let actual_value = update_stat(*stat_block, stat_wasted_per_block);

    *stat = fill_stat_value(actual_value, fill_per_action);
    *stat_block = exec::block_height();

    reply_with(*stat);
}

fn update_stat(stat_block: u32, stat_wasted_per_block: u32) -> u32 {
    let stat_lost = (exec::block_height() - stat_block) * stat_wasted_per_block;

    let stat = 10000u32.saturating_sub(stat_lost).saturating_add(1);

    stat
}

fn fill_stat_value(stat: u32, fill_per_action: u32) -> u32 {
    let filled_stat = stat + fill_per_action;
    filled_stat.max(1).min(10000)
}

fn update_stats(
    tamagotchi: &mut Tamagotchi,
    fed_block: u32,
    entertained_block: u32,
    slept_block: u32,
) {
    tamagotchi.fed = update_stat(fed_block, HUNGER_PER_BLOCK);
    tamagotchi.entertained = update_stat(entertained_block, BOREDOM_PER_BLOCK);
    tamagotchi.slept = update_stat(slept_block, ENERGY_PER_BLOCK);
}

async fn approve_tokens(tamagotchi: &mut Tamagotchi, account: &ActorId, amount: u128) {
    let _result_approve = msg::send_for_reply_as::<_, FTokenEvent>(
        tamagotchi.ft_contract_id.unwrap(),
        FTokenAction::Message {
            transaction_id: tamagotchi.transaction_id,
            payload: LogicAction::Approve {
                approved_account: account.clone(),
                amount,
            },
        },
        0,
        0,
    )
    .expect("Error in sending a message `FTokenAction::Message`")
    .await;
}

async fn buy_attribute(store: &ActorId, attribute: u32) {
    let _result_buy = msg::send_for_reply_as::<_, StoreEvent>(
        store.clone(),
        StoreAction::BuyAttribute {
            attribute_id: attribute,
        },
        0,
        0,
    )
    .expect("Error in sending a message `StoreAction::BuyAttribute`")
    .await;
}

fn gas_reservation_handler_state_mut() -> &'static mut GasReservationHandler {
    let state = unsafe { GAS_RESERVATION_HANDLER.as_mut() };
    debug_assert!(state.is_some(), "State is not initialized");
    unsafe { state.unwrap_unchecked() }
}
