#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod dot;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount},
};

use dot::program::*;
use std::{cell::RefCell, rc::Rc};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod seahorse_util {
    use super::*;

    #[cfg(feature = "pyth-sdk-solana")]
    pub use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed};
    use std::{collections::HashMap, fmt::Debug, ops::Deref};

    pub struct Mutable<T>(Rc<RefCell<T>>);

    impl<T> Mutable<T> {
        pub fn new(obj: T) -> Self {
            Self(Rc::new(RefCell::new(obj)))
        }
    }

    impl<T> Clone for Mutable<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<T> Deref for Mutable<T> {
        type Target = Rc<RefCell<T>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: Debug> Debug for Mutable<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl<T: Default> Default for Mutable<T> {
        fn default() -> Self {
            Self::new(T::default())
        }
    }

    impl<T: Clone> Mutable<Vec<T>> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index >= 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    impl<T: Clone, const N: usize> Mutable<[T; N]> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index >= 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    #[derive(Clone)]
    pub struct Empty<T: Clone> {
        pub account: T,
        pub bump: Option<u8>,
    }

    #[derive(Clone, Debug)]
    pub struct ProgramsMap<'info>(pub HashMap<&'static str, AccountInfo<'info>>);

    impl<'info> ProgramsMap<'info> {
        pub fn get(&self, name: &'static str) -> AccountInfo<'info> {
            self.0.get(name).unwrap().clone()
        }
    }

    #[derive(Clone, Debug)]
    pub struct WithPrograms<'info, 'entrypoint, A> {
        pub account: &'entrypoint A,
        pub programs: &'entrypoint ProgramsMap<'info>,
    }

    impl<'info, 'entrypoint, A> Deref for WithPrograms<'info, 'entrypoint, A> {
        type Target = A;

        fn deref(&self) -> &Self::Target {
            &self.account
        }
    }

    pub type SeahorseAccount<'info, 'entrypoint, A> =
        WithPrograms<'info, 'entrypoint, Box<Account<'info, A>>>;

    pub type SeahorseSigner<'info, 'entrypoint> = WithPrograms<'info, 'entrypoint, Signer<'info>>;

    #[derive(Clone, Debug)]
    pub struct CpiAccount<'info> {
        #[doc = "CHECK: CpiAccounts temporarily store AccountInfos."]
        pub account_info: AccountInfo<'info>,
        pub is_writable: bool,
        pub is_signer: bool,
        pub seeds: Option<Vec<Vec<u8>>>,
    }

    #[macro_export]
    macro_rules! seahorse_const {
        ($ name : ident , $ value : expr) => {
            macro_rules! $name {
                () => {
                    $value
                };
            }

            pub(crate) use $name;
        };
    }

    #[macro_export]
    macro_rules! assign {
        ($ lval : expr , $ rval : expr) => {{
            let temp = $rval;

            $lval = temp;
        }};
    }

    #[macro_export]
    macro_rules! index_assign {
        ($ lval : expr , $ idx : expr , $ rval : expr) => {
            let temp_rval = $rval;
            let temp_idx = $idx;

            $lval[temp_idx] = temp_rval;
        };
    }

    pub(crate) use assign;

    pub(crate) use index_assign;

    pub(crate) use seahorse_const;
}

#[program]
mod farm {
    use super::*;
    use seahorse_util::*;
    use std::collections::HashMap;

