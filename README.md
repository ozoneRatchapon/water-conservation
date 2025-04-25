# Greenmove Protocol

## Project Overview

Greenmove is a Solana-based protocol designed to incentivize water conservation by rewarding users with points that can be redeemed at local businesses. The protocol leverages DePIN (Decentralized Physical Infrastructure Networks) to obtain accurate water usage data, creating a transparent and verifiable system for tracking and rewarding conservation efforts.

## Value Proposition

* Promotes environmental sustainability by encouraging water conservation.
* Supports local economies by driving customers to participating businesses.
* Provides tangible benefits to users for their eco-conscious behavior.
* Creates a community-driven approach to resource management.

## Target Users

* Urban and suburban households in Thailand (initially Bangkok)
* Environmentally conscious individuals in Thailand
* Local businesses (restaurants, cafes, shops, service providers) in Thailand
* Metropolitan Waterworks Authority (MWA) and Provincial Waterworks Authority (PWA) in Thailand (potential partners)

## Program Architecture (Smart Contract)

The Greenmove protocol is implemented as a Solana smart contract program. It defines the following key components:

###   1. Accounts

* **User Data Account:**
    * Stores user-specific data related to water usage, reward points, and redemption history.
    * Key Fields:
        * `owner: Pubkey`: The user's Solana wallet address.
        * `water_account_id: String`: Identifier for the user's external water account (e.g., MWA customer ID).
        * `depin_feed_address: Pubkey`: Address of the Switchboard feed providing water usage data.
        * `usage_history: Vec<UsageRecord>`: History of water usage records.
            * `UsageRecord: { timestamp: i64, usage: u64 }`
        * `baseline_usage: u64`: User's baseline water consumption.
        * `reward_token_balance: u64`: User's Greenmove point balance.
        * `redemption_history: Vec<RedemptionRecord>`: History of reward redemptions.
            * `RedemptionRecord: { timestamp: i64, reward_id: String, token_amount: u64 }`
        * `registration_timestamp: i64`: Timestamp of user registration.
* **(Optional) Reward Offer Account:**
    * Stores information about reward offers provided by local businesses.
    * Key Fields:
        * `offer_id: String`: Unique identifier for the reward offer.
        * `business_owner: Pubkey`: Solana address of the business owner.
        * `description: String`: Description of the reward.
        * `point_cost: u64`: Cost of the reward in Greenmove points.
        * `quantity_available: u64`: Number of rewards available.
        * `start_time: i64`: Timestamp when the reward offer starts.
        * `end_time: i64`: Timestamp when the reward offer ends.

###   2. Instructions

The Greenmove program provides the following instructions:

* **`ConnectDePINFeedAddress` (or `ConnectExternalAccount`):**
    * Allows users to link their external water account (e.g., MWA account) by providing their account identifier.
    * Stores the DePIN feed address in the `User Data Account`.
    * * Parameters:
            * `water_account_id: String`
            * `depin_feed_address: Pubkey`
* **`ReceiveWaterUsage`:**
    * Receives water usage data from the designated DePIN feed.
    * Updates the `usage_history` in the `User Data Account`.
    * Triggers the `CalculateBaseline` and `CalculateRewards` logic.
    * * Parameters:
            * `usage_data: Vec<UsageRecord>` (or individual UsageRecord)
* **`CalculateBaseline`:**
    * Calculates the user's baseline water consumption based on their `usage_history`.
    * Updates the `baseline_usage` in the `User Data Account`.
    * * Parameters: (Potentially none, or a window of history to use)
            * `history_window: u64` (e.g., number of months to average)
* **`CalculateRewards` (or part of `ReceiveWaterUsage`):**
    * Calculates the user's water savings by comparing current usage with the `baseline_usage`.
    * Determines the number of Greenmove points to award based on predefined rules.
    * Updates the `reward_token_balance` in the `User Data Account`.
    * * Parameters:
            * `current_usage: u64`
* **`RedeemReward`:**
    * Allows users to redeem their Greenmove points for rewards.
    * Deducts the reward cost from the `reward_token_balance`.
    * Creates a `RedemptionRecord` and adds it to the `redemption_history`.
    * * Parameters:
            * `reward_id: String`
            * `token_amount: u64`
* **(Optional) Reward Offer Management Instructions:**
    * **`CreateRewardOffer`:** Allows businesses to create new reward offers.
        * * Parameters:
                * `offer_id: String`
                * `description: String`
                * `point_cost: u64`
                * `quantity_available: u64`
                * `start_time: i64`
                * `end_time: i64`
    * **`UpdateRewardOffer`:** Allows businesses to modify existing reward offers.
        * * Parameters: (Varies based on what can be updated)
                * `offer_id: String`
                * `new_description: String` (Optional)
                * `new_point_cost: u64` (Optional)
                * ...
    * **`DeleteRewardOffer`:** Allows businesses to remove reward offers.
        * * Parameters:
                * `offer_id: String`

###   3. Program Logic

