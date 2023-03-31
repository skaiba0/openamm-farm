#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct Farm {
    pub owner: Pubkey,
    pub created_at: u64,
    pub stake_vault: Pubkey,
    pub stake_mint: Pubkey,
    pub last_updated_at: u64,
    pub total_staked_amount: u64,
    pub stakers: u64,
    pub crop_vault: [Pubkey; 8],
    pub crop_rewards_per_second: [u64; 8],
    pub crop_rewards_per_token: [u128; 8],
    pub crop_end_date: [u64; 8],
    pub crop_created_at: [u64; 8],
    pub crop_stakers_finished: [u64; 8],
}

impl<'info, 'entrypoint> Farm {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedFarm<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let created_at = account.created_at;
        let stake_vault = account.stake_vault.clone();
        let stake_mint = account.stake_mint.clone();
        let last_updated_at = account.last_updated_at;
        let total_staked_amount = account.total_staked_amount;
        let stakers = account.stakers;
        let crop_vault = Mutable::new(account.crop_vault.clone());
        let crop_rewards_per_second = Mutable::new(account.crop_rewards_per_second.clone());
        let crop_rewards_per_token = Mutable::new(account.crop_rewards_per_token.clone());
        let crop_end_date = Mutable::new(account.crop_end_date.clone());
        let crop_created_at = Mutable::new(account.crop_created_at.clone());
        let crop_stakers_finished = Mutable::new(account.crop_stakers_finished.clone());

        Mutable::new(LoadedFarm {
            __account__: account,
            __programs__: programs_map,
            owner,
            created_at,
            stake_vault,
            stake_mint,
            last_updated_at,
            total_staked_amount,
            stakers,
            crop_vault,
            crop_rewards_per_second,
            crop_rewards_per_token,
            crop_end_date,
            crop_created_at,
            crop_stakers_finished,
        })
    }

    pub fn store(loaded: Mutable<LoadedFarm>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let created_at = loaded.created_at;

        loaded.__account__.created_at = created_at;

        let stake_vault = loaded.stake_vault.clone();

        loaded.__account__.stake_vault = stake_vault;

        let stake_mint = loaded.stake_mint.clone();

        loaded.__account__.stake_mint = stake_mint;

        let last_updated_at = loaded.last_updated_at;

        loaded.__account__.last_updated_at = last_updated_at;

        let total_staked_amount = loaded.total_staked_amount;

        loaded.__account__.total_staked_amount = total_staked_amount;

        let stakers = loaded.stakers;

        loaded.__account__.stakers = stakers;

        let crop_vault = loaded.crop_vault.borrow().clone();

        loaded.__account__.crop_vault = crop_vault;

        let crop_rewards_per_second = loaded.crop_rewards_per_second.borrow().clone();

        loaded.__account__.crop_rewards_per_second = crop_rewards_per_second;

        let crop_rewards_per_token = loaded.crop_rewards_per_token.borrow().clone();

        loaded.__account__.crop_rewards_per_token = crop_rewards_per_token;

        let crop_end_date = loaded.crop_end_date.borrow().clone();

        loaded.__account__.crop_end_date = crop_end_date;

        let crop_created_at = loaded.crop_created_at.borrow().clone();

        loaded.__account__.crop_created_at = crop_created_at;

        let crop_stakers_finished = loaded.crop_stakers_finished.borrow().clone();

        loaded.__account__.crop_stakers_finished = crop_stakers_finished;
    }
}

#[derive(Debug)]
pub struct LoadedFarm<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Farm>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub created_at: u64,
    pub stake_vault: Pubkey,
    pub stake_mint: Pubkey,
    pub last_updated_at: u64,
    pub total_staked_amount: u64,
    pub stakers: u64,
    pub crop_vault: Mutable<[Pubkey; 8]>,
    pub crop_rewards_per_second: Mutable<[u64; 8]>,
    pub crop_rewards_per_token: Mutable<[u128; 8]>,
    pub crop_end_date: Mutable<[u64; 8]>,
    pub crop_created_at: Mutable<[u64; 8]>,
    pub crop_stakers_finished: Mutable<[u64; 8]>,
}

#[account]
#[derive(Debug)]
pub struct Protocol {
    pub bump_seed: u8,
}

impl<'info, 'entrypoint> Protocol {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedProtocol<'info, 'entrypoint>> {
        let bump_seed = account.bump_seed;

        Mutable::new(LoadedProtocol {
            __account__: account,
            __programs__: programs_map,
            bump_seed,
        })
    }

