import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { WaterConservation } from "../target/types/water_conservation";
import { assert } from "chai";

describe("water-conservation", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .waterConservation as Program<WaterConservation>;
  const user = provider.wallet.publicKey;

  // Derive PDAs
  const userData = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user_data"), user.toBytes()],
    program.programId,
  )[0];

  // Test data
  const propertyExternalId = "property123";
  const waterExternalId = "water123";
  const energyExternalId = "energy123";
  const waterDepinFeedAddress = anchor.web3.SystemProgram.programId;
  const energyDepinFeedAddress = anchor.web3.SystemProgram.programId;

  const propertyAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("property"), user.toBytes(), Buffer.from(propertyExternalId)],
    program.programId,
  )[0];

  const waterMeterAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("water_meter"),
      user.toBytes(),
      Buffer.from(propertyExternalId),
      Buffer.from(waterExternalId),
    ],
    program.programId,
  )[0];

  const energyMeterAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("energy_meter"),
      user.toBytes(),
      Buffer.from(propertyExternalId),
      Buffer.from(energyExternalId),
    ],
    program.programId,
  )[0];

  const rewardAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user_reward"), user.toBytes()],
    program.programId,
  )[0];

  const systemProgram = anchor.web3.SystemProgram.programId;
  const track_energy = true;
  // Test connect_depin instruction
  it("Initialize user and connect DePIN feeds", async () => {
    const tx = await program.methods
      .connectDepinFeedAddress(
        propertyExternalId,
        waterExternalId,
        energyExternalId,
        waterDepinFeedAddress,
        energyDepinFeedAddress,
        track_energy,
      )
      .accounts({
        user,
        userData,
        propertyAccount,
        waterMeterAccount,
        energyMeterAccount,
        rewardAccount,
        systemProgram,
      })
      .rpc();

    console.log("Initialize transaction signature:", tx);

    // Verify the accounts were created correctly
    const userDataAccount = await program.account.user.fetch(userData);
    assert.ok(userDataAccount.owner.equals(user));
    assert.ok(userDataAccount.propertyAccount[0].equals(propertyAccount));
    assert.ok(userDataAccount.rewardAccount.equals(rewardAccount));

    const propertyAccountData =
      await program.account.property.fetch(propertyAccount);
    assert.ok(propertyAccountData.owner.equals(user));
    assert.equal(propertyAccountData.propertyExternalId, propertyExternalId);
    assert.ok(
      propertyAccountData.waterMeterAccounts[0].equals(waterMeterAccount),
    );
    if (track_energy) {
      assert.ok(
        propertyAccountData.energyMeterAccounts[0].equals(energyMeterAccount),
      );
    }
  });

  // Test receive water usage and calculate reward
  it("Receive water usage and calculate reward", async () => {
    const usageAmount = new anchor.BN(100); // Using BN for u64 values
    const tx = await program.methods
      .receiveWaterUsage(waterExternalId, usageAmount)
      .accounts({
        owner: user,
        userData,
        property: propertyAccount,
        waterMeterAccount,
        energyMeterAccount,
        rewardAccount,
      })
      .rpc();

    console.log("Water usage transaction signature:", tx);

    // Verify the water meter account was updated
    const waterMeterData =
      await program.account.waterMeter.fetch(waterMeterAccount);
    assert.equal(
      waterMeterData.totalWaterConsumed.toString(),
      usageAmount.toString(),
    );
    assert.ok(waterMeterData.usageHistory.length > 0);

    // Verify the reward account was updated
    const rewardData = await program.account.userReward.fetch(rewardAccount);
    assert.ok(rewardData.totalRewardBalance.gt(new anchor.BN(0)));
  });

  // Test receive energy usage and calculate reward
  it("Receive energy usage and calculate reward", async () => {
    const usageAmount = new anchor.BN(50); // Using BN for u64 values
    const tx = await program.methods
      .receiveEnergyConsumption(energyExternalId, usageAmount)
      .accounts({
        owner: user,
        userData,
        property: propertyAccount,
        waterMeterAccount,
        energyMeterAccount,
        rewardAccount,
      })
      .rpc();

    console.log("Energy usage transaction signature:", tx);

    // Verify the energy meter account was updated
    const energyMeterData =
      await program.account.energyMeter.fetch(energyMeterAccount);
    assert.equal(
      energyMeterData.totalEnergyConsumed.toString(),
      usageAmount.toString(),
    );
    assert.ok(energyMeterData.consumptionHistory.length > 0);

    // Verify the reward account was updated
    const rewardData = await program.account.userReward.fetch(rewardAccount);
    assert.ok(rewardData.totalRewardBalance.gt(new anchor.BN(0)));
  });

  // Test redeem rewards
  it("Redeem rewards", async () => {
    // First get the current reward balance
    const rewardData = await program.account.userReward.fetch(rewardAccount);
    const currentBalance = rewardData.totalRewardBalance;

    // Try to redeem half of the current balance
    const redeemAmount = currentBalance.div(new anchor.BN(2));

    const tx = await program.methods
      .redeemRewards(redeemAmount)
      .accounts({
        owner: user,
        rewardAccount,
        systemProgram,
      })
      .rpc();

    console.log("Redeem rewards transaction signature:", tx);

    // Verify the reward account was updated
    const updatedRewardData =
      await program.account.userReward.fetch(rewardAccount);
    assert.equal(
      updatedRewardData.totalRewardBalance.toString(),
      currentBalance.sub(redeemAmount).toString(),
    );
    assert.ok(updatedRewardData.redemptionHistory.length > 0);
  });

  // Test redeem rewards with insufficient balance
  it("Should fail to redeem rewards with insufficient balance", async () => {
    const rewardData = await program.account.userReward.fetch(rewardAccount);
    const currentBalance = rewardData.totalRewardBalance;

    // Try to redeem more than the current balance
    const redeemAmount = currentBalance.add(new anchor.BN(1));

    try {
      await program.methods
        .redeemRewards(redeemAmount)
        .accounts({
          owner: user,
          rewardAccount,
          systemProgram,
        })
        .rpc();
      assert.fail("Expected an error but got success");
    } catch (err) {
      assert.ok(err.toString().includes("InsufficientPoints"));
    }
  });
});
