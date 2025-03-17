#[starknet::contract]
mod SpecificStorageMigrationEIC {
    use starknet::SyscallResultTrait;
    use core::starknet::syscalls::storage_write_syscall;
    use core::option::OptionTrait;

    // Keys to remove (delete)
    const KEY_TO_REMOVE_1: felt252 =
        0xe8fc4f1b6b3dc661208f9a8a5017a6c059098327e31518722e0a5c3a5a7e86;
    const KEY_TO_REMOVE_2: felt252 =
        0x64cc4b710049e33feab8cea32c551afef950caf7d7ac24aada1eed439bfa571;

    // Keys to add with their values
    const NEW_KEY_1: felt252 = 0xb6ce5410fca59d078ee9b2a4371a9d684c530d697c64fbef0ae6d5e8f0ac72;
    const NEW_VALUE_1: felt252 = 0x4;

    const NEW_KEY_2: felt252 = 0x341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1;
    const NEW_VALUE_2: felt252 = 0xe;

    const NEW_KEY_3: felt252 = 0x1c789464ad40743bc8a10c1b00fb11a9c2a6fb9697600ed12f48df50a9cc740;
    const NEW_VALUE_3: felt252 = 0x5354524b;

    const NEW_KEY_4: felt252 = 0x2bd557f4ba80dfabefabe45e9b2dd35db1b9a78e96c72bc2b69b655ce47a930;
    const NEW_VALUE_4: felt252 = 0x1903ec7c4ee6a8fa0a403663b6bc4dc599c57bbe01b55ac38f119067e936ed6;

    const NEW_KEY_5: felt252 = 0x110e2f729c9c2b988559994a3daccd838cf52faf88e18101373e67dd061455a;
    const NEW_VALUE_5: felt252 = 0x8ac7230489e80000;

    const NEW_KEY_6: felt252 = 0x7b22f6af0c07a11d1e063665f52a20f2271a002ffcd7e1866dfa700d534d39b;
    const NEW_VALUE_6: felt252 = 0x8ac7230489e80000;

    const NEW_KEY_7: felt252 = 0x35b0c37f7f34be47076c6cfbcf811ad0769dbc81f4c509ee9613e0b0c648ca9;
    const NEW_VALUE_7: felt252 = 0x537461726b6e657420546f6b656e;

    #[storage]
    struct Storage {}

    /// @notice Initializes the contract with specific storage values
    /// @dev This function doesn't require any input data as all values are hardcoded
    #[external(v0)]
    fn eic_initialize(ref self: ContractState, eic_init_data: Span<felt252>) {
        // Step 1: Remove (clear) old storage keys
        self.clear_storage_key(KEY_TO_REMOVE_1);
        self.clear_storage_key(KEY_TO_REMOVE_2);

        // Step 2: Set new storage keys with their values
        self.set_storage_key(NEW_KEY_1, NEW_VALUE_1);
        self.set_storage_key(NEW_KEY_2, NEW_VALUE_2);
        self.set_storage_key(NEW_KEY_3, NEW_VALUE_3);
        self.set_storage_key(NEW_KEY_4, NEW_VALUE_4);
        self.set_storage_key(NEW_KEY_5, NEW_VALUE_5);
        self.set_storage_key(NEW_KEY_6, NEW_VALUE_6);
        self.set_storage_key(NEW_KEY_7, NEW_VALUE_7);
    }

    #[generate_trait]
    impl SpecificStorageMigrationEICImpl of SpecificStorageMigrationEICTrait {
        /// @dev Clears a specific storage key by setting its value to 0
        fn clear_storage_key(ref self: ContractState, key: felt252) {
            storage_write_syscall(0, key.try_into().unwrap(), 0).unwrap_syscall();
        }

        /// @dev Sets a storage key to a specific value
        fn set_storage_key(ref self: ContractState, key: felt252, value: felt252) {
            storage_write_syscall(0, key.try_into().unwrap(), value).unwrap_syscall();
        }
    }
}