    pub fn store(loaded: Mutable<LoadedProtocol>) {
        let mut loaded = loaded.borrow_mut();
        let bump_seed = loaded.bump_seed;

        loaded.__account__.bump_seed = bump_seed;
    }
}

#[derive(Debug)]
pub struct LoadedProtocol<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Protocol>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub bump_seed: u8,
}

#[account]
#[derive(Debug)]
pub struct Stake {
    pub owner: Pubkey,
    pub created_at: u64,
    pub amount_staked: u64,
    pub last_updated_at: u64,
    pub farm: Pubkey,
    pub reward_debt: [u128; 8],
    pub last_gathered_at: [u64; 8],
    pub amount_owed: [u64; 8],
}

impl<'info, 'entrypoint> Stake {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedStake<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let created_at = account.created_at;
        let amount_staked = account.amount_staked;
        let last_updated_at = account.last_updated_at;
        let farm = account.farm.clone();
        let reward_debt = Mutable::new(account.reward_debt.clone());
        let last_gathered_at = Mutable::new(account.last_gathered_at.clone());
        let amount_owed = Mutable::new(account.amount_owed.clone());

        Mutable::new(LoadedStake {
            __account__: account,
            __programs__: programs_map,
            owner,
            created_at,
            amount_staked,
            last_updated_at,
            farm,
            reward_debt,
            last_gathered_at,
            amount_owed,
        })
    }

    pub fn store(loaded: Mutable<LoadedStake>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let created_at = loaded.created_at;

        loaded.__account__.created_at = created_at;

        let amount_staked = loaded.amount_staked;

        loaded.__account__.amount_staked = amount_staked;

        let last_updated_at = loaded.last_updated_at;

        loaded.__account__.last_updated_at = last_updated_at;

        let farm = loaded.farm.clone();

        loaded.__account__.farm = farm;

        let reward_debt = loaded.reward_debt.borrow().clone();

        loaded.__account__.reward_debt = reward_debt;

        let last_gathered_at = loaded.last_gathered_at.borrow().clone();

        loaded.__account__.last_gathered_at = last_gathered_at;

        let amount_owed = loaded.amount_owed.borrow().clone();

        loaded.__account__.amount_owed = amount_owed;
    }
}

#[derive(Debug)]
pub struct LoadedStake<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Stake>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub created_at: u64,
    pub amount_staked: u64,
    pub last_updated_at: u64,
    pub farm: Pubkey,
    pub reward_debt: Mutable<[u128; 8]>,
    pub last_gathered_at: Mutable<[u64; 8]>,
    pub amount_owed: Mutable<[u64; 8]>,
}

