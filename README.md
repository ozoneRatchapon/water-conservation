# 💧 + ⚡ = 🟩 Greenmove Protocol

Welcome to **Greenmove**!  
Our mission: Turn your water 💧 and energy ⚡ savings into a greener 🌱, more sustainable future.

Greenmove is a Solana-powered protocol that rewards you for saving water **and** energy! 🌱💦⚡  
Connect your utility accounts, track your usage, and earn points you can redeem at local businesses.

*For this Proof of Concept, usage data is entered manually. In the future, we’ll use DePIN (Decentralized Physical Infrastructure Networks) to make your usage data accurate, transparent, and tamper-proof!*

---

## 🌟 Why Greenmove?

- 🌍 **Save the planet:** Every drop and every watt counts!
- 🛍️ **Support local:** Redeem your points at your favorite cafes, shops, and service providers.
- 🎁 **Get rewarded:** Eco-friendly habits = real-world rewards!
- 🤝 **Build community:** Join a movement for a greener, more sustainable future.

---

## 👤 Who’s Greenmove for?

- 🏠 Households in Thailand (starting with Bangkok)
- 🌏 Eco-conscious individuals
- 🏪 Local businesses (cafes, shops, etc.)
- 💧 Water authorities (MWA, PWA) & ⚡ Energy providers — potential partners!

---

## 🛠️ How Greenmove Works (Under the Hood)

Greenmove is a Solana smart contract program. Here’s how it all fits together:

### 1️⃣ Accounts

- **User Data Account:**  
  Stores your water & energy usage, points, and redemption history.
  - `owner`: Your Solana wallet
  - `water_account_id` / `energy_account_id`: Your utility company customer IDs
  - `usage_history`: List of your water & energy usage records
  - `baseline_usage`: Your average usage (for rewards)
  - `reward_token_balance`: Your Greenmove points
  - `redemption_history`: What you’ve redeemed
  - `registration_timestamp`: When you joined

- **Reward Offer Account (optional):**  
  Businesses can create offers you can redeem with points!
  - `offer_id`, `business_owner`, `description`, `point_cost`, `quantity_available`, `start_time`, `end_time`

---

### 2️⃣ What Can You Do? (Instructions)

- **ConnectExternalAccount:**  
  Link your water and/or energy account (manual input for PoC).
- **ReceiveUsage:**  
  Add new water or energy usage data (manual input for PoC).  
  👉 This also triggers baseline and reward calculations!
- **CalculateBaseline:**  
  Figure out your average usage (usually over 6 months).
- **CalculateRewards:**  
  See how much you’ve saved vs. your baseline and get points!
- **RedeemReward:**  
  Spend your points on cool stuff from local businesses.
- **Reward Offer Management (for businesses):**  
  Create, update, or delete reward offers.

---

### 3️⃣ How Rewards Work

- **Savings Calculation:**  
  We compare your latest usage (water or energy) to your baseline (average).
- **Points Calculation:**  
  The more you save, the more you earn:
  - 💯 16%+ reduction = 100 points
  - 🥈 11%+ = 50 points
  - 🥉 6%+ = 25 points
  - 👍 1%+ = 10 points
  - 😅 No reduction = 0 points

  ```rust
  // Example logic
  baseline_usage = calculate_6_month_average(usage_history);
  reduction_percentage = (baseline_usage - current_usage) * 100 / baseline_usage;

  if reduction_percentage >= 16 {
      points = 100;
  } else if reduction_percentage >= 11 {
      points = 50;
  } else if reduction_percentage >= 6 {
      points = 25;
  } else if reduction_percentage >= 1 {
      points = 10;
  } else {
      points = 0;
  }
  ```

- **Redeeming Points:**  
  When you redeem, your points go down and a redemption record is saved.

---

### 4️⃣ User Stories and Functionality Mapping

