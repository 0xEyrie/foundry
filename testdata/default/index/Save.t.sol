// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "../../cheats/Vm.sol";

contract SaveTest{
    address constant HEVM_ADDRESS = address(bytes20(uint160(uint256(keccak256("hevm cheat code")))));
    Vm constant vm = Vm(HEVM_ADDRESS);

    address caller = 0x0000000000000000000000000000000000000001;
    address sender = 0x0000000000000000000000000000000000000002;

    Vm.UniswapSend data = Vm.UniswapSend(Vm.Indexed.address_indexed, caller);

    function testSave() public {
        vm.save(data);
    }
}