pub fn add_crop_handler<'info>(
    mut crop_index: u8,
    mut reward_amount: u64,
    mut rewards_per_second: u64,
    mut farm: Mutable<LoadedFarm<'info, '_>>,
    mut crop_vault: SeahorseAccount<'info, '_, TokenAccount>,
    mut signer_reward: SeahorseAccount<'info, '_, TokenAccount>,
    mut clock: Sysvar<'info, Clock>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    if !(farm.borrow().owner == signer.key()) {
        panic!("Wrong signer");
    }

    if !(crop_index < <u8 as TryFrom<_>>::try_from(5).unwrap()) {
        panic!("Index too high");
    }

    if !(crop_vault.mint == signer_reward.mint) {
        panic!("Wrong mint");
    }

    if !(farm.borrow().crop_created_at.borrow()[farm
        .borrow()
        .crop_created_at
        .wrapped_index((crop_index as i128) as i128)]
        == 0)
    {
        panic!("Crop already active at this index");
    }

    let mut current_timestamp =
        <u64 as TryFrom<_>>::try_from(clock.unix_timestamp.clone()).unwrap();

    let mut precision_scaler = get_precision_scaler();

    for mut i in 0..8 {
        if farm.borrow().crop_created_at.borrow()
            [farm.borrow().crop_created_at.wrapped_index(i as i128)]
            == 0
        {
            continue;
        }

        if farm.borrow().last_updated_at
            > farm.borrow().crop_end_date.borrow()
                [farm.borrow().crop_end_date.wrapped_index(i as i128)]
        {
            continue;
        }

        if farm.borrow().total_staked_amount == 0 {
            continue;
        }

        let mut current_time = current_timestamp.min(
            farm.borrow().crop_end_date.borrow()
                [farm.borrow().crop_end_date.wrapped_index(i as i128)],
        );

        if farm.borrow().total_staked_amount > <u64 as TryFrom<_>>::try_from(0).unwrap() {
            let mut time_diff = current_time - farm.borrow().last_updated_at;
            let mut rewards = <u128 as TryFrom<_>>::try_from(
                (time_diff
                    * farm.borrow().crop_rewards_per_second.borrow()[farm
                        .borrow()
                        .crop_rewards_per_second
                        .wrapped_index(i as i128)]),
            )
            .unwrap()
                * precision_scaler;

            let mut rewards_per_token = rewards / (farm.borrow().total_staked_amount as u128);
            let mut current_rewards_per_token = farm.borrow().crop_rewards_per_token.borrow()[farm
                .borrow()
                .crop_rewards_per_token
                .wrapped_index(i as i128)];

            index_assign!(
                farm.borrow_mut().crop_rewards_per_token.borrow_mut(),
                farm.borrow_mut()
                    .crop_rewards_per_token
                    .wrapped_index(i as i128),
                current_rewards_per_token
                    + <u128 as TryFrom<_>>::try_from(rewards_per_token.clone()).unwrap()
            );

            assign!(farm.borrow_mut().last_updated_at, current_time);
        }
    }

    token::transfer(
        CpiContext::new(
            signer_reward.programs.get("token_program"),
            token::Transfer {
                from: signer_reward.to_account_info(),
                authority: signer.clone().to_account_info(),
                to: crop_vault.clone().to_account_info(),
            },
        ),
        reward_amount.clone(),
    )
    .unwrap();

    index_assign!(
        farm.borrow_mut().crop_rewards_per_second.borrow_mut(),
        farm.borrow_mut()
            .crop_rewards_per_second
            .wrapped_index((crop_index as i128) as i128),
        rewards_per_second
    );

    index_assign!(
        farm.borrow_mut().crop_created_at.borrow_mut(),
        farm.borrow_mut()
            .crop_created_at
            .wrapped_index((crop_index as i128) as i128),
        current_timestamp
    );

    index_assign!(
        farm.borrow_mut().crop_stakers_finished.borrow_mut(),
        farm.borrow_mut()
            .crop_stakers_finished
            .wrapped_index((crop_index as i128) as i128),
        <u64 as TryFrom<_>>::try_from(0).unwrap()
    );

    index_assign!(
        farm.borrow_mut().crop_vault.borrow_mut(),
        farm.borrow_mut()
            .crop_vault
            .wrapped_index((crop_index as i128) as i128),
        crop_vault.key()
    );

    index_assign!(
        farm.borrow_mut().crop_rewards_per_token.borrow_mut(),
        farm.borrow_mut()
            .crop_rewards_per_token
            .wrapped_index((crop_index as i128) as i128),
        <u128 as TryFrom<_>>::try_from(0).unwrap()
    );

    let mut crop_active_time = reward_amount / rewards_per_second;
    let mut crop_famine_time = current_timestamp + crop_active_time;

    index_assign!(
        farm.borrow_mut().crop_end_date.borrow_mut(),
        farm.borrow_mut()
            .crop_end_date
            .wrapped_index((crop_index as i128) as i128),
        crop_famine_time
    );

    assign!(farm.borrow_mut().last_updated_at, current_timestamp);
}

pub fn create_crop_vault_handler<'info>(
    mut farm: Mutable<LoadedFarm<'info, '_>>,
    mut crop_vault: Empty<SeahorseAccount<'info, '_, TokenAccount>>,
    mut mint: SeahorseAccount<'info, '_, Mint>,
    mut protocol: Mutable<LoadedProtocol<'info, '_>>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    if !(farm.borrow().owner == signer.key()) {
        panic!("Wrong signer");
    }

    crop_vault.account.clone();
}

pub fn create_farm_handler<'info>(
    mut farm: Empty<Mutable<LoadedFarm<'info, '_>>>,
    mut stake_mint: SeahorseAccount<'info, '_, Mint>,
    mut stake_vault: Empty<SeahorseAccount<'info, '_, TokenAccount>>,
    mut clock: Sysvar<'info, Clock>,
    mut protocol: Mutable<LoadedProtocol<'info, '_>>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    solana_program::msg!("{}", "Creating farm".to_string());

    let mut created_farm = farm.account.clone();

    assign!(created_farm.borrow_mut().owner, signer.key());

    assign!(
        created_farm.borrow_mut().created_at,
        <u64 as TryFrom<_>>::try_from(clock.unix_timestamp.clone()).unwrap()
    );

    let mut created_stake_vault = stake_vault.account.clone();

    assign!(
        created_farm.borrow_mut().stake_vault,
        created_stake_vault.key()
    );

    assign!(created_farm.borrow_mut().stake_mint, stake_mint.key());

    assign!(
        created_farm.borrow_mut().last_updated_at,
        <u64 as TryFrom<_>>::try_from(clock.unix_timestamp.clone()).unwrap()
    );

    assign!(
        created_farm.borrow_mut().total_staked_amount,
        <u64 as TryFrom<_>>::try_from(0).unwrap()
    );
}