| User Story | What It Means | Smart Contract Functionality |
|------------|--------------|-----------------------------|
| Connect your utility account | Link your MWA/PWA or energy provider account for auto data (manual for PoC) | `ConnectExternalAccount` |
| See your usage & progress | Compare this month to your average | `ReceiveUsage`, `CalculateBaseline`, `CalculateRewards` |
| Earn points for saving | Get rewarded for using less water or energy | `ReceiveUsage`/`CalculateRewards` |
| Redeem points for rewards | Spend points at local businesses | `RedeemReward`, `RewardOffer` management |
| Businesses create offers | Local shops can offer rewards | `CreateRewardOffer`, `UpdateRewardOffer`, `DeleteRewardOffer` |

---

### 5️⃣ Integrations

- **Manual Input (PoC):**  
  For this proof of concept, usage data is entered manually.
- **DePIN Oracles (Switchboard) — Future:**  
  We plan to use oracles to fetch your water & energy usage data securely and transparently.
- **(Planned) MWA/PWA & Energy Provider Integration:**  
  Direct API connections for even more accurate data.
- **(Planned) Payment Processing:**  
  On-chain payments for reward redemption (e.g., SPL tokens).
- **Frontend Color Blend:**  
  Our frontend visually blends blue (💧) and yellow (⚡) to create a unique shade of green (🟩) each day—reflecting the living, growing impact of our community’s water and energy savings.

---

### 6️⃣ Security & Upgradability 🔒

- **Input validation:** No funny business with your data.
- **Access control:** Only you can update your info; only businesses manage their offers.
- **Audits:** We review our code for safety.
- **Upgradeable:**  
  Solana programs can be upgraded if you keep the upgrade authority. This means we can add new features or fix bugs, but can’t remove old features or change history.  
  (If the upgrade authority is renounced, the program becomes immutable.)

---

### 7️⃣ Roadmap

**Today:**  
✅ Proof of concept complete; tracking infrastructure established.

**Phase 1 (3-6 Months):**  
- MVP finalization with Pinocchio 
- Community partnerships secured

**Phase 2 (6-12 Months):**  
- Pilot launch with users  
- Initial points system

**Phase 3 (12-18 Months):**  
- DePIN integration starts  
- Local rewards onboarded

**Phase 4 & Beyond:**  
- Scale ecosystem  
- Tokenization planning  
- Broader adoption

---

### 8️⃣ Status

🎉 **Proof of Concept: Done!**  
(Manual input, core tracking and rewards logic working.)

---

## 📚 Resources

- [Pitch Video](https://www.youtube.com/watch?v=BWQSnwP5hBI)
- [Technical Demo Video](https://www.youtube.com/watch?v=TpkopWAHOO0)
- [Letter of Intent](https://docs.google.com/document/d/1kkMqne-SzWN5mJFg95JVARCcgG9ITbpxNLBwqhn4hz8/edit?tab=t.0)
- [User Stories](https://docs.google.com/document/d/1Dpb_j8kbXYtqELoXmkU9bv1uRtuHRKKF18civexZWqA/edit?tab=t.0)
- [Architecture Design](https://docs.google.com/document/d/17tKyNBVipTkv1_Pf0oVQGh0dJNx5lmJCYh2epf0klYY/edit?tab=t.0)
- [Details (Google Sheets)](https://docs.google.com/spreadsheets/d/1cMpozqiYWtkpydqJ7kJoDdEDw09tvufJHCQP2K3Djqs/edit?gid=920189763#gid=920189763)

---

## Award

- [Thailand Local Breakout Hackathon: 4th Prize Winner](https://youtu.be/GTnrXkePcLs?t=2222)

---

## 🧑‍💻 Deployed Program (Devnet)

The Greenmove Solana program (water & energy conservation) is deployed on devnet:

- **Program ID:** 6LVvE5CKocFdR2ozS7URNpieUego5Wu9wjM6WiERm4nS

---

Thanks for checking out Greenmove!  
Let’s save water, energy, and build a greener future together! 💧⚡🟩🌱
