import * as anchor from '@project-serum/anchor';
import { Program, BN } from '@project-serum/anchor';
import { Farm, IDL } from '../target/types/farm';
import {
  mintTo,
  createMint,
  getOrCreateAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
} from '@solana/spl-token';

import {
  Connection,
  SystemProgram,
  LAMPORTS_PER_SOL,
  Transaction,
  sendAndConfirmTransaction,
  PublicKey,
  Keypair,
} from '@solana/web3.js';
import { assert } from 'chai';
import lumina from '@lumina-dev/test';

lumina();

const wallet = anchor.workspace.Farm.provider.wallet.payer as Keypair;

describe('farm', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Farm as Program<Farm>;
  let stakeMint: PublicKey;
  let protocol: PublicKey;
  let farm: PublicKey;
  let rewardMintOne: PublicKey;
  let cropVault: PublicKey;
  let walletReward: PublicKey;
  let userOne: Keypair;
  let userOneProgram: Program<Farm>;
  let userOneStake: PublicKey;
  let userTwoStake: PublicKey;
  let stakeVault: PublicKey;
  let userOneStakeMint: PublicKey;
  let userOneReward: PublicKey;
  let userTwo: Keypair;
  let userTwoProgram: Program<Farm>;
  let userTwoStakeMint: PublicKey;
  let userTwoReward: PublicKey;

  it('Creates a protocol', async () => {
    // Add your test here.
    const method = program.methods.createProtocol();
    const keys = await method.pubkeys();
    protocol = keys.protocol;
    const tx = await method.rpc();
  });

  it('Creates a farm', async () => {
    stakeMint = await createMint(
      program.provider.connection,
      wallet,
      wallet.publicKey,
      wallet.publicKey,
      6,
    );

    const createFarm = await program.methods
      .createFarm()
      .accounts({
        stakeMint,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        protocol,
      })
      .rpcAndKeys();

    farm = createFarm.pubkeys.farm;
    stakeVault = createFarm.pubkeys.stakeVault;
  });

  it('Creates a crop for the farm', async () => {
    rewardMintOne = await createMint(
      program.provider.connection,
      wallet,
      wallet.publicKey,
      wallet.publicKey,
      6,
    );

    walletReward = (
      await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        wallet,
        rewardMintOne,
        wallet.publicKey,
      )
    ).address;

    await mintTo(
      program.provider.connection,
      wallet,
      rewardMintOne,
      walletReward,
      wallet,
      1000000000,
    );

    const createCropVault = await program.methods
      .createCropVault()
      .accounts({
        farm,
        protocol,
        mint: rewardMintOne,
      })
      .rpcAndKeys();

    cropVault = createCropVault.pubkeys.cropVault;
    const createCrop = await program.methods
      .addCrop(0, new BN(1000), new BN(100))
      .accounts({
        farm,
        signerReward: walletReward,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        cropVault,
      })
      .rpc();
  });

  it('Creates a single stake for the farm', async () => {
    userOne = new Keypair();
    const UserOneWallet = new anchor.Wallet(userOne);
    const provider = new anchor.AnchorProvider(
      program.provider.connection,
      UserOneWallet,
      {},
    );
    userOneProgram = new anchor.Program<Farm>(IDL, program.programId, provider);

    const airdropSignature = await program.provider.connection.requestAirdrop(
      userOne.publicKey,
      LAMPORTS_PER_SOL,
    );

    userOneReward = (
      await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        wallet,
        rewardMintOne,
        userOne.publicKey,
      )
    ).address;

    const createStake = await userOneProgram.methods
      .createStake()
      .accounts({
        farm,
      })
      .rpcAndKeys();

    userOneStake = createStake.pubkeys.stake;
  });

  it('Stakes tokens for single stake', async () => {
    userOneStakeMint = (
      await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        wallet,
        stakeMint,
        userOne.publicKey,
      )
    ).address;

    await mintTo(
      program.provider.connection,
      wallet,
      stakeMint,
      userOneStakeMint,
      wallet,
      100000000,
    );

    const stakeTokens = await userOneProgram.methods
      .stakeTokens(new BN(100000))
      .accounts({
        farm,
        stake: userOneStake,
        signerToken: userOneStakeMint,
        stakeVault,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      })
      .rpc();
  });

  it('Unstakes tokens for single stake', async () => {
    // Stake for two seconds
    await new Promise(r => setTimeout(r, 9000));
    const unstakeTokens = await userOneProgram.methods
      .unstakeTokens(new BN(100000))
      .accounts({
        farm,
        stake: userOneStake,
        signerToken: userOneStakeMint,
        stakeVault,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        protocol,
      })
      .rpc();

    const stakeBalance =
      await program.provider.connection.getTokenAccountBalance(
        userOneStakeMint,
      );

    const stakeAccount = await program.account.stake.fetch(userOneStake);

    // assert.strictEqual(0, stakeAccount.totalAmountWithdrawn[0].toNumber());
    // assert.strictEqual(stakeBalance.value.amount, '100000000');
  });

  it('Claims rewards for single stake', async () => {
    const claimRewards = await userOneProgram.methods
      .gatherRewards(0)
      .accounts({
        farm,
        cropVault,
        stake: userOneStake,
        signerReward: userOneReward,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        protocol,
      })
      .rpc();

    const rewardBalance =
      await program.provider.connection.getTokenAccountBalance(userOneReward);

    assert.isAtLeast(Number.parseFloat(rewardBalance.value.amount), 100);
  });

  it('Removes the crop', async () => {
    const removeCrop = await program.methods
      .removeCrop(0)
      .accounts({
        farm,
        cropVault,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        signerReward: walletReward,
        protocol,
      })
      .rpc();
  });

  it('Stakes to farm before crop creation with two users', async () => {
    const stakeTokens = await userOneProgram.methods
      .stakeTokens(new BN(100000))
      .accounts({
        farm,
        stake: userOneStake,
        signerToken: userOneStakeMint,
        stakeVault,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      })
      .rpc();

    userTwo = new Keypair();
    const UserTwoWallet = new anchor.Wallet(userTwo);
    const provider = new anchor.AnchorProvider(
      program.provider.connection,
      UserTwoWallet,
      {},
    );
    userTwoProgram = new anchor.Program<Farm>(IDL, program.programId, provider);

    const airdropSignature = await program.provider.connection.requestAirdrop(
      userTwo.publicKey,
      LAMPORTS_PER_SOL,
    );

    const createStakeUserTwo = await userTwoProgram.methods
      .createStake()
      .accounts({
        farm,
      })
      .rpcAndKeys();

    userTwoStake = createStakeUserTwo.pubkeys.stake;

    userTwoStakeMint = (
      await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        wallet,
        stakeMint,
        userTwo.publicKey,
      )
    ).address;

    await mintTo(
      program.provider.connection,
      wallet,
      stakeMint,
      userTwoStakeMint,
      wallet,
      100000000,
    );

    const userTwoStakeTokens = await userTwoProgram.methods
      .stakeTokens(new BN(100000))
      .accounts({
        farm,
        stake: userTwoStake,
        signerToken: userTwoStakeMint,
        stakeVault,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      })
      .rpc();

    const farmAccount = await program.account.farm.fetch(farm);
    assert.strictEqual(farmAccount.totalStakedAmount.toString(), '200000');
  });

  it('Creates a new crop at the same index with a new mint', async () => {
    rewardMintOne = await createMint(
      program.provider.connection,
      wallet,
      wallet.publicKey,
      wallet.publicKey,
      6,
    );

    walletReward = (
      await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        wallet,
        rewardMintOne,
        wallet.publicKey,
      )
    ).address;

    await mintTo(
      program.provider.connection,
      wallet,
      rewardMintOne,
      walletReward,
      wallet,
      1000000000,
    );

    const createCropVault = await program.methods
      .createCropVault()
      .accounts({
        farm,
        protocol,
        mint: rewardMintOne,
      })
      .rpcAndKeys();

    cropVault = createCropVault.pubkeys.cropVault;
    const createCrop = await program.methods
      .addCrop(0, new BN(1000), new BN(100))
      .accounts({
        farm,
        signerReward: walletReward,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        cropVault,
      })
      .rpc();
  });

  it('gathers rewards for the two stakes', async () => {
    userOneReward = (
      await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        wallet,
        rewardMintOne,
        userOne.publicKey,
      )
    ).address;

    userTwoReward = (
      await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        wallet,
        rewardMintOne,
        userTwo.publicKey,
      )
    ).address;

    await new Promise(r => setTimeout(r, 10000));

    const claimRewardsUserOne = await userOneProgram.methods
      .gatherRewards(0)
      .accounts({
        farm,
        cropVault,
        stake: userOneStake,
        signerReward: userOneReward,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        protocol,
      })
      .rpc();

    const claimRewardsUserTwo = await userTwoProgram.methods
      .gatherRewards(0)
      .accounts({
        farm,
        cropVault,
        stake: userTwoStake,
        signerReward: userTwoReward,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        protocol,
      })
      .rpc();

    const userOneRewardBalance =
      await program.provider.connection.getTokenAccountBalance(userOneReward);

    const userTwoRewardBalance =
      await program.provider.connection.getTokenAccountBalance(userTwoReward);

    assert.strictEqual(
      userOneRewardBalance.value.amount,
      userTwoRewardBalance.value.amount,
    );
    assert.strictEqual(userOneRewardBalance.value.amount, '499');
    assert.strictEqual(userTwoRewardBalance.value.amount, '499');
  });
});