pub fn create_protocol_handler<'info>(
    mut protocol: Empty<Mutable<LoadedProtocol<'info, '_>>>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    let mut created_protocol = protocol.account.clone();

    assign!(
        created_protocol.borrow_mut().bump_seed,
        Pubkey::find_program_address(
            Mutable::new(vec!["protocol".to_string().as_bytes().as_ref()])
                .borrow()
                .as_slice(),
            &id()
        )
        .1
    );
}

pub fn create_stake_handler<'info>(
    mut stake: Empty<Mutable<LoadedStake<'info, '_>>>,
    mut farm: Mutable<LoadedFarm<'info, '_>>,
    mut clock: Sysvar<'info, Clock>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    let mut created_stake = stake.account.clone();

    assign!(
        created_stake.borrow_mut().farm,
        farm.borrow().__account__.key()
    );

    let mut owner = signer.key();

    assign!(
        created_stake.borrow_mut().last_updated_at,
        <u64 as TryFrom<_>>::try_from(clock.unix_timestamp.clone()).unwrap()
    );

    assign!(
        created_stake.borrow_mut().amount_staked,
        <u64 as TryFrom<_>>::try_from(0).unwrap()
    );

    assign!(
        created_stake.borrow_mut().created_at,
        <u64 as TryFrom<_>>::try_from(0).unwrap()
    );

    assign!(created_stake.borrow_mut().owner, owner);

    for mut i in 0..8 {
        index_assign!(
            created_stake.borrow_mut().reward_debt.borrow_mut(),
            created_stake
                .borrow_mut()
                .reward_debt
                .wrapped_index(i as i128),
            <u128 as TryFrom<_>>::try_from(0).unwrap()
        );

        index_assign!(
            created_stake.borrow_mut().last_gathered_at.borrow_mut(),
            created_stake
                .borrow_mut()
                .last_gathered_at
                .wrapped_index(i as i128),
            <u64 as TryFrom<_>>::try_from(0).unwrap()
        );

        index_assign!(
            created_stake.borrow_mut().amount_owed.borrow_mut(),
            created_stake
                .borrow_mut()
                .amount_owed
                .wrapped_index(i as i128),
            <u64 as TryFrom<_>>::try_from(0).unwrap()
        );
    }
}

