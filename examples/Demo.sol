// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Demo {
    uint256 count;

    event Incremented(uint256 newCount);
    event Decremented(uint256 newCount);

    constructor(uint256 _initial) {
        count = _initial;
    }

    function increment() public {
        count++;
        emit Incremented(count);
    }

    function decrement() public {
        require(count > 0, "Count is already zero");
        count--;
        emit Decremented(count);
    }

    function getCount() public view returns (uint256) {
        return count;
    }

    function setCount(uint256 _count) public {
        count = _count;
    }
}
