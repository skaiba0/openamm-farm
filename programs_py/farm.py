# farm
# Built with Seahorse v0.2.7

from seahorse.prelude import *

declare_id("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS")


# class Crop:
#     name: u64


# class Crop:
#     reward_vault: Pubkey
#     reward_per_second: u64
#     last_update_at: i64
#     rewards_accured_per_share: u64


def get_precision_scaler() -> u128:
    return u128(1) << u128(63)


class Stake(Account):
    owner: Pubkey
    created_at: u64
    amount_staked: u64
    last_updated_at: u64
    farm: Pubkey
    reward_debt: Array[u128, 8]  # one for each crop
    last_gathered_at: Array[u64, 8]  # one for each crop
    # For staking and unstaking, we can keep track of the amount of tokens earned
    amount_owed: Array[u64, 8]


class Farm(Account):
    owner: Pubkey
    created_at: u64
    stake_vault: Pubkey
    stake_mint: Pubkey
    last_updated_at: u64
    # The program should either take end date as a parameter
    total_staked_amount: u64
    stakers: u64
    crop_vault: Array[Pubkey, 8]
    crop_rewards_per_second: Array[u64, 8]
    crop_rewards_per_token: Array[u128, 8]
    crop_end_date: Array[u64, 8]
    crop_created_at: Array[u64, 8]
    crop_stakers_finished: Array[u64, 8]


class Protocol(Account):
    bump_seed: u8


def update_reward_debt(
    farm: Farm,
    stake: Stake,
):
    precision_scaler = get_precision_scaler()
    for i in range(8):
        stake.reward_debt[i] = stake.amount_staked * farm.crop_rewards_per_token[i]


@instruction
def create_protocol(protocol: Empty[Protocol], signer: Signer):
    created_protocol = protocol.init(
        payer=signer,
        seeds=["protocol"],
    )
    created_protocol.bump_seed = Pubkey.find_program_address(["protocol"])[1]


@instruction
def create_farm(
    farm: Empty[Farm],
    stake_mint: TokenMint,
    stake_vault: Empty[TokenAccount],
    clock: Clock,
    protocol: Protocol,
    signer: Signer,
):
    print("Creating farm")
    created_farm = farm.init(payer=signer, seeds=["farm", signer.key(), stake_mint])
    created_farm.owner = signer.key()
    created_farm.created_at = u64(clock.unix_timestamp)
    created_stake_vault = stake_vault.init(
        payer=signer,
        seeds=["farm-stake-vault", signer.key(), stake_mint],
        authority=protocol,
        mint=stake_mint,
    )
    created_farm.stake_vault = created_stake_vault.key()
    created_farm.stake_mint = stake_mint.key()
    created_farm.last_updated_at = u64(clock.unix_timestamp)
    created_farm.total_staked_amount = u64(0)


# This is not in the same instruction as add_crop in case that someone wants to create a second crop with the same mint
@instruction
def create_crop_vault(
    farm: Farm,
    crop_vault: Empty[TokenAccount],
    mint: TokenMint,
    protocol: Protocol,
    signer: Signer,
):
    assert farm.owner == signer.key(), "Wrong signer"
    crop_vault.init(
        payer=signer,
        seeds=["farm-crop-vault", farm.key(), mint.key()],
        mint=mint,
        authority=protocol,
    )


@instruction
def add_crop(
    crop_index: u8,
    reward_amount: u64,
    rewards_per_second: u64,
    farm: Farm,
    crop_vault: TokenAccount,
    signer_reward: TokenAccount,
    clock: Clock,
    signer: Signer,
):
    assert farm.owner == signer.key(), "Wrong signer"
    assert crop_index < u8(5), "Index too high"
    assert crop_vault.mint() == signer_reward.mint(), "Wrong mint"
    # assert crop_vault.key() == farm.crop_vault[crop_index], "Wrong crop vault"
    assert farm.crop_created_at[crop_index] == 0, "Crop already active at this index"

    # UPDATE CROPS
    current_timestamp = u64(clock.unix_timestamp)
    precision_scaler = get_precision_scaler()
    for i in range(8):
        # Crop not active
        if farm.crop_created_at[i] == 0:
            continue
        # Crop already ended
        if farm.last_updated_at > farm.crop_end_date[i]:
            continue
        if farm.total_staked_amount == 0:
            continue

        # UPDATE CROP INDEX
        current_time = min(current_timestamp, farm.crop_end_date[i])
        if farm.total_staked_amount > u64(0):
            time_diff = current_time - farm.last_updated_at
            rewards = (
                u128(time_diff * farm.crop_rewards_per_second[i]) * precision_scaler
            )
            rewards_per_token = rewards // farm.total_staked_amount
            current_rewards_per_token = farm.crop_rewards_per_token[i]
            farm.crop_rewards_per_token[i] = current_rewards_per_token + u128(
                rewards_per_token
            )
            farm.last_updated_at = current_time
        # END UPDATE CROP INDEX

    signer_reward.transfer(
        authority=signer,
        to=crop_vault,
        amount=reward_amount,
    )
    farm.crop_rewards_per_second[crop_index] = rewards_per_second
    farm.crop_created_at[crop_index] = current_timestamp
    farm.crop_stakers_finished[crop_index] = u64(0)
    farm.crop_vault[crop_index] = crop_vault.key()
    farm.crop_rewards_per_token[crop_index] = u128(0)

    crop_active_time = reward_amount // rewards_per_second
    crop_famine_time = current_timestamp + crop_active_time
    farm.crop_end_date[crop_index] = crop_famine_time
    farm.last_updated_at = current_timestamp