pub fn gather_rewards_handler<'info>(
    mut crop_index: u8,
    mut farm: Mutable<LoadedFarm<'info, '_>>,
    mut crop_vault: SeahorseAccount<'info, '_, TokenAccount>,
    mut stake: Mutable<LoadedStake<'info, '_>>,
    mut signer_reward: SeahorseAccount<'info, '_, TokenAccount>,
    mut clock: Sysvar<'info, Clock>,
    mut signer: SeahorseSigner<'info, '_>,
    mut protocol: Mutable<LoadedProtocol<'info, '_>>,
) -> () {
    if !(stake.borrow().farm == farm.borrow().__account__.key()) {
        panic!("Wrong farm");
    }

    if !(farm.borrow().crop_vault.borrow()[farm
        .borrow()
        .crop_vault
        .wrapped_index((crop_index as i128) as i128)]
        == crop_vault.key())
    {
        panic!("Wrong stake vault");
    }

    if !(signer_reward.mint == crop_vault.mint) {
        panic!("Wrong mint");
    }

    if !((stake.borrow().owner == signer.key()) || (farm.borrow().owner == signer.key())) {
        panic!("Wrong signer");
    }

    if !(signer_reward.owner == signer.key()) {
        panic!("Not signers token account");
    }

    let mut current_timestamp =
        <u64 as TryFrom<_>>::try_from(clock.unix_timestamp.clone()).unwrap();

    let mut precision_scaler = get_precision_scaler();

    for mut i in 0..8 {
        if farm.borrow().crop_created_at.borrow()
            [farm.borrow().crop_created_at.wrapped_index(i as i128)]
            == 0
        {
            continue;
        }

        if farm.borrow().last_updated_at
            > farm.borrow().crop_end_date.borrow()
                [farm.borrow().crop_end_date.wrapped_index(i as i128)]
        {
            continue;
        }

        if farm.borrow().total_staked_amount == 0 {
            continue;
        }

        let mut current_time = current_timestamp.min(
            farm.borrow().crop_end_date.borrow()
                [farm.borrow().crop_end_date.wrapped_index(i as i128)],
        );

        if farm.borrow().total_staked_amount > <u64 as TryFrom<_>>::try_from(0).unwrap() {
            let mut time_diff = current_time - farm.borrow().last_updated_at;
            let mut rewards = <u128 as TryFrom<_>>::try_from(
                (time_diff
                    * farm.borrow().crop_rewards_per_second.borrow()[farm
                        .borrow()
                        .crop_rewards_per_second
                        .wrapped_index(i as i128)]),
            )
            .unwrap()
                * precision_scaler;

            let mut rewards_per_token = rewards / (farm.borrow().total_staked_amount as u128);
            let mut current_rewards_per_token = farm.borrow().crop_rewards_per_token.borrow()[farm
                .borrow()
                .crop_rewards_per_token
                .wrapped_index(i as i128)];

            index_assign!(
                farm.borrow_mut().crop_rewards_per_token.borrow_mut(),
                farm.borrow_mut()
                    .crop_rewards_per_token
                    .wrapped_index(i as i128),
                current_rewards_per_token
                    + <u128 as TryFrom<_>>::try_from(rewards_per_token.clone()).unwrap()
            );

            assign!(farm.borrow_mut().last_updated_at, current_time);
        }

        if stake.borrow().amount_staked == 0 {
            continue;
        }

        let mut pending_rewards =
            ((<u128 as TryFrom<_>>::try_from(stake.borrow().amount_staked.clone()).unwrap()
                * farm.borrow().crop_rewards_per_token.borrow()[farm
                    .borrow()
                    .crop_rewards_per_token
                    .wrapped_index(i as i128)])
                - stake.borrow().reward_debt.borrow()
                    [stake.borrow().reward_debt.wrapped_index(i as i128)])
                / precision_scaler;

        let mut amount_owed = stake.borrow().amount_owed.borrow()
            [stake.borrow().amount_owed.wrapped_index(i as i128)];

        index_assign!(
            stake.borrow_mut().amount_owed.borrow_mut(),
            stake.borrow_mut().amount_owed.wrapped_index(i as i128),
            amount_owed + <u64 as TryFrom<_>>::try_from(pending_rewards.clone()).unwrap()
        );
    }

    let mut rewards_earned = stake.borrow().amount_owed.borrow()[stake
        .borrow()
        .amount_owed
        .wrapped_index((crop_index as i128) as i128)];

    index_assign!(
        stake.borrow_mut().last_gathered_at.borrow_mut(),
        stake
            .borrow_mut()
            .last_gathered_at
            .wrapped_index((crop_index as i128) as i128),
        current_timestamp
    );

    let mut bump = protocol.borrow().bump_seed;

    token::transfer(
        CpiContext::new_with_signer(
            crop_vault.programs.get("token_program"),
            token::Transfer {
                from: crop_vault.to_account_info(),
                authority: protocol.borrow().__account__.to_account_info(),
                to: signer_reward.clone().to_account_info(),
            },
            &[Mutable::new(vec![
                "protocol".to_string().as_bytes().as_ref(),
                bump.to_le_bytes().as_ref(),
            ])
            .borrow()
            .as_slice()],
        ),
        rewards_earned.clone(),
    )
    .unwrap();

    index_assign!(
        stake.borrow_mut().amount_owed.borrow_mut(),
        stake
            .borrow_mut()
            .amount_owed
            .wrapped_index((crop_index as i128) as i128),
        <u64 as TryFrom<_>>::try_from(0).unwrap()
    );

    update_reward_debt(farm.clone(), stake.clone());
}

pub fn get_precision_scaler() -> u128 {
    return <u128 as TryFrom<_>>::try_from(1).unwrap()
        << <u128 as TryFrom<_>>::try_from(63).unwrap();
}

