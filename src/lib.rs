use near_sdk::{near, AccountId};
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
        };
        
        self.listings.push(new_list);
    }

    pub fn get_listings(&self) -> Vec<Listing> {

        self.listings.iter().map(|l| l.clone()).collect()
    }
    pub fn buy(&mut self, p_id: u64){
        for item in &mut self.listings{
            if(item.product_id==p_id){
                item.purchase_number+=1;
                break;
            }
        }
    }
}