@instruction
def remove_crop(
    farm: Farm,
    crop_index: u8,
    crop_vault: TokenAccount,
    signer_reward: TokenAccount,
    clock: Clock,
    protocol: Protocol,
    signer: Signer,
):
    assert farm.owner == signer.key(), "Wrong signer"
    assert crop_index < u8(5), "Index too high"
    assert farm.crop_created_at[crop_index] != 0, "Crop is empty"
    # assert crop_index < farm.total_crops, "Crop index out of bounds"
    current_time = u64(clock.unix_timestamp)
    assert current_time > farm.crop_end_date[crop_index], "Crop not ended"
    assert (
        farm.stakers == farm.crop_stakers_finished[crop_index]
    ), "More stakers need to finish"
    assert crop_vault.mint() == signer_reward.mint(), "Wrong mint"

    # Reset the pubkey if possible
    # farm.crop_vault[crop_index] = Pubkey(0)
    farm.crop_rewards_per_second[crop_index] = u64(0)
    farm.crop_rewards_per_token[crop_index] = u128(0)
    farm.crop_end_date[crop_index] = u64(0)
    farm.crop_created_at[crop_index] = u64(0)
    farm.crop_stakers_finished[crop_index] = u64(0)

    leftover_rewards = crop_vault.amount()
    bump = protocol.bump_seed
    crop_vault.transfer(
        authority=protocol,
        to=signer_reward,
        amount=leftover_rewards,
        signer=["protocol", bump],
    )


@instruction
def create_stake(stake: Empty[Stake], farm: Farm, clock: Clock, signer: Signer):
    created_stake = stake.init(
        payer=signer,
        seeds=["stake", farm.key(), signer.key()],
    )
    created_stake.farm = farm.key()
    owner = signer.key()
    created_stake.last_updated_at = u64(clock.unix_timestamp)
    created_stake.amount_staked = u64(0)
    created_stake.created_at = u64(0)
    created_stake.owner = owner
    for i in range(8):
        created_stake.reward_debt[i] = u128(0)
        created_stake.last_gathered_at[i] = u64(0)
        created_stake.amount_owed[i] = u64(0)


@instruction
def stake_tokens(
    amount: u64,
    farm: Farm,
    stake_vault: TokenAccount,
    stake: Stake,
    signer_token: TokenAccount,
    clock: Clock,
    signer: Signer,
):
    assert stake.farm == farm.key(), "Wrong farm"
    assert farm.stake_vault == stake_vault.key(), "Wrong stake vault"
    assert stake.owner == signer.key(), "Wrong signer"
    assert signer_token.mint() == stake_vault.mint(), "Wrong mint"
    assert signer_token.authority() == signer.key(), "Not signers token account"

    # UPDATE CROPS
    current_timestamp = u64(clock.unix_timestamp)
    precision_scaler = get_precision_scaler()
    for i in range(8):
        # Crop not active
        if farm.crop_created_at[i] == 0:
            continue
        # Crop already ended
        if farm.last_updated_at > farm.crop_end_date[i]:
            continue
        if farm.total_staked_amount == 0:
            continue

        # UPDATE CROP INDEX
        current_time = min(current_timestamp, farm.crop_end_date[i])
        if farm.total_staked_amount > u64(0):
            time_diff = current_time - farm.last_updated_at
            rewards = (
                u128(time_diff * farm.crop_rewards_per_second[i]) * precision_scaler
            )

            rewards_per_token = rewards // farm.total_staked_amount
            current_rewards_per_token = farm.crop_rewards_per_token[i]
            farm.crop_rewards_per_token[i] = current_rewards_per_token + u128(
                rewards_per_token
            )
            farm.last_updated_at = current_time
        # END UPDATE CROP INDEX

        # UPDATE REWARDS
        if stake.amount_staked == 0:
            continue

        pending_rewards = (
            u128(stake.amount_staked) * farm.crop_rewards_per_token[i]
            - stake.reward_debt[i]
        ) // precision_scaler
        amount_owed = stake.amount_owed[i]
        stake.amount_owed[i] = amount_owed + u64(pending_rewards)

        # END UPDATE REWARDS

    farm.total_staked_amount += amount
    stake.amount_staked += amount

    signer_token.transfer(
        authority=signer,
        to=stake_vault,
        amount=amount,
    )

    stake.last_updated_at = u64(clock.unix_timestamp)

    update_reward_debt(farm, stake)