pub fn remove_crop_handler<'info>(
    mut farm: Mutable<LoadedFarm<'info, '_>>,
    mut crop_index: u8,
    mut crop_vault: SeahorseAccount<'info, '_, TokenAccount>,
    mut signer_reward: SeahorseAccount<'info, '_, TokenAccount>,
    mut clock: Sysvar<'info, Clock>,
    mut protocol: Mutable<LoadedProtocol<'info, '_>>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    if !(farm.borrow().owner == signer.key()) {
        panic!("Wrong signer");
    }

    if !(crop_index < <u8 as TryFrom<_>>::try_from(5).unwrap()) {
        panic!("Index too high");
    }

    if !(farm.borrow().crop_created_at.borrow()[farm
        .borrow()
        .crop_created_at
        .wrapped_index((crop_index as i128) as i128)]
        != 0)
    {
        panic!("Crop is empty");
    }

    let mut current_time = <u64 as TryFrom<_>>::try_from(clock.unix_timestamp.clone()).unwrap();

    if !(current_time
        > farm.borrow().crop_end_date.borrow()[farm
            .borrow()
            .crop_end_date
            .wrapped_index((crop_index as i128) as i128)])
    {
        panic!("Crop not ended");
    }

    if !(farm.borrow().stakers
        == farm.borrow().crop_stakers_finished.borrow()[farm
            .borrow()
            .crop_stakers_finished
            .wrapped_index((crop_index as i128) as i128)])
    {
        panic!("More stakers need to finish");
    }

    if !(crop_vault.mint == signer_reward.mint) {
        panic!("Wrong mint");
    }

    index_assign!(
        farm.borrow_mut().crop_rewards_per_second.borrow_mut(),
        farm.borrow_mut()
            .crop_rewards_per_second
            .wrapped_index((crop_index as i128) as i128),
        <u64 as TryFrom<_>>::try_from(0).unwrap()
    );

    index_assign!(
        farm.borrow_mut().crop_rewards_per_token.borrow_mut(),
        farm.borrow_mut()
            .crop_rewards_per_token
            .wrapped_index((crop_index as i128) as i128),
        <u128 as TryFrom<_>>::try_from(0).unwrap()
    );

    index_assign!(
        farm.borrow_mut().crop_end_date.borrow_mut(),
        farm.borrow_mut()
            .crop_end_date
            .wrapped_index((crop_index as i128) as i128),
        <u64 as TryFrom<_>>::try_from(0).unwrap()
    );

    index_assign!(
        farm.borrow_mut().crop_created_at.borrow_mut(),
        farm.borrow_mut()
            .crop_created_at
            .wrapped_index((crop_index as i128) as i128),
        <u64 as TryFrom<_>>::try_from(0).unwrap()
    );

    index_assign!(
        farm.borrow_mut().crop_stakers_finished.borrow_mut(),
        farm.borrow_mut()
            .crop_stakers_finished
            .wrapped_index((crop_index as i128) as i128),
        <u64 as TryFrom<_>>::try_from(0).unwrap()
    );

    let mut leftover_rewards = crop_vault.amount;
    let mut bump = protocol.borrow().bump_seed;

    token::transfer(
        CpiContext::new_with_signer(
            crop_vault.programs.get("token_program"),
            token::Transfer {
                from: crop_vault.to_account_info(),
                authority: protocol.borrow().__account__.to_account_info(),
                to: signer_reward.clone().to_account_info(),
            },
            &[Mutable::new(vec![
                "protocol".to_string().as_bytes().as_ref(),
                bump.to_le_bytes().as_ref(),
            ])
            .borrow()
            .as_slice()],
        ),
        leftover_rewards.clone(),
    )
    .unwrap();
}