The Greenmove program implements the following core logic:

* **Data Validation:**
    * Validates input data (e.g., account identifiers, reward IDs).
    * Ensures sufficient point balance before redemption.
    * (Optional) Implements access control for reward offer management.
* **Water Savings Calculation:**
    * Calculates the difference between the user's current water usage and their baseline usage.
    * * Example: `savings = baseline_usage.saturating_sub(current_usage)`
* **Reward Point Calculation:**
* Determines the number of points to award based on the calculated water savings.
    * The protocol supports different reward calculation methods:

        * **1. Standard:**
            * **1.1 Use a Shorter-Term Average:** The baseline usage is calculated as the average water consumption over the previous 6 months.
            * The points are awarded based on the percentage reduction in water consumption compared to the baseline:
                * If the reduction is 16% or more, award 100 points.
                * If the reduction is 11% or more, award 50 points.
                * If the reduction is 6% or more, award 25 points.
                * If the reduction is 1% or more, award 10 points.
                * If none of the above conditions are met (i.e., no reduction or increased consumption), award 0 points.
            * * Example:
                ```
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

        * **2. Campaign:**
            * (TBD)

        * **3. Tournament:**
            * (TBD)

* **Reward Redemption Logic:**
    * Handles the deduction of points and the recording of redemption events.
    * Example:
        ```
        user_data_account.reward_token_balance -= token_amount;
        ```

###   4. User Stories and Functionality Mapping

|   User Story ID   |   User Story   |   Smart Contract Functionality   |
| :---------------- | :------------- | :------------------------------- |
|   GM-001a         |   As a homeowner, I want to securely connect my MWA account using my customer ID so that my monthly water usage is automatically fetched and displayed in the app.   |   `ConnectDePINFeedAddress` (stores `water_account_id`)   |
|   GM-001b         |   As a homeowner, I want to see a clear comparison of my current month's water usage to my average usage or a community average so that I can understand my conservation progress.   |   `ReceiveWaterUsage` (stores `usage_history`), `CalculateBaseline` (stores `baseline_usage`), `CalculateRewards` (calculates savings)   |
|   GM-002a         |   As a homeowner, I want to automatically earn Greenmove points each month based on the amount of water I save so that I am incentivized to continue conserving.   |   `ReceiveWaterUsage` (or `CalculateRewards`) (calculates and updates `reward_token_balance`)   |
|   GM-003a         |   As a Greenmove user, I want to browse a marketplace of local businesses offering rewards that I can redeem with my Greenmove points so that I can support my community and enjoy the benefits of my conservation efforts.   |   `RedeemReward` (deducts points, records redemption), (Optional: `RewardOffer` accounts and management instructions for marketplace functionality)   |
|   GM-004a         |   As a local cafe owner, I want to create and manage reward offers (e.g., discounts) for Greenmove users based on their point balance so that I can incentivize sustainable behavior and attract new customers.   |   (Optional: `CreateRewardOffer`, `UpdateRewardOffer`, `DeleteRewardOffer` instructions and `RewardOffer` accounts)   |

###   5. External Integrations

* **DePIN Oracles (Switchboard):**
    * The Greenmove protocol relies on DePIN oracles like Switchboard to provide accurate and reliable water usage data. This ensures transparency and reduces the risk of data manipulation.
* **(Potential) MWA/PWA Integration:**
    * Direct integration with the Metropolitan Waterworks Authority (MWA) and/or Provincial Waterworks Authority (PWA) could streamline the process of fetching water usage data and improve data accuracy. This would likely involve APIs provided by these authorities.
* **(Potential) Payment Processing:**
    * If reward redemption involves any on-chain transactions (e.g., transferring SPL tokens), integration with a payment processing service might be necessary.

###   6. Security Considerations

* **Data Validation:**
    * Robust input validation is crucial to prevent malicious attacks and ensure data integrity.
* **Access Control:**
    * Appropriate access control mechanisms should be implemented to restrict access to sensitive functions (e.g., reward offer management).
* **Security Audits:**
    * Regular security audits are essential to identify and address potential vulnerabilities in the smart contract code.
* **Immutable Code:**
    * Once deployed, the smart contract code should be immutable to prevent unauthorized modifications.

###   7. Future Enhancements

* **Dynamic Reward Systems:**
    * Implement more sophisticated reward systems that adapt to changing water conservation goals or user behavior.
* **Gamification:**
    * Incorporate gamification elements to further incentivize water conservation (e.g., badges, leaderboards).
* **Community Features:**
    * Add features that foster a sense of community among users (e.g., forums, social sharing).
* **Expansion to Other Utilities:**
    * Extend the protocol to incentivize the conservation of other utilities, such as electricity or gas.

###   8. Development Status
// in progress

### Additional Resources

* Project Planning and Details: [Greenmove Project Details](https://docs.google.com/spreadsheets/d/1cMpozqiYWtkpydqJ7kJoDdEDw09tvufJHCQP2K3Djqs/edit?gid=920189763#gid=920189763)
