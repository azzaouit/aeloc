// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.7;

import {AelocRunner} from "./AelocRunner.sol";

contract Runner is AelocRunner {
    uint256 public _geocode_node;
    uint256 public _reverse_geocode_node;
    uint256[5] public _bounding_box_nodes;

    constructor(address dispatcher) AelocRunner(dispatcher){}

    function _geocode_callback(uint256 node) public override {
        _geocode_node = node;
    }

    function _reverse_geocode_callback(uint256 node) public override {
        _reverse_geocode_node = node;
    }

    function _bounding_box_callback(uint256[] calldata nodes) public override {
        for(uint i = 0; i < 5; i++){
            _bounding_box_nodes[i] = nodes[i];
        }
    }
}