pub fn stake_tokens_handler<'info>(
    mut amount: u64,
    mut farm: Mutable<LoadedFarm<'info, '_>>,
    mut stake_vault: SeahorseAccount<'info, '_, TokenAccount>,
    mut stake: Mutable<LoadedStake<'info, '_>>,
    mut signer_token: SeahorseAccount<'info, '_, TokenAccount>,
    mut clock: Sysvar<'info, Clock>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    if !(stake.borrow().farm == farm.borrow().__account__.key()) {
        panic!("Wrong farm");
    }

    if !(farm.borrow().stake_vault == stake_vault.key()) {
        panic!("Wrong stake vault");
    }

    if !(stake.borrow().owner == signer.key()) {
        panic!("Wrong signer");
    }

    if !(signer_token.mint == stake_vault.mint) {
        panic!("Wrong mint");
    }

    if !(signer_token.owner == signer.key()) {
        panic!("Not signers token account");
    }

    let mut current_timestamp =
        <u64 as TryFrom<_>>::try_from(clock.unix_timestamp.clone()).unwrap();

    let mut precision_scaler = get_precision_scaler();

    for mut i in 0..8 {
        if farm.borrow().crop_created_at.borrow()
            [farm.borrow().crop_created_at.wrapped_index(i as i128)]
            == 0
        {
            continue;
        }

        if farm.borrow().last_updated_at
            > farm.borrow().crop_end_date.borrow()
                [farm.borrow().crop_end_date.wrapped_index(i as i128)]
        {
            continue;
        }

        if farm.borrow().total_staked_amount == 0 {
            continue;
        }

        let mut current_time = current_timestamp.min(
            farm.borrow().crop_end_date.borrow()
                [farm.borrow().crop_end_date.wrapped_index(i as i128)],
        );

        if farm.borrow().total_staked_amount > <u64 as TryFrom<_>>::try_from(0).unwrap() {
            let mut time_diff = current_time - farm.borrow().last_updated_at;
            let mut rewards = <u128 as TryFrom<_>>::try_from(
                (time_diff
                    * farm.borrow().crop_rewards_per_second.borrow()[farm
                        .borrow()
                        .crop_rewards_per_second
                        .wrapped_index(i as i128)]),
            )
            .unwrap()
                * precision_scaler;

            let mut rewards_per_token = rewards / (farm.borrow().total_staked_amount as u128);
            let mut current_rewards_per_token = farm.borrow().crop_rewards_per_token.borrow()[farm
                .borrow()
                .crop_rewards_per_token
                .wrapped_index(i as i128)];

            index_assign!(
                farm.borrow_mut().crop_rewards_per_token.borrow_mut(),
                farm.borrow_mut()
                    .crop_rewards_per_token
                    .wrapped_index(i as i128),
                current_rewards_per_token
                    + <u128 as TryFrom<_>>::try_from(rewards_per_token.clone()).unwrap()
            );

            assign!(farm.borrow_mut().last_updated_at, current_time);
        }

        if stake.borrow().amount_staked == 0 {
            continue;
        }

        let mut pending_rewards =
            ((<u128 as TryFrom<_>>::try_from(stake.borrow().amount_staked.clone()).unwrap()
                * farm.borrow().crop_rewards_per_token.borrow()[farm
                    .borrow()
                    .crop_rewards_per_token
                    .wrapped_index(i as i128)])
                - stake.borrow().reward_debt.borrow()
                    [stake.borrow().reward_debt.wrapped_index(i as i128)])
                / precision_scaler;

        let mut amount_owed = stake.borrow().amount_owed.borrow()
            [stake.borrow().amount_owed.wrapped_index(i as i128)];

        index_assign!(
            stake.borrow_mut().amount_owed.borrow_mut(),
            stake.borrow_mut().amount_owed.wrapped_index(i as i128),
            amount_owed + <u64 as TryFrom<_>>::try_from(pending_rewards.clone()).unwrap()
        );
    }

    assign!(
        farm.borrow_mut().total_staked_amount,
        farm.borrow().total_staked_amount + amount
    );

    assign!(
        stake.borrow_mut().amount_staked,
        stake.borrow().amount_staked + amount
    );

    token::transfer(
        CpiContext::new(
            signer_token.programs.get("token_program"),
            token::Transfer {
                from: signer_token.to_account_info(),
                authority: signer.clone().to_account_info(),
                to: stake_vault.clone().to_account_info(),
            },
        ),
        amount.clone(),
    )
    .unwrap();

    assign!(
        stake.borrow_mut().last_updated_at,
        <u64 as TryFrom<_>>::try_from(clock.unix_timestamp.clone()).unwrap()
    );

    update_reward_debt(farm.clone(), stake.clone());
}

