#[starknet::contract]
mod Counter {
    use starknet::get_caller_address;
    use starknet::ContractAddress;

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        CounterIncremented: CounterIncremented,
    }
    
    #[derive(Drop, starknet::Event)]
    struct CounterIncremented {
        #[key]
        user: ContractAddress,
        new_value: u128,
    }

    #[storage]
    struct Storage {
        counter: u128,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {
        self.counter.write(0);
    }

    #[external(v0)]
    fn get_counter(self: @ContractState) -> u128 {
        self.counter.read()
    }

    #[external(v0)]
    fn increment(ref self: ContractState) {
        let caller = get_caller_address();
        let current = self.counter.read();
        let new_value = current + 1;
        self.counter.write(new_value);

        self.emit(
            CounterIncremented { user: caller, new_value }
        );
    }
}