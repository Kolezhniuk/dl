// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;
import "hardhat/console.sol";

contract A {
    uint256 public a = 1;
    uint256 private deposited;

    constructor() {
        possibleRevert();
    }

    function deposit(uint256 amount) public virtual {
        require(amount > 0, "Amount is zero");

        deposited = amount;
    }

    function getDeposited() external view returns (uint256) {
        return deposited;
    }

    function possibleRevert() public pure virtual {
        revert("Oops");
    }
}

contract B is A {
    uint256 private b = a += 10;

    function deposit(uint256) public virtual override {
        console.log("B.deposit");
        revert("Deposit is blocked");
    }
}

contract C is A {
    uint256 private c = a *= 2;

    function deposit(uint256 amount) public virtual override {
        console.log("C.deposit");
        super.deposit(amount);
    }

    function getC() external view returns (uint256) {
        return c;
    }
}

contract D is C, B {
    constructor() {
    }

    function deposit(uint256 amount) public virtual override(B, C) {
        C.deposit(amount);
    }

    function possibleRevert() public pure override {
        // Do nothing
    }
}

contract Validator4 {
    function validate(address d_) external returns (bool) {
        console.log("Validator4.validate");
        A(d_).deposit(10);
        uint256 deposit = A(d_).getDeposited(); 

        console.log("deposit: %s", deposit);

        return A(d_).getDeposited() > 0 && C(d_).getC() == 2;
    }
}