@instruction
def unstake_tokens(
    amount: u64,
    farm: Farm,
    stake_vault: TokenAccount,
    stake: Stake,
    signer_token: TokenAccount,
    clock: Clock,
    protocol: Protocol,
    signer: Signer,
):
    assert stake.farm == farm.key(), "Wrong farm"
    assert farm.stake_vault == stake_vault.key(), "Wrong stake vault"
    assert stake.owner == signer.key(), "Wrong signer"
    assert signer_token.mint() == stake_vault.mint(), "Wrong mint"
    assert signer_token.authority() == signer.key(), "Not signers token account"

    # Essentially the logic for this instruction is to update amount_owed for the user, then update earned_per_share for everybody else

    # farm.total_staked_amount += amount

    precision_scaler = get_precision_scaler()
    # UPDATE CROPS
    current_timestamp = u64(clock.unix_timestamp)
    precision_scaler = get_precision_scaler()
    for i in range(8):
        # Crop not active
        if farm.crop_created_at[i] == 0:
            continue
        # Crop already ended
        if farm.last_updated_at > farm.crop_end_date[i]:
            continue
        if farm.total_staked_amount == 0:
            continue

        # UPDATE CROP INDEX
        current_time = min(current_timestamp, farm.crop_end_date[i])
        if farm.total_staked_amount > u64(0):
            time_diff = current_time - farm.last_updated_at
            rewards = (
                u128(time_diff * farm.crop_rewards_per_second[i]) * precision_scaler
            )

            rewards_per_token = rewards // farm.total_staked_amount
            current_rewards_per_token = farm.crop_rewards_per_token[i]
            farm.crop_rewards_per_token[i] = current_rewards_per_token + u128(
                rewards_per_token
            )
            farm.last_updated_at = current_time
        # END UPDATE CROP INDEX

        # UPDATE REWARDS
        if stake.amount_staked == 0:
            continue

        pending_rewards = (
            u128(stake.amount_staked) * farm.crop_rewards_per_token[i]
            - stake.reward_debt[i]
        ) // precision_scaler
        amount_owed = stake.amount_owed[i]
        stake.amount_owed[i] = amount_owed + u64(pending_rewards)

        # END UPDATE REWARDS

    bump = protocol.bump_seed
    stake_vault.transfer(
        authority=protocol, to=signer_token, amount=amount, signer=["protocol", bump]
    )

    farm.total_staked_amount -= amount
    stake.last_updated_at = u64(clock.unix_timestamp)
    stake.amount_staked -= amount

    update_reward_debt(farm, stake)


@instruction
def gather_rewards(
    crop_index: u8,
    farm: Farm,
    crop_vault: TokenAccount,
    stake: Stake,
    signer_reward: TokenAccount,
    clock: Clock,
    signer: Signer,
    protocol: Protocol,
):
    assert stake.farm == farm.key(), "Wrong farm"
    assert farm.crop_vault[crop_index] == crop_vault.key(), "Wrong stake vault"
    assert signer_reward.mint() == crop_vault.mint(), "Wrong mint"
    assert stake.owner == signer.key() or farm.owner == signer.key(), "Wrong signer"
    assert signer_reward.authority() == signer.key(), "Not signers token account"
    # Crop was refreshed

    # UPDATE CROPS
    current_timestamp = u64(clock.unix_timestamp)
    precision_scaler = get_precision_scaler()
    for i in range(8):
        # Crop not active
        if farm.crop_created_at[i] == 0:
            continue
        # Crop already ended
        if farm.last_updated_at > farm.crop_end_date[i]:
            continue
        if farm.total_staked_amount == 0:
            continue

        # UPDATE CROP INDEX
        current_time = min(current_timestamp, farm.crop_end_date[i])
        if farm.total_staked_amount > u64(0):
            time_diff = current_time - farm.last_updated_at
            rewards = (
                u128(time_diff * farm.crop_rewards_per_second[i]) * precision_scaler
            )

            rewards_per_token = rewards // farm.total_staked_amount
            current_rewards_per_token = farm.crop_rewards_per_token[i]
            farm.crop_rewards_per_token[i] = current_rewards_per_token + u128(
                rewards_per_token
            )
            farm.last_updated_at = current_time
        # END UPDATE CROP INDEX

        # UPDATE REWARDS
        if stake.amount_staked == 0:
            continue

        pending_rewards = (
            u128(stake.amount_staked) * farm.crop_rewards_per_token[i]
            - stake.reward_debt[i]
        ) // precision_scaler
        amount_owed = stake.amount_owed[i]
        stake.amount_owed[i] = amount_owed + u64(pending_rewards)

        # END UPDATE REWARDS

    rewards_earned = stake.amount_owed[crop_index]
    stake.last_gathered_at[crop_index] = current_timestamp
    bump = protocol.bump_seed
    crop_vault.transfer(
        authority=protocol,
        to=signer_reward,
        amount=rewards_earned,
        signer=["protocol", bump],
    )
    stake.amount_owed[crop_index] = u64(0)
    update_reward_debt(farm, stake)
