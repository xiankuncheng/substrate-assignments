#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod erc20 {
    use ink_storage::Mapping;

    #[ink(storage)]
    pub struct ERC20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        approval: Mapping<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        owner: AccountId,
        spender: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientApproval,
    }

    impl ERC20 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();

            // transfer total_supply to msg sender
            let sender = Self::env().caller();

            balances.insert(&sender, &total_supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(sender),
                value: total_supply,
            });

            Self {
                total_supply,
                balances,
                approval: Default::default(),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, who: AccountId) -> Balance {
            self.balances.get(&who).unwrap_or_default()
        }

        #[ink(message)]
        pub fn approval(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.approval.get(&(owner, spender)).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(
            &mut self,
            to: AccountId,
            value: Balance,
        ) -> core::result::Result<(), Error> {
            let from = self.env().caller();

            self.inner_transfer(from, to, value)
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            let approval = self.approval(from, caller);
            if approval < value {
                return Err(Error::InsufficientApproval);
            }

            self.inner_transfer(from, to, value)?;
            self.approval.insert((from, caller), &(approval - value));

            Ok(())
        }

        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, value: Balance) -> Result<(), Error> {
            let owner = self.env().caller();

            self.approval.insert((owner, to), &value);

            self.env().emit_event(Approval {
                owner,
                spender: to,
                value,
            });

            Ok(())
        }

        pub fn inner_transfer(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<(), Error> {
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert(&from, &(from_balance - value));

            let to_balance = self.balance_of(to);
            self.balances.insert(&to, &(to_balance + value));

            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });

            Ok(())
        }
    }
}
