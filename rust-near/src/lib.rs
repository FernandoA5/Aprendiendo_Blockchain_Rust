//CONTADOR QUE INCREMENTA, DECREMENTA
//OBTENER ESTADO Y RESETEAR A 0
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contador{
    valor: i8,
}

impl Contador {
    //OBTENER CONTADOR

    //INCREMENTAR CONTADOR

    //DECREMENTAR

    //RESETEAR
}