    #[derive(Accounts)]
    # [instruction (crop_index : u8 , reward_amount : u64 , rewards_per_second : u64)]
    pub struct AddCrop<'info> {
        #[account(mut)]
        pub farm: Box<Account<'info, dot::program::Farm>>,
        #[account(mut)]
        pub crop_vault: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub signer_reward: Box<Account<'info, TokenAccount>>,
        #[account()]
        pub clock: Sysvar<'info, Clock>,
        #[account(mut)]
        pub signer: Signer<'info>,
        pub token_program: Program<'info, Token>,
    }

    pub fn add_crop(
        ctx: Context<AddCrop>,
        crop_index: u8,
        reward_amount: u64,
        rewards_per_second: u64,
    ) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let farm = dot::program::Farm::load(&mut ctx.accounts.farm, &programs_map);
        let crop_vault = SeahorseAccount {
            account: &ctx.accounts.crop_vault,
            programs: &programs_map,
        };

        let signer_reward = SeahorseAccount {
            account: &ctx.accounts.signer_reward,
            programs: &programs_map,
        };

        let clock = &ctx.accounts.clock.clone();
        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        add_crop_handler(
            crop_index,
            reward_amount,
            rewards_per_second,
            farm.clone(),
            crop_vault.clone(),
            signer_reward.clone(),
            clock.clone(),
            signer.clone(),
        );

        dot::program::Farm::store(farm);

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct CreateCropVault<'info> {
        #[account(mut)]
        pub farm: Box<Account<'info, dot::program::Farm>>,
        # [account (init , payer = signer , seeds = ["farm-crop-vault" . as_bytes () . as_ref () , farm . key () . as_ref () , mint . key () . as_ref ()] , bump , token :: mint = mint , token :: authority = protocol)]
        pub crop_vault: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub mint: Box<Account<'info, Mint>>,
        #[account(mut)]
        pub protocol: Box<Account<'info, dot::program::Protocol>>,
        #[account(mut)]
        pub signer: Signer<'info>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
        pub token_program: Program<'info, Token>,
    }

    pub fn create_crop_vault(ctx: Context<CreateCropVault>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let farm = dot::program::Farm::load(&mut ctx.accounts.farm, &programs_map);
        let crop_vault = Empty {
            account: SeahorseAccount {
                account: &ctx.accounts.crop_vault,
                programs: &programs_map,
            },
            bump: ctx.bumps.get("crop_vault").map(|bump| *bump),
        };

        let mint = SeahorseAccount {
            account: &ctx.accounts.mint,
            programs: &programs_map,
        };

        let protocol = dot::program::Protocol::load(&mut ctx.accounts.protocol, &programs_map);
        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        create_crop_vault_handler(
            farm.clone(),
            crop_vault.clone(),
            mint.clone(),
            protocol.clone(),
            signer.clone(),
        );

        dot::program::Farm::store(farm);

        dot::program::Protocol::store(protocol);

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct CreateFarm<'info> {
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: Farm > () + 8 , payer = signer , seeds = ["farm" . as_bytes () . as_ref () , signer . key () . as_ref () , stake_mint . key () . as_ref ()] , bump)]
        pub farm: Box<Account<'info, dot::program::Farm>>,
        #[account(mut)]
        pub stake_mint: Box<Account<'info, Mint>>,
        # [account (init , payer = signer , seeds = ["farm-stake-vault" . as_bytes () . as_ref () , signer . key () . as_ref () , stake_mint . key () . as_ref ()] , bump , token :: mint = stake_mint , token :: authority = protocol)]
        pub stake_vault: Box<Account<'info, TokenAccount>>,
        #[account()]
        pub clock: Sysvar<'info, Clock>,
        #[account(mut)]
        pub protocol: Box<Account<'info, dot::program::Protocol>>,
        #[account(mut)]
        pub signer: Signer<'info>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
        pub token_program: Program<'info, Token>,
    }

    pub fn create_farm(ctx: Context<CreateFarm>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let farm = Empty {
            account: dot::program::Farm::load(&mut ctx.accounts.farm, &programs_map),
            bump: ctx.bumps.get("farm").map(|bump| *bump),
        };

        let stake_mint = SeahorseAccount {
            account: &ctx.accounts.stake_mint,
            programs: &programs_map,
        };

        let stake_vault = Empty {
            account: SeahorseAccount {
                account: &ctx.accounts.stake_vault,
                programs: &programs_map,
            },
            bump: ctx.bumps.get("stake_vault").map(|bump| *bump),
        };

        let clock = &ctx.accounts.clock.clone();
        let protocol = dot::program::Protocol::load(&mut ctx.accounts.protocol, &programs_map);
        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        create_farm_handler(
            farm.clone(),
            stake_mint.clone(),
            stake_vault.clone(),
            clock.clone(),
            protocol.clone(),
            signer.clone(),
        );

        dot::program::Farm::store(farm.account);

        dot::program::Protocol::store(protocol);

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct CreateProtocol<'info> {
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: Protocol > () + 8 , payer = signer , seeds = ["protocol" . as_bytes () . as_ref ()] , bump)]
        pub protocol: Box<Account<'info, dot::program::Protocol>>,
        #[account(mut)]
        pub signer: Signer<'info>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn create_protocol(ctx: Context<CreateProtocol>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let protocol = Empty {
            account: dot::program::Protocol::load(&mut ctx.accounts.protocol, &programs_map),
            bump: ctx.bumps.get("protocol").map(|bump| *bump),
        };

        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        create_protocol_handler(protocol.clone(), signer.clone());

        dot::program::Protocol::store(protocol.account);

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct CreateStake<'info> {
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: Stake > () + 8 , payer = signer , seeds = ["stake" . as_bytes () . as_ref () , farm . key () . as_ref () , signer . key () . as_ref ()] , bump)]
        pub stake: Box<Account<'info, dot::program::Stake>>,
        #[account(mut)]
        pub farm: Box<Account<'info, dot::program::Farm>>,
        #[account()]
        pub clock: Sysvar<'info, Clock>,
        #[account(mut)]
        pub signer: Signer<'info>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn create_stake(ctx: Context<CreateStake>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let stake = Empty {
            account: dot::program::Stake::load(&mut ctx.accounts.stake, &programs_map),
            bump: ctx.bumps.get("stake").map(|bump| *bump),
        };

        let farm = dot::program::Farm::load(&mut ctx.accounts.farm, &programs_map);
        let clock = &ctx.accounts.clock.clone();
        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        create_stake_handler(stake.clone(), farm.clone(), clock.clone(), signer.clone());

        dot::program::Stake::store(stake.account);

        dot::program::Farm::store(farm);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (crop_index : u8)]
    pub struct GatherRewards<'info> {
        #[account(mut)]
        pub farm: Box<Account<'info, dot::program::Farm>>,
        #[account(mut)]
        pub crop_vault: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub stake: Box<Account<'info, dot::program::Stake>>,
        #[account(mut)]
        pub signer_reward: Box<Account<'info, TokenAccount>>,
        #[account()]
        pub clock: Sysvar<'info, Clock>,
        #[account(mut)]
        pub signer: Signer<'info>,
        #[account(mut)]
        pub protocol: Box<Account<'info, dot::program::Protocol>>,
        pub token_program: Program<'info, Token>,
    }

    pub fn gather_rewards(ctx: Context<GatherRewards>, crop_index: u8) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let farm = dot::program::Farm::load(&mut ctx.accounts.farm, &programs_map);
        let crop_vault = SeahorseAccount {
            account: &ctx.accounts.crop_vault,
            programs: &programs_map,
        };

        let stake = dot::program::Stake::load(&mut ctx.accounts.stake, &programs_map);
        let signer_reward = SeahorseAccount {
            account: &ctx.accounts.signer_reward,
            programs: &programs_map,
        };

        let clock = &ctx.accounts.clock.clone();
        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        let protocol = dot::program::Protocol::load(&mut ctx.accounts.protocol, &programs_map);

        gather_rewards_handler(
            crop_index,
            farm.clone(),
            crop_vault.clone(),
            stake.clone(),
            signer_reward.clone(),
            clock.clone(),
            signer.clone(),
            protocol.clone(),
        );

        dot::program::Farm::store(farm);

        dot::program::Stake::store(stake);

        dot::program::Protocol::store(protocol);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (crop_index : u8)]
    pub struct RemoveCrop<'info> {
        #[account(mut)]
        pub farm: Box<Account<'info, dot::program::Farm>>,
        #[account(mut)]
        pub crop_vault: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub signer_reward: Box<Account<'info, TokenAccount>>,
        #[account()]
        pub clock: Sysvar<'info, Clock>,
        #[account(mut)]
        pub protocol: Box<Account<'info, dot::program::Protocol>>,
        #[account(mut)]
        pub signer: Signer<'info>,
        pub token_program: Program<'info, Token>,
    }

    pub fn remove_crop(ctx: Context<RemoveCrop>, crop_index: u8) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let farm = dot::program::Farm::load(&mut ctx.accounts.farm, &programs_map);
        let crop_vault = SeahorseAccount {
            account: &ctx.accounts.crop_vault,
            programs: &programs_map,
        };

        let signer_reward = SeahorseAccount {
            account: &ctx.accounts.signer_reward,
            programs: &programs_map,
        };

        let clock = &ctx.accounts.clock.clone();
        let protocol = dot::program::Protocol::load(&mut ctx.accounts.protocol, &programs_map);
        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        remove_crop_handler(
            farm.clone(),
            crop_index,
            crop_vault.clone(),
            signer_reward.clone(),
            clock.clone(),
            protocol.clone(),
            signer.clone(),
        );

        dot::program::Farm::store(farm);

        dot::program::Protocol::store(protocol);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (amount : u64)]
    pub struct StakeTokens<'info> {
        #[account(mut)]
        pub farm: Box<Account<'info, dot::program::Farm>>,
        #[account(mut)]
        pub stake_vault: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub stake: Box<Account<'info, dot::program::Stake>>,
        #[account(mut)]
        pub signer_token: Box<Account<'info, TokenAccount>>,
        #[account()]
        pub clock: Sysvar<'info, Clock>,
        #[account(mut)]
        pub signer: Signer<'info>,
        pub token_program: Program<'info, Token>,
    }

    pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let farm = dot::program::Farm::load(&mut ctx.accounts.farm, &programs_map);
        let stake_vault = SeahorseAccount {
            account: &ctx.accounts.stake_vault,
            programs: &programs_map,
        };

        let stake = dot::program::Stake::load(&mut ctx.accounts.stake, &programs_map);
        let signer_token = SeahorseAccount {
            account: &ctx.accounts.signer_token,
            programs: &programs_map,
        };

        let clock = &ctx.accounts.clock.clone();
        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        stake_tokens_handler(
            amount,
            farm.clone(),
            stake_vault.clone(),
            stake.clone(),
            signer_token.clone(),
            clock.clone(),
            signer.clone(),
        );

        dot::program::Farm::store(farm);

        dot::program::Stake::store(stake);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (amount : u64)]
    pub struct UnstakeTokens<'info> {
        #[account(mut)]
        pub farm: Box<Account<'info, dot::program::Farm>>,
        #[account(mut)]
        pub stake_vault: Box<Account<'info, TokenAccount>>,
        #[account(mut)]
        pub stake: Box<Account<'info, dot::program::Stake>>,
        #[account(mut)]
        pub signer_token: Box<Account<'info, TokenAccount>>,
        #[account()]
        pub clock: Sysvar<'info, Clock>,
        #[account(mut)]
        pub protocol: Box<Account<'info, dot::program::Protocol>>,
        #[account(mut)]
        pub signer: Signer<'info>,
        pub token_program: Program<'info, Token>,
    }

    pub fn unstake_tokens(ctx: Context<UnstakeTokens>, amount: u64) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "token_program",
            ctx.accounts.token_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let farm = dot::program::Farm::load(&mut ctx.accounts.farm, &programs_map);
        let stake_vault = SeahorseAccount {
            account: &ctx.accounts.stake_vault,
            programs: &programs_map,
        };

        let stake = dot::program::Stake::load(&mut ctx.accounts.stake, &programs_map);
        let signer_token = SeahorseAccount {
            account: &ctx.accounts.signer_token,
            programs: &programs_map,
        };

        let clock = &ctx.accounts.clock.clone();
        let protocol = dot::program::Protocol::load(&mut ctx.accounts.protocol, &programs_map);
        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        unstake_tokens_handler(
            amount,
            farm.clone(),
            stake_vault.clone(),
            stake.clone(),
            signer_token.clone(),
            clock.clone(),
            protocol.clone(),
            signer.clone(),
        );

        dot::program::Farm::store(farm);

        dot::program::Stake::store(stake);

        dot::program::Protocol::store(protocol);

        return Ok(());
    }
}
