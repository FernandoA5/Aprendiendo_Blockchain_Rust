//CONTADOR QUE INCREMENTA, DECREMENTA
//OBTENER ESTADO Y RESETEAR A 0
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contador{
    valor: i32,
}
#[near_bindgen]
impl Contador {
    //OBTENER CONTADOR
    pub fn get_num(&self) -> i32{
        self.valor
    }
    //INCREMENTAR CONTADOR
    pub fn incrementar(&mut self){
        self.valor += 1;
        let log_message=format!("Incrementado el contador a {}", self.valor);
        
        env::log(log_message.as_bytes());
        despues_de_que_cambie();
    }
    //DECREMENTAR
    pub fn decrementar(&mut self){
        self.valor -= 1;
        let log_message: String = format!("Decrementado el valor a {}", self.valor);

        env::log(log_message.as_bytes());
        despues_de_que_cambie();
    }
    //RESETEAR
    pub fn resetear(&mut self){
        self.valor = 0;
        let log_message: String = format!("Se ha reseteado el contador a {}", self.valor);

        env::log(log_message.as_bytes());
        despues_de_que_cambie();
    }
}

fn despues_de_que_cambie(){
    env::log("El valor ha cambiado".as_bytes());
}

//PRUEBAS UNITARIAS

#[cfg(test)]
mod test{
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    //NECESITAMOS UNA VM DEL BLOCKCHAIN
    fn get_context(input: Vec<u8>, is_view:bool) ->VMContext{
        VMContext{
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance:0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }
    #[test]
    fn incrementar(){
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Contador{valor: 0};
        contract.incrementar();

        println!("Valor despues del incremento {}", contract.get_num());

        assert_eq!(1, contract.get_num());
    }
    #[test]
    fn decrementar(){
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract: Contador = Contador{valor: 0};
        contract.decrementar();

        println!("Valor despues del incremento {}", contract.get_num());

        assert_eq!(-1, contract.get_num());
    }
}

