use near_sdk::{env, near, AccountId};
use near_sdk::store::Vector;

#[near(serializers = [json, borsh])]
#[derive(Clone)] 
pub enum ListingKind {
    Image,
    Dataset,
    Audio,
    Other,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)] 
pub struct Listing {
    pub product_id: u64,
    pub price: u32,
    pub nova_group_id: String,
    pub owner: AccountId,
    pub purchase_number: u32,
    pub list_type: ListingKind,
    pub cid: String,
    pub is_active: bool,
    pub buyers: Vec<AccountId>,
    pub buyers_with_access: Vec<AccountId>,
}

#[near(contract_state)]
pub struct Contract {
    listings: Vector<Listing>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            listings: Vector::new(b"l"),
        }
    }
}

#[near]
impl Contract {
    pub fn create_listing(
        &mut self,
        product_id: u64,
        price: u32,
        nova_group_id: String,
        list_type: ListingKind,
        cid: String,
        gp_owner: AccountId,
    ) {
        let new_list = Listing {
            product_id,
            price,
            nova_group_id,
            owner: gp_owner,
            purchase_number: 0,
            list_type,
            cid,
            is_active: true,
            buyers: Vec::new(),
            buyers_with_access: Vec::new(), 
        };
        
        self.listings.push(new_list);
    }

    pub fn get_listings(&self) -> Vec<Listing> {
        self.listings.iter().map(|l| l.clone()).collect()
    }
    
    pub fn buy(&mut self, p_id: u64) {
        let buyer_account: AccountId = env::predecessor_account_id();
        
        for i in 0..self.listings.len() {
            if let Some(item) = self.listings.get(i) {
                if item.product_id == p_id {
                    let mut updated_item = item.clone();
                    
                    updated_item.purchase_number += 1;
                    
                    if !updated_item.buyers.contains(&buyer_account) {
                        updated_item.buyers.push(buyer_account.clone());
                    }
                    
                    self.listings.set(i, updated_item);
                    break;
                }
            }
        }
    }
    
    pub fn grant_buyer_access(&mut self, p_id: u64, buyer: AccountId) {
        let caller = env::predecessor_account_id();
        
        for i in 0..self.listings.len() {
            if let Some(item) = self.listings.get(i) {
                if item.product_id == p_id {
                    assert_eq!(
                        item.owner, caller,
                        "Only the listing owner can grant access"
                    );
                    
                    assert!(
                        item.buyers.contains(&buyer),
                        "Account has not purchased this listing"
                    );
                    
                    let mut updated_item = item.clone();
                    
                    if !updated_item.buyers_with_access.contains(&buyer) {
                        updated_item.buyers_with_access.push(buyer);
                    }
                    
                    self.listings.set(i, updated_item);
                    break;
                }
            }
        }
    }
    
    pub fn revoke_buyer_access(&mut self, p_id: u64, buyer: AccountId) {
        let caller = env::predecessor_account_id();
        
        for i in 0..self.listings.len() {
            if let Some(item) = self.listings.get(i) {
                if item.product_id == p_id {
                    assert_eq!(
                        item.owner, caller,
                        "Only the listing owner can revoke access"
                    );
                    
                    let mut updated_item = item.clone();
                    
                    updated_item.buyers_with_access.retain(|b| b != &buyer);
                    
                    self.listings.set(i, updated_item);
                    break;
                }
            }
        }
    }
    
    pub fn get_pending_access_buyers(&self, p_id: u64) -> Vec<AccountId> {
        if let Some(listing) = self.get_listing(p_id) {
            listing.buyers
                .into_iter()
                .filter(|buyer| !listing.buyers_with_access.contains(buyer))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    pub fn get_buyers_with_access(&self, p_id: u64) -> Vec<AccountId> {
        if let Some(listing) = self.get_listing(p_id) {
            listing.buyers_with_access
        } else {
            Vec::new()
        }
    }
    
    pub fn has_access(&self, p_id: u64, buyer: AccountId) -> bool {
        if let Some(listing) = self.get_listing(p_id) {
            listing.buyers_with_access.contains(&buyer)
        } else {
            false
        }
    }
    
    pub fn get_listing(&self, p_id: u64) -> Option<Listing> {
        for item in self.listings.iter() {
            if item.product_id == p_id {
                return Some(item.clone());
            }
        }
        None
    }
    
    pub fn has_purchased(&self, p_id: u64, account_id: AccountId) -> bool {
        if let Some(listing) = self.get_listing(p_id) {
            listing.buyers.contains(&account_id)
        } else {
            false
        }
    }
}