pub fn unstake_tokens_handler<'info>(
    mut amount: u64,
    mut farm: Mutable<LoadedFarm<'info, '_>>,
    mut stake_vault: SeahorseAccount<'info, '_, TokenAccount>,
    mut stake: Mutable<LoadedStake<'info, '_>>,
    mut signer_token: SeahorseAccount<'info, '_, TokenAccount>,
    mut clock: Sysvar<'info, Clock>,
    mut protocol: Mutable<LoadedProtocol<'info, '_>>,
    mut signer: SeahorseSigner<'info, '_>,
) -> () {
    if !(stake.borrow().farm == farm.borrow().__account__.key()) {
        panic!("Wrong farm");
    }

    if !(farm.borrow().stake_vault == stake_vault.key()) {
        panic!("Wrong stake vault");
    }

    if !(stake.borrow().owner == signer.key()) {
        panic!("Wrong signer");
    }

    if !(signer_token.mint == stake_vault.mint) {
        panic!("Wrong mint");
    }

    if !(signer_token.owner == signer.key()) {
        panic!("Not signers token account");
    }

    let mut precision_scaler = get_precision_scaler();
    let mut current_timestamp =
        <u64 as TryFrom<_>>::try_from(clock.unix_timestamp.clone()).unwrap();

    let mut precision_scaler = get_precision_scaler();

    for mut i in 0..8 {
        if farm.borrow().crop_created_at.borrow()
            [farm.borrow().crop_created_at.wrapped_index(i as i128)]
            == 0
        {
            continue;
        }

        if farm.borrow().last_updated_at
            > farm.borrow().crop_end_date.borrow()
                [farm.borrow().crop_end_date.wrapped_index(i as i128)]
        {
            continue;
        }

        if farm.borrow().total_staked_amount == 0 {
            continue;
        }

        let mut current_time = current_timestamp.min(
            farm.borrow().crop_end_date.borrow()
                [farm.borrow().crop_end_date.wrapped_index(i as i128)],
        );

        if farm.borrow().total_staked_amount > <u64 as TryFrom<_>>::try_from(0).unwrap() {
            let mut time_diff = current_time - farm.borrow().last_updated_at;
            let mut rewards = <u128 as TryFrom<_>>::try_from(
                (time_diff
                    * farm.borrow().crop_rewards_per_second.borrow()[farm
                        .borrow()
                        .crop_rewards_per_second
                        .wrapped_index(i as i128)]),
            )
            .unwrap()
                * precision_scaler;

            let mut rewards_per_token = rewards / (farm.borrow().total_staked_amount as u128);
            let mut current_rewards_per_token = farm.borrow().crop_rewards_per_token.borrow()[farm
                .borrow()
                .crop_rewards_per_token
                .wrapped_index(i as i128)];

            index_assign!(
                farm.borrow_mut().crop_rewards_per_token.borrow_mut(),
                farm.borrow_mut()
                    .crop_rewards_per_token
                    .wrapped_index(i as i128),
                current_rewards_per_token
                    + <u128 as TryFrom<_>>::try_from(rewards_per_token.clone()).unwrap()
            );

            assign!(farm.borrow_mut().last_updated_at, current_time);
        }

        if stake.borrow().amount_staked == 0 {
            continue;
        }

        let mut pending_rewards =
            ((<u128 as TryFrom<_>>::try_from(stake.borrow().amount_staked.clone()).unwrap()
                * farm.borrow().crop_rewards_per_token.borrow()[farm
                    .borrow()
                    .crop_rewards_per_token
                    .wrapped_index(i as i128)])
                - stake.borrow().reward_debt.borrow()
                    [stake.borrow().reward_debt.wrapped_index(i as i128)])
                / precision_scaler;

        let mut amount_owed = stake.borrow().amount_owed.borrow()
            [stake.borrow().amount_owed.wrapped_index(i as i128)];

        index_assign!(
            stake.borrow_mut().amount_owed.borrow_mut(),
            stake.borrow_mut().amount_owed.wrapped_index(i as i128),
            amount_owed + <u64 as TryFrom<_>>::try_from(pending_rewards.clone()).unwrap()
        );
    }

    let mut bump = protocol.borrow().bump_seed;

    token::transfer(
        CpiContext::new_with_signer(
            stake_vault.programs.get("token_program"),
            token::Transfer {
                from: stake_vault.to_account_info(),
                authority: protocol.borrow().__account__.to_account_info(),
                to: signer_token.clone().to_account_info(),
            },
            &[Mutable::new(vec![
                "protocol".to_string().as_bytes().as_ref(),
                bump.to_le_bytes().as_ref(),
            ])
            .borrow()
            .as_slice()],
        ),
        amount.clone(),
    )
    .unwrap();

    assign!(
        farm.borrow_mut().total_staked_amount,
        farm.borrow().total_staked_amount - amount
    );

    assign!(
        stake.borrow_mut().last_updated_at,
        <u64 as TryFrom<_>>::try_from(clock.unix_timestamp.clone()).unwrap()
    );

    assign!(
        stake.borrow_mut().amount_staked,
        stake.borrow().amount_staked - amount
    );

    update_reward_debt(farm.clone(), stake.clone());
}

pub fn update_reward_debt<'info>(
    mut farm: Mutable<LoadedFarm<'info, '_>>,
    mut stake: Mutable<LoadedStake<'info, '_>>,
) -> () {
    let mut precision_scaler = get_precision_scaler();

    for mut i in 0..8 {
        index_assign!(
            stake.borrow_mut().reward_debt.borrow_mut(),
            stake.borrow_mut().reward_debt.wrapped_index(i as i128),
            (stake.borrow().amount_staked as u128)
                * farm.borrow().crop_rewards_per_token.borrow()[farm
                    .borrow()
                    .crop_rewards_per_token
                    .wrapped_index(i as i128)]
        );
    }
}
